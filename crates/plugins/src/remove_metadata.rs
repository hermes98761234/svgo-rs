use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove <metadata> elements.
pub struct RemoveMetadata;

impl Plugin for RemoveMetadata {
    fn name(&self) -> &'static str {
        "removeMetadata"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                if el.name == "metadata" {
                    VisitAction::Remove
                } else {
                    VisitAction::Continue
                }
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
