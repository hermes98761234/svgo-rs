use std::sync::Arc;

use svgo_core::{
    optimize, parse, stringify, visit, Config, Context, Plugin, Registry, StringifyOptions,
    VisitAction, Visitor,
};

// ── Dummy plugin: uppercase all comment text ──

struct UppercaseComments;

impl Plugin for UppercaseComments {
    fn name(&self) -> &'static str {
        "uppercaseComments"
    }
    fn apply(&self, doc: &mut svgo_core::Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn comment(&mut self, c: &mut String, _ctx: &Context) -> VisitAction {
                *c = c.to_uppercase();
                VisitAction::Continue
            }
        }
        visit(doc, &mut V);
    }
}

// ── Tests ──

#[test]
fn dummy_plugin_uppercases_comments() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><!-- hello --></svg>"#;
    let mut doc = parse(input).unwrap();

    let plugin = UppercaseComments;
    plugin.apply(&mut doc, &serde_json::json!({}));

    let output = stringify(&doc, &StringifyOptions::default());
    assert!(
        output.contains("HELLO"),
        "expected uppercased comment, got: {}",
        output
    );
    assert!(!output.contains("hello"));
}

#[test]
fn visitor_removes_comments() {
    let input =
        r#"<svg xmlns="http://www.w3.org/2000/svg"><!-- remove me --><path d="M0 0"/></svg>"#;
    let mut doc = parse(input).unwrap();

    struct RemoveComments;
    impl Visitor for RemoveComments {
        fn comment(&mut self, _c: &mut String, _ctx: &Context) -> VisitAction {
            VisitAction::Remove
        }
    }
    visit(&mut doc, &mut RemoveComments);

    let output = stringify(&doc, &StringifyOptions::default());
    assert!(
        !output.contains("<!--"),
        "expected no comments, got: {}",
        output
    );
    assert!(output.contains("<path"));
}

#[test]
fn config_json_parses() {
    let json = r#"{
        "multipass": true,
        "floatPrecision": 5,
        "plugins": ["removeComments"],
        "js2svg": { "pretty": true, "indent": 4 }
    }"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert!(config.multipass);
    assert_eq!(config.float_precision, 5);
    assert!(config.js2svg.pretty);
    assert_eq!(config.js2svg.indent, 4);
    assert_eq!(config.plugins.len(), 1);
    assert_eq!(config.plugins[0].name(), "removeComments");
}

#[test]
fn multipass_stops_at_fixpoint() {
    // A no-op plugin: multipass should converge in 2 passes (first produces output,
    // second produces same length, so it stops).
    struct Noop;
    impl Plugin for Noop {
        fn name(&self) -> &'static str {
            "noop"
        }
        fn apply(&self, _doc: &mut svgo_core::Document, _params: &serde_json::Value) {}
    }

    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0"/></svg>"#;
    let mut registry = Registry::new();
    registry.register(
        "noop",
        Arc::new(|_params: &serde_json::Value| Box::new(Noop)),
    );

    let config: Config = serde_json::from_str(
        r#"{
        "multipass": true,
        "plugins": ["noop"]
    }"#,
    )
    .unwrap();

    let result = optimize(input, &config, &registry).unwrap();
    // Output should be the same as a single-pass
    let single = {
        let mut doc2 = parse(input).unwrap();
        let plugin = Noop;
        plugin.apply(&mut doc2, &serde_json::json!({}));
        stringify(&doc2, &StringifyOptions::default())
    };
    assert_eq!(result, single);
}

#[test]
fn optimize_with_no_plugins_returns_input() {
    let input = r#"<svg xmlns="http://www.w3.org/2000/svg"><path d="M0 0"/></svg>"#;
    let registry = Registry::new();
    let config = Config::default();

    let result = optimize(input, &config, &registry).unwrap();
    assert_eq!(result, input);
}
