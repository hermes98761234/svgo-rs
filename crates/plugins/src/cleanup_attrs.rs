//! cleanupAttrs plugin — cleanups attributes from newlines, trailing and repeating spaces.
//!
//! Ported from SVGO's plugins/cleanupAttrs.js

use svgo_core::ast::Element;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct CleanupAttrs;

impl Plugin for CleanupAttrs {
    fn name(&self) -> &'static str {
        "cleanupAttrs"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, params: &serde_json::Value) {
        let newlines = params
            .get("newlines")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let trim = params.get("trim").and_then(|v| v.as_bool()).unwrap_or(true);
        let spaces = params
            .get("spaces")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut v = CleanupAttrsVisitor {
            newlines,
            trim,
            spaces,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct CleanupAttrsVisitor {
    newlines: bool,
    trim: bool,
    spaces: bool,
}

impl Visitor for CleanupAttrsVisitor {
    fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
        for value in el.attributes.values_mut() {
            if self.newlines {
                // Replace newlines that need a space instead
                let mut result = String::with_capacity(value.len());
                let mut chars = value.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '\r' && chars.peek() == Some(&'\n') {
                        // \r\n: check if surrounded by non-whitespace
                        // We need to look back at the last char in result
                        if let Some(last) = result.chars().last() {
                            if !last.is_whitespace() {
                                // peek next after \n
                                chars.next(); // consume \n
                                if let Some(&next) = chars.peek() {
                                    if !next.is_whitespace() {
                                        result.push(' ');
                                    } else {
                                        // just skip the newline
                                    }
                                }
                                continue;
                            }
                        }
                        // skip the \r
                    } else if c == '\n' {
                        if let Some(last) = result.chars().last() {
                            if !last.is_whitespace() {
                                if let Some(&next) = chars.peek() {
                                    if !next.is_whitespace() {
                                        result.push(' ');
                                    }
                                }
                            }
                        }
                        continue;
                    }
                    result.push(c);
                }
                *value = result;

                // Remove remaining newlines
                value.retain(|c| c != '\n' && c != '\r');
            }
            if self.trim {
                *value = value.trim().to_string();
            }
            if self.spaces {
                // Collapse multiple spaces into one
                let mut result = String::with_capacity(value.len());
                let mut prev_was_space = false;
                for c in value.chars() {
                    if c.is_whitespace() {
                        if !prev_was_space {
                            result.push(' ');
                            prev_was_space = true;
                        }
                    } else {
                        result.push(c);
                        prev_was_space = false;
                    }
                }
                *value = result;
            }
        }
        VisitAction::Continue
    }
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
    fn cleanup_attrs_collapses_newlines() {
        let plug = CleanupAttrs;
        let input =
            "<svg xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 0\nL10 10\"/>\n</svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("d").unwrap(), "M0 0 L10 10");
    }

    #[test]
    fn cleanup_attrs_trims() {
        let plug = CleanupAttrs;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"  M0 0  \"/>\n</svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("d").unwrap(), "M0 0");
    }

    #[test]
    fn cleanup_attrs_collapses_spaces() {
        let plug = CleanupAttrs;
        let input =
            "<svg xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0   0    L10   10\"/>\n</svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert_eq!(path.attributes.get("d").unwrap(), "M0 0 L10 10");
    }
}
