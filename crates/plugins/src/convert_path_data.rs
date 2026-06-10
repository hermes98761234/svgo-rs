use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

use crate::path::{parse_path_data, stringify_path_data, PathSeg};

/// Converts path data to a more compact and normalized form.
/// Ports convertPathData from SVGO.
pub struct ConvertPathData;

impl Plugin for ConvertPathData {
    fn name(&self) -> &'static str {
        "convertPathData"
    }

    fn apply(&self, doc: &mut Document, params: &serde_json::Value) {
        let float_precision = params
            .get("floatPrecision")
            .and_then(|v| v.as_u64())
            .map(|v| v as u8)
            .unwrap_or(3);
        let utilise_abs = params
            .get("utiliseAbsoluteCoords")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        struct V {
            float_precision: u8,
            utilise_abs: bool,
        }

        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                if let Some(d) = el.attributes.get("d").cloned() {
                    let segs = parse_path_data(&d);
                    if segs.is_empty() {
                        return VisitAction::Continue;
                    }
                    let optimized = optimize_path(segs, self.float_precision, self.utilise_abs);
                    let s = stringify_path_data(&optimized, Some(self.float_precision));
                    el.attributes.insert("d".to_string(), s);
                }
                VisitAction::Continue
            }
        }

        let mut v = V {
            float_precision,
            utilise_abs,
        };
        svgo_core::visitor::visit(doc, &mut v);
    }
}

fn round_f64(v: f64, precision: u8) -> f64 {
    let factor = 10f64.powi(precision as i32);
    (v * factor).round() / factor
}

/// Round args of a segment to precision.
fn round_seg(seg: &PathSeg, precision: u8) -> PathSeg {
    let args = seg.args.iter().map(|&a| round_f64(a, precision)).collect();
    PathSeg {
        command: seg.command,
        args,
    }
}

/// Convert an absolute segment to relative given current position.
fn to_relative(seg: &PathSeg, cx: f64, cy: f64) -> PathSeg {
    let cmd = seg.command;
    match cmd {
        b'M' => PathSeg {
            command: b'm',
            args: vec![seg.args[0] - cx, seg.args[1] - cy],
        },
        b'L' => PathSeg {
            command: b'l',
            args: vec![seg.args[0] - cx, seg.args[1] - cy],
        },
        b'H' => PathSeg {
            command: b'h',
            args: vec![seg.args[0] - cx],
        },
        b'V' => PathSeg {
            command: b'v',
            args: vec![seg.args[0] - cy],
        },
        b'C' => PathSeg {
            command: b'c',
            args: vec![
                seg.args[0] - cx,
                seg.args[1] - cy,
                seg.args[2] - cx,
                seg.args[3] - cy,
                seg.args[4] - cx,
                seg.args[5] - cy,
            ],
        },
        b'S' => PathSeg {
            command: b's',
            args: vec![
                seg.args[0] - cx,
                seg.args[1] - cy,
                seg.args[2] - cx,
                seg.args[3] - cy,
            ],
        },
        b'Q' => PathSeg {
            command: b'q',
            args: vec![
                seg.args[0] - cx,
                seg.args[1] - cy,
                seg.args[2] - cx,
                seg.args[3] - cy,
            ],
        },
        b'T' => PathSeg {
            command: b't',
            args: vec![seg.args[0] - cx, seg.args[1] - cy],
        },
        b'A' => PathSeg {
            command: b'a',
            args: vec![
                seg.args[0],
                seg.args[1],
                seg.args[2],
                seg.args[3],
                seg.args[4],
                seg.args[5] - cx,
                seg.args[6] - cy,
            ],
        },
        _ => seg.clone(),
    }
}

/// Convert a relative segment to absolute given current position.
fn to_absolute(seg: &PathSeg, cx: f64, cy: f64) -> PathSeg {
    let cmd = seg.command;
    match cmd {
        b'm' => PathSeg {
            command: b'M',
            args: vec![seg.args[0] + cx, seg.args[1] + cy],
        },
        b'l' => PathSeg {
            command: b'L',
            args: vec![seg.args[0] + cx, seg.args[1] + cy],
        },
        b'h' => PathSeg {
            command: b'H',
            args: vec![seg.args[0] + cx],
        },
        b'v' => PathSeg {
            command: b'V',
            args: vec![seg.args[0] + cy],
        },
        b'c' => PathSeg {
            command: b'C',
            args: vec![
                seg.args[0] + cx,
                seg.args[1] + cy,
                seg.args[2] + cx,
                seg.args[3] + cy,
                seg.args[4] + cx,
                seg.args[5] + cy,
            ],
        },
        b's' => PathSeg {
            command: b'S',
            args: vec![
                seg.args[0] + cx,
                seg.args[1] + cy,
                seg.args[2] + cx,
                seg.args[3] + cy,
            ],
        },
        b'q' => PathSeg {
            command: b'Q',
            args: vec![
                seg.args[0] + cx,
                seg.args[1] + cy,
                seg.args[2] + cx,
                seg.args[3] + cy,
            ],
        },
        b't' => PathSeg {
            command: b'T',
            args: vec![seg.args[0] + cx, seg.args[1] + cy],
        },
        b'a' => PathSeg {
            command: b'A',
            args: vec![
                seg.args[0],
                seg.args[1],
                seg.args[2],
                seg.args[3],
                seg.args[4],
                seg.args[5] + cx,
                seg.args[6] + cy,
            ],
        },
        _ => seg.clone(),
    }
}

