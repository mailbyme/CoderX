use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::io;

use super::types::{Plugin, PluginManifest};

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    plugin_dir: PathBuf,
    enabled_plugins: Vec<String>,
}

impl PluginManager {
    pub fn new() -> Self {
        let plugin_dir = Self::get_default_plugin_dir();

        Self {
            plugins: HashMap::new(),
            plugin_dir,
            enabled_plugins: Vec::new(),
        }
    }

    fn get_default_plugin_dir() -> PathBuf {
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            PathBuf::from(home).join(".coderex").join("plugins")
        } else {
            PathBuf::from(".coderex").join("plugins")
        }
    }

    pub fn set_plugin_dir(&mut self, path: PathBuf) {
        self.plugin_dir = path;
    }

    pub fn initialize(&mut self) -> io::Result<()> {
        fs::create_dir_all(&self.plugin_dir)?;
        self.load_all()?;
        Ok(())
    }

    pub fn load_all(&mut self) -> io::Result<()> {
        self.plugins.clear();

        if !self.plugin_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(plugin) = self.load_plugin(&path)? {
                    self.plugins.insert(plugin.name.clone(), plugin);
                }
            }
        }

        Ok(())
    }

    fn load_plugin(&self, path: &PathBuf) -> io::Result<Option<Plugin>> {
        let manifest_path = path.join("plugin.json");

        if !manifest_path.exists() {
            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");

            return Ok(Some(Plugin::new(name)));
        }

        let content = fs::read_to_string(&manifest_path)?;
        let manifest = self.parse_manifest(&content)?;

        let plugin = Plugin::from_manifest(&manifest, &path.to_string_lossy());
        Ok(Some(plugin))
    }

    fn parse_manifest(&self, content: &str) -> io::Result<PluginManifest> {
        let mut manifest = PluginManifest::new("unknown");

        let lines: Vec<&str> = content.lines().collect();
        let mut _in_object = false;
        let mut _current_key = String::new();
        let mut depth = 0;

        for line in lines {
            let trimmed = line.trim();

            if trimmed == "{" {
                _in_object = true;
                depth += 1;
                continue;
            }

            if trimmed == "}" {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                continue;
            }

            if trimmed.starts_with("\"name\"") {
                if let Some(value) = self.extract_string_value(trimmed) {
                    manifest.name = value;
                }
            } else if trimmed.starts_with("\"version\"") {
                if let Some(value) = self.extract_string_value(trimmed) {
                    manifest.version = value;
                }
            } else if trimmed.starts_with("\"description\"") {
                if let Some(value) = self.extract_string_value(trimmed) {
                    manifest.description = value;
                }
            } else if trimmed.starts_with("\"author\"") {
                if let Some(value) = self.extract_string_value(trimmed) {
                    manifest.author = Some(value);
                }
            }
        }

        Ok(manifest)
    }

    fn extract_string_value(&self, line: &str) -> Option<String> {
        if let Some(colon_pos) = line.find(':') {
            let after_colon = &line[colon_pos + 1..];
            let start = after_colon.find('"')?;
            let rest = &after_colon[start + 1..];
            let end = rest.find('"')?;
            return Some(rest[..end].to_string());
        }
        None
    }

    pub fn install(&mut self, name: &str, source: &str) -> io::Result<()> {
        let plugin_dir = self.plugin_dir.join(name);
        fs::create_dir_all(&plugin_dir)?;

        let manifest = PluginManifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: format!("Plugin installed from {}", source),
            author: None,
            commands: Vec::new(),
            abilities: Vec::new(),
            hooks: HashMap::new(),
        };

        let manifest_content = format!(
            r#"{{
    "name": "{}",
    "version": "{}",
    "description": "{}"
}}"#,
            manifest.name, manifest.version, manifest.description
        );

        fs::write(plugin_dir.join("plugin.json"), manifest_content)?;

        let plugin = Plugin::from_manifest(&manifest, &plugin_dir.to_string_lossy());
        self.plugins.insert(name.to_string(), plugin);

        Ok(())
    }

    pub fn uninstall(&mut self, name: &str) -> io::Result<()> {
        if let Some(plugin) = self.plugins.remove(name) {
            let plugin_path = PathBuf::from(&plugin.path);
            if plugin_path.exists() {
                fs::remove_dir_all(plugin_path)?;
            }
        }
        Ok(())
    }

    pub fn enable(&mut self, name: &str) {
        if let Some(plugin) = self.plugins.get_mut(name) {
            plugin.enabled = true;
            if !self.enabled_plugins.contains(&name.to_string()) {
                self.enabled_plugins.push(name.to_string());
            }
        }
    }

    pub fn disable(&mut self, name: &str) {
        if let Some(plugin) = self.plugins.get_mut(name) {
            plugin.enabled = false;
            self.enabled_plugins.retain(|n| n != name);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }

    pub fn list(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }

    pub fn list_enabled(&self) -> Vec<&Plugin> {
        self.plugins.values().filter(|p| p.enabled).collect()
    }

    pub fn get_commands(&self) -> HashMap<String, String> {
        let mut commands = HashMap::new();

        for plugin in self.plugins.values().filter(|p| p.enabled) {
            for cmd in &plugin.commands {
                commands.insert(cmd.clone(), plugin.name.clone());
            }
        }

        commands
    }

    pub fn get_plugin_count(&self) -> usize {
        self.plugins.len()
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}
