use svgo_core::ast::{Document, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::ELEMS_GROUPS;

/// Removes content of defs and non-rendering elements without ids.
pub struct RemoveUselessDefs;

impl Plugin for RemoveUselessDefs {
    fn name(&self) -> &'static str {
        "removeUselessDefs"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                let is_defs = el.name == "defs";
                let non_rendering = ELEMS_GROUPS.get("nonRendering");
                let is_non_rendering_no_id = non_rendering
                    .map(|set| set.contains(el.name.as_str()))
                    .unwrap_or(false)
                    && el.attr("id").is_none();

                if is_defs || is_non_rendering_no_id {
                    // Collect useful children (those with id or <style>)
                    let mut useful = Vec::new();
                    collect_useful_nodes(&el.children, &mut useful);
                    if useful.is_empty() {
                        return VisitAction::Remove;
                    }
                    el.children = useful;
                }
                VisitAction::Continue
            }
        }

        fn collect_useful_nodes(children: &[Node], useful: &mut Vec<Node>) {
            for child in children {
                if let Node::Element(el) = child {
                    if el.attr("id").is_some() || el.name == "style" {
                        useful.push(child.clone());
                    } else {
                        collect_useful_nodes(&el.children, useful);
                    }
                }
            }
        }

        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
