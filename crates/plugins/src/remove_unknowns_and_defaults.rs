//! removeUnknownsAndDefaults plugin — removes unknown elements content and attributes,
//! removes attrs with default values.
//!
//! Ported from SVGO's plugins/removeUnknownsAndDefaults.js
//!
//! NOTE: This is a simplified port. The full SVGO plugin uses CSS stylesheet analysis
//! (computeStyle) to check computed parent style and CSS rule selectors. Since we don't
//! have a full CSS engine yet, we skip:
//!   - defaultAttrs checking against computed parent style
//!   - uselessOverrides checking against computed parent style
//!   - unknownContent checking against contentGroups (only direct content lists)
//!
//! These limitations are noted in the fixture tests with SKIP comments.

use crate::collections::{ELEMS, ELEMS_CONTENT};
use std::collections::{HashMap, HashSet};
use svgo_core::ast::Element;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct RemoveUnknownsAndDefaults;

impl Plugin for RemoveUnknownsAndDefaults {
    fn name(&self) -> &'static str {
        "removeUnknownsAndDefaults"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, params: &serde_json::Value) {
        let unknown_content = params
            .get("unknownContent")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let unknown_attrs = params
            .get("unknownAttrs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let _default_attrs = params
            .get("defaultAttrs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let default_markup_declarations = params
            .get("defaultMarkupDeclarations")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let _useless_overrides = params
            .get("uselessOverrides")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let keep_data_attrs = params
            .get("keepDataAttrs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let keep_aria_attrs = params
            .get("keepAriaAttrs")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let keep_role_attr = params
            .get("keepRoleAttr")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let mut v = RemoveUnknownsAndDefaultsVisitor {
            unknown_content,
            unknown_attrs,
            default_attrs: _default_attrs,
            default_markup_declarations,
            useless_overrides: _useless_overrides,
            keep_data_attrs,
            keep_aria_attrs,
            keep_role_attr,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct RemoveUnknownsAndDefaultsVisitor {
    unknown_content: bool,
    unknown_attrs: bool,
    default_attrs: bool,
    default_markup_declarations: bool,
    #[allow(dead_code)]
    useless_overrides: bool,
    keep_data_attrs: bool,
    keep_aria_attrs: bool,
    keep_role_attr: bool,
}

impl RemoveUnknownsAndDefaultsVisitor {
    /// Check if an element name is a known SVG element.
    fn is_known_element(&self, name: &str) -> bool {
        ELEMS.contains_key(name)
    }

    /// Get allowed attributes for an element.
    fn allowed_attrs(&self, name: &str) -> Option<&HashSet<&str>> {
        ELEMS.get(name).map(|(attrs, _)| attrs)
    }

    /// Get default attributes for an element.
    fn default_attrs(&self, name: &str) -> Option<&HashMap<&str, &str>> {
        ELEMS.get(name).map(|(_, defaults)| defaults)
    }

    /// Check if a child element is allowed inside a parent element.
    #[allow(dead_code)]
    fn is_allowed_child(&self, parent: &str, child: &str) -> bool {
        if let Some(allowed) = ELEMS_CONTENT.get(parent) {
            allowed.contains(child)
        } else {
            // Unknown parent: if it has no entry in ELEMS_CONTENT, be permissive
            true
        }
    }
}

impl Visitor for RemoveUnknownsAndDefaultsVisitor {
    fn element_enter(&mut self, el: &mut Element, ctx: &Context) -> VisitAction {
        // Skip namespaced elements
        if el.name.contains(':') {
            return VisitAction::Continue;
        }

        // Skip visiting foreignObject subtree
        if el.name == "foreignObject" {
            return VisitAction::SkipChildren;
        }

        // Get parent element name from ancestors
        let parent_name = ctx.ancestors.last().map(|s| s.as_str());

        // Remove unknown element's content
        if self.unknown_content {
            if let Some(_parent) = parent_name {
                if !self.is_known_element(&el.name) {
                    // Unknown element — remove it
                    return VisitAction::Remove;
                }
                // NOTE: Full implementation also checks allowed children per parent
                // via contentGroups resolution. Skipped here since ELEMS_CONTENT
                // only has direct content lists, not resolved groups.
            }
        }

        // Remove element's unknown attrs and attrs with default values
        let has_id = el.attributes.contains_key("id");
        let allowed = self.allowed_attrs(&el.name).cloned();
        let defaults = self.default_attrs(&el.name).cloned();

        let keys_to_check: Vec<String> = el.attributes.keys().cloned().collect();
        for name in &keys_to_check {
            if self.keep_data_attrs && name.starts_with("data-") {
                continue;
            }
            if self.keep_aria_attrs && name.starts_with("aria-") {
                continue;
            }
            if self.keep_role_attr && name == "role" {
                continue;
            }
            if name == "xmlns" {
                continue;
            }
            // Skip namespaced attributes except xml:* and xlink:*
            if name.contains(':') {
                let prefix = name.split(':').next().unwrap();
                if prefix != "xml" && prefix != "xlink" {
                    continue;
                }
            }

            // Remove unknown attrs
            if self.unknown_attrs {
                if let Some(ref allowed_set) = allowed {
                    if !allowed_set.contains(name.as_str()) {
                        el.attributes.swap_remove(name);
                        continue;
                    }
                }
            }

            // Remove attrs with default values (only if element has no id)
            if self.default_attrs && !has_id {
                if let Some(ref defaults_map) = defaults {
                    if let Some(&default_val) = defaults_map.get(name.as_str()) {
                        if let Some(val) = el.attributes.get(name) {
                            if val == default_val {
                                // SKIP: Full implementation checks computed parent style
                                // and CSS selectors. Without CSS engine, we remove the default.
                                el.attributes.swap_remove(name);
                            }
                        }
                    }
                }
            }
        }

        VisitAction::Continue
    }

    fn instruction(
        &mut self,
        _name: &mut String,
        value: &mut String,
        _ctx: &Context,
    ) -> VisitAction {
        if self.default_markup_declarations {
            // Remove standalone="no" from XML declaration
            *value = value
                .replace("standalone=\"no\"", "")
                .replace("standalone='no'", "");
            // Clean up extra whitespace
            *value = value.split_whitespace().collect::<Vec<_>>().join(" ");
        }
        VisitAction::Continue
    }
}

#[allow(dead_code)]
fn find_element<'a>(
    doc: &'a svgo_core::ast::Document,
    name: &str,
) -> Option<&'a svgo_core::ast::Element> {
    fn find_in_children<'a>(
        children: &'a [svgo_core::ast::Node],
        name: &str,
    ) -> Option<&'a svgo_core::ast::Element> {
        for child in children {
            if let svgo_core::ast::Node::Element(el) = child {
                if el.name == name {
                    return Some(el);
                }
                if let Some(found) = find_in_children(&el.children, name) {
                    return Some(found);
                }
            }
        }
        None
    }
    find_in_children(&doc.children, name)
}

#[allow(dead_code)]
fn find_element_mut<'a>(
    doc: &'a mut svgo_core::ast::Document,
    name: &str,
) -> Option<&'a mut svgo_core::ast::Element> {
    fn find_in_children<'a>(
        children: &'a mut [svgo_core::ast::Node],
        name: &str,
    ) -> Option<&'a mut svgo_core::ast::Element> {
        for child in children.iter_mut() {
            if let svgo_core::ast::Node::Element(el) = child {
                if el.name == name {
                    return Some(el);
                }
                if let Some(found) = find_in_children(&mut el.children, name) {
                    return Some(found);
                }
            }
        }
        None
    }
    find_in_children(&mut doc.children, name)
}
#[cfg(test)]
mod tests {
    use super::*;
    use svgo_core::parse;

