use svgo_core::ast::{Element, Node};
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::collections::pathElems;
use crate::path::{parse_path_data, stringify_path_data, PathSeg};

/// Merges multiple paths into one where possible.
pub struct MergePaths;

impl Plugin for MergePaths {
    fn name(&self) -> &'static str {
        "mergePaths"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let _force = params
            .get("force")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let precision = params
            .get("floatPrecision")
            .and_then(|v| v.as_u64())
            .map(|v| v as u8);

        struct V {
            precision: Option<u8>,
        }
        impl Visitor for V {
            fn element_enter(&mut self, el: &mut Element, _ctx: &Context) -> VisitAction {
                if el.children.len() <= 1 {
                    return VisitAction::Continue;
                }

                let mut elements_to_remove: Vec<usize> = Vec::new();
                let mut prev_child_idx: usize = 0;
                let mut prev_path_data: Option<Vec<PathSeg>> = None;

                for i in 1..el.children.len() {
                    let prev_is_path = matches!(
                        &el.children[prev_child_idx],
                        Node::Element(ref e)
                            if pathElems.contains(e.name.as_str())
                                && e.children.is_empty()
                                && e.attributes.contains_key("d")
                    );

                    if !prev_is_path {
                        if let Some(ref pd) = prev_path_data {
                            update_prev_path(&mut el.children[prev_child_idx], pd, self.precision);
                        }
                        prev_child_idx = i;
                        prev_path_data = None;
                        continue;
                    }

                    let child_is_path = matches!(
                        &el.children[i],
                        Node::Element(ref e)
                            if pathElems.contains(e.name.as_str())
                                && e.children.is_empty()
                                && e.attributes.contains_key("d")
                    );

                    if !child_is_path {
                        if let Some(ref pd) = prev_path_data {
                            update_prev_path(&mut el.children[prev_child_idx], pd, self.precision);
                        }
                        prev_child_idx = i;
                        prev_path_data = None;
                        continue;
                    }

                    // Check if attributes match (excluding 'd')
                    let prev_attrs_match = match (&el.children[prev_child_idx], &el.children[i]) {
                        (Node::Element(prev), Node::Element(curr)) => {
                            let prev_keys: Vec<&str> = prev
                                .attributes
                                .keys()
                                .filter(|k| *k != "d")
                                .map(|k| k.as_str())
                                .collect();
                            let curr_keys: Vec<&str> = curr
                                .attributes
                                .keys()
                                .filter(|k| *k != "d")
                                .map(|k| k.as_str())
                                .collect();
                            if prev_keys.len() != curr_keys.len() {
                                false
                            } else {
                                prev_keys
                                    .iter()
                                    .all(|k| curr.attributes.get(*k) == prev.attributes.get(*k))
                            }
                        }
                        _ => false,
                    };

                    if !prev_attrs_match {
                        if let Some(ref pd) = prev_path_data {
                            update_prev_path(&mut el.children[prev_child_idx], pd, self.precision);
                        }
                        prev_child_idx = i;
                        prev_path_data = None;
                        continue;
                    }

                    // Both are paths with matching attributes - try to merge
                    let current_path_data = match &el.children[i] {
                        Node::Element(e) => {
                            let d = e.attributes.get("d").map(|s| s.as_str()).unwrap_or("");
                            parse_path_data(d)
                        }
                        _ => unreachable!(),
                    };

                    if prev_path_data.is_none() {
                        prev_path_data = Some(match &el.children[prev_child_idx] {
                            Node::Element(e) => {
                                let d = e.attributes.get("d").map(|s| s.as_str()).unwrap_or("");
                                parse_path_data(d)
                            }
                            _ => unreachable!(),
                        });
                    }

                    // Simple bounding-box intersection check
                    let no_intersect = {
                        let prev_data = prev_path_data.as_ref().unwrap();
                        !intersects(prev_data, &current_path_data)
                    };
                    if no_intersect {
                        prev_path_data
                            .as_mut()
                            .unwrap()
                            .extend_from_slice(&current_path_data);
                        elements_to_remove.push(i);
                    } else {
                        update_prev_path(
                            &mut el.children[prev_child_idx],
                            prev_path_data.as_ref().unwrap(),
                            self.precision,
                        );
                        prev_child_idx = i;
                        prev_path_data = None;
                    }
                }

                if let Some(ref pd) = prev_path_data {
                    update_prev_path(&mut el.children[prev_child_idx], pd, self.precision);
                }

                // Remove merged children (in reverse order to preserve indices)
                for &idx in elements_to_remove.iter().rev() {
                    el.children.remove(idx);
                }

                VisitAction::Continue
            }
        }
        let mut v = V { precision };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

fn update_prev_path(child: &mut Node, path_data: &[PathSeg], precision: Option<u8>) {
    if let Node::Element(ref mut el) = child {
        el.attributes
            .insert("d".to_string(), stringify_path_data(path_data, precision));
    }
}

/// Check if two paths intersect using bounding box check.
fn intersects(path1: &[PathSeg], path2: &[PathSeg]) -> bool {
    let bb1 = bounding_box(path1);
    let bb2 = bounding_box(path2);

    // If bounding boxes don't overlap, paths don't intersect
    bb1.0 > bb2.2 || bb2.0 > bb1.2 || bb1.1 > bb2.3 || bb2.1 > bb1.3
}

/// Compute bounding box of a path: (min_x, min_y, max_x, max_y)
fn bounding_box(path: &[PathSeg]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut cursor_x = 0.0;
    let mut cursor_y = 0.0;
    let mut start_x = 0.0;
    let mut start_y = 0.0;

    for seg in path {
        match seg.command {
            b'M' => {
                cursor_x = seg.args[0];
                cursor_y = seg.args[1];
                start_x = cursor_x;
                start_y = cursor_y;
            }
            b'm' => {
                cursor_x += seg.args[0];
                cursor_y += seg.args[1];
                start_x = cursor_x;
                start_y = cursor_y;
            }
            b'L' | b'T' => {
                cursor_x = seg.args[seg.args.len() - 2];
                cursor_y = seg.args[seg.args.len() - 1];
            }
            b'l' | b't' => {
                cursor_x += seg.args[seg.args.len() - 2];
                cursor_y += seg.args[seg.args.len() - 1];
            }
            b'H' => {
                cursor_x = seg.args[0];
            }
            b'h' => {
                cursor_x += seg.args[0];
            }
            b'V' => {
                cursor_y = seg.args[0];
            }
            b'v' => {
                cursor_y += seg.args[0];
            }
            b'C' => {
                for (i, &val) in seg.args.iter().enumerate() {
                    if i % 2 == 0 {
                        min_x = min_x.min(val);
                        max_x = max_x.max(val);
                    } else {
                        min_y = min_y.min(val);
                        max_y = max_y.max(val);
                    }
                }
                cursor_x = seg.args[4];
                cursor_y = seg.args[5];
            }
            b'c' => {
                for (i, &val) in seg.args.iter().enumerate() {
                    let abs_val = if i % 2 == 0 {
                        val + cursor_x
                    } else {
                        val + cursor_y
                    };
                    if i % 2 == 0 {
                        min_x = min_x.min(abs_val);
                        max_x = max_x.max(abs_val);
                    } else {
                        min_y = min_y.min(abs_val);
                        max_y = max_y.max(abs_val);
                    }
                }
                cursor_x += seg.args[4];
                cursor_y += seg.args[5];
            }
            b'S' | b'Q' => {
                cursor_x = seg.args[seg.args.len() - 2];
                cursor_y = seg.args[seg.args.len() - 1];
                for (i, &val) in seg.args.iter().enumerate() {
                    if i % 2 == 0 {
                        min_x = min_x.min(val);
                        max_x = max_x.max(val);
                    } else {
                        min_y = min_y.min(val);
                        max_y = max_y.max(val);
                    }
                }
            }
            b's' | b'q' => {
                cursor_x += seg.args[seg.args.len() - 2];
                cursor_y += seg.args[seg.args.len() - 1];
                for (i, &val) in seg.args.iter().enumerate() {
                    let abs_val = if i % 2 == 0 {
                        val + cursor_x
                    } else {
                        val + cursor_y
                    };
                    if i % 2 == 0 {
                        min_x = min_x.min(abs_val);
                        max_x = max_x.max(abs_val);
                    } else {
                        min_y = min_y.min(abs_val);
                        max_y = max_y.max(abs_val);
                    }
                }
            }
            b'A' => {
                cursor_x = seg.args[5];
                cursor_y = seg.args[6];
                // Approximate arc bounds
                let rx = seg.args[0].abs();
                let ry = seg.args[1].abs();
                min_x = min_x.min(cursor_x - rx);
                max_x = max_x.max(cursor_x + rx);
                min_y = min_y.min(cursor_y - ry);
                max_y = max_y.max(cursor_y + ry);
            }
            b'a' => {
                cursor_x += seg.args[5];
                cursor_y += seg.args[6];
                let rx = seg.args[0].abs();
                let ry = seg.args[1].abs();
                min_x = min_x.min(cursor_x - rx);
                max_x = max_x.max(cursor_x + rx);
                min_y = min_y.min(cursor_y - ry);
                max_y = max_y.max(cursor_y + ry);
            }
            b'Z' | b'z' => {
                cursor_x = start_x;
                cursor_y = start_y;
            }
            _ => {}
        }

        min_x = min_x.min(cursor_x);
        min_y = min_y.min(cursor_y);
        max_x = max_x.max(cursor_x);
        max_y = max_y.max(cursor_y);
    }

    if min_x == f64::INFINITY {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

use svgo_core::ast::Document;
