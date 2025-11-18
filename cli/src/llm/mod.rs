pub mod ollama;

use anyhow::Result;
use async_trait::async_trait;

/// Trait for LLM providers to implement
#[async_trait]
pub trait LLMProvider {
    /// Generate animation code from a text prompt
    async fn generate_animation_code(&self, prompt: &str, model: &str) -> Result<String>;

    /// Stream response for real-time updates (optional)
    async fn generate_streaming(
        &self,
        prompt: &str,
        model: &str,
        callback: Box<dyn Fn(String) + Send>,
    ) -> Result<String> {
        // Default implementation just calls non-streaming version
        let _ = callback;
        self.generate_animation_code(prompt, model).await
    }
}

/// System prompt for generating Manim animation code
pub fn get_system_prompt() -> &'static str {
    r#"You are an expert Manim animation code generator. Your task is to generate Python code using the Manim library based on user descriptions.

Requirements:
1. Always use "from manim import *" at the top
2. Create a class that inherits from Scene
3. Implement the construct() method
4. Use proper Manim syntax and objects (Text, Circle, Square, etc.)
5. Include smooth animations with self.play()
6. Add appropriate wait() calls for timing
7. Use meaningful variable names
8. Add comments to explain complex sections
9. Make the animation visually appealing
10. Keep the code clean and well-structured

Return ONLY the Python code, no explanations or markdown formatting.
"#
}
