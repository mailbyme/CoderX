use std::fs;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file: String,
    pub line_number: usize,
    pub line: String,
    pub context_before: Vec<String>,
    pub context_after: Vec<String>,
}

pub struct CodeSearcher {
    context_lines: usize,
    max_results: usize,
}

impl CodeSearcher {
    pub fn new() -> Self {
        Self {
            context_lines: 3,
            max_results: 100,
        }
    }

    pub fn set_context_lines(&mut self, lines: usize) {
        self.context_lines = lines;
    }

    pub fn set_max_results(&mut self, max: usize) {
        self.max_results = max;
    }

    pub fn search_in_file(&self, path: &Path, pattern: &str, case_sensitive: bool) -> Vec<SearchResult> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut results = Vec::new();
        let search_pattern = if case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        for (idx, line) in lines.iter().enumerate() {
            let search_line = if case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            if search_line.contains(&search_pattern) {
                let context_start = idx.saturating_sub(self.context_lines);
                let context_end = (idx + self.context_lines + 1).min(lines.len());

                let context_before: Vec<String> = lines[context_start..idx]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

                let context_after: Vec<String> = lines[idx + 1..context_end]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

                results.push(SearchResult {
                    file: path.to_string_lossy().to_string(),
                    line_number: idx + 1,
                    line: line.to_string(),
                    context_before,
                    context_after,
                });

                if results.len() >= self.max_results {
                    break;
                }
            }
        }

        results
    }

    pub fn search_in_files(&self, files: &[String], pattern: &str, case_sensitive: bool) -> Vec<SearchResult> {
        let mut all_results = Vec::new();

        for file in files {
            let path = Path::new(file);
            let results = self.search_in_file(path, pattern, case_sensitive);
            all_results.extend(results);

            if all_results.len() >= self.max_results {
                break;
            }
        }

        all_results
    }

    pub fn search_regex_in_file(&self, path: &Path, pattern: &str) -> Vec<SearchResult> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Vec::new(),
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut results = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            if self.simple_pattern_match(line, pattern) {
                let context_start = idx.saturating_sub(self.context_lines);
                let context_end = (idx + self.context_lines + 1).min(lines.len());

                let context_before: Vec<String> = lines[context_start..idx]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

                let context_after: Vec<String> = lines[idx + 1..context_end]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();

                results.push(SearchResult {
                    file: path.to_string_lossy().to_string(),
                    line_number: idx + 1,
                    line: line.to_string(),
                    context_before,
                    context_after,
                });

                if results.len() >= self.max_results {
                    break;
                }
            }
        }

        results
    }

    fn simple_pattern_match(&self, text: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return text.starts_with(prefix) && text.ends_with(suffix);
            }
        }

        if pattern.starts_with('^') {
            return text.starts_with(&pattern[1..]);
        }

        if pattern.ends_with('$') {
            return text.ends_with(&pattern[..pattern.len() - 1]);
        }

        text.contains(pattern)
    }

    pub fn find_definitions(&self, files: &[String], symbol: &str) -> Vec<SearchResult> {
        let definition_patterns = [
            format!("fn {}(", symbol),
            format!("def {}(", symbol),
            format!("function {}(", symbol),
            format!("class {}(", symbol),
            format!("struct {} ", symbol),
            format!("enum {} ", symbol),
            format!("interface {} ", symbol),
            format!("const {} =", symbol),
            format!("let {} =", symbol),
            format!("var {} =", symbol),
            format!("pub fn {}(", symbol),
            format!("pub struct {} ", symbol),
            format!("pub enum {} ", symbol),
        ];

        let mut results = Vec::new();

        for file in files {
            let path = Path::new(file);
            for pattern in &definition_patterns {
                let found = self.search_in_file(path, pattern, true);
                results.extend(found);
                if results.len() >= self.max_results {
                    return results;
                }
            }
        }

        results
    }

    pub fn count_matches(&self, path: &Path, pattern: &str, case_sensitive: bool) -> usize {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return 0,
        };

        let search_pattern = if case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };

        let search_content = if case_sensitive {
            content
        } else {
            content.to_lowercase()
        };

        search_content.matches(&search_pattern).count()
    }

    pub fn get_file_stats(&self, path: &Path) -> HashMap<String, usize> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return HashMap::new(),
        };

        let lines = content.lines().count();
        let chars = content.chars().count();
        let words = content.split_whitespace().count();

        let mut stats = HashMap::new();
        stats.insert("lines".to_string(), lines);
        stats.insert("chars".to_string(), chars);
        stats.insert("words".to_string(), words);

        stats
    }
}

impl Default for CodeSearcher {
    fn default() -> Self {
        Self::new()
    }
}
