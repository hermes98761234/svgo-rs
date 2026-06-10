use svgo_core::ast::Document;
use svgo_core::plugin::Plugin;
use svgo_core::visitor::{Context, VisitAction, Visitor};

/// Remove or cleanup enable-background attribute when possible.
///
/// Note: The full SVGO plugin uses css-tree to parse inline styles.
/// This version handles the attribute only (not inline style declarations).
pub struct CleanupEnableBackground;

impl Plugin for CleanupEnableBackground {
    fn name(&self) -> &'static str {
        "cleanupEnableBackground"
    }

    fn apply(&self, doc: &mut Document, _params: &serde_json::Value) {
        struct V;
        impl Visitor for V {
            fn element_enter(
                &mut self,
                el: &mut svgo_core::ast::Element,
                _ctx: &Context,
            ) -> VisitAction {
                let has_dimensions = el.attr("width").is_some() && el.attr("height").is_some();
                let target_element = matches!(el.name.as_str(), "svg" | "mask" | "pattern");

                if has_dimensions && target_element {
                    if let Some(value) = el.attr("enable-background") {
                        // Parse "new 0 0 W H" pattern
                        if let Some(cleaned) = cleanup_value(
                            value,
                            el.name.as_str(),
                            el.attr("width").unwrap(),
                            el.attr("height").unwrap(),
                        ) {
                            el.set_attr("enable-background", cleaned);
                        } else {
                            el.remove_attr("enable-background");
                        }
                    }
                }

                VisitAction::Continue
            }
        }

        fn cleanup_value(value: &str, name: &str, width: &str, height: &str) -> Option<String> {
            // Pattern: "new 0 0 W H"
            let parts: Vec<&str> = value.split_whitespace().collect();
            if parts.len() == 5
                && parts[0] == "new"
                && parts[1] == "0"
                && parts[2] == "0"
                && parts[3] == width
                && parts[4] == height
            {
                if name == "svg" {
                    return None; // Remove entirely for svg
                } else {
                    return Some("new".to_string()); // Simplify for mask/pattern
                }
            }
            Some(value.to_string())
        }

        let mut v = V;
        svgo_core::visitor::visit(doc, &mut v);
    }
}
