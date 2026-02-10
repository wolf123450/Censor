use serde::{Deserialize, Serialize};
use std::fs;
use crate::models::{AppConfig, LocaleConfig, NudityStandards};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
}

impl Config {
    pub fn default() -> Self {
        Config {
            app: AppConfig {
                ollama_endpoint: "http://localhost:11434".to_string(),
                ollama_model: "qwen3-vl:30b".to_string(),
                default_censoring_method: "blur".to_string(),
                supported_locales: vec![
                    LocaleConfig {
                        code: "us".to_string(),
                        name: "United States".to_string(),
                        nudity_standards: NudityStandards {
                            censor_female_nipples: true,
                            censor_genitals: true,
                            censor_buttocks: false,
                            strictness_level: 5,
                        },
                    },
                    LocaleConfig {
                        code: "eu".to_string(),
                        name: "European Union".to_string(),
                        nudity_standards: NudityStandards {
                            censor_female_nipples: false,
                            censor_genitals: true,
                            censor_buttocks: false,
                            strictness_level: 3,
                        },
                    },
                    LocaleConfig {
                        code: "jp".to_string(),
                        name: "Japan".to_string(),
                        nudity_standards: NudityStandards {
                            censor_female_nipples: true,
                            censor_genitals: true,
                            censor_buttocks: true,
                            strictness_level: 8,
                        },
                    },
                ],
            },
        }
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        fs::write(path, content)
            .map_err(|e| format!("Failed to write config: {}", e))
    }
}
