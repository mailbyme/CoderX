use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use crate::i18n::Language;

#[derive(Debug, Clone)]
pub struct Config {
    pub model: String,
    pub provider: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub language: Language,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: "claude-3-5-haiku-20241022".to_string(),
            provider: "anthropic".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            language: Self::detect_language(),
        }
    }
}

impl Config {
    fn detect_language() -> Language {
        if let Ok(lang) = std::env::var("LANG") {
            if lang.starts_with("zh") || lang.starts_with("zh_CN") {
                return Language::Chinese;
            }
        }
        Language::English
    }
}

pub struct SessionState {
    config: RwLock<Config>,
    is_running: RwLock<bool>,
    variables: RwLock<HashMap<String, String>>,
}

pub type SharedSessionState = Arc<SessionState>;

impl SessionState {
    pub fn new() -> SharedSessionState {
        Arc::new(Self {
            config: RwLock::new(Config::default()),
            is_running: RwLock::new(true),
            variables: RwLock::new(HashMap::new()),
        })
    }

    pub fn update_config(&self, config: Config) {
        *self.config.write().unwrap() = config;
    }

    pub fn get_config(&self) -> Config {
        self.config.read().unwrap().clone()
    }

    pub fn set_variable(&self, key: &str, value: &str) {
        self.variables.write().unwrap().insert(key.to_string(), value.to_string());
    }

    pub fn get_variable(&self, key: &str) -> Option<String> {
        self.variables.read().unwrap().get(key).cloned()
    }

    pub fn stop(&self) {
        *self.is_running.write().unwrap() = false;
    }

    pub fn is_running(&self) -> bool {
        *self.is_running.read().unwrap()
    }
}
