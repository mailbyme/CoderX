use crate::infrastructure::json_parser::JsonParser;
use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub provider: ProviderConfig,
    pub general: GeneralConfig,
}

#[derive(Debug, Clone)]
pub struct ProviderConfig {
    pub current_provider: String,
    pub anthropic: AnthropicConfig,
    pub openai: OpenAIConfig,
    pub bedrock: BedrockConfig,
    pub vertex: VertexConfig,
    pub meta: MetaConfig,
    pub mistral: MistralConfig,
    pub qwen: QwenConfig,
    pub wenxin: WenxinConfig,
    pub hunyuan: HunyuanConfig,
    pub glm: GlmConfig,
    pub deepseek: DeepSeekConfig,
    pub yi: YiConfig,
    pub cohere: CohereConfig,
    pub xiaomi: XiaomiConfig,
    pub custom: CustomConfig,
}

#[derive(Debug, Clone)]
pub struct AnthropicConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OpenAIConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BedrockConfig {
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub region: Option<String>,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct VertexConfig {
    pub project_id: Option<String>,
    pub location: Option<String>,
    pub api_key: Option<String>,
    pub model: String,
}

#[derive(Debug, Clone)]
pub struct MetaConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MistralConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct QwenConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WenxinConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HunyuanConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GlmConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct YiConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CohereConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct XiaomiConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CustomConfig {
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeneralConfig {
    pub language: String,
    pub auto_save: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: ProviderConfig::default(),
            general: GeneralConfig::default(),
        }
    }
}

impl Default for ProviderConfig {
    fn default() -> Self {
        ProviderConfig {
            current_provider: "anthropic".to_string(),
            anthropic: AnthropicConfig::default(),
            openai: OpenAIConfig::default(),
            bedrock: BedrockConfig::default(),
            vertex: VertexConfig::default(),
            meta: MetaConfig::default(),
            mistral: MistralConfig::default(),
            qwen: QwenConfig::default(),
            wenxin: WenxinConfig::default(),
            hunyuan: HunyuanConfig::default(),
            glm: GlmConfig::default(),
            deepseek: DeepSeekConfig::default(),
            yi: YiConfig::default(),
            cohere: CohereConfig::default(),
            xiaomi: XiaomiConfig::default(),
            custom: CustomConfig::default(),
        }
    }
}

impl Default for AnthropicConfig {
    fn default() -> Self {
        AnthropicConfig {
            api_key: None,
            model: "claude-sonnet-4-6".to_string(),
            base_url: None,
        }
    }
}

impl Default for OpenAIConfig {
    fn default() -> Self {
        OpenAIConfig {
            api_key: None,
            model: "gpt-5.4".to_string(),
            base_url: None,
        }
    }
}

impl Default for BedrockConfig {
    fn default() -> Self {
        BedrockConfig {
            access_key: None,
            secret_key: None,
            region: None,
            model: "anthropic.claude-sonnet-4-20240229-v1:0".to_string(),
        }
    }
}

impl Default for VertexConfig {
    fn default() -> Self {
        VertexConfig {
            project_id: None,
            location: None,
            api_key: None,
            model: "gemini-2.0-flash".to_string(),
        }
    }
}

impl Default for MetaConfig {
    fn default() -> Self {
        MetaConfig {
            api_key: None,
            model: "llama-3-70b-instruct".to_string(),
            base_url: None,
        }
    }
}

impl Default for MistralConfig {
    fn default() -> Self {
        MistralConfig {
            api_key: None,
            model: "mistral-large-latest".to_string(),
            base_url: None,
        }
    }
}

impl Default for QwenConfig {
    fn default() -> Self {
        QwenConfig {
            api_key: None,
            model: "qwen-plus".to_string(),
            base_url: None,
        }
    }
}

impl Default for WenxinConfig {
    fn default() -> Self {
        WenxinConfig {
            api_key: None,
            model: "ernie-4.0".to_string(),
            base_url: None,
        }
    }
}

impl Default for HunyuanConfig {
    fn default() -> Self {
        HunyuanConfig {
            api_key: None,
            model: "hunyuan-pro".to_string(),
            base_url: None,
        }
    }
}

