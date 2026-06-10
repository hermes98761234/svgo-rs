#![allow(clippy::all)]
#![allow(unused)]
pub mod collections;

pub mod cleanup_enable_background;
pub mod remove_comments;
pub mod remove_desc;
pub mod remove_doctype;
pub mod remove_editors_ns_data;
pub mod remove_empty_attrs;
pub mod remove_empty_containers;
pub mod remove_empty_text;
pub mod remove_hidden_elems;
pub mod remove_metadata;
pub mod remove_non_inheritable_group_attrs;
pub mod remove_unused_ns;
pub mod remove_useless_defs;
pub mod remove_view_box;
pub mod remove_xml_proc_inst;

// Batch B plugins
pub mod cleanup_attrs;
pub mod cleanup_ids;
pub mod cleanup_numeric_values;
pub mod convert_colors;
pub mod remove_unknowns_and_defaults;
pub mod remove_useless_stroke_and_fill;
pub mod sort_attrs;
pub mod sort_defs_children;

// Batch C plugins
pub mod convert_ellipse_to_circle;
pub mod convert_path_data;
pub mod convert_shape_to_path;
pub mod convert_transform;
pub mod merge_paths;
pub mod path;
pub mod transforms;

// Batch D plugins
pub mod collapse_groups;
pub mod inline_styles;
pub mod merge_styles;
pub mod minify_styles;
pub mod move_elems_attrs_to_group;
pub mod move_group_attrs_to_elems;

use collapse_groups::CollapseGroups;
use inline_styles::InlineStyles;
use merge_styles::MergeStyles;
use minify_styles::MinifyStyles;
use move_elems_attrs_to_group::MoveElemsAttrsToGroup;
use move_group_attrs_to_elems::MoveGroupAttrsToElems;

use svgo_core::plugin::Registry;

