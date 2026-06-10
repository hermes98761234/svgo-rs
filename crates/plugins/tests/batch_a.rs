//! Batch A unit tests: removal & cleanup plugins.
//!
//! These tests verify core plugin logic without depending on exact
//! stringifier output formatting.

use svgo_core::ast::{Document, Node};
use svgo_core::parse;
use svgo_core::plugin::Plugin;

fn find_child_element<'a>(doc: &'a Document, name: &str) -> Option<&'a svgo_core::ast::Element> {
    fn find_in_children<'a>(
        children: &'a [Node],
        name: &str,
    ) -> Option<&'a svgo_core::ast::Element> {
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

fn has_child_element(doc: &Document, name: &str) -> bool {
    find_child_element(doc, name).is_some()
}

fn child_element_count(doc: &Document, parent_name: &str, child_name: &str) -> usize {
    fn find_parent<'a>(children: &'a [Node], name: &str) -> Option<&'a svgo_core::ast::Element> {
        for child in children {
            if let Node::Element(el) = child {
                if el.name == name {
                    return Some(el);
                }
                if let Some(found) = find_parent(&el.children, name) {
                    return Some(found);
                }
            }
        }
        None
    }
    if let Some(parent) = find_parent(&doc.children, parent_name) {
        parent
            .children
            .iter()
            .filter(|n| matches!(n, Node::Element(el) if el.name == child_name))
            .count()
    } else {
        0
    }
}

// ==================== removeDoctype ====================

#[test]
fn remove_doctype_removes_doctype() {
    let plug = svgo_plugins::remove_doctype::RemoveDoctype;
    let input = r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg xmlns="http://www.w3.org/2000/svg">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    // Doctype should be removed
    let has_doctype = doc.children.iter().any(|n| matches!(n, Node::Doctype(_)));
    assert!(!has_doctype, "Doctype should be removed");
    // SVG should remain
    assert!(has_child_element(&doc, "svg"), "SVG element should remain");
}

// ==================== removeXMLProcInst ====================

#[test]
fn remove_xml_proc_inst_removes_xml_decl() {
    let plug = svgo_plugins::remove_xml_proc_inst::RemoveXmlProcInst;
    let input = r#"<?xml version="1.0" encoding="utf-8"?>
<svg xmlns="http://www.w3.org/2000/svg">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let has_pi = doc
        .children
        .iter()
        .any(|n| matches!(n, Node::Instruction { name, .. } if name == "xml"));
    assert!(!has_pi, "XML processing instruction should be removed");
}

// ==================== removeComments ====================

#[test]
fn remove_comments_removes_comments() {
    let plug = svgo_plugins::remove_comments::RemoveComments;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <!-- test comment -->
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let has_comment = doc.children.iter().any(|n| matches!(n, Node::Comment(_)));
    assert!(!has_comment, "Comments should be removed");
}

#[test]
fn remove_comments_preserves_legal_comments() {
    let plug = svgo_plugins::remove_comments::RemoveComments;
    let input = r#"<!--!Copyright 2023-->
<svg xmlns="http://www.w3.org/2000/svg">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let has_comment = doc
        .children
        .iter()
        .any(|n| matches!(n, Node::Comment(c) if c.contains("Copyright")));
    assert!(
        has_comment,
        "Legal comments (starting with !) should be preserved"
    );
}

// ==================== removeMetadata ====================

#[test]
fn remove_metadata_removes_metadata() {
    let plug = svgo_plugins::remove_metadata::RemoveMetadata;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <metadata>...</metadata>
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "metadata"),
        "Metadata should be removed"
    );
    assert!(has_child_element(&doc, "g"), "g element should remain");
}

// ==================== removeEditorsNSData ====================

#[test]
fn remove_editors_ns_data_removes_editor_ns() {
    let plug = svgo_plugins::remove_editors_ns_data::RemoveEditorsNSData;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd">
    <path d="M0 0" sodipodi:nodetypes="cccc"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    // sodipodi namespace should be removed
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        !svg.attributes.contains_key("xmlns:sodipodi"),
        "sodipodi namespace should be removed"
    );
    // sodipodi attributes should be removed
    let path = find_child_element(&doc, "path").unwrap();
    assert!(
        !path.attributes.contains_key("sodipodi:nodetypes"),
        "sodipodi attributes should be removed"
    );
}

// ==================== removeDesc ====================

#[test]
fn remove_desc_removes_standard_desc() {
    let plug = svgo_plugins::remove_desc::RemoveDesc;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <desc>Created with Sketch.</desc>
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "desc"),
        "Standard desc should be removed"
    );
}

