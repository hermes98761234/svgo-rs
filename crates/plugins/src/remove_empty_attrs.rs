use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::ATTRS_GROUPS;

/// Remove attributes with empty values.
pub struct RemoveEmptyAttrs;

impl Plugin for RemoveEmptyAttrs {
    fn name(&self) -> &'static str {
        "removeEmptyAttrs"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                let conditional = ATTRS_GROUPS.get("conditionalProcessing");
                el.attributes.retain(|name, value| {
                    if value.is_empty() {
                        // Don't remove empty conditional processing attributes
                        if let Some(set) = conditional {
                            return set.contains(name.as_str());
                        }
                    }
                    true
                });
                VisitAction::Continue
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
