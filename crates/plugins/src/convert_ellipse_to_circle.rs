use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Converts non-eccentric `<ellipse>`s to `<circle>`s.
pub struct ConvertEllipseToCircle;

impl Plugin for ConvertEllipseToCircle {
    fn name(&self) -> &'static str {
        "convertEllipseToCircle"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                if el.name == "ellipse" {
                    let rx = el
                        .attributes
                        .get("rx")
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "0".to_string());
                    let ry = el
                        .attributes
                        .get("ry")
                        .map(|s| s.clone())
                        .unwrap_or_else(|| "0".to_string());
                    if rx == ry || rx == "auto" || ry == "auto" {
                        el.name = "circle".to_string();
                        let radius = if rx == "auto" { ry.clone() } else { rx.clone() };
                        el.attributes.shift_remove("rx");
                        el.attributes.shift_remove("ry");
                        el.attributes.insert("r".to_string(), radius);
                    }
                }
                VisitAction::Continue
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
