use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProvider {
    Ollama {
        endpoint: String,
        model: String,
    },
    Gemini {
        api_key: String,
        model: String,
    },
    Claude {
        api_key: String,
        model: String,
    },
}

impl LLMProvider {
    pub fn from_config(backend: &str, model: &str, api_key: Option<&str>, endpoint: Option<&str>) -> Option<Self> {
        match backend {
            "ollama" => Some(LLMProvider::Ollama {
                endpoint: endpoint.unwrap_or("http://localhost:11434").to_string(),
                model: model.to_string(),
            }),
            "gemini" => {
                let key = api_key?;
                Some(LLMProvider::Gemini {
                    api_key: key.to_string(),
                    model: model.to_string(),
                })
            }
            "claude" => {
                let key = api_key?;
                Some(LLMProvider::Claude {
                    api_key: key.to_string(),
                    model: model.to_string(),
                })
            }
            _ => None,
        }
    }

    pub fn model_name(&self) -> &str {
        match self {
            LLMProvider::Ollama { model, .. } => model,
            LLMProvider::Gemini { model, .. } => model,
            LLMProvider::Claude { model, .. } => model,
        }
    }

    pub fn provider_name(&self) -> &str {
        match self {
            LLMProvider::Ollama { .. } => "ollama",
            LLMProvider::Gemini { .. } => "gemini",
            LLMProvider::Claude { .. } => "claude",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_from_config_ollama() {
        let provider = LLMProvider::from_config(
            "ollama",
            "llama2",
            None,
            Some("http://localhost:11434"),
        );

        assert!(provider.is_some());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "ollama");
        assert_eq!(provider.model_name(), "llama2");
    }

    #[test]
    fn test_provider_from_config_gemini() {
        let provider = LLMProvider::from_config(
            "gemini",
            "gemini-pro",
            Some("test_key"),
            None,
        );

        assert!(provider.is_some());
        let provider = provider.unwrap();
        assert_eq!(provider.provider_name(), "gemini");
        assert_eq!(provider.model_name(), "gemini-pro");
    }
}
