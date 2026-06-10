//! sortAttrs plugin — sorts element attributes for better compression.
//!
//! Ported from SVGO's plugins/sortAttrs.js

use indexmap::IndexMap;
use svgo_core::ast::Element;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct SortAttrs;

impl Plugin for SortAttrs {
    fn name(&self) -> &'static str {
        "sortAttrs"
    }

    fn apply(&self, doc: &mut svgo_core::ast::Document, params: &serde_json::Value) {
        let order: Vec<String> = params
            .get("order")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_else(|| {
                vec![
                    "id", "width", "height", "x", "x1", "x2", "y", "y1", "y2", "cx", "cy", "r",
                    "fill", "stroke", "marker", "d", "points",
                ]
                .into_iter()
                .map(String::from)
                .collect()
            });
        let xmlns_order = params
            .get("xmlnsOrder")
            .and_then(|v| v.as_str())
            .unwrap_or("front");

        let mut v = SortAttrsVisitor {
            order,
            xmlns_order: xmlns_order.to_string(),
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

struct SortAttrsVisitor {
    order: Vec<String>,
    xmlns_order: String,
}

impl SortAttrsVisitor {
    fn get_ns_priority(&self, name: &str) -> i32 {
        if self.xmlns_order == "front" {
            if name == "xmlns" {
                return 3;
            }
            if name.starts_with("xmlns:") {
                return 2;
            }
        }
        if name.contains(':') {
            return 1;
        }
        0
    }

    fn compare_attrs(&self, a: &str, b: &str) -> std::cmp::Ordering {
        let a_priority = self.get_ns_priority(a);
        let b_priority = self.get_ns_priority(b);
        let priority_ns = b_priority - a_priority;
        if priority_ns != 0 {
            return priority_ns.cmp(&0);
        }

        // Extract the first part from attributes (e.g. "fill" from "fill-opacity")
        let a_part = a.split('-').next().unwrap_or(a);
        let b_part = b.split('-').next().unwrap_or(b);

        if a_part != b_part {
            let a_in_order = self.order.contains(&a_part.to_string());
            let b_in_order = self.order.contains(&b_part.to_string());

            if a_in_order && b_in_order {
                let a_idx = self.order.iter().position(|x| x == a_part).unwrap();
                let b_idx = self.order.iter().position(|x| x == b_part).unwrap();
                return a_idx.cmp(&b_idx);
            }

            if a_in_order != b_in_order {
                if b_in_order {
                    return std::cmp::Ordering::Greater;
                }
                return std::cmp::Ordering::Less;
            }
        }

        // Sort alphabetically
        a.cmp(b)
    }
}

impl Visitor for SortAttrsVisitor {
    fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
        let mut entries: Vec<(String, String)> = el
            .attributes
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        entries.sort_by(|(a, _), (b, _)| self.compare_attrs(a, b));

        let mut sorted = IndexMap::new();
        for (k, v) in entries {
            sorted.insert(k, v);
        }
        el.attributes = sorted;

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
    fn sort_attrs_id_first() {
        let plug = SortAttrs;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\" fill=\"red\" id=\"myId\" stroke=\"blue\"><path d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let svg = find_element(&doc, "svg").unwrap();
        let keys: Vec<&str> = svg.attributes.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys[0], "xmlns");
    }

    #[test]
    fn sort_attrs_xmlns_first() {
        let plug = SortAttrs;
        let input =
            "<svg fill=\"red\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"M0 0\"/></svg>";
        let mut doc = parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let svg = find_element(&doc, "svg").unwrap();
        let keys: Vec<&str> = svg.attributes.keys().map(|k| k.as_str()).collect();
        assert_eq!(keys[0], "xmlns");
    }
}