/// Register all batch A plugins into the given registry.
pub fn register_all(r: &mut Registry) {
    use cleanup_enable_background::CleanupEnableBackground;
    use remove_comments::RemoveComments;
    use remove_desc::RemoveDesc;
    use remove_doctype::RemoveDoctype;
    use remove_editors_ns_data::RemoveEditorsNSData;
    use remove_empty_attrs::RemoveEmptyAttrs;
    use remove_empty_containers::RemoveEmptyContainers;
    use remove_empty_text::RemoveEmptyText;
    use remove_hidden_elems::RemoveHiddenElems;
    use remove_metadata::RemoveMetadata;
    use remove_non_inheritable_group_attrs::RemoveNonInheritableGroupAttrs;
    use remove_unused_ns::RemoveUnusedNS;
    use remove_useless_defs::RemoveUselessDefs;
    use remove_view_box::RemoveViewBox;
    use remove_xml_proc_inst::RemoveXmlProcInst;

    r.register(
        "removeDoctype",
        std::sync::Arc::new(|_| Box::new(RemoveDoctype)),
    );
    r.register(
        "removeXMLProcInst",
        std::sync::Arc::new(|_| Box::new(RemoveXmlProcInst)),
    );
    r.register(
        "removeComments",
        std::sync::Arc::new(|_| Box::new(RemoveComments)),
    );
    r.register(
        "removeMetadata",
        std::sync::Arc::new(|_| Box::new(RemoveMetadata)),
    );
    r.register(
        "removeEditorsNSData",
        std::sync::Arc::new(|_| Box::new(RemoveEditorsNSData)),
    );
    r.register("removeDesc", std::sync::Arc::new(|_| Box::new(RemoveDesc)));
    r.register(
        "removeUselessDefs",
        std::sync::Arc::new(|_| Box::new(RemoveUselessDefs)),
    );
    r.register(
        "removeEmptyAttrs",
        std::sync::Arc::new(|_| Box::new(RemoveEmptyAttrs)),
    );
    r.register(
        "removeEmptyText",
        std::sync::Arc::new(|_| Box::new(RemoveEmptyText)),
    );
    r.register(
        "removeEmptyContainers",
        std::sync::Arc::new(|_| Box::new(RemoveEmptyContainers)),
    );
    r.register(
        "removeHiddenElems",
        std::sync::Arc::new(|_| Box::new(RemoveHiddenElems)),
    );
    r.register(
        "removeUnusedNS",
        std::sync::Arc::new(|_| Box::new(RemoveUnusedNS)),
    );
    r.register(
        "removeViewBox",
        std::sync::Arc::new(|_| Box::new(RemoveViewBox)),
    );
    r.register(
        "cleanupEnableBackground",
        std::sync::Arc::new(|_| Box::new(CleanupEnableBackground)),
    );
    r.register(
        "removeNonInheritableGroupAttrs",
        std::sync::Arc::new(|_| Box::new(RemoveNonInheritableGroupAttrs)),
    );

    // Batch B plugins
    use cleanup_attrs::CleanupAttrs;
    use cleanup_ids::CleanupIds;
    use cleanup_numeric_values::CleanupNumericValues;
    use convert_colors::ConvertColors;
    use remove_unknowns_and_defaults::RemoveUnknownsAndDefaults;
    use remove_useless_stroke_and_fill::RemoveUselessStrokeAndFill;
    use sort_attrs::SortAttrs;
    use sort_defs_children::SortDefsChildren;

    r.register(
        "cleanupAttrs",
        std::sync::Arc::new(|_| Box::new(CleanupAttrs)),
    );
    r.register(
        "cleanupNumericValues",
        std::sync::Arc::new(|_| Box::new(CleanupNumericValues)),
    );
    r.register(
        "convertColors",
        std::sync::Arc::new(|_| Box::new(ConvertColors)),
    );
    r.register(
        "removeUnknownsAndDefaults",
        std::sync::Arc::new(|_| Box::new(RemoveUnknownsAndDefaults)),
    );
    r.register(
        "removeUselessStrokeAndFill",
        std::sync::Arc::new(|_| Box::new(RemoveUselessStrokeAndFill)),
    );
    r.register("sortAttrs", std::sync::Arc::new(|_| Box::new(SortAttrs)));
    r.register(
        "sortDefsChildren",
        std::sync::Arc::new(|_| Box::new(SortDefsChildren)),
    );
    r.register("cleanupIds", std::sync::Arc::new(|_| Box::new(CleanupIds)));

    // Batch C plugins
    use convert_ellipse_to_circle::ConvertEllipseToCircle;
    use convert_path_data::ConvertPathData;
    use convert_shape_to_path::ConvertShapeToPath;
    use convert_transform::ConvertTransform;
    use merge_paths::MergePaths;

    r.register(
        "convertEllipseToCircle",
        std::sync::Arc::new(|_| Box::new(ConvertEllipseToCircle)),
    );
    r.register(
        "convertPathData",
        std::sync::Arc::new(|_| Box::new(ConvertPathData)),
    );
    r.register(
        "convertShapeToPath",
        std::sync::Arc::new(|_| Box::new(ConvertShapeToPath)),
    );
    r.register(
        "convertTransform",
        std::sync::Arc::new(|_| Box::new(ConvertTransform)),
    );
    r.register("mergePaths", std::sync::Arc::new(|_| Box::new(MergePaths)));

    // Batch D plugins
    r.register(
        "mergeStyles",
        std::sync::Arc::new(|_| Box::new(MergeStyles)),
    );
    r.register(
        "minifyStyles",
        std::sync::Arc::new(|_| Box::new(MinifyStyles)),
    );
    r.register(
        "inlineStyles",
        std::sync::Arc::new(|_| Box::new(InlineStyles)),
    );
    r.register(
        "moveElemsAttrsToGroup",
        std::sync::Arc::new(|_| Box::new(MoveElemsAttrsToGroup)),
    );
    r.register(
        "moveGroupAttrsToElems",
        std::sync::Arc::new(|_| Box::new(MoveGroupAttrsToElems)),
    );
    r.register(
        "collapseGroups",
        std::sync::Arc::new(|_| Box::new(CollapseGroups)),
    );
}
