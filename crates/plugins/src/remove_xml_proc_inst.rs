use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove XML processing instructions (<?xml ...?>).
pub struct RemoveXmlProcInst;

impl Plugin for RemoveXmlProcInst {
    fn name(&self) -> &'static str {
        "removeXMLProcInst"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn instruction(
                &mut self,
                name: &mut String,
                _value: &mut String,
                _ctx: &Context,
            ) -> VisitAction {
                if name == "xml" {
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
