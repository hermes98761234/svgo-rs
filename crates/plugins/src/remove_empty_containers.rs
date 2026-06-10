use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::ELEMS_GROUPS;

/// Remove empty container elements.
pub struct RemoveEmptyContainers;

impl Plugin for RemoveEmptyContainers {
    fn name(&self) -> &'static str {
        "removeEmptyContainers"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                ctx: &Context,
            ) -> VisitAction {
                // Skip non-container elements and svg
                if el.name == "svg" {
                    return VisitAction::Continue;
                }
                let is_container = ELEMS_GROUPS
                    .get("container")
                    .map(|set| set.contains(el.name.as_str()))
                    .unwrap_or(false);
                if !is_container {
                    return VisitAction::Continue;
                }
                // Skip non-empty containers
                if !el.children.is_empty() {
                    return VisitAction::Continue;
                }
                // Empty patterns with attributes may contain reusable configuration
                if el.name == "pattern" && !el.attributes.is_empty() {
                    return VisitAction::Continue;
                }
                // Empty <mask> with id hides masked element
                if el.name == "mask" && el.attr("id").is_some() {
                    return VisitAction::Continue;
                }
                // Don't remove children of <switch>
                if ctx.ancestors.last().map(|s| s.as_str()) == Some("switch") {
                    return VisitAction::Continue;
                }
                // Note: filter check on <g> requires CSS style computation
                // which we don't have yet - this is a simplification
                VisitAction::Remove
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
