use crate::state::Message;
use super::token_estimator::TokenEstimator;

#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub original_tokens: usize,
    pub compressed_tokens: usize,
    pub messages_removed: usize,
    pub summary: String,
}

pub struct ContextCompressor {
    estimator: TokenEstimator,
    target_ratio: f32,
    min_messages_to_keep: usize,
}

impl ContextCompressor {
    pub fn new() -> Self {
        Self {
            estimator: TokenEstimator::new(),
            target_ratio: 0.3,
            min_messages_to_keep: 5,
        }
    }

    pub fn compress(&self, messages: &[Message], target_tokens: usize) -> CompressionResult {
        let original_tokens = self.estimator.estimate_messages(messages);

        if original_tokens <= target_tokens {
            return CompressionResult {
                original_tokens,
                compressed_tokens: original_tokens,
                messages_removed: 0,
                summary: String::new(),
            };
        }

        let summary = self.generate_summary(messages);
        let summary_tokens = self.estimator.estimate_text(&summary);

        CompressionResult {
            original_tokens,
            compressed_tokens: summary_tokens,
            messages_removed: messages.len(),
            summary,
        }
    }

    pub fn partial_compress(
        &self,
        messages: &[Message],
        from_start: usize,
        target_tokens: usize,
    ) -> CompressionResult {
        let original_tokens = self.estimator.estimate_messages(messages);

        if from_start >= messages.len() {
            return CompressionResult {
                original_tokens,
                compressed_tokens: original_tokens,
                messages_removed: 0,
                summary: String::new(),
            };
        }

        let to_compress = &messages[..from_start];
        let to_keep = &messages[from_start..];

        let summary = self.generate_summary(to_compress);
        let summary_tokens = self.estimator.estimate_text(&summary);
        let kept_tokens = self.estimator.estimate_messages(to_keep);

        CompressionResult {
            original_tokens,
            compressed_tokens: summary_tokens + kept_tokens,
            messages_removed: from_start,
            summary,
        }
    }

    fn generate_summary(&self, messages: &[Message]) -> String {
        let mut summary = String::from("# Conversation Summary\n\n");

        summary.push_str("## Key Topics\n");
        let topics = self.extract_topics(messages);
        for topic in topics {
            summary.push_str(&format!("- {}\n", topic));
        }

        summary.push_str("\n## Decisions Made\n");
        let decisions = self.extract_decisions(messages);
        for decision in decisions {
            summary.push_str(&format!("- {}\n", decision));
        }

        summary.push_str("\n## Files Mentioned\n");
        let files = self.extract_files(messages);
        for file in files {
            summary.push_str(&format!("- {}\n", file));
        }

        summary.push_str("\n## Errors Encountered\n");
        let errors = self.extract_errors(messages);
        for error in errors {
            summary.push_str(&format!("- {}\n", error));
        }

        summary
    }

    fn extract_topics(&self, messages: &[Message]) -> Vec<String> {
        let mut topics = Vec::new();
        let keywords = ["implement", "create", "fix", "update", "refactor", "add", "remove"];

        for message in messages {
            let content = message.content.to_lowercase();
            for keyword in keywords {
                if content.contains(keyword) {
                    let words: Vec<&str> = message.content.split_whitespace().take(8).collect();
                    let topic = words.join(" ");
                    if !topics.contains(&topic) && topics.len() < 10 {
                        topics.push(topic);
                    }
                    break;
                }
            }
        }

        topics
    }

    fn extract_decisions(&self, messages: &[Message]) -> Vec<String> {
        let mut decisions = Vec::new();
        let patterns = ["decided", "chose", "selected", "will use", "going to use"];

        for message in messages {
            let content = message.content.to_lowercase();
            for pattern in patterns {
                if content.contains(pattern) {
                    if let Some(pos) = message.content.to_lowercase().find(pattern) {
                        let start = pos.max(0);
                        let end = (pos + 100).min(message.content.len());
                        let decision = message.content[start..end].trim().to_string();
                        if !decisions.contains(&decision) && decisions.len() < 5 {
                            decisions.push(decision);
                        }
                    }
                    break;
                }
            }
        }

        decisions
    }

    fn extract_files(&self, messages: &[Message]) -> Vec<String> {
        let mut files = Vec::new();

        for message in messages {
            let words: Vec<&str> = message.content.split_whitespace().collect();
            for word in words {
                if word.contains('/') || word.contains('\\') {
                    let cleaned = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '\\' && c != '.' && c != '_');
                    if !cleaned.is_empty() && !files.contains(&cleaned.to_string()) && files.len() < 15 {
                        files.push(cleaned.to_string());
                    }
                }
            }
        }

        files
    }

    fn extract_errors(&self, messages: &[Message]) -> Vec<String> {
        let mut errors = Vec::new();
        let patterns = ["error:", "failed:", "exception:", "bug:"];

        for message in messages {
            let content = message.content.to_lowercase();
            for pattern in patterns {
                if content.contains(pattern) {
                    if let Some(pos) = message.content.to_lowercase().find(pattern) {
                        let end = (pos + 100).min(message.content.len());
                        let error = message.content[pos..end].trim().to_string();
                        if !errors.contains(&error) && errors.len() < 5 {
                            errors.push(error);
                        }
                    }
                    break;
                }
            }
        }

        errors
    }

    pub fn truncate_messages(&self, messages: &mut Vec<Message>, max_tokens: usize) -> usize {
        let mut current_tokens = self.estimator.estimate_messages(messages);
        let mut removed = 0;

        while current_tokens > max_tokens && messages.len() > self.min_messages_to_keep {
            messages.remove(0);
            removed += 1;
            current_tokens = self.estimator.estimate_messages(messages);
        }

        removed
    }

    pub fn smart_truncate(&self, messages: &mut Vec<Message>, max_tokens: usize) -> usize {
        let mut current_tokens = self.estimator.estimate_messages(messages);
        let mut removed = 0;

        while current_tokens > max_tokens && messages.len() > self.min_messages_to_keep {
            let mut best_idx = 0;
            let mut best_score = 0;

            for (idx, msg) in messages.iter().enumerate() {
                if idx < self.min_messages_to_keep {
                    continue;
                }

                let score = self.calculate_importance_score(msg);
                if score < best_score || best_idx == 0 {
                    best_score = score;
                    best_idx = idx;
                }
            }

            if best_idx > 0 && best_idx < messages.len() {
                messages.remove(best_idx);
                removed += 1;
            } else {
                break;
            }

            current_tokens = self.estimator.estimate_messages(messages);
        }

        removed
    }

    fn calculate_importance_score(&self, message: &Message) -> i32 {
        let mut score = 50;

        let content = message.content.to_lowercase();

        if content.contains("error") || content.contains("failed") {
            score += 20;
        }

        if content.contains("important") || content.contains("critical") {
            score += 30;
        }

        if content.contains("todo") || content.contains("fixme") {
            score += 15;
        }

        if content.contains("decided") || content.contains("chose") {
            score += 25;
        }

        if message.content.len() < 50 {
            score -= 20;
        }

        score
    }

    pub fn get_compression_ratio(&self, result: &CompressionResult) -> f32 {
        if result.original_tokens == 0 {
            return 0.0;
        }
        result.compressed_tokens as f32 / result.original_tokens as f32
    }
}

impl Default for ContextCompressor {
    fn default() -> Self {
        Self::new()
    }
}