#[test]
fn remove_desc_removes_empty_desc() {
    let plug = svgo_plugins::remove_desc::RemoveDesc;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <desc></desc>
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "desc"),
        "Empty desc should be removed"
    );
}

#[test]
fn remove_desc_keeps_custom_desc_by_default() {
    let plug = svgo_plugins::remove_desc::RemoveDesc;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <desc>Custom description</desc>
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        has_child_element(&doc, "desc"),
        "Custom desc should be kept by default"
    );
}

#[test]
fn remove_desc_removes_any_with_param() {
    let plug = svgo_plugins::remove_desc::RemoveDesc;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <desc>Custom description</desc>
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({"removeAny": true}));
    assert!(
        !has_child_element(&doc, "desc"),
        "Any desc should be removed with removeAny=true"
    );
}

// ==================== removeUselessDefs ====================

#[test]
fn remove_useless_defs_keeps_defs_with_id() {
    let plug = svgo_plugins::remove_useless_defs::RemoveUselessDefs;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="grad1"/>
    </defs>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        has_child_element(&doc, "linearGradient"),
        "defs with id should be kept"
    );
}

// ==================== removeEmptyAttrs ====================

#[test]
fn remove_empty_attrs_removes_empty_attrs() {
    let plug = svgo_plugins::remove_empty_attrs::RemoveEmptyAttrs;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g attr1="" attr2=""/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let g = find_child_element(&doc, "g").unwrap();
    assert!(
        !g.attributes.contains_key("attr1"),
        "Empty attr1 should be removed"
    );
    assert!(
        !g.attributes.contains_key("attr2"),
        "Empty attr2 should be removed"
    );
}

#[test]
fn remove_empty_attrs_preserves_conditional_processing() {
    let plug = svgo_plugins::remove_empty_attrs::RemoveEmptyAttrs;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g requiredFeatures=""/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let g = find_child_element(&doc, "g").unwrap();
    assert!(
        g.attributes.contains_key("requiredFeatures"),
        "Empty conditional processing attrs should be preserved"
    );
}

// ==================== removeEmptyText ====================

#[test]
fn remove_empty_text_removes_empty_text() {
    let plug = svgo_plugins::remove_empty_text::RemoveEmptyText;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g>
        <text></text>
    </g>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert_eq!(
        child_element_count(&doc, "g", "text"),
        0,
        "Empty text should be removed"
    );
}

#[test]
fn remove_empty_text_removes_empty_tspan() {
    let plug = svgo_plugins::remove_empty_text::RemoveEmptyText;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g>
        <tspan></tspan>
    </g>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert_eq!(
        child_element_count(&doc, "g", "tspan"),
        0,
        "Empty tspan should be removed"
    );
}

#[test]
fn remove_empty_text_removes_tref_without_href() {
    let plug = svgo_plugins::remove_empty_text::RemoveEmptyText;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g>
        <tref>...</tref>
    </g>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert_eq!(
        child_element_count(&doc, "g", "tref"),
        0,
        "tref without xlink:href should be removed"
    );
}

// ==================== removeEmptyContainers ====================

#[test]
fn remove_empty_containers_removes_empty_g() {
    let plug = svgo_plugins::remove_empty_containers::RemoveEmptyContainers;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g>
        <g/>
    </g>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    // Inner empty g should be removed
    let outer_g = find_child_element(&doc, "g").unwrap();
    assert_eq!(
        outer_g
            .children
            .iter()
            .filter(|n| matches!(n, Node::Element(el) if el.name == "g"))
            .count(),
        0,
        "Empty inner g should be removed"
    );
}

#[test]
fn remove_empty_containers_keeps_svg() {
    let plug = svgo_plugins::remove_empty_containers::RemoveEmptyContainers;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    // svg should not be removed even if empty
    assert!(has_child_element(&doc, "svg"), "SVG should not be removed");
}

// ==================== removeHiddenElems ====================

#[test]
fn remove_hidden_elems_removes_display_none() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g display="none"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        child_element_count(&doc, "svg", "g") == 0,
        "g with display:none should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_visibility_hidden() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g visibility="hidden"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        child_element_count(&doc, "svg", "g") == 0,
        "g with visibility:hidden should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_circle_r0() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <circle r="0"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "circle"),
        "circle with r=0 should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_ellipse_rx0() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <ellipse rx="0"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "ellipse"),
        "ellipse with rx=0 should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_rect_width0() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <rect width="0"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "rect"),
        "rect with width=0 should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_pattern_width0() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <pattern width="0"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "pattern"),
        "pattern with width=0 should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_image_width0() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <image width="0"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "image"),
        "image with width=0 should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_polyline_no_points() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <polyline/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "polyline"),
        "polyline without points should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_polygon_no_points() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <polygon/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "polygon"),
        "polygon without points should be removed"
    );
}

