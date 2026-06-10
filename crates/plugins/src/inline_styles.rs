use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct InlineStyles;

impl Plugin for InlineStyles {
    fn name(&self) -> &'static str {
        "inlineStyles"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let only_matched_once = params
            .get("onlyMatchedOnce")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let _remove_matched = params
            .get("removeMatchedSelectors")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        // Phase 1: collect all <style> elements and their CSS text
        struct StyleCollector {
            styles: Vec<(*mut Element, String)>,
        }

        impl Visitor for StyleCollector {
            fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
                if el.name == "foreignObject" {
                    return VisitAction::SkipChildren;
                }
                if el.name != "style" || el.children.is_empty() {
                    return VisitAction::Continue;
                }
                if let Some(t) = el.attr("type") {
                    if !t.is_empty() && t != "text/css" {
                        return VisitAction::Continue;
                    }
                }
                let mut css = String::new();
                for child in &el.children {
                    if let Node::Text(t) = child {
                        css.push_str(t);
                    }
                    if let Node::CData(c) = child {
                        css.push_str(c);
                    }
                }
                if !css.trim().is_empty() {
                    self.styles.push((el as *mut Element, css));
                }
                VisitAction::Continue
            }
        }

        let mut collector = StyleCollector { styles: Vec::new() };
        svgo_core::visitor::visit(doc, &mut collector);

        if collector.styles.is_empty() {
            return;
        }

        // Collect all element pointers in the document
        let mut all_elements: Vec<*mut Element> = Vec::new();
        collect_all_elements(doc, &mut all_elements);

        // Phase 2: for each style element, parse CSS and inline matching rules
        for (ptr, css_text) in &collector.styles {
            unsafe {
                let _style_el = &mut **ptr;
                let stylesheet = match StyleSheet::parse(css_text, ParserOptions::default()) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                let rules = extract_rules(&stylesheet);
                if rules.is_empty() {
                    continue;
                }

                for rule in &rules {
                    let mut matched: Vec<*mut Element> = Vec::new();
                    for &el_ptr in &all_elements {
                        let el = &*el_ptr;
                        if selector_matches_any(&rule.selectors, el) {
                            matched.push(el_ptr);
                        }
                    }

                    if matched.is_empty() {
                        continue;
                    }
                    if only_matched_once && matched.len() > 1 {
                        continue;
                    }

                    for &el_ptr in &matched {
                        let el = &mut *el_ptr;
                        for (prop, val) in &rule.declarations {
                            let existing = el.attr("style").unwrap_or("");
                            let merged = merge_inline_style(existing, prop, val);
                            el.set_attr("style", &merged);
                        }
                    }
                }
            }
        }
    }
}

struct CssRule {
    selectors: Vec<SimpleSelector>,
    declarations: Vec<(String, String)>,
}

struct SimpleSelector {
    selector_type: SelectorType,
    value: String,
}

enum SelectorType {
    Type,
    Class,
    Id,
}

fn extract_rules(stylesheet: &StyleSheet) -> Vec<CssRule> {
    use lightningcss::properties::Property;
    use lightningcss::rules::CssRule as LcRule;
    use lightningcss::selector::Selector;
    use lightningcss::traits::ToCss;

    let mut rules = Vec::new();

    for rule in stylesheet.rules.0.iter() {
        if let LcRule::Style(style_rule) = rule {
            let mut selector_text = String::new();
            for sel in style_rule.selectors.0.iter() {
                let mut printer = lightningcss::printer::Printer::new(
                    &mut selector_text,
                    PrinterOptions {
                        minify: true,
                        ..Default::default()
                    },
                );
                Selector::to_css(sel, &mut printer).ok();
            }

            let simple_selectors = parse_simple_selectors(&selector_text);
            if simple_selectors.is_empty() {
                continue;
            }

            let mut declarations = Vec::new();
            for prop in style_rule.declarations.declarations.iter() {
                let mut name_s = String::new();
                let mut printer = lightningcss::printer::Printer::new(
                    &mut name_s,
                    PrinterOptions {
                        minify: true,
                        ..Default::default()
                    },
                );
                prop.property_id().to_css(&mut printer).ok();
                let prop_name = name_s.trim_end_matches(':').to_string();
                let prop_value = prop
                    .to_css_string(
                        false,
                        PrinterOptions {
                            minify: true,
                            ..Default::default()
                        },
                    )
                    .unwrap_or_default();
                declarations.push((prop_name, prop_value));
            }

            if declarations.is_empty() {
                continue;
            }

            rules.push(CssRule {
                selectors: simple_selectors,
                declarations,
            });
        }
    }

    rules
}

