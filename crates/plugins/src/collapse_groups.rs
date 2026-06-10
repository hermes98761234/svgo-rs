use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::INHERITABLE_ATTRS;

const ANIMATION_ELEMS: &[&str] = &[
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "set",
];

pub struct CollapseGroups;

impl Plugin for CollapseGroups {
    fn name(&self) -> &'static str {
        "collapseGroups"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        fn has_animated_attr(node: &Node, name: &str) -> bool {
            match node {
                Node::Element(el) => {
                    if ANIMATION_ELEMS.contains(&el.name.as_str())
                        && el.attr("attributeName").map_or(false, |n| n == name)
                    {
                        return true;
                    }
                    el.children.iter().any(|c| has_animated_attr(c, name))
                }
                _ => false,
            }
        }

        fn can_merge_attrs(group_el: &Element, child_el: &Element) -> bool {
            if group_el.attr("filter").is_some() {
                return false;
            }
            if child_el.attr("id").is_some() {
                return false;
            }
            if (group_el.attr("clip-path").is_some() || group_el.attr("mask").is_some())
                && !(child_el.name == "g"
                    && group_el.attr("transform").is_none()
                    && child_el.attr("transform").is_none())
            {
                return false;
            }
            if group_el.attr("class").is_some() && child_el.attr("class").is_some() {
                return false;
            }
            true
        }

        fn merge_into_child(group_el: &Element, child_el: &mut Element) -> bool {
            let attrs: Vec<(String, String)> = group_el
                .attributes
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            for (name, value) in attrs {
                if has_animated_attr(&Node::Element(child_el.clone()), name.as_str()) {
                    return false;
                }
                if child_el.attributes.contains_key(&name) && name != "transform" {
                    if !INHERITABLE_ATTRS.contains(name.as_str()) {
                        return false;
                    }
                }
                if child_el.attr(&name).is_none() {
                    child_el.set_attr(&name, &value);
                } else if name == "transform" {
                    child_el.set_attr(
                        "transform",
                        &format!("{} {}", value, child_el.attr("transform").unwrap_or("")),
                    );
                }
            }
            true
        }

        fn process_children(children: &mut Vec<Node>) {
            let mut i = 0;
            while i < children.len() {
                if let Node::Element(child_el) = &mut children[i] {
                    process_children(&mut child_el.children);

                    if child_el.name == "g"
                        && child_el.children.len() == 1
                        && !child_el.attributes.is_empty()
                    {
                        let can_merge = {
                            if let Node::Element(first_child) = &child_el.children[0] {
                                can_merge_attrs(child_el, first_child)
                            } else {
                                false
                            }
                        };
                        if can_merge {
                            let mut first_child_el = if let Node::Element(el) = std::mem::replace(
                                &mut child_el.children[0],
                                Node::Text(String::new()),
                            ) {
                                el
                            } else {
                                unreachable!()
                            };
                            if merge_into_child(child_el, &mut first_child_el) {
                                child_el.attributes.clear();
                                child_el.children[0] = Node::Element(first_child_el);
                            }
                        }
                    }

                    if child_el.name == "g" && child_el.attributes.is_empty() {
                        let has_anim = child_el.children.iter().any(|c| match c {
                            Node::Element(e) => ANIMATION_ELEMS.contains(&e.name.as_str()),
                            _ => false,
                        });
                        if !has_anim {
                            let children_to_insert = std::mem::take(&mut child_el.children);
                            children.splice(i..i + 1, children_to_insert);
                            continue;
                        }
                    }
                }
                i += 1;
            }
        }

        process_children(&mut doc.children);
    }
}
