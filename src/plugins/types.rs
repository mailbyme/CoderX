use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub enabled: bool,
    pub path: String,
    pub commands: Vec<String>,
    pub abilities: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub when_to_use: Option<String>,
    pub argument_hint: Option<String>,
    pub allowed_tools: Vec<String>,
    pub model: Option<String>,
    pub content: String,
    pub source: AbilitySource,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AbilitySource {
    Builtin,
    User,
    Project,
    Plugin(String),
}

#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: Option<String>,
    pub commands: Vec<CommandDefinition>,
    pub abilities: Vec<AbilityDefinition>,
    pub hooks: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CommandDefinition {
    pub name: String,
    pub description: String,
    pub template: String,
}

#[derive(Debug, Clone)]
pub struct AbilityDefinition {
    pub name: String,
    pub description: String,
    pub when_to_use: Option<String>,
    pub argument_hint: Option<String>,
    pub allowed_tools: Vec<String>,
}

impl Plugin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: None,
            enabled: true,
            path: String::new(),
            commands: Vec::new(),
            abilities: Vec::new(),
        }
    }

    pub fn from_manifest(manifest: &PluginManifest, path: &str) -> Self {
        Self {
            name: manifest.name.clone(),
            version: manifest.version.clone(),
            description: manifest.description.clone(),
            author: manifest.author.clone(),
            enabled: true,
            path: path.to_string(),
            commands: manifest.commands.iter().map(|c| c.name.clone()).collect(),
            abilities: manifest.abilities.iter().map(|s| s.name.clone()).collect(),
        }
    }
}

impl Ability {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            when_to_use: None,
            argument_hint: None,
            allowed_tools: Vec::new(),
            model: None,
            content: String::new(),
            source: AbilitySource::User,
        }
    }

    pub fn builtin(name: &str, description: &str, content: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            when_to_use: None,
            argument_hint: None,
            allowed_tools: Vec::new(),
            model: None,
            content: content.to_string(),
            source: AbilitySource::Builtin,
        }
    }

    pub fn with_allowed_tools(mut self, tools: Vec<&str>) -> Self {
        self.allowed_tools = tools.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_argument_hint(mut self, hint: &str) -> Self {
        self.argument_hint = Some(hint.to_string());
        self
    }

    pub fn with_when_to_use(mut self, when: &str) -> Self {
        self.when_to_use = Some(when.to_string());
        self
    }

    pub fn get_prompt(&self, args: &str) -> String {
        let mut prompt = format!("# Ability: {}\n\n", self.name);
        
        if let Some(ref when) = self.when_to_use {
            prompt.push_str(&format!("When to use: {}\n\n", when));
        }

        prompt.push_str(&self.content);

        if !args.is_empty() {
            prompt.push_str(&format!("\n\nArguments: {}", args));
        }

        prompt
    }
}

impl PluginManifest {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            description: String::new(),
            author: None,
            commands: Vec::new(),
            abilities: Vec::new(),
            hooks: HashMap::new(),
        }
    }
}
