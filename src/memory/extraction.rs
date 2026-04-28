use std::collections::HashSet;

pub struct MemoryExtractor {
    max_memories_per_session: usize,
    min_content_length: usize,
    max_content_length: usize,
}

impl MemoryExtractor {
    pub fn new() -> Self {
        Self {
            max_memories_per_session: 5,
            min_content_length: 10,
            max_content_length: 2000,
        }
    }

    pub fn extract_from_conversation(&self, messages: &[crate::state::Message]) -> Vec<super::memory::Memory> {
        let mut memories = Vec::new();
        let mut seen_topics = HashSet::new();

        for message in messages {
            if memories.len() >= self.max_memories_per_session {
                break;
            }

            if let Some(memory) = self.extract_from_message(message) {
                let topic_key = memory.name.to_lowercase();
                if !seen_topics.contains(&topic_key) {
                    seen_topics.insert(topic_key);
                    memories.push(memory);
                }
            }
        }

        memories
    }

    fn extract_from_message(&self, message: &crate::state::Message) -> Option<super::memory::Memory> {
        let content = &message.content;

        if content.len() < self.min_content_length {
            return None;
        }

        let patterns = [
            ("user_preference", vec!["I prefer", "I like", "I want", "I need", "please use", "always use"]),
            ("project_context", vec!["this project", "the project", "our codebase", "the codebase"]),
            ("technical_decision", vec!["we decided", "we chose", "the solution is", "the approach is"]),
            ("error_learned", vec!["error:", "failed:", "exception:", "bug:"]),
            ("todo_item", vec!["TODO:", "FIXME:", "HACK:", "XXX:"]),
        ];

        for (memory_type, keywords) in patterns.iter() {
            for keyword in keywords {
                if content.to_lowercase().contains(&keyword.to_lowercase()) {
                    let name = format!("{}_{}", memory_type, self.generate_id());
                    let mut memory = super::memory::Memory::new(
                        name,
                        self.get_memory_type(memory_type),
                    );

                    let truncated: String = content.chars().take(self.max_content_length).collect();
                    memory.content = truncated;
                    memory.description = self.extract_description(content, keyword);

                    return Some(memory);
                }
            }
        }

        None
    }

    fn get_memory_type(&self, type_str: &str) -> super::memory::MemoryType {
        match type_str {
            "user_preference" => super::memory::MemoryType::User,
            "error_learned" | "technical_decision" => super::memory::MemoryType::Feedback,
            "reference" => super::memory::MemoryType::Reference,
            _ => super::memory::MemoryType::Project,
        }
    }

    fn extract_description(&self, content: &str, keyword: &str) -> String {
        if let Some(pos) = content.to_lowercase().find(&keyword.to_lowercase()) {
            let start = pos.max(0);
            let end = (pos + 100).min(content.len());
            let description = &content[start..end];
            
            let cleaned = description
                .lines()
                .next()
                .unwrap_or("")
                .trim();

            if cleaned.len() > 100 {
                cleaned[..100].to_string()
            } else {
                cleaned.to_string()
            }
        } else {
            "Extracted memory".to_string()
        }
    }

    fn generate_id(&self) -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        format!("{}", now % 100000)
    }

    pub fn extract_code_patterns(&self, content: &str) -> Vec<super::memory::Memory> {
        let mut memories = Vec::new();

        let code_patterns = [
            ("function_definition", "fn ", "function"),
            ("class_definition", "struct ", "class"),
            ("interface_definition", "trait ", "interface"),
            ("import_pattern", "use ", "import"),
            ("config_pattern", "config", "configuration"),
        ];

        for (pattern_type, rust_keyword, generic_keyword) in code_patterns.iter() {
            if content.contains(rust_keyword) || content.contains(generic_keyword) {
                let name = format!("code_pattern_{}", self.generate_id());
                let mut memory = super::memory::Memory::new(
                    name,
                    super::memory::MemoryType::Project,
                );
                memory.description = format!("Code pattern: {}", pattern_type);
                memory.content = content.chars().take(self.max_content_length).collect();
                memories.push(memory);
            }
        }

        memories
    }

    pub fn extract_errors(&self, content: &str) -> Vec<super::memory::Memory> {
        let mut memories = Vec::new();

        let error_indicators = ["error:", "Error:", "ERROR:", "failed:", "Failed:", "exception:", "Exception:"];

        for indicator in error_indicators {
            if content.contains(indicator) {
                let name = format!("error_{}", self.generate_id());
                let mut memory = super::memory::Memory::new(
                    name,
                    super::memory::MemoryType::Feedback,
                );
                memory.description = "Error encountered".to_string();
                
                if let Some(pos) = content.find(indicator) {
                    let start = pos.max(0);
                    let end = (pos + 500).min(content.len());
                    memory.content = content[start..end].to_string();
                }

                memories.push(memory);
            }
        }

        memories
    }

    pub fn summarize_session(&self, messages: &[crate::state::Message]) -> String {
        let mut summary = String::new();
        let mut topics = HashSet::new();

        for message in messages {
            let content = &message.content;
            
            let topic_patterns = [
                "implemented", "created", "fixed", "updated", "refactored",
                "added", "removed", "changed", "optimized", "debugged",
            ];

            for pattern in topic_patterns {
                if content.to_lowercase().contains(pattern) {
                    let words: Vec<&str> = content.split_whitespace().take(10).collect();
                    let topic = words.join(" ");
                    if !topics.contains(&topic) {
                        topics.insert(topic.clone());
                        summary.push_str(&format!("- {}\n", topic));
                    }
                    break;
                }
            }
        }

        if summary.is_empty() {
            summary = "No significant activities recorded.".to_string();
        }

        summary
    }
}

impl Default for MemoryExtractor {
    fn default() -> Self {
        Self::new()
    }
}
