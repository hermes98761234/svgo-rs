//! cleanupNumericValues plugin — rounds numeric values, strips leading zeros, removes px.
//!
//! Ported from SVGO's plugins/cleanupNumericValues.js

use std::collections::HashMap;
use svgo_core::ast::Element;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct CleanupNumericValues;

impl Plugin for CleanupNumericValues {
    fn name(&self) -> &'static str {
        "cleanupNumericValues"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, params: &serde_json::Value) {
        let float_precision = params
            .get("floatPrecision")
            .and_then(|v| v.as_u64())
            .unwrap_or(3) as u32;
        let leading_zero = params
            .get("leadingZero")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let default_px = params
            .get("defaultPx")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let convert_to_px = params
            .get("convertToPx")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut v = CleanupNumericValuesVisitor {
            float_precision,
            leading_zero,
            default_px,
            convert_to_px,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct CleanupNumericValuesVisitor {
    float_precision: u32,
    leading_zero: bool,
    default_px: bool,
    convert_to_px: bool,
}

impl CleanupNumericValuesVisitor {
    fn round(&self, num: f64) -> f64 {
        let pow = 10f64.powi(self.float_precision as i32);
        (num * pow).round() / pow
    }

    fn remove_leading_zero(&self, num: f64) -> String {
        let s = num.to_string();
        if num > 0.0 && num < 1.0 && s.starts_with("0") {
            s[1..].to_string()
        } else if num < 0.0 && num > -1.0 {
            // "-0.xxx" -> "-.xxx"
            let mut result = String::with_capacity(s.len());
            result.push('-');
            result.push_str(&s[2..]);
            result
        } else {
            s
        }
    }
}

impl Visitor for CleanupNumericValuesVisitor {
    fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
        let absolute_lengths: HashMap<&str, f64> = [
            ("cm", 96.0 / 2.54),
            ("mm", 96.0 / 25.4),
            ("in", 96.0),
            ("pt", 4.0 / 3.0),
            ("pc", 16.0),
            ("px", 1.0),
        ]
        .iter()
        .cloned()
        .collect();

        // Handle viewBox specially
        if let Some(viewbox) = el.attributes.get("viewBox") {
            let numbers: Vec<&str> = viewbox
                .split(|c: char| c.is_whitespace() || c == ',')
                .filter(|s| !s.is_empty())
                .collect();
            let processed: Vec<String> = numbers
                .iter()
                .map(|s| {
                    s.parse::<f64>()
                        .ok()
                        .map(|n| self.round(n).to_string())
                        .unwrap_or_else(|| s.to_string())
                })
                .collect();
            el.attributes
                .insert("viewBox".to_string(), processed.join(" "));
        }

        let keys: Vec<String> = el.attributes.keys().cloned().collect();
        for name in keys {
            if name == "version" {
                continue;
            }
            let value = el.attributes.get(&name).unwrap().clone();

            // Parse numeric value: number + optional unit
            let trimmed = value.trim();
            // Match: optional sign, digits, optional decimal, optional exponent, optional unit
            if let Some(caps) = parse_numeric(trimmed) {
                let num_str = &caps[0];
                let unit_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");

                if let Ok(num) = num_str.parse::<f64>() {
                    let mut num = self.round(num);
                    let mut unit = unit_str.to_string();

                    // Convert absolute values to pixels
                    if self.convert_to_px && !unit.is_empty() {
                        if let Some(&factor) = absolute_lengths.get(unit.as_str()) {
                            let px_num = self.round(factor * num_str.parse::<f64>().unwrap());
                            if px_num.to_string().len() < trimmed.len() {
                                num = px_num;
                                unit = "px".to_string();
                            }
                        }
                    }

                    // Remove leading zero
                    let num_str = if self.leading_zero {
                        self.remove_leading_zero(num)
                    } else {
                        num.to_string()
                    };

                    // Remove default px
                    if self.default_px && unit == "px" {
                        unit = String::new();
                    }

                    el.attributes.insert(name, format!("{}{}", num_str, unit));
                }
            }
        }
        VisitAction::Continue
    }
}

/// Parse a numeric value with optional unit suffix.
/// Returns Some(captures) if the value matches, None otherwise.
fn parse_numeric(input: &str) -> Option<Vec<String>> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    // Match: [-+]?digits.digits?([eE][-+]?digits)?(px|pt|pc|mm|cm|m|in|ft|em|ex|%)?
    let mut pos = 0;
    let chars: Vec<char> = input.chars().collect();

    // Optional sign
    if pos < chars.len() && (chars[pos] == '+' || chars[pos] == '-') {
        pos += 1;
    }

    // Digits before decimal
    let has_digits_before = pos < chars.len() && chars[pos].is_ascii_digit();
    while pos < chars.len() && chars[pos].is_ascii_digit() {
        pos += 1;
    }

    // Decimal point and digits after
    let has_dot = pos < chars.len() && chars[pos] == '.';
    if has_dot {
        pos += 1;
        while pos < chars.len() && chars[pos].is_ascii_digit() {
            pos += 1;
        }
    }

    // Must have at least some digits
    if !has_digits_before && !has_dot {
        return None;
    }
    if has_dot && pos == 0 {
        return None;
    }

    // Check we have at least one digit somewhere
    let has_any_digit = input.chars().take(pos).any(|c| c.is_ascii_digit());
    if !has_any_digit {
        return None;
    }

    // Optional exponent
    if pos < chars.len() && (chars[pos] == 'e' || chars[pos] == 'E') {
        let exp_pos = pos;
        pos += 1;
        if pos < chars.len() && (chars[pos] == '+' || chars[pos] == '-') {
            pos += 1;
        }
        let has_exp_digits = pos < chars.len() && chars[pos].is_ascii_digit();
        if !has_exp_digits {
            // Not a valid exponent, backtrack
            pos = exp_pos;
        } else {
            while pos < chars.len() && chars[pos].is_ascii_digit() {
                pos += 1;
            }
        }
    }

    let num_part = &input[..pos];
    let rest = &input[pos..];

    // Optional unit
    let units = [
        "px", "pt", "pc", "mm", "cm", "m", "in", "ft", "em", "ex", "%",
    ];
    let mut unit_part = "";
    for u in &units {
        if rest == *u {
            unit_part = u;
            break;
        }
    }

    // If there's remaining text that's not a unit, it's not a pure numeric value
    if !rest.is_empty() && unit_part.is_empty() {
        return None;
    }

    Some(vec![num_part.to_string(), unit_part.to_string()])
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
    fn cleanup_numeric_values_rounds() {
        let plug = CleanupNumericValues;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 10.123456 20.789\"><path d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let svg = find_element(&doc, "svg").unwrap();
        assert_eq!(svg.attributes.get("viewBox").unwrap(), "0 0 10.123 20.789");
    }

    #[test]
    fn cleanup_numeric_values_strips_leading_zero() {
        let plug = CleanupNumericValues;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path opacity=\"0.5\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("opacity").unwrap(), ".5");
    }

    #[test]
    fn cleanup_numeric_values_removes_px() {
        let plug = CleanupNumericValues;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100px\" height=\"200px\"><path d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let svg = find_element(&doc, "svg").unwrap();
        assert_eq!(svg.attributes.get("width").unwrap(), "100");
        assert_eq!(svg.attributes.get("height").unwrap(), "200");
    }

    #[test]
    fn cleanup_numeric_values_preserves_version() {
        let plug = CleanupNumericValues;
        let input =
            "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\"><path d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let svg = find_element(&doc, "svg").unwrap();
        assert_eq!(svg.attributes.get("version").unwrap(), "1.1");
    }
}
