use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct MinifyStyles;

impl Plugin for MinifyStyles {
    fn name(&self) -> &'static str {
        "minifyStyles"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct StyleCollector {
            style_elements: Vec<*mut Element>,
            style_attrs: Vec<*mut Element>,
        }

        impl Visitor for StyleCollector {
            fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
                if el.name == "style" {
                    if !el.children.is_empty() {
                        self.style_elements.push(el as *mut Element);
                    }
                } else if let Some(style_val) = el.attr("style") {
                    if !style_val.is_empty() {
                        self.style_attrs.push(el as *mut Element);
                    }
                }
                VisitAction::Continue
            }
        }

        let mut collector = StyleCollector {
            style_elements: Vec::new(),
            style_attrs: Vec::new(),
        };
        svgo_core::visitor::visit(doc, &mut collector);

        for &ptr in &collector.style_elements {
            unsafe {
                let el = &mut *ptr;
                if let Some(first_child) = el.children.first() {
                    let css_text = match first_child {
                        Node::Text(t) => t.clone(),
                        Node::CData(c) => c.clone(),
                        _ => continue,
                    };
                    let mut stylesheet =
                        match StyleSheet::parse(&css_text, ParserOptions::default()) {
                            Ok(s) => s,
                            Err(_) => continue,
                        };
                    stylesheet.minify(MinifyOptions::default()).unwrap();
                    let result = stylesheet.to_css(PrinterOptions::default()).unwrap();
                    let minified = result.code;
                    if minified.is_empty() {
                        el.children.clear();
                    } else if css_text.contains('>') || css_text.contains('<') {
                        el.children.clear();
                        el.children.push(Node::CData(minified));
                    } else {
                        el.children.clear();
                        el.children.push(Node::Text(minified));
                    }
                }
            }
        }

        for &ptr in &collector.style_attrs {
            unsafe {
                let el = &mut *ptr;
                if let Some(style_val) = el.attr("style") {
                    let css_text = style_val.to_string();
                    let mut stylesheet =
                        match StyleSheet::parse(&css_text, ParserOptions::default()) {
                            Ok(s) => s,
                            Err(_) => continue,
                        };
                    stylesheet.minify(MinifyOptions::default()).unwrap();
                    let result = stylesheet.to_css(PrinterOptions::default()).unwrap();
                    el.set_attr("style", result.code.trim());
                }
            }
        }

        remove_empty_styles(doc);
    }
}

fn remove_empty_styles(doc: &mut Document) {
    fn remove_from_children(children: &mut Vec<Node>) {
        let mut i = 0;
        while i < children.len() {
            match &mut children[i] {
                Node::Element(el) => {
                    if el.name == "style" && el.children.is_empty() {
                        children.remove(i);
                        continue;
                    }
                    remove_from_children(&mut el.children);
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }
    }
    remove_from_children(&mut doc.children);
}
