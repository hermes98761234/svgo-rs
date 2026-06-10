use std::collections::HashSet;

use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove unused namespace declarations from the root SVG element.
pub struct RemoveUnusedNS;

impl Plugin for RemoveUnusedNS {
    fn name(&self) -> &'static str {
        "removeUnusedNS"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V {
            unused_namespaces: HashSet<String>,
            is_root_svg: bool,
        }
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                ctx: &Context,
            ) -> VisitAction {
                // Collect namespaces from root svg element
                if el.name == "svg" && ctx.ancestors.is_empty() {
                    self.is_root_svg = true;
                    for name in el.attributes.keys() {
                        if name.starts_with("xmlns:") {
                            let ns = name.strip_prefix("xmlns:").unwrap().to_string();
                            self.unused_namespaces.insert(ns);
                        }
                    }
                }

                if !self.unused_namespaces.is_empty() {
                    // Remove namespace if used in element name
                    if let Some(colon_pos) = el.name.find(':') {
                        let prefix = &el.name[..colon_pos];
                        self.unused_namespaces.remove(prefix);
                    }
                    // Remove namespace if used in attribute names
                    let used_prefixes: Vec<String> = el
                        .attributes
                        .keys()
                        .filter_map(|name| name.find(':').map(|pos| name[..pos].to_string()))
                        .collect();
                    for prefix in used_prefixes {
                        self.unused_namespaces.remove(&prefix);
                    }
                }

                VisitAction::Continue
            }

            fn element_exit(&mut self, el: &mut svgo_core::ast::Element, ctx: &Context) {
                // Remove unused namespace attributes from root svg element
                if el.name == "svg" && ctx.ancestors.is_empty() {
                    let to_remove: Vec<String> = self
                        .unused_namespaces
                        .iter()
                        .map(|ns| format!("xmlns:{}", ns))
                        .collect();
                    for name in to_remove {
                        el.attributes.shift_remove(&name);
                    }
                }
            }
        }

        let mut v = V {
            unused_namespaces: HashSet::new(),
            is_root_svg: false,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
