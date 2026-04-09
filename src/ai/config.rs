use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub openai_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
    pub model: Option<String>,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            openai_api_key: None,
            anthropic_api_key: None,
            model: None,
        }
    }
}

impl AiConfig {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("xeli")
            .join("config.toml")
    }

    pub fn load() -> Self {
        let mut config = Self::default();

        // Try loading from config file
        if let Ok(content) = std::fs::read_to_string(Self::config_path()) {
            if let Ok(file_config) = toml::from_str::<AiConfig>(&content) {
                config = file_config;
            }
        }

        // Env vars override file config
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            config.openai_api_key = Some(key);
            if config.provider == "openai" || config.anthropic_api_key.is_none() {
                config.provider = "openai".to_string();
            }
        }
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            config.anthropic_api_key = Some(key);
            if config.openai_api_key.is_none() {
                config.provider = "anthropic".to_string();
            }
        }

        config
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn has_api_key(&self) -> bool {
        match self.provider.as_str() {
            "openai" => self.openai_api_key.is_some(),
            "anthropic" => self.anthropic_api_key.is_some(),
            _ => false,
        }
    }
}