impl Default for GlmConfig {
    fn default() -> Self {
        GlmConfig {
            api_key: None,
            model: "glm-4".to_string(),
            base_url: None,
        }
    }
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        DeepSeekConfig {
            api_key: None,
            model: "deepseek-chat".to_string(),
            base_url: None,
        }
    }
}

impl Default for YiConfig {
    fn default() -> Self {
        YiConfig {
            api_key: None,
            model: "yi-plus".to_string(),
            base_url: None,
        }
    }
}

impl Default for CohereConfig {
    fn default() -> Self {
        CohereConfig {
            api_key: None,
            model: "command-r-plus".to_string(),
            base_url: None,
        }
    }
}

impl Default for XiaomiConfig {
    fn default() -> Self {
        XiaomiConfig {
            api_key: None,
            model: "mi-large".to_string(),
            base_url: None,
        }
    }
}

impl Default for CustomConfig {
    fn default() -> Self {
        CustomConfig {
            api_key: None,
            model: "default".to_string(),
            base_url: None,
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        GeneralConfig {
            language: "en".to_string(),
            auto_save: true,
        }
    }
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }
        
        let mut file = File::open(&config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        Self::from_json(&content)
    }
    
    pub fn save(&self) -> io::Result<()> {
        let config_path = Self::get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = self.to_json();
        let mut file = File::create(&config_path)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    fn get_config_path() -> io::Result<PathBuf> {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
        let config_dir = match home {
            Ok(home) => PathBuf::from(home).join(".config").join("coderex"),
            Err(_) => env::current_dir()?.join(".coderex"),
        };
        
        Ok(config_dir.join("config.json"))
    }
    
    fn from_json(json: &str) -> io::Result<Self> {
        let value = JsonParser::parse(json).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("JSON parse error: {}", e))
        })?;
        
        let mut config = Config::default();
        
        if let Some(obj) = value.as_object() {
            if let Some(provider_obj) = obj.get("provider").and_then(|v| v.as_object()) {
                if let Some(current) = provider_obj.get("current_provider").and_then(|v| v.as_str()) {
                    config.provider.current_provider = current.to_string();
                }
                
                if let Some(anthropic_obj) = provider_obj.get("anthropic").and_then(|v| v.as_object()) {
                    if let Some(key) = anthropic_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.anthropic.api_key = Some(key.to_string());
                    }
                    if let Some(model) = anthropic_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.anthropic.model = model.to_string();
                    }
                    if let Some(url) = anthropic_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.anthropic.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(openai_obj) = provider_obj.get("openai").and_then(|v| v.as_object()) {
                    if let Some(key) = openai_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.openai.api_key = Some(key.to_string());
                    }
                    if let Some(model) = openai_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.openai.model = model.to_string();
                    }
                    if let Some(url) = openai_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.openai.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(bedrock_obj) = provider_obj.get("bedrock").and_then(|v| v.as_object()) {
                    if let Some(key) = bedrock_obj.get("access_key").and_then(|v| v.as_str()) {
                        config.provider.bedrock.access_key = Some(key.to_string());
                    }
                    if let Some(key) = bedrock_obj.get("secret_key").and_then(|v| v.as_str()) {
                        config.provider.bedrock.secret_key = Some(key.to_string());
                    }
                    if let Some(region) = bedrock_obj.get("region").and_then(|v| v.as_str()) {
                        config.provider.bedrock.region = Some(region.to_string());
                    }
                    if let Some(model) = bedrock_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.bedrock.model = model.to_string();
                    }
                }
                
                if let Some(vertex_obj) = provider_obj.get("vertex").and_then(|v| v.as_object()) {
                    if let Some(id) = vertex_obj.get("project_id").and_then(|v| v.as_str()) {
                        config.provider.vertex.project_id = Some(id.to_string());
                    }
                    if let Some(loc) = vertex_obj.get("location").and_then(|v| v.as_str()) {
                        config.provider.vertex.location = Some(loc.to_string());
                    }
                    if let Some(key) = vertex_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.vertex.api_key = Some(key.to_string());
                    }
                    if let Some(model) = vertex_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.vertex.model = model.to_string();
                    }
                }
                
                if let Some(meta_obj) = provider_obj.get("meta").and_then(|v| v.as_object()) {
                    if let Some(key) = meta_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.meta.api_key = Some(key.to_string());
                    }
                    if let Some(model) = meta_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.meta.model = model.to_string();
                    }
                    if let Some(url) = meta_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.meta.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(mistral_obj) = provider_obj.get("mistral").and_then(|v| v.as_object()) {
                    if let Some(key) = mistral_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.mistral.api_key = Some(key.to_string());
                    }
                    if let Some(model) = mistral_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.mistral.model = model.to_string();
                    }
                    if let Some(url) = mistral_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.mistral.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(qwen_obj) = provider_obj.get("qwen").and_then(|v| v.as_object()) {
                    if let Some(key) = qwen_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.qwen.api_key = Some(key.to_string());
                    }
                    if let Some(model) = qwen_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.qwen.model = model.to_string();
                    }
                    if let Some(url) = qwen_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.qwen.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(wenxin_obj) = provider_obj.get("wenxin").and_then(|v| v.as_object()) {
                    if let Some(key) = wenxin_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.wenxin.api_key = Some(key.to_string());
                    }
                    if let Some(model) = wenxin_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.wenxin.model = model.to_string();
                    }
                    if let Some(url) = wenxin_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.wenxin.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(hunyuan_obj) = provider_obj.get("hunyuan").and_then(|v| v.as_object()) {
                    if let Some(key) = hunyuan_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.hunyuan.api_key = Some(key.to_string());
                    }
                    if let Some(model) = hunyuan_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.hunyuan.model = model.to_string();
                    }
                    if let Some(url) = hunyuan_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.hunyuan.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(glm_obj) = provider_obj.get("glm").and_then(|v| v.as_object()) {
                    if let Some(key) = glm_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.glm.api_key = Some(key.to_string());
                    }
                    if let Some(model) = glm_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.glm.model = model.to_string();
                    }
                    if let Some(url) = glm_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.glm.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(deepseek_obj) = provider_obj.get("deepseek").and_then(|v| v.as_object()) {
                    if let Some(key) = deepseek_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.deepseek.api_key = Some(key.to_string());
                    }
                    if let Some(model) = deepseek_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.deepseek.model = model.to_string();
                    }
                    if let Some(url) = deepseek_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.deepseek.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(yi_obj) = provider_obj.get("yi").and_then(|v| v.as_object()) {
                    if let Some(key) = yi_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.yi.api_key = Some(key.to_string());
                    }
                    if let Some(model) = yi_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.yi.model = model.to_string();
                    }
                    if let Some(url) = yi_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.yi.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(cohere_obj) = provider_obj.get("cohere").and_then(|v| v.as_object()) {
                    if let Some(key) = cohere_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.cohere.api_key = Some(key.to_string());
                    }
                    if let Some(model) = cohere_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.cohere.model = model.to_string();
                    }
                    if let Some(url) = cohere_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.cohere.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(xiaomi_obj) = provider_obj.get("xiaomi").and_then(|v| v.as_object()) {
                    if let Some(key) = xiaomi_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.xiaomi.api_key = Some(key.to_string());
                    }
                    if let Some(model) = xiaomi_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.xiaomi.model = model.to_string();
                    }
                    if let Some(url) = xiaomi_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.xiaomi.base_url = Some(url.to_string());
                    }
                }
                
                if let Some(custom_obj) = provider_obj.get("custom").and_then(|v| v.as_object()) {
                    if let Some(key) = custom_obj.get("api_key").and_then(|v| v.as_str()) {
                        config.provider.custom.api_key = Some(key.to_string());
                    }
                    if let Some(model) = custom_obj.get("model").and_then(|v| v.as_str()) {
                        config.provider.custom.model = model.to_string();
                    }
                    if let Some(url) = custom_obj.get("base_url").and_then(|v| v.as_str()) {
                        config.provider.custom.base_url = Some(url.to_string());
                    }
                }
            }
            
            if let Some(general_obj) = obj.get("general").and_then(|v| v.as_object()) {
                if let Some(lang) = general_obj.get("language").and_then(|v| v.as_str()) {
                    config.general.language = lang.to_string();
                }
                if let Some(auto_save) = general_obj.get("auto_save").and_then(|v| v.as_bool()) {
                    config.general.auto_save = auto_save;
                }
            }
        }
        
        Ok(config)
    }
    
    fn to_json(&self) -> String {
        let mut anthropic_fields = Vec::new();
        if let Some(key) = &self.provider.anthropic.api_key {
            anthropic_fields.push(("api_key", JsonParser::string(key)));
        }
        anthropic_fields.push(("model", JsonParser::string(&self.provider.anthropic.model)));
        if let Some(url) = &self.provider.anthropic.base_url {
            anthropic_fields.push(("base_url", JsonParser::string(url)));
        }
        let anthropic_obj = JsonParser::object(&anthropic_fields);
        
        let mut openai_fields = Vec::new();
        if let Some(key) = &self.provider.openai.api_key {
            openai_fields.push(("api_key", JsonParser::string(key)));
        }
        openai_fields.push(("model", JsonParser::string(&self.provider.openai.model)));
        if let Some(url) = &self.provider.openai.base_url {
            openai_fields.push(("base_url", JsonParser::string(url)));
        }
        let openai_obj = JsonParser::object(&openai_fields);
        
        let mut bedrock_fields = Vec::new();
        if let Some(key) = &self.provider.bedrock.access_key {
            bedrock_fields.push(("access_key", JsonParser::string(key)));
        }
        if let Some(key) = &self.provider.bedrock.secret_key {
            bedrock_fields.push(("secret_key", JsonParser::string(key)));
        }
        if let Some(region) = &self.provider.bedrock.region {
            bedrock_fields.push(("region", JsonParser::string(region)));
        }
        bedrock_fields.push(("model", JsonParser::string(&self.provider.bedrock.model)));
        let bedrock_obj = JsonParser::object(&bedrock_fields);
        
        let mut vertex_fields = Vec::new();
        if let Some(id) = &self.provider.vertex.project_id {
            vertex_fields.push(("project_id", JsonParser::string(id)));
        }
        if let Some(loc) = &self.provider.vertex.location {
            vertex_fields.push(("location", JsonParser::string(loc)));
        }
        if let Some(key) = &self.provider.vertex.api_key {
            vertex_fields.push(("api_key", JsonParser::string(key)));
        }
        vertex_fields.push(("model", JsonParser::string(&self.provider.vertex.model)));
        let vertex_obj = JsonParser::object(&vertex_fields);
        
        let mut meta_fields = Vec::new();
        if let Some(key) = &self.provider.meta.api_key {
            meta_fields.push(("api_key", JsonParser::string(key)));
        }
        meta_fields.push(("model", JsonParser::string(&self.provider.meta.model)));
        if let Some(url) = &self.provider.meta.base_url {
            meta_fields.push(("base_url", JsonParser::string(url)));
        }
        let meta_obj = JsonParser::object(&meta_fields);
        
        let mut mistral_fields = Vec::new();
        if let Some(key) = &self.provider.mistral.api_key {
            mistral_fields.push(("api_key", JsonParser::string(key)));
        }
        mistral_fields.push(("model", JsonParser::string(&self.provider.mistral.model)));
        if let Some(url) = &self.provider.mistral.base_url {
            mistral_fields.push(("base_url", JsonParser::string(url)));
        }
        let mistral_obj = JsonParser::object(&mistral_fields);
        
        let mut qwen_fields = Vec::new();
        if let Some(key) = &self.provider.qwen.api_key {
            qwen_fields.push(("api_key", JsonParser::string(key)));
        }
        qwen_fields.push(("model", JsonParser::string(&self.provider.qwen.model)));
        if let Some(url) = &self.provider.qwen.base_url {
            qwen_fields.push(("base_url", JsonParser::string(url)));
        }
        let qwen_obj = JsonParser::object(&qwen_fields);
        
        let mut wenxin_fields = Vec::new();
        if let Some(key) = &self.provider.wenxin.api_key {
            wenxin_fields.push(("api_key", JsonParser::string(key)));
        }
        wenxin_fields.push(("model", JsonParser::string(&self.provider.wenxin.model)));
        if let Some(url) = &self.provider.wenxin.base_url {
            wenxin_fields.push(("base_url", JsonParser::string(url)));
        }
        let wenxin_obj = JsonParser::object(&wenxin_fields);
        
        let mut hunyuan_fields = Vec::new();
        if let Some(key) = &self.provider.hunyuan.api_key {
            hunyuan_fields.push(("api_key", JsonParser::string(key)));
        }
        hunyuan_fields.push(("model", JsonParser::string(&self.provider.hunyuan.model)));
        if let Some(url) = &self.provider.hunyuan.base_url {
            hunyuan_fields.push(("base_url", JsonParser::string(url)));
        }
        let hunyuan_obj = JsonParser::object(&hunyuan_fields);
        
        let mut glm_fields = Vec::new();
        if let Some(key) = &self.provider.glm.api_key {
            glm_fields.push(("api_key", JsonParser::string(key)));
        }
        glm_fields.push(("model", JsonParser::string(&self.provider.glm.model)));
        if let Some(url) = &self.provider.glm.base_url {
            glm_fields.push(("base_url", JsonParser::string(url)));
        }
        let glm_obj = JsonParser::object(&glm_fields);
        
        let mut deepseek_fields = Vec::new();
        if let Some(key) = &self.provider.deepseek.api_key {
            deepseek_fields.push(("api_key", JsonParser::string(key)));
        }
        deepseek_fields.push(("model", JsonParser::string(&self.provider.deepseek.model)));
        if let Some(url) = &self.provider.deepseek.base_url {
            deepseek_fields.push(("base_url", JsonParser::string(url)));
        }
        let deepseek_obj = JsonParser::object(&deepseek_fields);
        
        let mut yi_fields = Vec::new();
        if let Some(key) = &self.provider.yi.api_key {
            yi_fields.push(("api_key", JsonParser::string(key)));
        }
        yi_fields.push(("model", JsonParser::string(&self.provider.yi.model)));
        if let Some(url) = &self.provider.yi.base_url {
            yi_fields.push(("base_url", JsonParser::string(url)));
        }
        let yi_obj = JsonParser::object(&yi_fields);
        
        let mut cohere_fields = Vec::new();
        if let Some(key) = &self.provider.cohere.api_key {
            cohere_fields.push(("api_key", JsonParser::string(key)));
        }
        cohere_fields.push(("model", JsonParser::string(&self.provider.cohere.model)));
        if let Some(url) = &self.provider.cohere.base_url {
            cohere_fields.push(("base_url", JsonParser::string(url)));
        }
        let cohere_obj = JsonParser::object(&cohere_fields);
        
        let mut xiaomi_fields = Vec::new();
        if let Some(key) = &self.provider.xiaomi.api_key {
            xiaomi_fields.push(("api_key", JsonParser::string(key)));
        }
        xiaomi_fields.push(("model", JsonParser::string(&self.provider.xiaomi.model)));
        if let Some(url) = &self.provider.xiaomi.base_url {
            xiaomi_fields.push(("base_url", JsonParser::string(url)));
        }
        let xiaomi_obj = JsonParser::object(&xiaomi_fields);
        
        let mut custom_fields = Vec::new();
        if let Some(key) = &self.provider.custom.api_key {
            custom_fields.push(("api_key", JsonParser::string(key)));
        }
        custom_fields.push(("model", JsonParser::string(&self.provider.custom.model)));
        if let Some(url) = &self.provider.custom.base_url {
            custom_fields.push(("base_url", JsonParser::string(url)));
        }
        let custom_obj = JsonParser::object(&custom_fields);
        
        let provider_obj = JsonParser::object(&[
            ("current_provider", JsonParser::string(&self.provider.current_provider)),
            ("anthropic", anthropic_obj),
            ("openai", openai_obj),
            ("bedrock", bedrock_obj),
            ("vertex", vertex_obj),
            ("meta", meta_obj),
            ("mistral", mistral_obj),
            ("qwen", qwen_obj),
            ("wenxin", wenxin_obj),
            ("hunyuan", hunyuan_obj),
            ("glm", glm_obj),
            ("deepseek", deepseek_obj),
            ("yi", yi_obj),
            ("cohere", cohere_obj),
            ("xiaomi", xiaomi_obj),
            ("custom", custom_obj),
        ]);
        
        let general_obj = JsonParser::object(&[
            ("language", JsonParser::string(&self.general.language)),
            ("auto_save", JsonParser::bool(self.general.auto_save)),
        ]);
        
        let root_obj = JsonParser::object(&[
            ("provider", provider_obj),
            ("general", general_obj),
        ]);
        
        JsonParser::serialize(&root_obj)
    }
}
