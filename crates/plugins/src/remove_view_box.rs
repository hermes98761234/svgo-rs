use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove viewBox attribute when it matches width/height.
pub struct RemoveViewBox;

impl Plugin for RemoveViewBox {
    fn name(&self) -> &'static str {
        "removeViewBox"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                ctx: &Context,
            ) -> VisitAction {
                // Only applies to svg, pattern, symbol
                let applies = matches!(el.name.as_str(), "svg" | "pattern" | "symbol");
                if !applies {
                    return VisitAction::Continue;
                }

                // Skip nested svg elements
                if el.name == "svg" && !ctx.ancestors.is_empty() {
                    return VisitAction::Continue;
                }

                // Check if viewBox matches width/height
                if let (Some(viewbox), Some(width), Some(height)) =
                    (el.attr("viewBox"), el.attr("width"), el.attr("height"))
                {
                    let parts: Vec<&str> = viewbox.split([' ', ',']).collect();
                    if parts.len() == 4 {
                        let w = width.trim_end_matches("px");
                        let h = height.trim_end_matches("px");
                        if parts[0] == "0" && parts[1] == "0" && parts[2] == w && parts[3] == h {
                            el.remove_attr("viewBox");
                        }
                    }
                }

                VisitAction::Continue
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
