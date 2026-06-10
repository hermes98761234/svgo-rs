use indexmap::IndexMap;

/// Root document node.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document {
    pub children: Vec<Node>,
}

/// Any node in the SVG/XML tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
    CData(String),
    Doctype(String),
    Instruction { name: String, value: String },
}

/// An XML/SVG element.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Element {
    pub name: String,
    pub attributes: IndexMap<String, String>,
    pub children: Vec<Node>,
    /// If true, this element was originally parsed as self-closing (<elem/>)
    /// and should be stringified as such even if children are added later.
    /// If false and children is empty, still self-close (optimization).
    /// If false and the original source had explicit open+close tags with no
    /// children, preserve that form.
    pub self_closing: bool,
}

impl Element {
    /// Get an attribute value by name.
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Set or insert an attribute.
    pub fn set_attr(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.attributes.insert(name.into(), value.into());
    }

    /// Remove an attribute by name.
    pub fn remove_attr(&mut self, name: &str) {
        self.attributes.shift_remove(name);
    }

    /// Returns true if this element has no children.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns a mutable iterator over child elements only (skips text, comments, etc.).
    pub fn child_elements_mut(&mut self) -> impl Iterator<Item = &mut Element> + '_ {
        self.children.iter_mut().filter_map(|n| match n {
            Node::Element(el) => Some(el),
            _ => None,
        })
    }
}
