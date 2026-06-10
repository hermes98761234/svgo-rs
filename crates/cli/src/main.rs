mod args;
mod io;

use std::path::PathBuf;

use anyhow::{Context, Result};

use svgo_core::plugin::Registry;

use crate::args::Args;
use crate::io::{collect_inputs, load_config, process_stdin, process_svg, read_input};

fn main() -> Result<()> {
    let args = <Args as clap::Parser>::parse();

    // --show-plugins: list registered plugins and exit
    if args.show_plugins {
        let mut registry = Registry::new();
        svgo_plugins::register_all(&mut registry);
        let mut names: Vec<&str> = registry.names().collect();
        names.sort();
        for name in names {
            println!("{name}");
        }
        return Ok(());
    }

    // Load config
    let config = load_config(&args)?;

    // Build registry
    let mut registry = Registry::new();
    svgo_plugins::register_all(&mut registry);

    // Handle --string mode
    if let Some(ref input_str) = args.string {
        let output = svgo_core::optimize::optimize(input_str, &config, &registry)
            .context("Failed to optimize input string")?;
        print!("{output}");
        return Ok(());
    }

    // Collect inputs
    let inputs = collect_inputs(&args)?;

    // If no inputs, try stdin
    if inputs.is_empty() {
        let config = load_config(&args)?;
        let mut registry = Registry::new();
        svgo_plugins::register_all(&mut registry);
        let opts = io::build_stringify_options(&args);
        return process_stdin(&config, &registry, &opts);
    }

    // Determine output mode
    let has_explicit_output = !args.output.is_empty();

    for (i, (path, _)) in inputs.iter().enumerate() {
        let filename = path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "-".to_string());

        // Read input
        let input_content = if path.is_none() {
            read_input(None)?
        } else {
            read_input(path.as_ref())?
        };

        // Determine output path
        let output_path: Option<PathBuf> = if has_explicit_output {
            if args.output.len() == 1 {
                // Single output for all inputs
                let out = &args.output[0];
                if out == "-" {
                    None // stdout
                } else {
                    Some(PathBuf::from(out))
                }
            } else if i < args.output.len() {
                let out = &args.output[i];
                if out == "-" {
                    None
                } else {
                    Some(PathBuf::from(out))
                }
            } else {
                // More inputs than outputs: optimize in-place
                path.clone()
            }
        } else if path.is_none() {
            // stdin -> stdout
            None
        } else {
            // No -o: optimize in-place
            path.clone()
        };

        // For stdin->stdout, use the stdin path
        let effective_path = if path.is_none() && !has_explicit_output {
            None
        } else {
            output_path.as_ref()
        };

        let opts = io::build_stringify_options(&args);

        process_svg(
            &input_content,
            &config,
            &registry,
            &opts,
            effective_path,
            &filename,
            args.quiet,
        )?;
    }

    Ok(())
}
