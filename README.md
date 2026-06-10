# ⚡ svgo-rs

![CI](https://github.com/hermes98761234/svgo-rs/workflows/CI/badge.svg)
![Release](https://img.shields.io/github/v/release/hermes98761234/svgo-rs)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.78%2B-orange.svg)

**SVG Optimizer — a fast, principled Rust rewrite of [SVGO](https://github.com/svg/svgo).**

## ✨ Features

- **34 preset-default plugins** — ported from the original SVGO: removal, cleanup, path optimization, style inlining, and more
- **Multipass optimization** — `--multipass` applies plugins repeatedly until output stabilizes
- **SVGO-compatible CLI** — drop-in replacement with familiar flags (`-i`, `-o`, `-f`, `-r`, `--pretty`, `--precision`)
- **Stdin/stdout support** — pipe-friendly with `-i -` and `-o -`
- **Config file support** — `svgo.config.json` for plugin customization
- **Cross-platform** — prebuilt binaries for Linux, macOS, and Windows (x86_64 + aarch64)
- **Zero dependencies at runtime** — single static binary per target

### Before / After

```
$ cat sample.svg | wc -c
966

$ svgo -i sample.svg
sample.svg: 966 B -> 606 B (37%)

$ svgo --multipass -i sample.svg
sample.svg: 966 B -> 589 B (39%)
```

Before:
```svg
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <!-- This is a comment that should be removed -->
  <metadata>...</metadata>
  <rect x="10" y="10" width="180" height="180" fill="url(#grad1)" stroke="#000000" stroke-width="2"/>
  <circle cx="100" cy="100" r="50" fill="#00ff00" opacity="0.5"/>
  <g transform="translate(10,10) scale(1.0)">
    <path d="M 0 0 L 50 50 L 100 0 Z" fill="none" stroke="blue" stroke-width="1"/>
  </g>
</svg>
```

After:
```svg
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200">
  <defs><linearGradient id="a" x1="0%" x2="100%" y1="0%" y2="0%">
      <stop offset="0%" style="stop-color:rgb(255,255,0);stop-opacity:1"/>
      <stop offset="100%" style="stop-color:rgb(255,0,0);stop-opacity:1"/>
    </linearGradient></defs>
  <path fill="url(#a)" stroke="#000" d="m10 10h180v180H10z"/>
  <circle cx="100" cy="100" r="50" fill="#0f0"/>
  <g fill="none" stroke="#00f">
    <path d="m0 0 50 50 50-50z" transform="translate(10 10)"/>
  </g>
</svg>
```

## 📦 Installation

### Prebuilt Binaries

Each [GitHub Release](https://github.com/hermes98761234/svgo-rs/releases) ships binaries for 7 targets:

| Target | Binary |
|--------|--------|
| Linux x86_64 (gnu) | `svgo-x86_64-unknown-linux-gnu` |
| Linux x86_64 (musl) | `svgo-x86_64-unknown-linux-musl` |
| Linux aarch64 (gnu) | `svgo-aarch64-unknown-linux-gnu` |
| Linux aarch64 (musl) | `svgo-aarch64-unknown-linux-musl` |
| macOS x86_64 | `svgo-x86_64-apple-darwin` |
| macOS aarch64 | `svgo-aarch64-apple-darwin` |
| Windows x86_64 | `svgo-x86_64-pc-windows-msvc.exe` |

A `SHA256SUMS.txt` covering all binaries is included in each release.

Download and install:
```sh
# Linux x86_64 example
curl -LO https://github.com/hermes98761234/svgo-rs/releases/latest/download/svgo-x86_64-unknown-linux-gnu
chmod +x svgo-x86_64-unknown-linux-gnu
sudo mv svgo-x86_64-unknown-linux-gnu /usr/local/bin/svgo
```

### From Source

Requires **Rust 1.78+** ([rustup](https://rustup.rs/)).

```sh
cargo install --git https://github.com/hermes98761234/svgo-rs
```

## 🚀 Usage

```sh
# Single file (outputs to stdout)
svgo icon.svg -o icon.min.svg

# Optimize in-place
svgo -i icon.svg

# Process a folder recursively
svgo -f ./assets/icons -r -i

# Stdin / stdout (pipe-friendly)
cat icon.svg | svgo -i - -o - > icon.min.svg

# Pretty-print output
svgo --pretty -i icon.svg -o icon.min.svg

# Control numeric precision
svgo --precision 2 -i icon.svg -o icon.min.svg

# Multipass optimization
svgo --multipass -i icon.svg -o icon.min.svg

# Custom indentation and line endings
svgo --pretty --indent 2 --eol lf -i icon.svg -o icon.min.svg

# Output as data URI
svgo --datauri base64 -i icon.svg -o icon.txt

# Suppress progress output
svgo -q -i icon.svg

# List all plugins
svgo --show-plugins
```

## ⚙️ Configuration

Create an `svgo.config.json` in your project root:

```json
{
  "multipass": true,
  "plugins": [
    { "name": "removeViewBox", "active": false },
    { "name": "cleanupIds", "params": { "minify": false } },
    { "name": "convertColors", "params": { "currentColor": true } }
  ]
}
```

```sh
svgo --config svgo.config.json -i icon.svg -o icon.min.svg
```

## 🔌 Plugins

All 34 preset-default plugins from SVGO are ported and registered:

| Plugin | Description |
|--------|-------------|
| `cleanupAttrs` | Cleans up attributes from newlines, trailing and repeating spaces |
| `cleanupEnableBackground` | Removes or cleans up `enable-background` attribute when possible |
| `cleanupIds` | Removes unused IDs and minifies used ones |
| `cleanupNumericValues` | Rounds numeric values to fixed precision, removes default `px` units |
| `collapseGroups` | Collapses useless groups |
| `convertColors` | Converts colors: `rgb()` to `#rrggbb` and `#rrggbb` to `#rgb` |
| `convertEllipseToCircle` | Converts non-eccentric `<ellipse>`s to `<circle>`s |
| `convertPathData` | Optimizes path data: writes in shorter form, applies transformations |
| `convertShapeToPath` | Converts basic shapes to more compact `<path>` form |
| `convertTransform` | Collapses multiple transformations and optimizes them |
| `inlineStyles` | Inlines `<style>` elements into element `style` attributes |
| `mergePaths` | Merges multiple paths into one where possible |
| `mergeStyles` | Merges multiple `<style>` elements into one |
| `minifyStyles` | Minifies styles and removes unused styles |
| `moveElemsAttrsToGroup` | Moves common attributes of group children to the group |
| `moveGroupAttrsToElems` | Moves some group attributes to the content elements |
| `removeComments` | Removes XML comments |
| `removeDesc` | Removes `<desc>` elements |
| `removeDoctype` | Removes DOCTYPE declarations |
| `removeEditorsNSData` | Removes editor-specific namespace data |
| `removeEmptyAttrs` | Removes empty attributes |
| `removeEmptyContainers` | Removes empty container elements |
| `removeEmptyText` | Removes empty `<text>` elements |
| `removeHiddenElems` | Removes hidden elements (zero-sized, with absent attributes) |
| `removeMetadata` | Removes `<metadata>` elements |
| `removeNonInheritableGroupAttrs` | Removes non-inheritable group presentational attributes |
| `removeUnknownsAndDefaults` | Removes unknown elements, content, attributes, and attrs with default values |
| `removeUnusedNS` | Removes unused namespace declarations |
| `removeUselessDefs` | Removes elements in `<defs>` without `id` |
| `removeUselessStrokeAndFill` | Removes useless `stroke` and `fill` attributes |
| `removeViewBox` | Removes `viewBox` attribute when possible |
| `removeXMLProcInst` | Removes XML processing instructions |
| `sortAttrs` | Sorts element attributes for better compression |
| `sortDefsChildren` | Sorts children of `<defs>` to improve compression |

See [docs/CONFORMANCE.md](docs/CONFORMANCE.md) for the full conformance report against the original SVGO.

## 🏗️ Architecture

The project is a Cargo workspace of 3 crates:

```
svgo-rs/
├── crates/
│   ├── core/       svgo-core — XML parser → mutable AST, stringifier, visitor
│                   pattern, plugin engine, config, multipass support
│   ├── plugins/    svgo-plugins — all 34 preset-default plugins
│   └── cli/        svgo binary — argument parsing, file I/O, pipeline
```

**Request flow:**

```
Input (file / stdin)
  → XML Parser → AST
    → Plugin Engine (visitor pattern, 34 plugins)
      → Stringifier → Output (file / stdout)
```

## 🧪 Testing

```sh
# Run all workspace tests
cargo test --workspace

# Run clippy lints
cargo clippy --workspace --all-targets -- -D warnings

# Check formatting
cargo fmt --all -- --check
```

Fixture tests are ported from the original [svg/svgo](https://github.com/svg/svgo) repository (`test/plugins/`). Each fixture compares pretty-printed output (4-space indent) against expected SVGO output. See [docs/CONFORMANCE.md](docs/CONFORMANCE.md) for pass/fail details.

## 🙏 Acknowledgments

svgo-rs is a Rust rewrite of [SVGO (SVG Optimizer)](https://github.com/svg/svgo), originally created by [Kir Belevich](https://github.com/deepsweet) and maintained by the [SVG Omtimizer team](https://github.com/svg/svgo/graphs-contributors). All plugin semantics and test fixtures are ported from the original project. This project would not exist without their excellent work.

## 📄 License

[MIT](LICENSE)
