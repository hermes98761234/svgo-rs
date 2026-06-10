use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use regex::Regex;

use svgo_core::config::Config;
use svgo_core::optimize::optimize;
use svgo_core::plugin::Registry;
use svgo_core::stringifier::StringifyOptions;

use crate::args::Args;

/// Load config from --config path, or search upward from cwd for svgo.config.json.
pub fn load_config(args: &Args) -> Result<Config> {
    if let Some(ref config_path) = args.config {
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        let config: Config = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;
        return Ok(config);
    }

    // Search upward from cwd for svgo.config.json
    let mut dir = std::env::current_dir()?;
    loop {
        let candidate = dir.join("svgo.config.json");
        if candidate.exists() {
            let content = fs::read_to_string(&candidate)
                .with_context(|| format!("Failed to read config file: {}", candidate.display()))?;
            let config: Config = serde_json::from_str(&content)
                .with_context(|| format!("Failed to parse config file: {}", candidate.display()))?;
            return Ok(config);
        }
        if !dir.pop() {
            break;
        }
    }

    // No config found, use defaults with preset-default plugins
    Ok(Config::default())
}

/// Build StringifyOptions from CLI args.
pub fn build_stringify_options(args: &Args) -> StringifyOptions {
    StringifyOptions {
        pretty: args.pretty,
        indent: " ".repeat(args.indent),
        final_newline: args.final_newline,
        eol: args
            .eol
            .as_ref()
            .map(|e| match e.as_str() {
                "crlf" => "\r\n".to_string(),
                _ => "\n".to_string(),
            })
            .unwrap_or_else(|| "\n".to_string()),
    }
}

/// Collect all input SVG paths from the CLI args.
pub fn collect_inputs(args: &Args) -> Result<Vec<(Option<PathBuf>, String)>> {
    let mut inputs: Vec<(Option<PathBuf>, String)> = Vec::new();

    // Positional args are treated like -i
    let positional: Vec<String> = if args.input.is_empty() && !args.positional.is_empty() {
        args.positional.clone()
    } else {
        args.input.clone()
    };

    for input in &positional {
        if input == "-" {
            // stdin marker
            inputs.push((None, input.clone()));
        } else {
            let path = PathBuf::from(input);
            if path.is_dir() {
                // Treat as folder
                let svg_files = walk_folder(&path, args.recursive, &args.exclude)?;
                for f in svg_files {
                    inputs.push((Some(f), String::new()));
                }
            } else {
                inputs.push((Some(path), String::new()));
            }
        }
    }

    // Explicit folder
    if let Some(ref folder) = args.folder {
        let svg_files = walk_folder(folder, args.recursive, &args.exclude)?;
        for f in svg_files {
            inputs.push((Some(f), String::new()));
        }
    }

    Ok(inputs)
}

/// Walk a folder for *.svg files.
fn walk_folder(folder: &Path, recursive: bool, exclude: &[String]) -> Result<Vec<PathBuf>> {
    let mut results = Vec::new();
    let exclude_regexes: Vec<Regex> = exclude
        .iter()
        .map(|p| Regex::new(p).with_context(|| format!("Invalid exclude regex: {p}")))
        .collect::<Result<_>>()?;

    walk_dir(folder, recursive, &exclude_regexes, &mut results)?;
    results.sort();
    Ok(results)
}

fn walk_dir(
    dir: &Path,
    recursive: bool,
    exclude: &[Regex],
    results: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "svg" {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    if !exclude.iter().any(|re| re.is_match(&filename)) {
                        results.push(path);
                    }
                }
            }
        } else if recursive && path.is_dir() {
            walk_dir(&path, recursive, exclude, results)?;
        }
    }
    Ok(())
}

/// Read input from a file path or stdin.
pub fn read_input(path: Option<&PathBuf>) -> Result<String> {
    match path {
        None => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
        Some(p) => {
            fs::read_to_string(p).with_context(|| format!("Failed to read file: {}", p.display()))
        }
    }
}

/// Write output to a file path or stdout.
pub fn write_output(path: Option<&PathBuf>, content: &str) -> Result<()> {
    match path {
        None => {
            print!("{content}");
            Ok(())
        }
        Some(p) => {
            fs::write(p, content).with_context(|| format!("Failed to write file: {}", p.display()))
        }
    }
}

/// Format a size in bytes as human-readable (KiB, MiB, etc.)
pub fn format_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} {}", bytes, UNITS[0])
    } else {
        format!("{:.3} {}", size, UNITS[unit_idx])
    }
}

/// Process a single SVG optimization.
pub fn process_svg(
    input: &str,
    config: &Config,
    registry: &Registry,
    _opts: &StringifyOptions,
    output_path: Option<&PathBuf>,
    filename: &str,
    quiet: bool,
) -> Result<()> {
    let original_size = input.len();

    let output = optimize(input, config, registry)
        .with_context(|| format!("Failed to optimize: {filename}"))?;

    let optimized_size = output.len();

    // Write output
    write_output(output_path, &output)?;

    // Report
    if !quiet {
        let ratio = if original_size > 0 {
            (1.0 - optimized_size as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };
        println!(
            "{filename}:\n  {} -> {} ({:.0}%)",
            format_size(original_size),
            format_size(optimized_size),
            ratio
        );
    }

    Ok(())
}

/// Process stdin -> stdout mode.
pub fn process_stdin(config: &Config, registry: &Registry, _opts: &StringifyOptions) -> Result<()> {
    let input = read_input(None)?;
    let output = optimize(&input, config, registry)?;
    write_output(None, &output)?;
    Ok(())
}
