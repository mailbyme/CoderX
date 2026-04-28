use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: String,
    pub score: i32,
}

pub struct FileIndex {
    files: Vec<String>,
    path_map: HashMap<String, usize>,
}

const SCORE_MATCH: i32 = 16;
const BONUS_BOUNDARY: i32 = 8;
const BONUS_CAMEL: i32 = 6;
const BONUS_CONSECUTIVE: i32 = 4;
const PENALTY_TEST_FILE: f32 = 1.05;

impl FileIndex {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            path_map: HashMap::new(),
        }
    }

    pub fn load_from_file_list(&mut self, files: Vec<String>) {
        self.files.clear();
        self.path_map.clear();

        let mut unique_files: Vec<String> = files.into_iter().collect();
        unique_files.sort();
        unique_files.dedup();

        for (idx, file) in unique_files.iter().enumerate() {
            self.path_map.insert(file.clone(), idx);
        }
        self.files = unique_files;
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<SearchResult> {
        if query.is_empty() {
            return self.files.iter().take(limit).map(|p| SearchResult {
                path: p.clone(),
                score: 0,
            }).collect();
        }

        let query_lower = query.to_lowercase();
        let mut results: Vec<SearchResult> = Vec::new();

        for file in &self.files {
            if let Some(score) = self.score_file(file, &query_lower) {
                results.push(SearchResult {
                    path: file.clone(),
                    score,
                });
            }
        }

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results.truncate(limit);
        results
    }

    fn score_file(&self, path: &str, query: &str) -> Option<i32> {
        let path_lower = path.to_lowercase();
        let path_chars: Vec<char> = path_lower.chars().collect();
        let query_chars: Vec<char> = query.chars().collect();

        if query_chars.is_empty() {
            return Some(0);
        }

        let mut score = 0i32;
        let mut query_idx = 0;
        let mut last_match_idx = 0i32;
        let mut consecutive = 0i32;

        for (i, &c) in path_chars.iter().enumerate() {
            if query_idx >= query_chars.len() {
                break;
            }

            if c == query_chars[query_idx] {
                if i == 0 || self.is_boundary(&path_chars, i) {
                    score += BONUS_BOUNDARY;
                } else if self.is_camel_boundary(&path_chars, i) {
                    score += BONUS_CAMEL;
                }

                if last_match_idx > 0 && (i as i32) == last_match_idx + 1 {
                    consecutive += 1;
                    score += BONUS_CONSECUTIVE * consecutive;
                } else {
                    consecutive = 0;
                }

                score += SCORE_MATCH;
                last_match_idx = i as i32;
                query_idx += 1;
            }
        }

        if query_idx < query_chars.len() {
            return None;
        }

        let is_test = path.contains("/test") || 
                      path.contains("/tests") || 
                      path.contains("_test.") || 
                      path.contains(".test.") ||
                      path.contains("/spec/") ||
                      path.contains("_spec.");
        
        if is_test {
            score = (score as f32 / PENALTY_TEST_FILE) as i32;
        }

        Some(score)
    }

    fn is_boundary(&self, chars: &[char], idx: usize) -> bool {
        if idx == 0 {
            return true;
        }
        
        let prev = chars[idx - 1];
        let curr = chars[idx];

        prev == '/' || prev == '\\' || prev == '-' || prev == '_' || prev == '.'
            || (prev.is_lowercase() && curr.is_uppercase())
    }

    fn is_camel_boundary(&self, chars: &[char], idx: usize) -> bool {
        if idx == 0 || idx >= chars.len() {
            return false;
        }

        let prev = chars[idx - 1];
        let curr = chars[idx];
        let next = chars.get(idx + 1).copied().unwrap_or('\0');

        prev.is_lowercase() && curr.is_uppercase() && next.is_lowercase()
    }

    pub fn add_file(&mut self, path: &str) {
        if !self.path_map.contains_key(path) {
            let idx = self.files.len();
            self.files.push(path.to_string());
            self.path_map.insert(path.to_string(), idx);
        }
    }

    pub fn remove_file(&mut self, path: &str) {
        if let Some(&idx) = self.path_map.get(path) {
            self.files.remove(idx);
            self.path_map.remove(path);
            for (file, old_idx) in self.path_map.iter_mut() {
                if *old_idx > idx {
                    *old_idx -= 1;
                }
            }
        }
    }

    pub fn get_file_count(&self) -> usize {
        self.files.len()
    }

    pub fn get_files(&self) -> &[String] {
        &self.files
    }

    pub fn contains(&self, path: &str) -> bool {
        self.path_map.contains_key(path)
    }
}

impl Default for FileIndex {
    fn default() -> Self {
        Self::new()
    }
}
