//! convertColors plugin — converts colors: rgb() to #rrggbb, names to hex, long to short hex.
//!
//! Ported from SVGO's plugins/convertColors.js

use crate::collections::{COLORS_NAMES, COLORS_PROPS, COLORS_SHORT_NAMES};
use svgo_core::ast::Element;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct ConvertColors;

impl Plugin for ConvertColors {
    fn name(&self) -> &'static str {
        "convertColors"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, params: &serde_json::Value) {
        let current_color = params
            .get("currentColor")
            .cloned()
            .unwrap_or(serde_json::json!(false));
        let names2hex = params
            .get("names2hex")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let rgb2hex = params
            .get("rgb2hex")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let convert_case = params
            .get("convertCase")
            .and_then(|v| v.as_str())
            .unwrap_or("lower");
        let shorthex = params
            .get("shorthex")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let shortname = params
            .get("shortname")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut v = ConvertColorsVisitor {
            current_color,
            names2hex,
            rgb2hex,
            convert_case: convert_case.to_string(),
            shorthex,
            shortname,
            mask_counter: 0,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct ConvertColorsVisitor {
    current_color: serde_json::Value,
    names2hex: bool,
    rgb2hex: bool,
    convert_case: String,
    shorthex: bool,
    shortname: bool,
    mask_counter: i32,
}

impl ConvertColorsVisitor {
    fn convert_rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }

    fn includes_url_reference(val: &str) -> bool {
        val.contains("url(")
    }

    fn includes_css_var_reference(val: &str) -> bool {
        val.contains("var(") && val.contains("--")
    }
}

impl Visitor for ConvertColorsVisitor {
    fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
        if el.name == "mask" {
            self.mask_counter += 1;
        }

        let keys: Vec<String> = el.attributes.keys().cloned().collect();
        for name in &keys {
            if !COLORS_PROPS.contains(name.as_str()) {
                continue;
            }
            let mut val = el.attributes.get(name).unwrap().clone();

            // Convert colors to currentColor
            if self.current_color != serde_json::json!(false) && self.mask_counter == 0 {
                let matched = match &self.current_color {
                    serde_json::Value::String(s) => val == *s,
                    serde_json::Value::Bool(b) => *b && val != "none",
                    _ => false,
                };
                if matched {
                    val = "currentColor".to_string();
                }
            }

            // Convert color name keyword to long hex
            if self.names2hex {
                let lower = val.to_lowercase();
                if let Some(&hex) = COLORS_NAMES.get(lower.as_str()) {
                    val = hex.to_string();
                }
            }

            // Convert rgb() to long hex
            if self.rgb2hex {
                if let Some(hex) = parse_rgb(&val) {
                    val = hex;
                }
            }

            // Convert case
            if !self.convert_case.is_empty()
                && !Self::includes_url_reference(&val)
                && !Self::includes_css_var_reference(&val)
                && val != "currentColor"
            {
                if self.convert_case == "lower" {
                    val = val.to_lowercase();
                } else if self.convert_case == "upper" {
                    val = val.to_uppercase();
                }
            }

            // Convert long hex to short hex
            if self.shorthex {
                if val.len() == 7 && val.starts_with('#') {
                    let bytes = val.as_bytes();
                    if bytes[1] == bytes[2] && bytes[3] == bytes[4] && bytes[5] == bytes[6] {
                        val = format!(
                            "#{}{}{}",
                            bytes[1] as char, bytes[3] as char, bytes[5] as char
                        );
                    }
                }
            }

            // Convert hex to short name
            if self.shortname {
                let lower = val.to_lowercase();
                if let Some(&name) = COLORS_SHORT_NAMES.get(lower.as_str()) {
                    val = name.to_string();
                }
            }

            el.attributes.insert(name.clone(), val);
        }

        VisitAction::Continue
    }

    fn element_exit(&mut self, el: &mut Element, _ctx: &Context) {
        if el.name == "mask" {
            self.mask_counter -= 1;
        }
    }
}

/// Parse rgb(r, g, b) and return hex string.
fn parse_rgb(input: &str) -> Option<String> {
    let input = input.trim();
    if !input.to_lowercase().starts_with("rgb(") || !input.ends_with(')') {
        return None;
    }

    let inner = &input[4..input.len() - 1];
    // Split by comma or whitespace
    let parts: Vec<&str> = inner
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.len() != 3 {
        return None;
    }

    let mut rgb: [u8; 3] = [0; 3];
    for (i, part) in parts.iter().enumerate() {
        let part = part.trim();
        let (num, is_pct) = if part.ends_with('%') {
            (&part[..part.len() - 1], true)
        } else {
            (part, false)
        };

        let n: f64 = num.parse().ok()?;
        rgb[i] = if is_pct {
            ((n / 100.0) * 255.0).round() as u8
        } else {
            n as u8
        };
    }

    Some(ConvertColorsVisitor::convert_rgb_to_hex(
        rgb[0], rgb[1], rgb[2],
    ))
}

#[allow(dead_code)]
fn find_element<'a>(
    doc: &'a svgo_core::ast::Document,
    name: &str,
) -> Option<&'a svgo_core::ast::Element> {
    fn find_in_children<'a>(
        children: &'a [svgo_core::ast::Node],
        name: &str,
    ) -> Option<&'a svgo_core::ast::Element> {
        for child in children {
            if let svgo_core::ast::Node::Element(el) = child {
                if el.name == name {
                    return Some(el);
                }
                if let Some(found) = find_in_children(&el.children, name) {
                    return Some(found);
                }
            }
        }
        None
    }
    find_in_children(&doc.children, name)
}

#[allow(dead_code)]
#[allow(dead_code)]
fn find_element_mut<'a>(
    doc: &'a mut svgo_core::ast::Document,
    name: &str,
) -> Option<&'a mut svgo_core::ast::Element> {
    fn find_in_children<'a>(
        children: &'a mut [svgo_core::ast::Node],
        name: &str,
    ) -> Option<&'a mut svgo_core::ast::Element> {
        for child in children.iter_mut() {
            if let svgo_core::ast::Node::Element(el) = child {
                if el.name == name {
                    return Some(el);
                }
                if let Some(found) = find_in_children(&mut el.children, name) {
                    return Some(found);
                }
            }
        }
        None
    }
    find_in_children(&mut doc.children, name)
}
#[cfg(test)]
mod tests {
    use super::*;
    use svgo_core::parse;

    #[test]
    fn convert_colors_rgb_to_hex() {
        let plug = ConvertColors;
        let input =
            "<svg xmlns=\"http://www.w3.org/2000/svg\"><path fill=\"rgb(255, 0, 255)\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("fill").unwrap(), "#f0f");
    }

    #[test]
    fn convert_colors_name_to_hex() {
        let plug = ConvertColors;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path fill=\"red\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("fill").unwrap(), "red");
    }

    #[test]
    fn convert_colors_long_to_short_hex() {
        let plug = ConvertColors;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path fill=\"#aabbcc\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("fill").unwrap(), "#abc");
    }

    #[test]
    fn convert_colors_short_name() {
        let plug = ConvertColors;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path fill=\"#ff0000\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        // #ff0000 -> short hex #f00 -> short name "red"
        assert_eq!(path.attributes.get("fill").unwrap(), "red");
    }
}
