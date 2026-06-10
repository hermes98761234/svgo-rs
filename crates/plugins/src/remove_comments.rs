use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove comments, optionally preserving those matching certain patterns.
///
/// By default, comments starting with `!` (e.g. `<!--! ... -->`) are preserved
/// for license/copyright information.
pub struct RemoveComments;

impl Plugin for RemoveComments {
    fn name(&self) -> &'static str {
        "removeComments"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        // Check if preservePatterns is explicitly set to false/null
        let preserve = !matches!(
            params.get("preservePatterns"),
            Some(serde_json::Value::Null) | Some(serde_json::Value::Bool(false))
        );

        struct V {
            preserve: bool,
        }
        impl Visitor for V {
            fn comment(&mut self, c: &mut String, _ctx: &Context) -> VisitAction {
                if self.preserve && c.starts_with('!') {
                    VisitAction::Continue
                } else {
                    VisitAction::Remove
                }
            }
        }
        let mut v = V { preserve };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
