use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::path::{parse_path_data, stringify_path_data, PathSeg};

/// Converts basic shapes (rect, line, polyline, polygon) to path elements.
/// Optionally converts circle/ellipse when `convertArcs` is true.
pub struct ConvertShapeToPath;

impl Plugin for ConvertShapeToPath {
    fn name(&self) -> &'static str {
        "convertShapeToPath"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let convert_arcs = params
            .get("convertArcs")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let precision = params
            .get("floatPrecision")
            .and_then(|v| v.as_u64())
            .map(|v| v as u8);

        struct V {
            convert_arcs: bool,
            precision: Option<u8>,
        }
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                // convert rect to path
                if el.name == "rect" {
                    if let (Some(width), Some(height)) =
                        (el.attributes.get("width"), el.attributes.get("height"))
                    {
                        if el.attributes.contains_key("rx") || el.attributes.contains_key("ry") {
                            return VisitAction::Continue;
                        }
                        let x: f64 = el
                            .attributes
                            .get("x")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let y: f64 = el
                            .attributes
                            .get("y")
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0.0);
                        let width: f64 = width.parse().unwrap_or(f64::NAN);
                        let height: f64 = height.parse().unwrap_or(f64::NAN);
                        if (x - y + width - height).is_nan() {
                            return VisitAction::Continue;
                        }
                        let path_data = vec![
                            PathSeg {
                                command: b'M',
                                args: vec![x, y],
                            },
                            PathSeg {
                                command: b'H',
                                args: vec![x + width],
                            },
                            PathSeg {
                                command: b'V',
                                args: vec![y + height],
                            },
                            PathSeg {
                                command: b'H',
                                args: vec![x],
                            },
                            PathSeg {
                                command: b'z',
                                args: vec![],
                            },
                        ];
                        el.name = "path".to_string();
                        el.attributes.insert(
                            "d".to_string(),
                            stringify_path_data(&path_data, self.precision),
                        );
                        el.attributes.shift_remove("x");
                        el.attributes.shift_remove("y");
                        el.attributes.shift_remove("width");
                        el.attributes.shift_remove("height");
                    }
                }

                // convert line to path
                if el.name == "line" {
                    let x1: f64 = el
                        .attributes
                        .get("x1")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let y1: f64 = el
                        .attributes
                        .get("y1")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let x2: f64 = el
                        .attributes
                        .get("x2")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let y2: f64 = el
                        .attributes
                        .get("y2")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    if (x1 - y1 + x2 - y2).is_nan() {
                        return VisitAction::Continue;
                    }
                    let path_data = vec![
                        PathSeg {
                            command: b'M',
                            args: vec![x1, y1],
                        },
                        PathSeg {
                            command: b'L',
                            args: vec![x2, y2],
                        },
                    ];
                    el.name = "path".to_string();
                    el.attributes.insert(
                        "d".to_string(),
                        stringify_path_data(&path_data, self.precision),
                    );
                    el.attributes.shift_remove("x1");
                    el.attributes.shift_remove("y1");
                    el.attributes.shift_remove("x2");
                    el.attributes.shift_remove("y2");
                }

                // convert polyline and polygon to path
                if el.name == "polyline" || el.name == "polygon" {
                    if let Some(points) = el.attributes.get("points") {
                        let coords: Vec<f64> = points
                            .split(|c: char| c.is_whitespace() || c == ',')
                            .filter(|s| !s.is_empty())
                            .filter_map(|s| s.parse::<f64>().ok())
                            .collect();
                        if coords.len() < 4 {
                            // Remove element with insufficient points
                            return VisitAction::Remove;
                        }
                        let mut path_data = Vec::new();
                        for (i, chunk) in coords.chunks(2).enumerate() {
                            if chunk.len() == 2 {
                                let cmd = if i == 0 { b'M' } else { b'L' };
                                path_data.push(PathSeg {
                                    command: cmd,
                                    args: vec![chunk[0], chunk[1]],
                                });
                            }
                        }
                        if el.name == "polygon" {
                            path_data.push(PathSeg {
                                command: b'z',
                                args: vec![],
                            });
                        }
                        el.name = "path".to_string();
                        el.attributes.insert(
                            "d".to_string(),
                            stringify_path_data(&path_data, self.precision),
                        );
                        el.attributes.shift_remove("points");
                    }
                }

                // optionally convert circle
                if el.name == "circle" && self.convert_arcs {
                    let cx: f64 = el
                        .attributes
                        .get("cx")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let cy: f64 = el
                        .attributes
                        .get("cy")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let r: f64 = el
                        .attributes
                        .get("r")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    if (cx - cy + r).is_nan() {
                        return VisitAction::Continue;
                    }
                    let path_data = vec![
                        PathSeg {
                            command: b'M',
                            args: vec![cx, cy - r],
                        },
                        PathSeg {
                            command: b'A',
                            args: vec![r, r, 0.0, 1.0, 0.0, cx, cy + r],
                        },
                        PathSeg {
                            command: b'A',
                            args: vec![r, r, 0.0, 1.0, 0.0, cx, cy - r],
                        },
                        PathSeg {
                            command: b'z',
                            args: vec![],
                        },
                    ];
                    el.name = "path".to_string();
                    el.attributes.insert(
                        "d".to_string(),
                        stringify_path_data(&path_data, self.precision),
                    );
                    el.attributes.shift_remove("cx");
                    el.attributes.shift_remove("cy");
                    el.attributes.shift_remove("r");
                }

                // optionally convert ellipse
                if el.name == "ellipse" && self.convert_arcs {
                    let ecx: f64 = el
                        .attributes
                        .get("cx")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let ecy: f64 = el
                        .attributes
                        .get("cy")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let rx: f64 = el
                        .attributes
                        .get("rx")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let ry: f64 = el
                        .attributes
                        .get("ry")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    if (ecx - ecy + rx - ry).is_nan() {
                        return VisitAction::Continue;
                    }
                    let path_data = vec![
                        PathSeg {
                            command: b'M',
                            args: vec![ecx, ecy - ry],
                        },
                        PathSeg {
                            command: b'A',
                            args: vec![rx, ry, 0.0, 1.0, 0.0, ecx, ecy + ry],
                        },
                        PathSeg {
                            command: b'A',
                            args: vec![rx, ry, 0.0, 1.0, 0.0, ecx, ecy - ry],
                        },
                        PathSeg {
                            command: b'z',
                            args: vec![],
                        },
                    ];
                    el.name = "path".to_string();
                    el.attributes.insert(
                        "d".to_string(),
                        stringify_path_data(&path_data, self.precision),
                    );
                    el.attributes.shift_remove("cx");
                    el.attributes.shift_remove("cy");
                    el.attributes.shift_remove("rx");
                    el.attributes.shift_remove("ry");
                }

                VisitAction::Continue
            }
        }
        let mut v = V {
            convert_arcs,
            precision,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}
