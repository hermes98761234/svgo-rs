use std::fs;

use svgo_core::config::Config;
use svgo_core::optimize::optimize;
use svgo_core::plugin::Registry;

/// Build a default Config with preset-default plugins and multipass enabled.
fn default_config() -> Config {
    let mut config = Config::default();
    config.plugins = Config::preset_default()
        .into_iter()
        .map(|name| svgo_core::config::PluginEntry::Name(name.to_string()))
        .collect();
    config.multipass = true;
    config
}

/// Build a Registry with all plugins registered.
fn default_registry() -> Registry {
    let mut registry = Registry::new();
    svgo_plugins::register_all(&mut registry);
    registry
}

/// Read a corpus SVG file by name (e.g. "01-svgo-logo.svg").
fn read_corpus(name: &str) -> String {
    let path = format!("tests/corpus/{}", name);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read corpus file {path}: {e}"))
}

/// Assert that the output is valid SVG (parsable by the parser).
fn assert_valid_svg(output: &str) {
    let trimmed = output.trim();
    assert!(
        trimmed.starts_with("<"),
        "output should start with '<', got: {}",
        &trimmed[..std::cmp::min(40, trimmed.len())]
    );
    svgo_core::parser::parse(output).expect("output should be valid parseable SVG");
}

// ---------------------------------------------------------------------------
// Corpus 1: SVGO logo (complex real-world SVG with gradients, paths, etc.)
// ---------------------------------------------------------------------------

#[test]
fn e2e_svgo_logo_smaller_and_valid() {
    let input = read_corpus("01-svgo-logo.svg");
    let config = default_config();
    let registry = default_registry();

    let output = optimize(&input, &config, &registry).expect("optimization should succeed");

    assert!(
        output.len() <= input.len(),
        "output ({} bytes) should be <= input ({} bytes)",
        output.len(),
        input.len()
    );
    assert_valid_svg(&output);
}

#[test]
fn e2e_svgo_logo_multipass_converges() {
    let input = read_corpus("01-svgo-logo.svg");
    let config = default_config();
    let registry = default_registry();

    // Single-pass output
    let mut single_config = config.clone();
    single_config.multipass = false;
    let single_output =
        optimize(&input, &single_config, &registry).expect("single pass should succeed");

    // Multipass output should be <= single-pass output
    let multi_output = optimize(&input, &config, &registry).expect("multipass should succeed");
    assert!(
        multi_output.len() <= single_output.len(),
        "multipass ({}) should be <= single pass ({})",
        multi_output.len(),
        single_output.len()
    );
    assert_valid_svg(&multi_output);
}

// ---------------------------------------------------------------------------
// Corpus 2: Complex shapes (gradients, paths, transforms, groups)
// ---------------------------------------------------------------------------

#[test]
fn e2e_complex_shapes_smaller_and_valid() {
    let input = read_corpus("02-complex-shapes.svg");
    let config = default_config();
    let registry = default_registry();

    let output = optimize(&input, &config, &registry).expect("optimization should succeed");

    assert!(
        output.len() <= input.len(),
        "output ({} bytes) should be <= input ({} bytes)",
        output.len(),
        input.len()
    );
    assert_valid_svg(&output);
}

#[test]
fn e2e_complex_shapes_multipass_converges() {
    let input = read_corpus("02-complex-shapes.svg");
    let config = default_config();
    let registry = default_registry();

    let mut single_config = config.clone();
    single_config.multipass = false;
    let single_output =
        optimize(&input, &single_config, &registry).expect("single pass should succeed");

    let multi_output = optimize(&input, &config, &registry).expect("multipass should succeed");
    assert!(
        multi_output.len() <= single_output.len(),
        "multipass ({}) should be <= single pass ({})",
        multi_output.len(),
        single_output.len()
    );
    assert_valid_svg(&multi_output);
}

// ---------------------------------------------------------------------------
// Corpus 3: Styled SVG (CSS, inline styles, patterns, markers)
// ---------------------------------------------------------------------------

#[test]
fn e2e_styled_smaller_and_valid() {
    let input = read_corpus("03-styled.svg");
    let config = default_config();
    let registry = default_registry();

    let output = optimize(&input, &config, &registry).expect("optimization should succeed");

    assert!(
        output.len() <= input.len(),
        "output ({} bytes) should be <= input ({} bytes)",
        output.len(),
        input.len()
    );
    assert_valid_svg(&output);
}

#[test]
fn e2e_styled_multipass_converges() {
    let input = read_corpus("03-styled.svg");
    let config = default_config();
    let registry = default_registry();

    let mut single_config = config.clone();
    single_config.multipass = false;
    let single_output =
        optimize(&input, &single_config, &registry).expect("single pass should succeed");

    let multi_output = optimize(&input, &config, &registry).expect("multipass should succeed");
    assert!(
        multi_output.len() <= single_output.len(),
        "multipass ({}) should be <= single pass ({})",
        multi_output.len(),
        single_output.len()
    );
    assert_valid_svg(&multi_output);
}
