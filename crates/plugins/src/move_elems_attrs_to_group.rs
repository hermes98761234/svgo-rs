use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::INHERITABLE_ATTRS;

/// Move common attributes of group children to the group.
///
/// Only moves inheritable attributes that ALL children share with the same value.
/// Does not move transform when group has filter/clip-path/mask or all children are paths.
/// Deoptimizes (skips) when style elements are present.
pub struct MoveElemsAttrsToGroup;

impl Plugin for MoveElemsAttrsToGroup {
    fn name(&self) -> &'static str {
        "moveElemsAttrsToGroup"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        // Check if any style element is present
        struct StyleDetector {
            has_style: bool,
        }
        impl Visitor for StyleDetector {
            fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
                if el.name == "style" {
                    self.has_style = true;
                    return VisitAction::SkipChildren;
                }
                VisitAction::Continue
            }
        }
        let mut detector = StyleDetector { has_style: false };
        svgo_core::visitor::visit(doc, &mut detector);
        if detector.has_style {
            return;
        }

        // Process groups on exit
        fn process_node(el: &mut Element) {
            if el.name != "g" || el.children.len() <= 1 {
                // Still recurse into children
                for child in el.children.iter_mut() {
                    if let Node::Element(child_el) = child {
                        process_node(child_el);
                    }
                }
                return;
            }

            // Check if all children are path elements
            let path_elms: std::collections::HashSet<&str> =
                ["glyph", "missing-glyph", "path"].iter().cloned().collect();
            let mut every_child_is_path = true;
            let mut common_attrs: Option<indexmap::IndexMap<String, String>> = None;

            for child in &el.children {
                if let Node::Element(child_el) = child {
                    if !path_elms.contains(child_el.name.as_str()) {
                        every_child_is_path = false;
                    }
                    let mut child_inheritable = indexmap::IndexMap::new();
                    for (name, value) in &child_el.attributes {
                        if INHERITABLE_ATTRS.contains(name.as_str()) {
                            child_inheritable.insert(name.clone(), value.clone());
                        }
                    }
                    match &mut common_attrs {
                        None => common_attrs = Some(child_inheritable),
                        Some(ref mut common) => {
                            common.retain(|k, v| {
                                child_inheritable.get(k).map_or(false, |cv| cv == v)
                            });
                        }
                    }
                }
            }

            let mut common_attrs = match common_attrs {
                Some(c) => c,
                None => {
                    for child in el.children.iter_mut() {
                        if let Node::Element(child_el) = child {
                            process_node(child_el);
                        }
                    }
                    return;
                }
            };

            // Remove transform if group has filter/clip-path/mask
            if el.attr("filter").is_some()
                || el.attr("clip-path").is_some()
                || el.attr("mask").is_some()
            {
                common_attrs.shift_remove("transform");
            }

            // Remove transform if all children are paths
            if every_child_is_path {
                common_attrs.shift_remove("transform");
            }

            // Add common attrs to group, remove from children
            for (name, value) in &common_attrs {
                if name == "transform" {
                    if let Some(existing) = el.attr("transform") {
                        el.set_attr("transform", &format!("{} {}", existing, value));
                    } else {
                        el.set_attr("transform", value);
                    }
                } else {
                    el.set_attr(name, value);
                }
            }

            for child in &mut el.children {
                if let Node::Element(child_el) = child {
                    for name in common_attrs.keys() {
                        child_el.remove_attr(name);
                    }
                }
            }
        }

        for child in doc.children.iter_mut() {
            if let Node::Element(el) = child {
                process_node(el);
            }
        }
    }
}
