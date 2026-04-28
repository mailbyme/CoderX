use crate::state::Message;

pub struct TokenEstimator {
    bytes_per_token: usize,
}

impl TokenEstimator {
    pub fn new() -> Self {
        Self { bytes_per_token: 4 }
    }

    pub fn estimate_text(&self, text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }
        text.len() / self.bytes_per_token
    }

    pub fn estimate_messages(&self, messages: &[Message]) -> usize {
        let mut total = 0;
        for message in messages {
            total += self.estimate_text(&message.content);
        }
        total
    }

    pub fn estimate_conversation(&self, messages: &[Message]) -> usize {
        let mut total = 0;

        for message in messages {
            total += self.estimate_text(&message.content);
            total += 4;
        }

        total
    }

    pub fn set_bytes_per_token(&mut self, bytes: usize) {
        self.bytes_per_token = bytes;
    }

    pub fn get_context_window_size(model: &str) -> usize {
        match model {
            m if m.starts_with("claude") => 200_000,
            m if m.starts_with("gpt-4") => 128_000,
            m if m.starts_with("gpt-3.5") => 16_000,
            m if m.starts_with("mistral") => 32_000,
            m if m.starts_with("llama") => 128_000,
            m if m.starts_with("qwen") => 32_000,
            m if m.starts_with("deepseek") => 64_000,
            _ => 100_000,
        }
    }

    pub fn calculate_usage_percentage(current: usize, model: &str) -> f32 {
        let max = Self::get_context_window_size(model);
        (current as f32 / max as f32) * 100.0
    }

    pub fn should_compact(current: usize, model: &str, buffer: usize) -> bool {
        let max = Self::get_context_window_size(model);
        current >= max.saturating_sub(buffer)
    }

    pub fn get_safe_token_limit(model: &str) -> usize {
        let max = Self::get_context_window_size(model);
        (max as f32 * 0.8) as usize
    }

    pub fn estimate_tool_definition(&self, tool_name: &str, description: &str) -> usize {
        let base = 20;
        base + self.estimate_text(tool_name) + self.estimate_text(description)
    }

    pub fn estimate_system_prompt(&self, prompt: &str) -> usize {
        self.estimate_text(prompt) + 10
    }
}

impl Default for TokenEstimator {
    fn default() -> Self {
        Self::new()
    }
}