/// Advance current position based on segment.
fn advance_position(
    seg: &PathSeg,
    cx: &mut f64,
    cy: &mut f64,
    start_x: &mut f64,
    start_y: &mut f64,
) {
    match seg.command {
        b'M' => {
            *cx = seg.args[0];
            *cy = seg.args[1];
            *start_x = *cx;
            *start_y = *cy;
        }
        b'm' => {
            *cx += seg.args[0];
            *cy += seg.args[1];
            *start_x = *cx;
            *start_y = *cy;
        }
        b'L' => {
            *cx = seg.args[0];
            *cy = seg.args[1];
        }
        b'l' => {
            *cx += seg.args[0];
            *cy += seg.args[1];
        }
        b'H' => {
            *cx = seg.args[0];
        }
        b'h' => {
            *cx += seg.args[0];
        }
        b'V' => {
            *cy = seg.args[0];
        }
        b'v' => {
            *cy += seg.args[0];
        }
        b'C' => {
            *cx = seg.args[4];
            *cy = seg.args[5];
        }
        b'c' => {
            *cx += seg.args[4];
            *cy += seg.args[5];
        }
        b'S' => {
            *cx = seg.args[2];
            *cy = seg.args[3];
        }
        b's' => {
            *cx += seg.args[2];
            *cy += seg.args[3];
        }
        b'Q' => {
            *cx = seg.args[2];
            *cy = seg.args[3];
        }
        b'q' => {
            *cx += seg.args[2];
            *cy += seg.args[3];
        }
        b'T' => {
            *cx = seg.args[0];
            *cy = seg.args[1];
        }
        b't' => {
            *cx += seg.args[0];
            *cy += seg.args[1];
        }
        b'A' => {
            *cx = seg.args[5];
            *cy = seg.args[6];
        }
        b'a' => {
            *cx += seg.args[5];
            *cy += seg.args[6];
        }
        b'Z' | b'z' => {
            *cx = *start_x;
            *cy = *start_y;
        }
        _ => {}
    }
}

fn is_abs(cmd: u8) -> bool {
    cmd.is_ascii_uppercase()
}

fn stringify_single(seg: &PathSeg, precision: u8) -> String {
    stringify_path_data(std::slice::from_ref(seg), Some(precision))
}

/// Optimize path: round precision, convert to shorter abs/rel form.
fn optimize_path(segs: Vec<PathSeg>, precision: u8, utilise_abs: bool) -> Vec<PathSeg> {
    let mut result = Vec::with_capacity(segs.len());
    let mut cx = 0f64;
    let mut cy = 0f64;
    let mut start_x = 0f64;
    let mut start_y = 0f64;

    for seg in &segs {
        // For Z/z, just pass through
        if seg.command == b'Z' || seg.command == b'z' {
            result.push(PathSeg {
                command: b'z',
                args: vec![],
            });
            advance_position(seg, &mut cx, &mut cy, &mut start_x, &mut start_y);
            continue;
        }

        let rounded = round_seg(seg, precision);

        if !utilise_abs {
            advance_position(&rounded, &mut cx, &mut cy, &mut start_x, &mut start_y);
            result.push(rounded);
            continue;
        }

        // Choose shorter of absolute vs relative
        let (abs_seg, rel_seg) = if is_abs(seg.command) {
            let abs = rounded.clone();
            let rel = to_relative(&rounded, cx, cy);
            (abs, rel)
        } else {
            let rel = rounded.clone();
            let abs = to_absolute(&rounded, cx, cy);
            (round_seg(&abs, precision), rel)
        };

        let abs_str = stringify_single(&abs_seg, precision);
        let rel_str = stringify_single(&rel_seg, precision);

        let chosen = if rel_str.len() <= abs_str.len() {
            rel_seg.clone()
        } else {
            abs_seg.clone()
        };

        advance_position(&chosen, &mut cx, &mut cy, &mut start_x, &mut start_y);
        result.push(chosen);
    }

    result
}