fn parse_simple_selectors(selector_text: &str) -> Vec<SimpleSelector> {
    let mut result = Vec::new();

    // Split by comma for selector list
    for part in selector_text.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        // Parse simple selectors from the compound selector
        // Handle: type, .class, #id (and combinations like div.class#id)
        let mut chars = part.chars().peekable();
        let mut current_token = String::new();

        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    // Flush current token as type selector
                    if !current_token.is_empty() {
                        result.push(SimpleSelector {
                            selector_type: SelectorType::Type,
                            value: current_token.clone(),
                        });
                        current_token.clear();
                    }
                    // Read class name
                    while let Some(c) = chars.peek() {
                        if c.is_alphanumeric() || *c == '-' || *c == '_' {
                            current_token.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if !current_token.is_empty() {
                        result.push(SimpleSelector {
                            selector_type: SelectorType::Class,
                            value: current_token.clone(),
                        });
                        current_token.clear();
                    }
                }
                '#' => {
                    // Flush current token as type selector
                    if !current_token.is_empty() {
                        result.push(SimpleSelector {
                            selector_type: SelectorType::Type,
                            value: current_token.clone(),
                        });
                        current_token.clear();
                    }
                    // Read id
                    while let Some(c) = chars.peek() {
                        if c.is_alphanumeric() || *c == '-' || *c == '_' {
                            current_token.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    if !current_token.is_empty() {
                        result.push(SimpleSelector {
                            selector_type: SelectorType::Id,
                            value: current_token.clone(),
                        });
                        current_token.clear();
                    }
                }
                c if c.is_alphanumeric() || c == '_' => {
                    current_token.push(c);
                    while let Some(c) = chars.peek() {
                        if c.is_alphanumeric() || *c == '-' || *c == '_' {
                            current_token.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // This is a type selector (element name)
                    // Only add if it's not followed by . or #
                    result.push(SimpleSelector {
                        selector_type: SelectorType::Type,
                        value: current_token.clone(),
                    });
                    current_token.clear();
                }
                _ => {
                    // Skip combinators, spaces, etc.
                    current_token.clear();
                }
            }
        }

        if !current_token.is_empty() {
            result.push(SimpleSelector {
                selector_type: SelectorType::Type,
                value: current_token,
            });
        }
    }

    result
}

fn selector_matches_any(selectors: &[SimpleSelector], el: &Element) -> bool {
    for sel in selectors {
        match sel.selector_type {
            SelectorType::Type => {
                if el.name != sel.value {
                    return false;
                }
            }
            SelectorType::Class => {
                let class_attr = el.attr("class").unwrap_or("");
                let classes: Vec<&str> = class_attr.split_whitespace().collect();
                if !classes.contains(&sel.value.as_str()) {
                    return false;
                }
            }
            SelectorType::Id => {
                if el.attr("id").map_or(true, |id| id != sel.value) {
                    return false;
                }
            }
        }
    }
    true
}

fn collect_all_elements(doc: &mut Document, result: &mut Vec<*mut Element>) {
    for child in doc.children.iter_mut() {
        if let Node::Element(el) = child {
            collect_from_element(el, result);
        }
    }
}

fn collect_from_element(el: &mut Element, result: &mut Vec<*mut Element>) {
    result.push(el as *mut Element);
    for child in el.children.iter_mut() {
        if let Node::Element(child_el) = child {
            collect_from_element(child_el, result);
        }
    }
}

fn merge_inline_style(existing: &str, prop: &str, val: &str) -> String {
    let mut props: Vec<(String, String)> = Vec::new();
    let mut found = false;

    for part in existing.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(colon_pos) = part.find(':') {
            let p = part[..colon_pos].trim().to_lowercase();
            let v = part[colon_pos + 1..].trim().to_string();
            if p == prop.to_lowercase() {
                props.push((p, val.to_string()));
                found = true;
            } else {
                props.push((p, v));
            }
        }
    }

    if !found {
        props.push((prop.to_string(), val.to_string()));
    }

    props
        .iter()
        .map(|(p, v)| format!("{}: {}", p, v))
        .collect::<Vec<_>>()
        .join("; ")
}
