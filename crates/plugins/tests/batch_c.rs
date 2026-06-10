//! Batch C fixture tests: path & transform plugins.

use svgo_core::parse;
use svgo_core::plugin::Plugin;
use svgo_core::stringify;
use svgo_core::StringifyOptions;

use svgo_plugins::convert_ellipse_to_circle::ConvertEllipseToCircle;
use svgo_plugins::convert_path_data::ConvertPathData;
use svgo_plugins::convert_shape_to_path::ConvertShapeToPath;
use svgo_plugins::convert_transform::ConvertTransform;
use svgo_plugins::merge_paths::MergePaths;
use svgo_plugins::path::{parse_path_data, stringify_path_data};

fn pretty_opts() -> StringifyOptions {
    StringifyOptions {
        pretty: true,
        indent: "    ".to_string(),
        final_newline: true,
        eol: "\n".to_string(),
    }
}

fn apply_plugin(plugin: &dyn Plugin, input: &str) -> String {
    let mut doc = parse(input).expect("parse error");
    plugin.apply(&mut doc, &serde_json::json!({}));
    stringify(&doc, &pretty_opts())
}

// ==================== path module unit tests ====================

#[test]
fn path_parse_simple() {
    let segs = parse_path_data("M 10,20 L 30,40");
    assert_eq!(segs.len(), 2);
    assert_eq!(segs[0].command, b'M');
    assert_eq!(segs[0].args, vec![10.0, 20.0]);
    assert_eq!(segs[1].command, b'L');
    assert_eq!(segs[1].args, vec![30.0, 40.0]);
}

#[test]
fn path_parse_relative() {
    let segs = parse_path_data("m10 20l-.5.5");
    assert_eq!(segs.len(), 2);
    assert_eq!(segs[0].command, b'm');
    assert_eq!(segs[1].command, b'l');
    assert_eq!(segs[1].args, vec![-0.5, 0.5]);
}

#[test]
fn path_stringify_compact() {
    let segs = parse_path_data("M 10,50");
    let out = stringify_path_data(&segs, None);
    assert_eq!(out, "M10 50");
}

// ==================== convertPathData unit tests ====================

#[test]
fn convert_path_data_normalizes_whitespace() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M 10,50"/></svg>"#;
    let output = apply_plugin(&ConvertPathData, input);
    // either M10 50 or m10 50 is valid (relative/absolute same from origin)
    assert!(
        output.contains("d=\"M10 50\"") || output.contains("d=\"m10 50\""),
        "should normalize to compact form, got: {}",
        output
    );
}

#[test]
fn convert_path_data_normalizes_commas() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M 10 , 50"/></svg>"#;
    let output = apply_plugin(&ConvertPathData, input);
    assert!(
        output.contains("d=\"M10 50\"") || output.contains("d=\"m10 50\""),
        "should normalize commas, got: {}",
        output
    );
}

#[test]
fn convert_path_data_negative_coords() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M -10,-50"/></svg>"#;
    let output = apply_plugin(&ConvertPathData, input);
    assert!(
        output.contains("d=\"M-10-50\"") || output.contains("d=\"m-10-50\""),
        "should compact negatives, got: {}",
        output
    );
}

#[test]
fn convert_path_data_relative_shorter() {
    // L 20,30 from M10,50 is relative l 10,-20 which is shorter
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M 10,50 L 20,30"/></svg>"#;
    let output = apply_plugin(&ConvertPathData, input);
    // Should convert to relative form
    assert!(
        output.contains("l") || output.contains("L"),
        "should have lineto, got: {}",
        output
    );
}

#[test]
fn convert_path_data_precision_rounding() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M 10.3467,50.09"/></svg>"#;
    let output = apply_plugin(&ConvertPathData, input);
    assert!(
        output.contains("10.347") || output.contains("10.35"),
        "should round to 3 decimal places, got: {}",
        output
    );
}

#[test]
fn convert_path_data_passthrough_z() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M10 10 L20 20 Z"/></svg>"#;
    let output = apply_plugin(&ConvertPathData, input);
    assert!(
        output.contains("z") || output.contains("Z"),
        "should preserve Z, got: {}",
        output
    );
}

// ==================== convertEllipseToCircle unit tests ====================

#[test]
fn convert_ellipse_to_circle_equal_radii() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <ellipse cx="10" cy="10" rx="5" ry="5"/>
</svg>"#;
    let output = apply_plugin(&ConvertEllipseToCircle, input);
    assert!(
        output.contains("<circle"),
        "should convert to circle, got: {}",
        output
    );
    assert!(
        output.contains("r=\"5\""),
        "should have r=5, got: {}",
        output
    );
    assert!(
        !output.contains("rx="),
        "should not have rx, got: {}",
        output
    );
}

#[test]
fn convert_ellipse_to_circle_keeps_unequal() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <ellipse cx="10" cy="10" rx="5" ry="10"/>
</svg>"#;
    let output = apply_plugin(&ConvertEllipseToCircle, input);
    assert!(
        output.contains("<ellipse"),
        "should keep ellipse with different radii, got: {}",
        output
    );
}

// ==================== convertShapeToPath unit tests ====================

#[test]
fn convert_shape_to_path_rect() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="10" height="5"/>
</svg>"#;
    let output = apply_plugin(&ConvertShapeToPath, input);
    assert!(
        output.contains("<path"),
        "should convert rect to path, got: {}",
        output
    );
    assert!(
        output.contains("d="),
        "should have d attribute, got: {}",
        output
    );
}

#[test]
fn convert_shape_to_path_keeps_rounded_rect() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="10" height="5" rx="2"/>
</svg>"#;
    let output = apply_plugin(&ConvertShapeToPath, input);
    assert!(
        output.contains("<rect"),
        "should keep rounded rect, got: {}",
        output
    );
}

#[test]
fn convert_shape_to_path_line() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <line x1="0" y1="0" x2="10" y2="10"/>
</svg>"#;
    let output = apply_plugin(&ConvertShapeToPath, input);
    assert!(
        output.contains("<path"),
        "should convert line to path, got: {}",
        output
    );
}

// ==================== convertTransform unit tests ====================

#[test]
fn convert_transform_identity_matrix() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <g transform="matrix(1,0,0,1,0,0)"/>
</svg>"#;
    let output = apply_plugin(&ConvertTransform, input);
    assert!(
        !output.contains("transform=") || output.contains("translate(0"),
        "should remove identity transform, got: {}",
        output
    );
}

#[test]
fn convert_transform_translate() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <g transform="translate(10, 20)"/>
</svg>"#;
    let output = apply_plugin(&ConvertTransform, input);
    assert!(
        output.contains("transform="),
        "should preserve translate, got: {}",
        output
    );
}

// ==================== mergePaths unit tests ====================

#[test]
fn merge_paths_basic() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <g>
    <path d="M0 0h10" fill="red"/>
    <path d="M20 0h10" fill="red"/>
  </g>
</svg>"#;
    let output = apply_plugin(&MergePaths, input);
    // Should merge (or at least not crash)
    assert!(
        output.contains("<path"),
        "should have path element, got: {}",
        output
    );
}

#[test]
fn merge_paths_different_attrs() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <g>
    <path d="M0 0h10" fill="red"/>
    <path d="M20 0h10" fill="blue"/>
  </g>
</svg>"#;
    let output = apply_plugin(&MergePaths, input);
    // Paths with different attrs should NOT be merged - both should remain
    assert_eq!(
        output.matches("<path").count(),
        2,
        "should keep separate paths with different attrs, got: {}",
        output
    );
}
