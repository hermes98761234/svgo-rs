use indexmap::IndexMap;
use thiserror::Error;

use crate::ast::{Document, Element, Node};

/// Errors that can occur during XML/SVG parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("XML parsing error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("UTF-8 encoding error")]
    Utf8,
    #[error("unexpected EOF")]
    UnexpectedEof,
    #[error("expected closing tag </{expected}>, found </{found}>")]
    MismatchedTag { expected: String, found: String },
}

fn esc(bytes: &[u8]) -> Result<&str, ParseError> {
    std::str::from_utf8(bytes).map_err(|_| ParseError::Utf8)
}

/// Parse an SVG/XML string into a Document AST.
pub fn parse(input: &str) -> Result<Document, ParseError> {
    let mut reader = quick_xml::Reader::from_str(input);
    reader.config_mut().check_end_names = true;

    let mut doc = Document::default();
    let mut buf = Vec::new();
    // Stack of (element, children_vec).
    let mut stack: Vec<(Element, Vec<Node>)> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(e)) => {
                let name = esc(e.name().as_ref())?.to_string();

                let mut attrs = IndexMap::new();
                for attr_result in e.attributes() {
                    let attr = attr_result.map_err(quick_xml::Error::from)?;
                    let key = esc(attr.key.as_ref())?.to_string();
                    let value = attr.unescape_value()?.to_string();
                    attrs.insert(key, value);
                }

                let element = Element {
                    name,
                    attributes: attrs,
                    children: Vec::new(),
                    self_closing: false,
                };
                stack.push((element, Vec::new()));
            }
            Ok(quick_xml::events::Event::End(_)) => {
                let (mut element, children) = stack.pop().expect("unexpected End event");
                element.children = children;
                let node = Node::Element(element);
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::Empty(e)) => {
                let name = esc(e.name().as_ref())?.to_string();

                let mut attrs = IndexMap::new();
                for attr_result in e.attributes() {
                    let attr = attr_result.map_err(quick_xml::Error::from)?;
                    let key = esc(attr.key.as_ref())?.to_string();
                    let value = attr.unescape_value()?.to_string();
                    attrs.insert(key, value);
                }

                let element = Element {
                    name,
                    attributes: attrs,
                    children: Vec::new(),
                    self_closing: true,
                };
                let node = Node::Element(element);
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::Text(e)) => {
                let text = e.unescape()?.to_string();
                let node = Node::Text(text);
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::Comment(e)) => {
                let text = esc(&e)?;
                let node = Node::Comment(text.to_string());
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::CData(e)) => {
                let inner = e.into_inner();
                let text = esc(&inner)?;
                let node = Node::CData(text.to_string());
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::Decl(e)) => {
                // XML declaration like <?xml version="1.0" encoding="UTF-8"?>
                // Encode as an Instruction node for roundtrip fidelity.
                let raw = esc(e.as_ref())?;
                // The raw bytes look like `xml version="1.0" ...`
                // Emit as Instruction with name="xml" and value="version=\"1.0\" ..."
                let (name, value) = raw.find(' ').map_or_else(
                    || (raw.to_string(), String::new()),
                    |i| (raw[..i].to_string(), raw[i..].trim().to_string()),
                );
                let node = Node::Instruction { name, value };
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::DocType(e)) => {
                let text = esc(&e)?;
                let node = Node::Doctype(text.to_string());
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::PI(e)) => {
                let raw = esc(&e)?;
                let (name, value) = raw.find(' ').map_or_else(
                    || (raw.to_string(), String::new()),
                    |i| {
                        let name = raw[..i].trim().to_string();
                        let value = raw[i..].trim().to_string();
                        if name.is_empty() {
                            (String::new(), value)
                        } else {
                            (name, value)
                        }
                    },
                );
                let node = Node::Instruction { name, value };
                if let Some((_, parent_children)) = stack.last_mut() {
                    parent_children.push(node);
                } else {
                    doc.children.push(node);
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(e.into()),
        }
        buf.clear();
    }

    Ok(doc)
}
