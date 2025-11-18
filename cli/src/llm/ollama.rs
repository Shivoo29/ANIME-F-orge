use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use super::get_system_prompt;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    endpoint: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    system: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
    done: bool,
}

impl OllamaClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn generate_animation_code(&self, prompt: &str, model: &str) -> Result<String> {
        let url = format!("{}/api/generate", self.endpoint);

        let full_prompt = format!(
            "{}\n\nUser request: {}\n\nGenerate the Manim animation code:",
            get_system_prompt(),
            prompt
        );

        let request = OllamaRequest {
            model: model.to_string(),
            prompt: full_prompt,
            stream: false,
            system: None,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context(format!(
                "Failed to connect to Ollama at {}. Is Ollama running?",
                self.endpoint
            ))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            anyhow::bail!(
                "Ollama API error ({}): {}\n\nMake sure Ollama is running: ollama serve",
                status,
                error_text
            );
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .context("Failed to parse Ollama response")?;

        Ok(self.extract_code(&ollama_response.response))
    }

    pub async fn generate_streaming<F>(
        &self,
        prompt: &str,
        model: &str,
        mut callback: F,
    ) -> Result<String>
    where
        F: FnMut(String),
    {
        let url = format!("{}/api/generate", self.endpoint);

        let full_prompt = format!(
            "{}\n\nUser request: {}\n\nGenerate the Manim animation code:",
            get_system_prompt(),
            prompt
        );

        let request = OllamaRequest {
            model: model.to_string(),
            prompt: full_prompt,
            stream: true,
            system: None,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        if !response.status().is_success() {
            anyhow::bail!("Ollama API error: {}", response.status());
        }

        let mut full_response = String::new();
        let body = response.text().await?;

        // Parse streaming response (NDJSON format)
        for line in body.lines() {
            if let Ok(chunk) = serde_json::from_str::<OllamaResponse>(line) {
                full_response.push_str(&chunk.response);
                callback(chunk.response);

                if chunk.done {
                    break;
                }
            }
        }

        Ok(self.extract_code(&full_response))
    }

    /// Extract Python code from the response, removing markdown formatting if present
    fn extract_code(&self, response: &str) -> String {
        let response = response.trim();

        // Check if response is wrapped in markdown code blocks
        if response.starts_with("```python") || response.starts_with("```") {
            // Extract code from markdown
            let lines: Vec<&str> = response.lines().collect();

            let start = if lines[0].starts_with("```") { 1 } else { 0 };

            let end = lines
                .iter()
                .rposition(|line| line.trim() == "```")
                .unwrap_or(lines.len());

            lines[start..end].join("\n")
        } else {
            response.to_string()
        }
    }

    pub async fn check_connection(&self) -> Result<bool> {
        let url = format!("{}/api/tags", self.endpoint);

        let response = self.client.get(&url).send().await;

        Ok(response.is_ok())
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.endpoint);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to Ollama")?;

        #[derive(Deserialize)]
        struct TagsResponse {
            models: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            name: String,
        }

        let tags: TagsResponse = response.json().await?;

        Ok(tags.models.into_iter().map(|m| m.name).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_code_plain() {
        let client = OllamaClient::new("http://localhost:11434");

        let response = "from manim import *\n\nclass Test(Scene):\n    pass";
        let extracted = client.extract_code(response);

        assert_eq!(extracted, response);
    }

    #[test]
    fn test_extract_code_markdown() {
        let client = OllamaClient::new("http://localhost:11434");

        let response = "```python\nfrom manim import *\n\nclass Test(Scene):\n    pass\n```";
        let extracted = client.extract_code(response);

        assert_eq!(extracted, "from manim import *\n\nclass Test(Scene):\n    pass");
    }

    #[test]
    fn test_extract_code_markdown_no_lang() {
        let client = OllamaClient::new("http://localhost:11434");

        let response = "```\nfrom manim import *\n\nclass Test(Scene):\n    pass\n```";
        let extracted = client.extract_code(response);

        assert_eq!(extracted, "from manim import *\n\nclass Test(Scene):\n    pass");
    }
}