#[test]
fn remove_hidden_elems_removes_path_no_d() {
    let plug = svgo_plugins::remove_hidden_elems::RemoveHiddenElems;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <path/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    assert!(
        !has_child_element(&doc, "path"),
        "path without d should be removed"
    );
}

// ==================== removeUnusedNS ====================

#[test]
fn remove_unused_ns_removes_unused_namespace() {
    let plug = svgo_plugins::remove_unused_ns::RemoveUnusedNS;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:foo="http://example.com/foo">
    <g/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        !svg.attributes.contains_key("xmlns:foo"),
        "Unused namespace should be removed"
    );
    assert!(
        svg.attributes.contains_key("xmlns"),
        "Used namespace (xmlns) should be kept"
    );
}

#[test]
fn remove_unused_ns_keeps_used_namespace() {
    let plug = svgo_plugins::remove_unused_ns::RemoveUnusedNS;
    let input = r##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <use xlink:href="#foo"/>
</svg>"##;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        svg.attributes.contains_key("xmlns:xlink"),
        "Used namespace (xlink) should be kept"
    );
}

// ==================== removeViewBox ====================

#[test]
fn remove_view_box_removes_matching_viewbox() {
    let plug = svgo_plugins::remove_view_box::RemoveViewBox;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50" viewBox="0 0 100 50">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        !svg.attributes.contains_key("viewBox"),
        "Matching viewBox should be removed"
    );
}

#[test]
fn remove_view_box_keeps_non_matching_viewbox() {
    let plug = svgo_plugins::remove_view_box::RemoveViewBox;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50" viewBox="0 0 200 100">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        svg.attributes.contains_key("viewBox"),
        "Non-matching viewBox should be kept"
    );
}

// ==================== cleanupEnableBackground ====================

#[test]
fn cleanup_enable_background_removes_matching() {
    let plug = svgo_plugins::cleanup_enable_background::CleanupEnableBackground;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50" enable-background="new 0 0 100 50">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        !svg.attributes.contains_key("enable-background"),
        "Matching enable-background should be removed"
    );
}

#[test]
fn cleanup_enable_background_keeps_non_matching() {
    let plug = svgo_plugins::cleanup_enable_background::CleanupEnableBackground;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50" enable-background="new 0 0 200 100">
    test
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let svg = find_child_element(&doc, "svg").unwrap();
    assert!(
        svg.attributes.contains_key("enable-background"),
        "Non-matching enable-background should be kept"
    );
}

// ==================== removeNonInheritableGroupAttrs ====================

#[test]
fn remove_non_inheritable_group_attrs_removes_non_inheritable() {
    let plug = svgo_plugins::remove_non_inheritable_group_attrs::RemoveNonInheritableGroupAttrs;
    let input = r##"<svg xmlns="http://www.w3.org/2000/svg">
    <g fill="red" stroke="blue" opacity="0.5" transform="rotate(45)"/>
</svg>"##;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let g = find_child_element(&doc, "g").unwrap();
    // fill is inheritable, should be kept
    assert!(
        g.attributes.contains_key("fill"),
        "Inheritable attr (fill) should be kept"
    );
    // opacity is inheritable, should be kept
    assert!(
        g.attributes.contains_key("opacity"),
        "Inheritable attr (opacity) should be kept"
    );
    // transform is NOT in presentation, should be kept
    assert!(
        g.attributes.contains_key("transform"),
        "Non-presentation attr (transform) should be kept"
    );
}

#[test]
fn remove_non_inheritable_group_attrs_removes_stroke() {
    let plug = svgo_plugins::remove_non_inheritable_group_attrs::RemoveNonInheritableGroupAttrs;
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
    <g stroke="blue"/>
</svg>"#;
    let mut doc = parse(input).unwrap();
    plug.apply(&mut doc, &serde_json::json!({}));
    let g = find_child_element(&doc, "g").unwrap();
    // stroke is inheritable, should be kept
    assert!(
        g.attributes.contains_key("stroke"),
        "Inheritable attr (stroke) should be kept"
    );
}
