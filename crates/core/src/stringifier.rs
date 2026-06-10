use crate::ast::{Document, Element, Node};

/// Options for stringifying a Document.
pub struct StringifyOptions {
    /// Pretty-print output (indentation, newlines).
    pub pretty: bool,
    /// Indentation string (default: two spaces).
    pub indent: String,
    /// Append final newline.
    pub final_newline: bool,
    /// End-of-line character(s).
    pub eol: String,
}

impl Default for StringifyOptions {
    fn default() -> Self {
        Self {
            pretty: false,
            indent: "  ".to_string(),
            final_newline: false,
            eol: "\n".to_string(),
        }
    }
}

/// Escape attribute values: & < > and " must be escaped.
fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escape text content: & < and > must be escaped.
fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Stringify a Document to an SVG/XML string.
pub fn stringify(doc: &Document, opts: &StringifyOptions) -> String {
    let mut out = String::new();
    for child in &doc.children {
        stringify_node(child, &mut out, opts, 0);
    }
    if opts.final_newline {
        out.push_str(&opts.eol);
    }
    out
}

fn stringify_node(node: &Node, out: &mut String, opts: &StringifyOptions, depth: usize) {
    if opts.pretty && depth > 0 {
        for _ in 0..depth {
            out.push_str(&opts.indent);
        }
    }
    match node {
        Node::Element(el) => stringify_element(el, out, opts, depth),
        Node::Text(text) => out.push_str(&escape_text(text)),
        Node::Comment(text) => {
            out.push_str("<!--");
            out.push_str(text);
            out.push_str("-->");
        }
        Node::CData(text) => {
            out.push_str("<![CDATA[");
            out.push_str(text);
            out.push_str("]]>");
        }
        Node::Doctype(text) => {
            out.push_str("<!DOCTYPE");
            if !text.is_empty() {
                out.push(' ');
                out.push_str(text);
            }
            out.push('>');
        }
        Node::Instruction { name, value } => {
            if name == "xml" {
                // Special-case: XML declaration
                out.push_str("<?");
                out.push_str(name);
                if !value.is_empty() {
                    out.push(' ');
                    out.push_str(value);
                }
                out.push_str("?>");
            } else {
                out.push_str("<?");
                out.push_str(name);
                if !value.is_empty() {
                    out.push(' ');
                    out.push_str(value);
                }
                out.push_str("?>");
            }
        }
    }
}

fn stringify_element(el: &Element, out: &mut String, opts: &StringifyOptions, depth: usize) {
    out.push('<');
    out.push_str(&el.name);

    for (key, value) in &el.attributes {
        out.push(' ');
        out.push_str(key);
        out.push_str("=\"");
        out.push_str(&escape_attr(value));
        out.push('"');
    }

    // Self-close only if it was originally self-closing AND has no children
    if el.self_closing && el.children.is_empty() {
        out.push_str("/>");
    } else {
        out.push('>');
        let has_only_text = el.children.len() == 1 && matches!(el.children[0], Node::Text(_));
        if opts.pretty && !has_only_text {
            out.push_str(&opts.eol);
        }
        for child in &el.children {
            stringify_node(child, out, opts, depth + 1);
        }
        if opts.pretty && !has_only_text {
            for _ in 0..depth {
                out.push_str(&opts.indent);
            }
        }
        out.push_str("</");
        out.push_str(&el.name);
        out.push('>');
    }
}
