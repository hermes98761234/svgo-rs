use std::collections::HashMap;
use std::sync::Arc;

use crate::ast::Document;

/// A plugin that can transform a Document.
pub trait Plugin: Send + Sync {
    /// The unique name of this plugin (e.g. `"removeDoctype"`).
    fn name(&self) -> &'static str;

    /// Apply this plugin's transformation to the document.
    fn apply(&self, doc: &mut Document, params: &serde_json::Value);
}

/// A factory that creates plugin instances.
pub type PluginFactory = Arc<dyn Fn(&serde_json::Value) -> Box<dyn Plugin> + Send + Sync>;

/// A registry mapping plugin names to factories.
///
/// In svgo-plugins, `register_all` populates a Registry with all
/// preset-default plugins. Plugins are instantiated with their params
/// at optimization time.
#[derive(Default)]
pub struct Registry {
    factories: HashMap<String, PluginFactory>,
}

impl Registry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin factory under the given name.
    pub fn register(&mut self, name: &str, factory: PluginFactory) {
        self.factories.insert(name.to_string(), factory);
    }

    /// Instantiate a plugin by name with the given params.
    pub fn instantiate(&self, name: &str, params: &serde_json::Value) -> Option<Box<dyn Plugin>> {
        self.factories.get(name).map(|f| f(params))
    }

    /// Check whether a plugin is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.factories.contains_key(name)
    }

    /// Returns all registered plugin names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.factories.keys().map(|s| s.as_str())
    }
}
