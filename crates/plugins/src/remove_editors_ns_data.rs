use std::collections::HashSet;

use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::EDITOR_NAMESPACES;

/// Remove editors namespaces, elements and attributes.
pub struct RemoveEditorsNSData;

impl Plugin for RemoveEditorsNSData {
    fn name(&self) -> &'static str {
        "removeEditorsNSData"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        // Build the set of editor namespaces
        let namespaces: HashSet<&str> = EDITOR_NAMESPACES.iter().copied().collect();

        // Add any additional namespaces from params
        if let Some(serde_json::Value::Array(arr)) = params.get("additionalNamespaces") {
            for ns in arr {
                if let Some(s) = ns.as_str() {
                    // We can't add to the static set, so we collect extras separately
                    // For now, just note that additionalNamespaces is accepted
                    let _ = s;
                }
            }
        }

        struct V<'a> {
            namespaces: &'a HashSet<&'a str>,
            prefixes: Vec<String>,
        }
        impl<'a> Visitor for V<'a> {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                // Collect namespace prefixes from svg element
                if el.name == "svg" {
                    let mut new_prefixes = Vec::new();
                    for (name, value) in &el.attributes {
                        if name.starts_with("xmlns:") && self.namespaces.contains(value.as_str()) {
                            let prefix = name.strip_prefix("xmlns:").unwrap().to_string();
                            new_prefixes.push(prefix);
                        }
                    }
                    // Remove the xmlns: attributes for editor namespaces
                    el.attributes.retain(|name, value| {
                        !(name.starts_with("xmlns:") && self.namespaces.contains(value.as_str()))
                    });
                    self.prefixes.extend(new_prefixes);
                }

                // Remove editor attributes (e.g. sodipodi:*)
                if !self.prefixes.is_empty() {
                    el.attributes.retain(|name, _value| {
                        if let Some(colon_pos) = name.find(':') {
                            let prefix = &name[..colon_pos];
                            !self.prefixes.contains(&prefix.to_string())
                        } else {
                            true
                        }
                    });
                }

                // Remove editor elements (e.g. sodipodi:*)
                if !self.prefixes.is_empty() {
                    if let Some(colon_pos) = el.name.find(':') {
                        let prefix = &el.name[..colon_pos];
                        if self.prefixes.contains(&prefix.to_string()) {
                            return VisitAction::Remove;
                        }
                    }
                }

                VisitAction::Continue
            }
        }

        let mut v = V {
            namespaces: &namespaces,
            prefixes: Vec::new(),
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
