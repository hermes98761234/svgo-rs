//! Batch B fixture tests: attribute & value plugins.
//!
//! Each plugin's fixtures are loaded from crates/plugins/tests/fixtures/<name>/
//! and driven via the shared harness from tests/common/mod.rs.

use svgo_plugins::cleanup_attrs::CleanupAttrs;
use svgo_plugins::cleanup_ids::CleanupIds;
use svgo_plugins::cleanup_numeric_values::CleanupNumericValues;
use svgo_plugins::convert_colors::ConvertColors;
use svgo_plugins::remove_unknowns_and_defaults::RemoveUnknownsAndDefaults;
use svgo_plugins::remove_useless_stroke_and_fill::RemoveUselessStrokeAndFill;
use svgo_plugins::sort_attrs::SortAttrs;
use svgo_plugins::sort_defs_children::SortDefsChildren;

mod common;
use common::run_fixture;

// ==================== cleanupAttrs ====================

#[test]
fn cleanup_attrs_01() {
    let fixture = include_str!("fixtures/cleanupAttrs/cleanupAttrs.01.svg.txt");
    run_fixture(&CleanupAttrs, fixture);
}

#[test]
fn cleanup_attrs_02() {
    let fixture = include_str!("fixtures/cleanupAttrs/cleanupAttrs.02.svg.txt");
    run_fixture(&CleanupAttrs, fixture);
}

// ==================== cleanupNumericValues ====================

#[test]
fn cleanup_numeric_values_01() {
    let fixture = include_str!("fixtures/cleanupNumericValues/cleanupNumericValues.01.svg.txt");
    run_fixture(&CleanupNumericValues, fixture);
}

#[test]
fn cleanup_numeric_values_02() {
    let fixture = include_str!("fixtures/cleanupNumericValues/cleanupNumericValues.02.svg.txt");
    run_fixture(&CleanupNumericValues, fixture);
}

#[test]
fn cleanup_numeric_values_03() {
    let fixture = include_str!("fixtures/cleanupNumericValues/cleanupNumericValues.03.svg.txt");
    run_fixture(&CleanupNumericValues, fixture);
}

// ==================== convertColors ====================

#[test]
fn convert_colors_01() {
    let fixture = include_str!("fixtures/convertColors/convertColors.01.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

#[test]
fn convert_colors_02() {
    let fixture = include_str!("fixtures/convertColors/convertColors.02.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

#[test]
fn convert_colors_03() {
    let fixture = include_str!("fixtures/convertColors/convertColors.03.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

#[test]
fn convert_colors_04() {
    let fixture = include_str!("fixtures/convertColors/convertColors.04.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

#[test]
fn convert_colors_05() {
    let fixture = include_str!("fixtures/convertColors/convertColors.05.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

#[test]
fn convert_colors_06() {
    let fixture = include_str!("fixtures/convertColors/convertColors.06.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

#[test]
fn convert_colors_07() {
    let fixture = include_str!("fixtures/convertColors/convertColors.07.svg.txt");
    run_fixture(&ConvertColors, fixture);
}

// ==================== removeUnknownsAndDefaults ====================

#[test]
fn remove_unknowns_and_defaults_01() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.01.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_02() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.02.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_03() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.03.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_04() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.04.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_05() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.05.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_06() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.06.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_07() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.07.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_08() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.08.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_09() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.09.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_10() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.10.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_11() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.11.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_12() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.12.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_13() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.13.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_14() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.14.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_15() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.15.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_16() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.16.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

#[test]
fn remove_unknowns_and_defaults_17() {
    let fixture =
        include_str!("fixtures/removeUnknownsAndDefaults/removeUnknownsAndDefaults.17.svg.txt");
    run_fixture(&RemoveUnknownsAndDefaults, fixture);
}

// ==================== removeUselessStrokeAndFill ====================

#[test]
fn remove_useless_stroke_and_fill_01() {
    let fixture =
        include_str!("fixtures/removeUselessStrokeAndFill/removeUselessStrokeAndFill.01.svg.txt");
    run_fixture(&RemoveUselessStrokeAndFill, fixture);
}

#[test]
fn remove_useless_stroke_and_fill_02() {
    let fixture =
        include_str!("fixtures/removeUselessStrokeAndFill/removeUselessStrokeAndFill.02.svg.txt");
    run_fixture(&RemoveUselessStrokeAndFill, fixture);
}

#[test]
fn remove_useless_stroke_and_fill_03() {
    let fixture =
        include_str!("fixtures/removeUselessStrokeAndFill/removeUselessStrokeAndFill.03.svg.txt");
    run_fixture(&RemoveUselessStrokeAndFill, fixture);
}

#[test]
fn remove_useless_stroke_and_fill_04() {
    let fixture =
        include_str!("fixtures/removeUselessStrokeAndFill/removeUselessStrokeAndFill.04.svg.txt");
    run_fixture(&RemoveUselessStrokeAndFill, fixture);
}

#[test]
fn remove_useless_stroke_and_fill_05() {
    let fixture =
        include_str!("fixtures/removeUselessStrokeAndFill/removeUselessStrokeAndFill.05.svg.txt");
    run_fixture(&RemoveUselessStrokeAndFill, fixture);
}

// ==================== sortAttrs ====================

#[test]
fn sort_attrs_01() {
    let fixture = include_str!("fixtures/sortAttrs/sortAttrs.01.svg.txt");
    run_fixture(&SortAttrs, fixture);
}

#[test]
fn sort_attrs_02() {
    let fixture = include_str!("fixtures/sortAttrs/sortAttrs.02.svg.txt");
    run_fixture(&SortAttrs, fixture);
}

#[test]
fn sort_attrs_03() {
    let fixture = include_str!("fixtures/sortAttrs/sortAttrs.03.svg.txt");
    run_fixture(&SortAttrs, fixture);
}

#[test]
fn sort_attrs_04() {
    let fixture = include_str!("fixtures/sortAttrs/sortAttrs.04.svg.txt");
    run_fixture(&SortAttrs, fixture);
}

// ==================== sortDefsChildren ====================

#[test]
fn sort_defs_children_01() {
    let fixture = include_str!("fixtures/sortDefsChildren/sortDefsChildren.01.svg.txt");
    run_fixture(&SortDefsChildren, fixture);
}

// ==================== cleanupIds ====================

#[test]
fn cleanup_ids_01() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.01.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_02() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.02.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_03() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.03.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_04() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.04.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_05() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.05.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_06() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.06.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_07() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.07.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_08() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.08.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_09() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.09.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_10() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.10.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_11() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.11.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_12() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.12.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_13() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.13.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_14() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.14.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_15() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.15.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_16() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.16.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_17() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.17.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_18() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.18.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_19() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.19.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_20() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.20.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_21() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.21.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_22() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.22.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_23() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.23.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_24() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.24.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_25() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.25.svg.txt");
    run_fixture(&CleanupIds, fixture);
}

#[test]
fn cleanup_ids_26() {
    let fixture = include_str!("fixtures/cleanupIds/cleanupIds.26.svg.txt");
    run_fixture(&CleanupIds, fixture);
}
