use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

pub struct MergeStyles;

impl Plugin for MergeStyles {
    fn name(&self) -> &'static str {
        "mergeStyles"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        // Phase 1: collect info about all valid <style> elements via visitor
        struct Collector {
            parts: Vec<(String, String)>,
            has_cdata: bool,
        }

        impl Visitor for Collector {
            fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
                if el.name == "foreignObject" {
                    return VisitAction::SkipChildren;
                }
                if el.name != "style" {
                    return VisitAction::Continue;
                }
                if let Some(t) = el.attr("type") {
                    if !t.is_empty() && t != "text/css" {
                        return VisitAction::Continue;
                    }
                }
                let mut css = String::new();
                for child in &el.children {
                    if let Node::Text(t) = child {
                        css.push_str(t.as_str());
                    }
                    if let Node::CData(c) = child {
                        self.has_cdata = true;
                        css.push_str(c.as_str());
                    }
                }
                let media = el.attr("media").unwrap_or("").to_string();
                self.parts.push((css, media));
                VisitAction::Continue
            }
        }

        let mut collector = Collector {
            parts: Vec::new(),
            has_cdata: false,
        };
        svgo_core::visitor::visit(doc, &mut collector);

        if collector.parts.len() < 2 {
            return;
        }

        // Build merged CSS
        let mut merged = String::new();
        for (css, media) in &collector.parts {
            if media.is_empty() {
                merged.push_str(css);
            } else {
                merged.push_str(&format!("@media {}{{{}}}", media, css));
            }
        }

        // Phase 2: update first <style> element, remove subsequent ones
        fn process_children(children: &mut Vec<Node>, merged: &str, has_cdata: bool) -> bool {
            let mut found_first = false;
            let mut to_remove = Vec::new();

            for (i, node) in children.iter_mut().enumerate() {
                if let Node::Element(el) = node {
                    if el.name == "style" {
                        if let Some(t) = el.attr("type") {
                            if !t.is_empty() && t != "text/css" {
                                continue;
                            }
                        }
                        if !found_first {
                            found_first = true;
                            el.remove_attr("media");
                            el.children.clear();
                            if has_cdata {
                                el.children.push(Node::CData(merged.to_string()));
                            } else {
                                el.children.push(Node::Text(merged.to_string()));
                            }
                        } else {
                            to_remove.push(i);
                        }
                    }
                }
            }

            for &idx in to_remove.iter().rev() {
                children.remove(idx);
            }

            // Recurse into remaining element children
            for node in children.iter_mut() {
                if let Node::Element(el) = node {
                    found_first =
                        process_children(&mut el.children, merged, has_cdata) || found_first;
                }
            }
            found_first
        }

        process_children(&mut doc.children, &merged, collector.has_cdata);
    }
}
