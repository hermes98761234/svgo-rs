use svgo_core::ast::{Document, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Removes <desc> elements that are empty or contain standard editor text.
/// Set `removeAny` to true to remove any description.
pub struct RemoveDesc;

impl Plugin for RemoveDesc {
    fn name(&self) -> &'static str {
        "removeDesc"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let remove_any = params
            .get("removeAny")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        struct V {
            remove_any: bool,
        }
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                if el.name != "desc" {
                    return VisitAction::Continue;
                }

                if self.remove_any {
                    return VisitAction::Remove;
                }

                // Remove if empty
                if el.children.is_empty() {
                    return VisitAction::Remove;
                }

                // Remove if contains standard editor text like "Created with ..."
                if let Some(Node::Text(text)) = el.children.first() {
                    if text.starts_with("Created with") || text.starts_with("Created using") {
                        return VisitAction::Remove;
                    }
                }

                VisitAction::Continue
            }
        }

        let mut v = V { remove_any };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
