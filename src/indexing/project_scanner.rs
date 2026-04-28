use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

pub struct ProjectScanner {
    root_path: PathBuf,
    ignore_patterns: Vec<String>,
    max_depth: usize,
    max_files: usize,
}

impl ProjectScanner {
    pub fn new() -> Self {
        Self {
            root_path: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            ignore_patterns: Self::default_ignore_patterns(),
            max_depth: 50,
            max_files: 100000,
        }
    }

    pub fn with_root(path: PathBuf) -> Self {
        Self {
            root_path: path,
            ignore_patterns: Self::default_ignore_patterns(),
            max_depth: 50,
            max_files: 100000,
        }
    }

    fn default_ignore_patterns() -> Vec<String> {
        vec![
            "node_modules".to_string(),
            "target".to_string(),
            ".git".to_string(),
            ".svn".to_string(),
            ".hg".to_string(),
            "__pycache__".to_string(),
            "*.pyc".to_string(),
            "*.pyo".to_string(),
            ".idea".to_string(),
            ".vscode".to_string(),
            "*.swp".to_string(),
            "*.swo".to_string(),
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            "dist".to_string(),
            "build".to_string(),
            "out".to_string(),
            ".cache".to_string(),
            "coverage".to_string(),
            ".nyc_output".to_string(),
            "vendor".to_string(),
            "*.lock".to_string(),
            "Cargo.lock".to_string(),
            "package-lock.json".to_string(),
            "yarn.lock".to_string(),
            "pnpm-lock.yaml".to_string(),
        ]
    }

    pub fn set_root(&mut self, path: PathBuf) {
        self.root_path = path;
    }

    pub fn add_ignore_pattern(&mut self, pattern: String) {
        self.ignore_patterns.push(pattern);
    }

    pub fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth;
    }

    pub fn set_max_files(&mut self, max: usize) {
        self.max_files = max;
    }

    pub fn scan(&self) -> Vec<String> {
        let mut files = Vec::new();
        let mut count = 0;
        self.scan_directory(&self.root_path, 0, &mut files, &mut count);
        files
    }

    fn scan_directory(&self, dir: &Path, depth: usize, files: &mut Vec<String>, count: &mut usize) {
        if depth > self.max_depth || *count >= self.max_files {
            return;
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if *count >= self.max_files {
                    break;
                }

                let path = entry.path();
                let name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if self.should_ignore(name) {
                    continue;
                }

                if path.is_dir() {
                    self.scan_directory(&path, depth + 1, files, count);
                } else if path.is_file() {
                    if let Ok(relative) = path.strip_prefix(&self.root_path) {
                        files.push(relative.to_string_lossy().to_string());
                        *count += 1;
                    }
                }
            }
        }
    }

    fn should_ignore(&self, name: &str) -> bool {
        for pattern in &self.ignore_patterns {
            if pattern.starts_with('*') {
                let suffix = &pattern[1..];
                if name.ends_with(suffix) {
                    return true;
                }
            } else if name == pattern {
                return true;
            }
        }
        false
    }

    pub fn scan_with_gitignore(&self) -> Vec<String> {
        let mut files = self.scan();
        
        if let Ok(gitignore) = self.load_gitignore() {
            files.retain(|f| !self.matches_gitignore(f, &gitignore));
        }

        files
    }

    fn load_gitignore(&self) -> Result<Vec<String>, std::io::Error> {
        let gitignore_path = self.root_path.join(".gitignore");
        let content = fs::read_to_string(gitignore_path)?;
        
        let patterns: Vec<String> = content
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.trim().to_string())
            .collect();
        
        Ok(patterns)
    }

    fn matches_gitignore(&self, path: &str, patterns: &[String]) -> bool {
        for pattern in patterns {
            if pattern.starts_with('!') {
                continue;
            }

            if pattern.ends_with('/') {
                let dir_pattern = &pattern[..pattern.len() - 1];
                if path.starts_with(dir_pattern) || path.contains(&format!("/{}/", dir_pattern)) {
                    return true;
                }
            } else if pattern.starts_with('*') {
                let suffix = &pattern[1..];
                if path.ends_with(suffix) {
                    return true;
                }
            } else if path == pattern || path.starts_with(&format!("{}/", pattern)) {
                return true;
            }
        }
        false
    }

    pub fn detect_project_type(&self) -> ProjectType {
        let mut indicators = HashSet::new();

        if let Ok(entries) = fs::read_dir(&self.root_path) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    indicators.insert(name.to_string());
                }
            }
        }

        if indicators.contains("Cargo.toml") {
            return ProjectType::Rust;
        }
        if indicators.contains("go.mod") {
            return ProjectType::Go;
        }
        if indicators.contains("package.json") {
            if indicators.contains("tsconfig.json") {
                return ProjectType::TypeScript;
            }
            return ProjectType::JavaScript;
        }
        if indicators.contains("requirements.txt") || indicators.contains("setup.py") || indicators.contains("pyproject.toml") {
            return ProjectType::Python;
        }
        if indicators.contains("pom.xml") || indicators.contains("build.gradle") {
            return ProjectType::Java;
        }
        if indicators.contains("Gemfile") {
            return ProjectType::Ruby;
        }
        if indicators.contains("composer.json") {
            return ProjectType::PHP;
        }
        if indicators.contains("CMakeLists.txt") {
            return ProjectType::Cpp;
        }

        ProjectType::Unknown
    }

    pub fn get_project_info(&self) -> ProjectInfo {
        ProjectInfo {
            root: self.root_path.clone(),
            project_type: self.detect_project_type(),
            file_count: self.scan().len(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectType {
    Rust,
    Go,
    JavaScript,
    TypeScript,
    Python,
    Java,
    Ruby,
    PHP,
    Cpp,
    Unknown,
}

impl ProjectType {
    pub fn display_name(&self) -> &str {
        match self {
            ProjectType::Rust => "Rust",
            ProjectType::Go => "Go",
            ProjectType::JavaScript => "JavaScript",
            ProjectType::TypeScript => "TypeScript",
            ProjectType::Python => "Python",
            ProjectType::Java => "Java",
            ProjectType::Ruby => "Ruby",
            ProjectType::PHP => "PHP",
            ProjectType::Cpp => "C/C++",
            ProjectType::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectInfo {
    pub root: PathBuf,
    pub project_type: ProjectType,
    pub file_count: usize,
}

impl Default for ProjectScanner {
    fn default() -> Self {
        Self::new()
    }
}
