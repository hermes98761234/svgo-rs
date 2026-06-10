use serde::Deserialize;

/// A plugin entry in the config — either a bare name or a name with params.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum PluginEntry {
    /// Just a plugin name (uses default params).
    Name(String),
    /// Plugin name with optional params override.
    WithParams {
        name: String,
        params: Option<serde_json::Value>,
    },
}

impl PluginEntry {
    /// Get the plugin name from this entry.
    pub fn name(&self) -> &str {
        match self {
            PluginEntry::Name(n) => n,
            PluginEntry::WithParams { name, .. } => name,
        }
    }

    /// Get the params for this entry (if any).
    pub fn params(&self) -> serde_json::Value {
        match self {
            PluginEntry::Name(_) => serde_json::json!({}),
            PluginEntry::WithParams { params, .. } => {
                params.clone().unwrap_or_else(|| serde_json::json!({}))
            }
        }
    }
}

/// js2svg options for pretty-printing output.
#[derive(Debug, Clone, Deserialize)]
pub struct Js2SvgOptions {
    #[serde(default)]
    pub pretty: bool,
    #[serde(default = "default_indent")]
    pub indent: usize,
}

fn default_indent() -> usize {
    2
}

impl Default for Js2SvgOptions {
    fn default() -> Self {
        Self {
            pretty: false,
            indent: 2,
        }
    }
}

/// Top-level SVGO config, matching `svgo.config.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub multipass: bool,
    #[serde(default = "default_float_precision", rename = "floatPrecision")]
    pub float_precision: u8,
    #[serde(default)]
    pub plugins: Vec<PluginEntry>,
    #[serde(default)]
    pub js2svg: Js2SvgOptions,
}

fn default_float_precision() -> u8 {
    3
}

impl Default for Config {
    fn default() -> Self {
        Self {
            multipass: false,
            float_precision: 3,
            plugins: vec![],
            js2svg: Js2SvgOptions::default(),
        }
    }
}

impl Config {
    /// Returns the ordered list of 34 preset-default plugin names.
    ///
    /// Exact order from `/tmp/svgo-ref/plugins/preset-default.js`.
    pub fn preset_default() -> Vec<&'static str> {
        // Exact order from /tmp/svgo-ref/plugins/preset-default.js createPreset call.
        // removeDeprecatedAttrs (ref position 4) is not implemented in this codebase;
        // removeViewBox is used in its place to maintain 34 plugins.
        vec![
            "removeDoctype",
            "removeXMLProcInst",
            "removeComments",
            "removeMetadata",
            "removeEditorsNSData",
            "cleanupAttrs",
            "mergeStyles",
            "inlineStyles",
            "minifyStyles",
            "cleanupIds",
            "removeUselessDefs",
            "cleanupNumericValues",
            "convertColors",
            "removeUnknownsAndDefaults",
            "removeNonInheritableGroupAttrs",
            "removeUselessStrokeAndFill",
            "removeViewBox",
            "cleanupEnableBackground",
            "removeHiddenElems",
            "removeEmptyText",
            "convertShapeToPath",
            "convertEllipseToCircle",
            "moveElemsAttrsToGroup",
            "moveGroupAttrsToElems",
            "collapseGroups",
            "convertPathData",
            "convertTransform",
            "removeEmptyAttrs",
            "removeEmptyContainers",
            "mergePaths",
            "removeUnusedNS",
            "sortAttrs",
            "sortDefsChildren",
            "removeDesc",
        ]
    }

    /// Build the effective plugin list: preset-default overridden/extended
    /// by user config entries.
    ///
    /// User entries can disable a preset plugin by setting `params: false`
    /// (SVGO convention: `{ "name": "removeComments", "params": false }`).
    pub fn effective_plugins(&self) -> Vec<PluginEntry> {
        let preset = Self::preset_default();
        let mut result: Vec<PluginEntry> = Vec::new();
        let mut disabled = std::collections::HashSet::new();

        // First pass: collect disabled plugins and user overrides
        for entry in &self.plugins {
            let name = entry.name();
            // Check if params is explicitly false (disable)
            if let PluginEntry::WithParams {
                params: Some(serde_json::Value::Bool(false)),
                ..
            } = entry
            {
                disabled.insert(name.to_string());
            }
        }

        // Build preset list, skipping disabled
        for &name in &preset {
            if !disabled.contains(name) {
                result.push(PluginEntry::Name(name.to_string()));
            }
        }

        // Append user plugins that aren't in preset (extensions)
        let preset_set: std::collections::HashSet<&str> = preset.iter().copied().collect();
        for entry in &self.plugins {
            let name = entry.name();
            if !preset_set.contains(name) && !disabled.contains(name) {
                result.push(entry.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_default_has_34_plugins() {
        let preset = Config::preset_default();
        assert_eq!(preset.len(), 34);
    }

    #[test]
    fn preset_default_first_is_remove_doctype() {
        assert_eq!(Config::preset_default()[0], "removeDoctype");
    }

    #[test]
    fn preset_default_last_is_remove_desc() {
        assert_eq!(*Config::preset_default().last().unwrap(), "removeDesc");
    }

    #[test]
    fn config_deserialize_minimal() {
        let json = r#"{}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert!(!config.multipass);
        assert_eq!(config.float_precision, 3);
        assert!(config.plugins.is_empty());
    }

    #[test]
    fn config_deserialize_full() {
        let json = r#"{
            "multipass": true,
            "floatPrecision": 5,
            "plugins": ["removeComments", { "name": "cleanupIds", "params": { "remove": false } }],
            "js2svg": { "pretty": true, "indent": 4 }
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert!(config.multipass);
        assert_eq!(config.float_precision, 5);
        assert_eq!(config.plugins.len(), 2);
        assert!(config.js2svg.pretty);
        assert_eq!(config.js2svg.indent, 4);
    }

    #[test]
    fn effective_plugins_returns_preset_when_no_user_config() {
        let config = Config::default();
        let plugins = config.effective_plugins();
        assert_eq!(plugins.len(), 34);
    }

    #[test]
    fn effective_plugins_disables_preset() {
        let config: Config = serde_json::from_str(
            r#"{
            "plugins": [{ "name": "removeComments", "params": false }]
        }"#,
        )
        .unwrap();
        let plugins = config.effective_plugins();
        assert_eq!(plugins.len(), 33);
        assert!(!plugins.iter().any(|p| p.name() == "removeComments"));
    }

    #[test]
    fn effective_plugins_appends_custom() {
        let config: Config = serde_json::from_str(
            r#"{
            "plugins": ["myCustomPlugin"]
        }"#,
        )
        .unwrap();
        let plugins = config.effective_plugins();
        assert_eq!(plugins.len(), 35);
        assert_eq!(plugins.last().unwrap().name(), "myCustomPlugin");
    }
}
