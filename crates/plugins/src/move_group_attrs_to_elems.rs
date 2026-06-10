use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::REFERENCES_PROPS;

const PATH_ELEMS_WITH_GROUPS_AND_TEXT: &[&str] = &[
    "glyph",
    "missing-glyph",
    "path",
    "circle",
    "ellipse",
    "line",
    "polygon",
    "polyline",
    "rect",
    "text",
    "g",
];

/// Push group attributes (esp. transform) down to content elements.
///
/// Only moves transform from a <g> to its children when:
/// - Group has a transform attr
/// - No children have url() references in href-like attrs (e.g. fill, clip-path)
/// - All children are path-like elements (including g, text) without id
pub struct MoveGroupAttrsToElems;

impl Plugin for MoveGroupAttrsToElems {
    fn name(&self) -> &'static str {
        "moveGroupAttrsToElems"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
                if el.name != "g" || el.children.is_empty() || el.attr("transform").is_none() {
                    return VisitAction::Continue;
                }

                // Check no reference attrs with url() values
                let has_url_ref = el.attributes.iter().any(|(name, value)| {
                    REFERENCES_PROPS.contains(name.as_str()) && value.contains("url(")
                });
                if has_url_ref {
                    return VisitAction::Continue;
                }

                // Check all children are path-like, no id
                let all_valid = el.children.iter().all(|child| match child {
                    Node::Element(child_el) => {
                        PATH_ELEMS_WITH_GROUPS_AND_TEXT.contains(&child_el.name.as_str())
                            && child_el.attr("id").is_none()
                    }
                    _ => true,
                });

                if !all_valid {
                    return VisitAction::Continue;
                }

                // Move transform to children
                let transform = el.attr("transform").unwrap_or_default().to_string();
                for child in &mut el.children {
                    if let Node::Element(child_el) = child {
                        if let Some(existing) = child_el.attr("transform") {
                            child_el.set_attr("transform", &format!("{} {}", transform, existing));
                        } else {
                            child_el.set_attr("transform", &transform);
                        }
                    }
                }
                el.remove_attr("transform");
                VisitAction::Continue
            }
        }
        svgo_core::visitor::visit(doc, &mut V);
    }
}