    #[test]
    fn remove_unknowns_removes_unknown_attrs() {
        let plug = RemoveUnknownsAndDefaults;
        let input =
            "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\" unknown=\"value\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(!path.attributes.contains_key("unknown"));
        assert!(path.attributes.contains_key("d"));
    }

    #[test]
    fn remove_unknowns_removes_default_attrs() {
        let plug = RemoveUnknownsAndDefaults;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><rect x=\"0\" y=\"0\" width=\"100\" height=\"100\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let rect = find_element(&doc, "rect").unwrap();
        // x="0" and y="0" are defaults for rect
        assert!(!rect.attributes.contains_key("x"));
        assert!(!rect.attributes.contains_key("y"));
        assert!(rect.attributes.contains_key("width"));
        assert!(rect.attributes.contains_key("height"));
    }

    #[test]
    fn remove_unknowns_keeps_data_attrs() {
        let plug = RemoveUnknownsAndDefaults;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\" data-custom=\"value\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(path.attributes.contains_key("data-custom"));
    }

    #[test]
    fn remove_unknowns_keeps_aria_attrs() {
        let plug = RemoveUnknownsAndDefaults;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\" aria-label=\"test\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(path.attributes.contains_key("aria-label"));
    }

    #[test]
    fn remove_unknowns_preserves_attrs_on_id_elements() {
        let plug = RemoveUnknownsAndDefaults;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><rect id=\"myRect\" x=\"0\" y=\"0\" width=\"100\" height=\"100\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let rect = find_element(&doc, "rect").unwrap();
        // With id, defaults should be preserved
        assert!(rect.attributes.contains_key("x"));
        assert!(rect.attributes.contains_key("y"));
    }
}
