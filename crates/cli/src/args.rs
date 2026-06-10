use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(name = "svgo", version, about = "SVG Optimizer — Rust rewrite of SVGO")]
pub struct Args {
    /// Input files (optimizes in-place when no --output). Use "-" for stdin.
    #[arg(short, long, num_args = 1..)]
    pub input: Vec<String>,

    /// Input SVG string
    #[arg(short = 's', long)]
    pub string: Option<String>,

    /// Input folder (process all *.svg files)
    #[arg(short, long)]
    pub folder: Option<PathBuf>,

    /// Recursively process folders
    #[arg(short, long)]
    pub recursive: bool,

    /// Exclude files matching regex pattern
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Output file(s). Use "-" for stdout.
    #[arg(short, long, num_args = 1..)]
    pub output: Vec<String>,

    /// Precision for numeric values
    #[arg(short, long)]
    pub precision: Option<u8>,

    /// Path to config file (svgo.config.json)
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Apply multipass optimization
    #[arg(long)]
    pub multipass: bool,

    /// Pretty-print output
    #[arg(long)]
    pub pretty: bool,

    /// Indentation size for pretty-printing (default: 4)
    #[arg(long, default_value_t = 4)]
    pub indent: usize,

    /// End-of-line style: lf or crlf
    #[arg(long)]
    pub eol: Option<String>,

    /// Append final newline
    #[arg(long)]
    pub final_newline: bool,

    /// Output as data URI (base64, enc, unenc)
    #[arg(long)]
    pub datauri: Option<String>,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,

    /// List all registered plugins and exit
    #[arg(long)]
    pub show_plugins: bool,

    /// Bare positional input files (like `svgo file.svg`)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub positional: Vec<String>,
}
