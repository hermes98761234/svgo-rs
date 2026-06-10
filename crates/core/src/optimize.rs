use crate::config::Config;
use crate::plugin::Registry;

/// Errors from the optimization pipeline.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parse error: {0}")]
    Parse(#[from] crate::parser::ParseError),
}

/// Optimize an SVG string using the given config and plugin registry.
///
/// Parses the input, runs enabled plugins in order, and stringifies.
/// If `config.multipass` is true, repeats up to 10 passes while the
/// output keeps shrinking.
pub fn optimize(input: &str, config: &Config, registry: &Registry) -> Result<String, Error> {
    let mut doc = crate::parser::parse(input)?;
    let plugins = config.effective_plugins();

    let max_passes = if config.multipass { 10 } else { 1 };
    let mut current_output = String::new();

    for _pass in 0..max_passes {
        // Run all enabled plugins
        for entry in &plugins {
            let name = entry.name();
            let params = entry.params();
            if let Some(plugin) = registry.instantiate(name, &params) {
                plugin.apply(&mut doc, &params);
            }
        }

        // Stringify
        let opts = crate::stringifier::StringifyOptions {
            pretty: config.js2svg.pretty,
            indent: " ".repeat(config.js2svg.indent),
            ..Default::default()
        };
        let output = crate::stringifier::stringify(&doc, &opts);

        // Check for multipass convergence
        if output.len() >= current_output.len() && !current_output.is_empty() {
            return Ok(current_output);
        }
        current_output = output;
    }

    Ok(current_output)
}
