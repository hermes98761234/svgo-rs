//! removeUselessStrokeAndFill plugin — removes useless stroke and fill attributes.
//!
//! Ported from SVGO's plugins/removeUselessStrokeAndFill.js
//!
//! NOTE: This is a simplified port without full CSS stylesheet analysis.
//! The full SVGO plugin uses computeStyle() to check inherited/computed styles.
//! Without a CSS engine, we implement basic logic:
//! - For stroke: remove stroke-* attrs when stroke="none" is explicitly set on the element
//! - For fill: remove fill-* attrs when fill="none" is explicitly set on the element
//! - When removeNone is true, remove elements with both stroke:none and fill:none

use crate::collections::ELEMS_GROUPS;
use svgo_core::ast::{Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct RemoveUselessStrokeAndFill;

impl Plugin for RemoveUselessStrokeAndFill {
    fn name(&self) -> &'static str {
        "removeUselessStrokeAndFill"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, params: &serde_json::Value) {
        let remove_stroke = params
            .get("stroke")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let remove_fill = params.get("fill").and_then(|v| v.as_bool()).unwrap_or(true);
        let remove_none = params
            .get("removeNone")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut v = RemoveUselessStrokeAndFillVisitor {
            remove_stroke,
            remove_fill,
            remove_none,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct RemoveUselessStrokeAndFillVisitor {
    remove_stroke: bool,
    remove_fill: bool,
    remove_none: bool,
}

impl RemoveUselessStrokeAndFillVisitor {
    fn is_shape(&self, name: &str) -> bool {
        ELEMS_GROUPS
            .get("shape")
            .map(|s| s.contains(name))
            .unwrap_or(false)
    }
}

impl Visitor for RemoveUselessStrokeAndFillVisitor {
    fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
        // Skip if element has id (deoptimizes)
        if el.attributes.contains_key("id") {
            return VisitAction::Continue;
        }

        if !self.is_shape(&el.name) {
            return VisitAction::Continue;
        }

        // Remove stroke attributes when stroke is none
        if self.remove_stroke {
            let stroke_val = el.attributes.get("stroke").map(|v| v.as_str());
            if stroke_val == Some("none") {
                let keys_to_remove: Vec<String> = el
                    .attributes
                    .keys()
                    .filter(|k| k.starts_with("stroke"))
                    .cloned()
                    .collect();
                for k in keys_to_remove {
                    el.attributes.swap_remove(&k);
                }
                // Set explicit none to not inherit from parent
                el.attributes
                    .insert("stroke".to_string(), "none".to_string());
            }
        }

        // Remove fill-* attributes when fill is none
        if self.remove_fill {
            if let Some(fill_val) = el.attributes.get("fill") {
                if fill_val == "none" {
                    let keys_to_remove: Vec<String> = el
                        .attributes
                        .keys()
                        .filter(|k| k.starts_with("fill-"))
                        .cloned()
                        .collect();
                    for k in keys_to_remove {
                        el.attributes.swap_remove(&k);
                    }
                }
            }
        }

        // Remove elements with both stroke:none and fill:none when removeNone is true
        if self.remove_none {
            let stroke_none = el
                .attributes
                .get("stroke")
                .map(|v| v == "none")
                .unwrap_or(true);
            let fill_none = el
                .attributes
                .get("fill")
                .map(|v| v == "none")
                .unwrap_or(false);
            if stroke_none && fill_none {
                return VisitAction::Remove;
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
    fn remove_useless_stroke_removes_stroke_attrs() {
        let plug = RemoveUselessStrokeAndFill;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path stroke=\"none\" stroke-width=\"2\" d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(!path.attributes.contains_key("stroke-width"));
        assert_eq!(path.attributes.get("stroke").unwrap(), "none");
    }

    #[test]
    fn remove_useless_fill_removes_fill_attrs() {
        let plug = RemoveUselessStrokeAndFill;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path fill=\"none\" fill-opacity=\"0.5\" d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(!path.attributes.contains_key("fill-opacity"));
    }

    #[test]
    fn remove_useless_stroke_and_fill_skips_id_elements() {
        let plug = RemoveUselessStrokeAndFill;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path id=\"myPath\" stroke=\"none\" stroke-width=\"2\" d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        // Elements with id should be skipped
        assert!(path.attributes.contains_key("stroke-width"));
    }

    #[test]
    fn remove_useless_stroke_and_fill_remove_none() {
        let plug = RemoveUselessStrokeAndFill;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path fill=\"none\" stroke=\"none\" d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({"removeNone": true}));
        let svg = find_element(&doc, "svg").unwrap();
        // The path should be removed
        assert_eq!(
            svg.children
                .iter()
                .filter(|n| matches!(n, Node::Element(el) if el.name == "path"))
                .count(),
            0
        );
    }
}
