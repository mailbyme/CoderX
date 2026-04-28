use crate::state::Message;
use super::token_estimator::TokenEstimator;
use super::compressor::{ContextCompressor, CompressionResult};

#[derive(Debug, Clone)]
pub struct ContextState {
    pub current_tokens: usize,
    pub max_tokens: usize,
    pub usage_percentage: f32,
    pub needs_compaction: bool,
}

pub struct ContextManager {
    estimator: TokenEstimator,
    compressor: ContextCompressor,
    model: String,
    auto_compact_threshold: usize,
    warning_threshold: usize,
    error_threshold: usize,
}

impl ContextManager {
    pub fn new(model: &str) -> Self {
        let estimator = TokenEstimator::new();
        let max_tokens = TokenEstimator::get_context_window_size(model);
        let auto_compact_threshold = max_tokens.saturating_sub(13_000);
        let warning_threshold = max_tokens.saturating_sub(20_000);
        let error_threshold = max_tokens.saturating_sub(10_000);

        Self {
            estimator,
            compressor: ContextCompressor::new(),
            model: model.to_string(),
            auto_compact_threshold,
            warning_threshold,
            error_threshold,
        }
    }

    pub fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
        let max_tokens = TokenEstimator::get_context_window_size(model);
        self.auto_compact_threshold = max_tokens.saturating_sub(13_000);
        self.warning_threshold = max_tokens.saturating_sub(20_000);
        self.error_threshold = max_tokens.saturating_sub(10_000);
    }

    pub fn get_state(&self, messages: &[Message]) -> ContextState {
        let current_tokens = self.estimator.estimate_messages(messages);
        let max_tokens = TokenEstimator::get_context_window_size(&self.model);
        let usage_percentage = TokenEstimator::calculate_usage_percentage(current_tokens, &self.model);

        ContextState {
            current_tokens,
            max_tokens,
            usage_percentage,
            needs_compaction: current_tokens >= self.auto_compact_threshold,
        }
    }

    pub fn should_auto_compact(&self, messages: &[Message]) -> bool {
        let current_tokens = self.estimator.estimate_messages(messages);
        current_tokens >= self.auto_compact_threshold
    }

    pub fn get_warning_level(&self, messages: &[Message]) -> WarningLevel {
        let current_tokens = self.estimator.estimate_messages(messages);

        if current_tokens >= self.error_threshold {
            WarningLevel::Critical
        } else if current_tokens >= self.warning_threshold {
            WarningLevel::Warning
        } else if current_tokens >= self.auto_compact_threshold {
            WarningLevel::Notice
        } else {
            WarningLevel::None
        }
    }

    pub fn compact(&self, messages: &[Message]) -> CompressionResult {
        let target_tokens = TokenEstimator::get_safe_token_limit(&self.model);
        self.compressor.compress(messages, target_tokens)
    }

    pub fn partial_compact(&self, messages: &[Message], from_start: usize) -> CompressionResult {
        let target_tokens = TokenEstimator::get_safe_token_limit(&self.model);
        self.compressor.partial_compress(messages, from_start, target_tokens)
    }

    pub fn truncate(&self, messages: &mut Vec<Message>, max_tokens: usize) -> usize {
        self.compressor.truncate_messages(messages, max_tokens)
    }

    pub fn smart_truncate(&self, messages: &mut Vec<Message>, max_tokens: usize) -> usize {
        self.compressor.smart_truncate(messages, max_tokens)
    }

    pub fn estimate(&self, text: &str) -> usize {
        self.estimator.estimate_text(text)
    }

    pub fn estimate_messages(&self, messages: &[Message]) -> usize {
        self.estimator.estimate_messages(messages)
    }

    pub fn get_max_context_size(&self) -> usize {
        TokenEstimator::get_context_window_size(&self.model)
    }

    pub fn get_remaining_tokens(&self, messages: &[Message]) -> usize {
        let current = self.estimator.estimate_messages(messages);
        let max = TokenEstimator::get_context_window_size(&self.model);
        max.saturating_sub(current)
    }

    pub fn can_fit(&self, messages: &[Message], additional_tokens: usize) -> bool {
        let current = self.estimator.estimate_messages(messages);
        let max = TokenEstimator::get_context_window_size(&self.model);
        current + additional_tokens < max
    }

    pub fn get_usage_stats(&self, messages: &[Message]) -> UsageStats {
        let current = self.estimator.estimate_messages(messages);
        let max = TokenEstimator::get_context_window_size(&self.model);
        let percentage = (current as f64 / max as f64) * 100.0;

        UsageStats {
            current_tokens: current,
            max_tokens: max,
            percentage,
            remaining_tokens: max.saturating_sub(current),
            message_count: messages.len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WarningLevel {
    None,
    Notice,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct UsageStats {
    pub current_tokens: usize,
    pub max_tokens: usize,
    pub percentage: f64,
    pub remaining_tokens: usize,
    pub message_count: usize,
}

impl UsageStats {
    pub fn format(&self) -> String {
        format!(
            "Context: {}/{} tokens ({:.1}%) | {} messages | {} remaining",
            self.current_tokens,
            self.max_tokens,
            self.percentage,
            self.message_count,
            self.remaining_tokens
        )
    }
}
