//! cleanupIds plugin — removes unused IDs and minifies used ones.
//!
//! Ported from SVGO's plugins/cleanupIds.js

use crate::collections::REFERENCES_PROPS;
use std::collections::{HashMap, HashSet};
use svgo_core::ast::{Document, Element, Node};
use svgo_core::plugin::Plugin;

pub struct CleanupIds;

impl Plugin for CleanupIds {
    fn name(&self) -> &'static str {
        "cleanupIds"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let remove = params
            .get("remove")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let minify = params
            .get("minify")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let preserve: HashSet<String> = params
            .get("preserve")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let preserve_prefixes: Vec<String> = params
            .get("preservePrefixes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let force = params
            .get("force")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Check for scripts/styles (unless force is true)
        let mut has_style_or_script = false;
        if !force {
            check_deopt(doc, &mut has_style_or_script);
        }

        if has_style_or_script {
            return;
        }

        // Check if SVG consists only of defs
        if !force {
            let mut all_defs = true;
            for child in &doc.children {
                match child {
                    Node::Element(el) if el.name == "defs" => continue,
                    Node::Text(t) if t.trim().is_empty() => continue,
                    _ => {
                        all_defs = false;
                        break;
                    }
                }
            }
            if all_defs {
                return;
            }
        }

        // Collect all IDs and references in one pass
        let mut node_by_id: HashMap<String, String> = HashMap::new(); // id -> element name (for context)
        let mut references_by_id: HashMap<String, Vec<(String, String)>> = HashMap::new(); // id -> [(attr_name, parent_name)]

        collect_ids_and_refs(doc, &mut node_by_id, &mut references_by_id);

        // Build set of referenced IDs
        let referenced_ids: HashSet<String> = references_by_id.keys().cloned().collect();

        // Generate minification mapping
        let mut id_mapping: HashMap<String, String> = HashMap::new();
        if minify {
            let mut current_id: Option<Vec<usize>> = None;

            for id in &referenced_ids {
                if is_preserved(id, &preserve, &preserve_prefixes) {
                    continue;
                }

                // Generate new minified ID
                loop {
                    current_id = Some(generate_id(current_id.as_deref()));
                    let new_id = get_id_string(current_id.as_ref().unwrap());
                    if !is_preserved(&new_id, &preserve, &preserve_prefixes)
                        && !referenced_ids.contains(&new_id)
                    {
                        id_mapping.insert(id.clone(), new_id);
                        break;
                    }
                }
            }
        }

        // Apply changes: minify IDs and update references
        apply_id_changes(
            doc,
            &id_mapping,
            &referenced_ids,
            &preserve,
            &preserve_prefixes,
            remove,
            minify,
        );
    }
}

fn check_deopt(doc: &Document, has_style_or_script: &mut bool) {
    fn check_element(el: &Element, flag: &mut bool) {
        if el.name == "style" && !el.children.is_empty() {
            *flag = true;
            return;
        }
        if el.name == "script" && !el.children.is_empty() {
            *flag = true;
            return;
        }
        for key in el.attributes.keys() {
            if key.starts_with("on") {
                *flag = true;
                return;
            }
        }
        for child in &el.children {
            if let Node::Element(child_el) = child {
                check_element(child_el, flag);
            }
        }
    }
    for child in &doc.children {
        if let Node::Element(el) = child {
            check_element(el, has_style_or_script);
        }
    }
}

fn collect_ids_and_refs(
    doc: &Document,
    node_by_id: &mut HashMap<String, String>,
    references_by_id: &mut HashMap<String, Vec<(String, String)>>,
) {
    for child in &doc.children {
        if let Node::Element(el) = child {
            collect_from_element(el, node_by_id, references_by_id);
        }
    }
}

fn collect_from_element(
    el: &Element,
    node_by_id: &mut HashMap<String, String>,
    references_by_id: &mut HashMap<String, Vec<(String, String)>>,
) {
    for (name, value) in &el.attributes {
        if name == "id" {
            if !node_by_id.contains_key(value) {
                node_by_id.insert(value.clone(), el.name.clone());
            }
        } else {
            let ids = find_references(name, value);
            for id in ids {
                references_by_id
                    .entry(id)
                    .or_default()
                    .push((name.clone(), el.name.clone()));
            }
        }
    }

    for child in &el.children {
        if let Node::Element(child_el) = child {
            collect_from_element(child_el, node_by_id, references_by_id);
        }
    }
}

fn find_references(attr_name: &str, value: &str) -> Vec<String> {
    let mut results = Vec::new();

    // Check url(#id) references
    if REFERENCES_PROPS.contains(attr_name) {
        let mut search_start = 0;
        while let Some(pos) = value[search_start..].find("url(#") {
            let start = search_start + pos + 5;
            if let Some(end) = value[start..].find(')') {
                let id = &value[start..start + end];
                results.push(id.to_string());
                search_start = start + end;
            } else {
                break;
            }
        }
    }

    // Check href references
    if attr_name == "href" || attr_name.ends_with(":href") {
        if value.starts_with('#') {
            results.push(value[1..].to_string());
        }
    }

    // Check begin references (e.g. "animId.end")
    if attr_name == "begin" {
        if let Some(dot_pos) = value.find('.') {
            results.push(value[..dot_pos].to_string());
        }
    }

    results
}

