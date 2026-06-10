use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::{ATTRS_GROUPS, INHERITABLE_ATTRS};

/// Remove non-inheritable group's presentational attributes.
pub struct RemoveNonInheritableGroupAttrs;

impl Plugin for RemoveNonInheritableGroupAttrs {
    fn name(&self) -> &'static str {
        "removeNonInheritableGroupAttrs"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                if el.name == "g" {
                    let presentation = ATTRS_GROUPS.get("presentation");
                    el.attributes.retain(|name, _value| {
                        if let Some(pres_set) = presentation {
                            if pres_set.contains(name.as_str()) {
                                // Keep only inheritable presentation attrs
                                return INHERITABLE_ATTRS.contains(name.as_str());
                            }
                        }
                        true
                    });
                }
                VisitAction::Continue
            }
        }
        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
