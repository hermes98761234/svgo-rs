use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove empty text elements.
pub struct RemoveEmptyText;

impl Plugin for RemoveEmptyText {
    fn name(&self) -> &'static str {
        "removeEmptyText"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let text = params.get("text").and_then(|v| v.as_bool()).unwrap_or(true);
        let tspan = params
            .get("tspan")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let tref = params.get("tref").and_then(|v| v.as_bool()).unwrap_or(true);

        struct V {
            text: bool,
            tspan: bool,
            tref: bool,
        }
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                // Remove empty text element
                if self.text && el.name == "text" && el.children.is_empty() {
                    return VisitAction::Remove;
                }
                // Remove empty tspan element
                if self.tspan && el.name == "tspan" && el.children.is_empty() {
                    return VisitAction::Remove;
                }
                // Remove tref without xlink:href
                if self.tref && el.name == "tref" && el.attr("xlink:href").is_none() {
                    return VisitAction::Remove;
                }
                VisitAction::Continue
            }
        }

        let mut v = V { text, tspan, tref };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
