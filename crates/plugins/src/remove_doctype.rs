use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove DOCTYPE declaration.
///
/// "Unfortunately the SVG DTDs are a source of so many
/// issues that the SVG WG has decided not to write one
/// for the upcoming SVG 1.2 standard. In fact SVG WG
/// members are even telling people not to use a DOCTYPE
/// declaration in SVG 1.0 and 1.1 documents"
/// https://jwatt.org/svg/authoring/#doctype-declaration
pub struct RemoveDoctype;

impl Plugin for RemoveDoctype {
    fn name(&self) -> &'static str {
        "removeDoctype"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn doctype(&mut self, _d: &mut String, _ctx: &Context) -> VisitAction {
                VisitAction::Remove
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