fn apply_id_changes(
    doc: &mut Document,
    id_mapping: &HashMap<String, String>,
    referenced_ids: &HashSet<String>,
    preserve: &HashSet<String>,
    preserve_prefixes: &[String],
    remove: bool,
    minify: bool,
) {
    apply_to_children(
        &mut doc.children,
        id_mapping,
        referenced_ids,
        preserve,
        preserve_prefixes,
        remove,
        minify,
    );
}

fn apply_to_children(
    children: &mut Vec<Node>,
    id_mapping: &HashMap<String, String>,
    referenced_ids: &HashSet<String>,
    preserve: &HashSet<String>,
    preserve_prefixes: &[String],
    remove: bool,
    minify: bool,
) {
    for child in children.iter_mut() {
        if let Node::Element(el) = child {
            // Handle id attribute
            if let Some(id) = el.attributes.get("id") {
                let id_str = id.clone();
                if referenced_ids.contains(&id_str) {
                    // Referenced: minify if needed
                    if minify {
                        if let Some(new_id) = id_mapping.get(&id_str) {
                            el.attributes.insert("id".to_string(), new_id.clone());
                        }
                    }
                } else if remove && !is_preserved(&id_str, preserve, preserve_prefixes) {
                    // Unreferenced: remove
                    el.attributes.swap_remove("id");
                }
            }

            // Update references in all attributes
            let keys: Vec<String> = el.attributes.keys().cloned().collect();
            for key in &keys {
                if let Some(value) = el.attributes.get(key) {
                    let mut new_value = value.clone();
                    for (old_id, new_id) in id_mapping {
                        if new_value.contains(&format!("#{}", old_id)) {
                            new_value =
                                new_value.replace(&format!("#{}", old_id), &format!("#{}", new_id));
                        }
                    }
                    if new_value != *value {
                        el.attributes.insert(key.clone(), new_value);
                    }
                }
            }

            // Recurse into children
            apply_to_children(
                &mut el.children,
                id_mapping,
                referenced_ids,
                preserve,
                preserve_prefixes,
                remove,
                minify,
            );
        }
    }
}

fn is_preserved(id: &str, preserve: &HashSet<String>, preserve_prefixes: &[String]) -> bool {
    if preserve.contains(id) {
        return true;
    }
    for prefix in preserve_prefixes {
        if id.starts_with(prefix) {
            return true;
        }
    }
    false
}

const GENERATE_ID_CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const MAX_ID_INDEX: usize = GENERATE_ID_CHARS.len() - 1;

fn generate_id(current_id: Option<&[usize]>) -> Vec<usize> {
    match current_id {
        None => vec![0],
        Some(id) => {
            let mut new_id = id.to_vec();
            let last = new_id.len() - 1;
            new_id[last] += 1;

            for i in (1..new_id.len()).rev() {
                if new_id[i] > MAX_ID_INDEX {
                    new_id[i] = 0;
                    new_id[i - 1] += 1;
                }
            }

            if new_id[0] > MAX_ID_INDEX {
                new_id[0] = 0;
                new_id.insert(0, 0);
            }

            new_id
        }
    }
}

fn get_id_string(arr: &[usize]) -> String {
    arr.iter().map(|&i| GENERATE_ID_CHARS[i] as char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn find_element<'a>(doc: &'a Document, name: &str) -> Option<&'a Element> {
        fn find_in_children<'a>(children: &'a [Node], name: &str) -> Option<&'a Element> {
            for child in children {
                if let Node::Element(el) = child {
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

    #[test]
    fn cleanup_ids_removes_unreferenced() {
        let plug = CleanupIds;
        let input =
            "<svg xmlns=\"http://www.w3.org/2000/svg\"><path id=\"unused\" d=\"M0 0\"/></svg>";
        let mut doc = svgo_core::parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(!path.attributes.contains_key("id"));
    }

    #[test]
    fn cleanup_ids_minifies_referenced() {
        let plug = CleanupIds;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><defs><linearGradient id=\"myGradient\"/></defs><path fill=\"url(#myGradient)\" d=\"M0 0\"/></svg>";
        let mut doc = svgo_core::parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let lg = find_element(&doc, "linearGradient").unwrap();
        let id = lg.attributes.get("id").unwrap();
        assert!(id.len() < "myGradient".len() || *id != "myGradient");
    }

    #[test]
    fn cleanup_ids_preserves_listed() {
        let plug = CleanupIds;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><defs><linearGradient id=\"keepThis\"/></defs><path fill=\"url(#keepThis)\" d=\"M0 0\"/></svg>";
        let mut doc = svgo_core::parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({"preserve": ["keepThis"]}));
        let lg = find_element(&doc, "linearGradient").unwrap();
        assert_eq!(lg.attributes.get("id").unwrap(), "keepThis");
    }

    #[test]
    fn cleanup_ids_skips_with_scripts() {
        let plug = CleanupIds;
        let input = "<svg xmlns=\"http://www.w3.org/2000/svg\"><script>alert(1)</script><path id=\"test\" d=\"M0 0\"/></svg>";
        let mut doc = svgo_core::parse(input).unwrap();
        plug.apply(&mut doc, &serde_json::json!({}));
        let path = find_element(&doc, "path").unwrap();
        assert!(path.attributes.contains_key("id"));
    }
}
