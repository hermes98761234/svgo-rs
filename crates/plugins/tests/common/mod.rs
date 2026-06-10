//! Shared fixture test harness for svgo-plugins.
//!
//! Fixture format: [description\n===\n]input SVG @@@ expected SVG (optional @@@ params JSON)
//! Output is pretty-printed with 4-space indent to match SVGO's test format.

use svgo_core::{parse, stringify, StringifyOptions};

/// Pretty-print options matching SVGO's test format (4-space indent).
pub fn pretty_opts() -> StringifyOptions {
    StringifyOptions {
        pretty: true,
        indent: "    ".to_string(),
        final_newline: true,
        eol: "\n".to_string(),
    }
}

/// Normalize XML string for comparison: trim trailing whitespace on each line,
/// remove empty lines at start/end, and ensure consistent formatting.
fn normalize_xml(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let mut result = String::new();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim_end();
        if i > 0 {
            result.push('\n');
        }
        result.push_str(trimmed);
    }
    result.trim().to_string()
}

/// Run a single fixture test.
///
/// Splits the fixture text on `@@@`, parses the input, applies the plugin,
/// and asserts the pretty-printed output matches the expected result.
pub fn run_fixture(plugin: &dyn svgo_core::plugin::Plugin, fixture: &str) {
    // Remove description section (everything before "===")
    let fixture = if let Some(pos) = fixture.find("===") {
        &fixture[pos + 3..]
    } else {
        fixture
    };

    let parts: Vec<&str> = fixture.splitn(3, "@@@").collect();
    let input = parts[0].trim();
    let expected = parts[1].trim();
    let params: serde_json::Value = parts
        .get(2)
        .map(|p| serde_json::from_str(p.trim()).unwrap())
        .unwrap_or(serde_json::json!({}));

    let mut doc = parse(input).unwrap_or_else(|e| panic!("{}: parse error: {}", plugin.name(), e));
    plugin.apply(&mut doc, &params);
    let output = stringify(&doc, &pretty_opts());

    let output_normalized = normalize_xml(&output);
    let expected_normalized = normalize_xml(expected);

    assert_eq!(
        output_normalized,
        expected_normalized,
        "\nPlugin: {}\nInput: {}\nExpected:\n{}\nGot:\n{}",
        plugin.name(),
        input,
        expected_normalized,
        output_normalized
    );
}
