//! sortDefsChildren plugin — sorts children of <defs> to improve compression.
//!
//! Ported from SVGO's plugins/sortDefsChildren.js

use std::collections::HashMap;
use svgo_core::ast::{Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct SortDefsChildren;

impl Plugin for SortDefsChildren {
    fn name(&self) -> &'static str {
        "sortDefsChildren"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, _params: &serde_json::Value) {
        let mut v = SortDefsChildrenVisitor;
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct SortDefsChildrenVisitor;

impl Visitor for SortDefsChildrenVisitor {
    fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
        if el.name != "defs" {
            return VisitAction::Continue;
        }

        // Count frequencies of element names
        let mut frequencies: HashMap<&str, usize> = HashMap::new();
        for child in &el.children {
            if let Node::Element(child_el) = child {
                *frequencies.entry(&child_el.name).or_insert(0) += 1;
            }
        }

        // Sort children: by frequency (desc), then by name length (desc), then by name (desc)
        let mut indexed: Vec<(usize, &Node)> = el.children.iter().enumerate().collect();
        indexed.sort_by(|(_ai, a), (_bi, b)| {
            let a_el = match a {
                Node::Element(el) => el,
                _ => return std::cmp::Ordering::Equal,
            };
            let b_el = match b {
                Node::Element(el) => el,
                _ => return std::cmp::Ordering::Equal,
            };

            let a_freq = frequencies.get(a_el.name.as_str()).unwrap_or(&0);
            let b_freq = frequencies.get(b_el.name.as_str()).unwrap_or(&0);
            let freq_cmp = b_freq.cmp(a_freq);
            if freq_cmp != std::cmp::Ordering::Equal {
                return freq_cmp;
            }

            let len_cmp = b_el.name.len().cmp(&a_el.name.len());
            if len_cmp != std::cmp::Ordering::Equal {
                return len_cmp;
            }

            // Reverse alphabetical for equal names
            b_el.name.cmp(&a_el.name)
        });

        // Reorder children based on sorted indices
        let new_order: Vec<usize> = indexed.iter().map(|(i, _)| *i).collect();
        let mut new_children = Vec::with_capacity(el.children.len());
        for &idx in &new_order {
            new_children.push(el.children[idx].clone());
        }
        el.children = new_children;

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
    fn sort_defs_children_sorts_by_frequency() {
        let plug = SortDefsChildren;
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="a"/>
        <linearGradient id="b"/>
        <radialGradient id="c"/>
    </defs>
</svg>"#;
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        // Find defs
        let _svg = find_element(&doc, "svg").unwrap();
        let defs = find_element(&doc, "defs").unwrap();
        // linearGradient appears twice, should come first
        let names: Vec<&str> = defs
            .children
            .iter()
            .filter_map(|n| match n {
                svgo_core::ast::Node::Element(el) => Some(el.name.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(
            names,
            vec!["linearGradient", "linearGradient", "radialGradient"]
        );
    }
}
