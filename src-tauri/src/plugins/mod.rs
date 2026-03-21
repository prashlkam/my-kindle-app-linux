// Plugin system - future extensibility
// Plugins will be loaded from ~/.config/kindled/plugins/
// Supported plugin types:
// - Format readers (WASM modules that parse new file formats)
// - Dictionary providers
// - Sync backends
// - Theme packages
// - Annotation exporters

pub struct PluginManager {
    plugin_dir: std::path::PathBuf,
}

impl PluginManager {
    pub fn new(config_dir: std::path::PathBuf) -> Self {
        let plugin_dir = config_dir.join("plugins");
        std::fs::create_dir_all(&plugin_dir).ok();
        Self { plugin_dir }
    }

    pub fn plugin_dir(&self) -> &std::path::Path {
        &self.plugin_dir
    }
}
