use super::{Tool, ToolError};
use std::fs;
use std::path::Path;

pub struct GrepTool;

impl GrepTool {
    const MAX_FILE_SIZE: u64 = 5 * 1024 * 1024; // 5MB
    const MAX_RESULTS: usize = 100;
    const SKIP_DIRS: &[&str] = &[
        "node_modules", ".git", "target", "build", "dist", 
        ".venv", "venv", "__pycache__", ".idea", ".vscode",
        "vendor", "bower_components", ".gradle", ".mvn"
    ];
    const SKIP_EXTENSIONS: &[&str] = &[
        ".min.js", ".min.css", ".lock", ".sum", ".log",
        ".png", ".jpg", ".jpeg", ".gif", ".ico", ".svg",
        ".woff", ".woff2", ".ttf", ".eot", ".otf",
        ".mp3", ".mp4", ".avi", ".mov", ".wav",
        ".zip", ".tar", ".gz", ".rar", ".7z",
        ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
        ".exe", ".dll", ".so", ".dylib", ".bin"
    ];
}

impl Tool for GrepTool {
    fn name(&self) -> &str { "grep" }
    fn description(&self) -> &str { "Search for patterns in files (Usage: grep <pattern> <path> [options])" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let parts: Vec<&str> = args.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(ToolError::InvalidArgs("Pattern is required. Usage: grep <pattern> <path> [options]".to_string()));
        }

        let pattern = parts[0];
        let path = parts.get(1).unwrap_or(&".");
        
        let case_sensitive = !parts.iter().any(|&p| p == "-i" || p == "--ignore-case");
        let show_line_numbers = !parts.iter().any(|&p| p == "-n" || p == "--no-line-number");
        let show_filenames = !parts.iter().any(|&p| p == "-h" || p == "--no-filename");

        let mut results = SearchResults::new(Self::MAX_RESULTS);
        let path = expand_tilde(path);
        let path = Path::new(&path);

        if !path.exists() {
            return Err(ToolError::InvalidArgs(format!("Path not found: {}", path.display())));
        }

        search_recursive(
            path, 
            pattern, 
            case_sensitive, 
            show_line_numbers, 
            show_filenames,
            &mut results
        )?;

        if results.count == 0 {
            Ok(format!("No matches found for '{}' in {}", pattern, path.display()))
        } else {
            Ok(results.output)
        }
    }
}

struct SearchResults {
    output: String,
    count: usize,
    max_results: usize,
    truncated: bool,
}

impl SearchResults {
    fn new(max_results: usize) -> Self {
        Self {
            output: String::new(),
            count: 0,
            max_results,
            truncated: false,
        }
    }

    fn add(&mut self, line: &str) {
        if self.count < self.max_results {
            self.output.push_str(line);
            self.output.push('\n');
        } else if !self.truncated {
            self.truncated = true;
        }
        self.count += 1;
    }

    fn finish(&mut self) {
        if self.truncated {
            self.output.push_str(&format!(
                "\n... {} more results (use more specific pattern or path)",
                self.count - self.max_results
            ));
        }
    }
}

fn search_recursive(
    path: &Path, 
    pattern: &str, 
    case_sensitive: bool,
    show_line_numbers: bool,
    show_filenames: bool,
    results: &mut SearchResults
) -> Result<(), ToolError> {
    if results.count >= results.max_results * 2 {
        return Ok(());
    }

    if path.is_file() {
        // Skip binary and large files
        if should_skip_file(path) {
            return Ok(());
        }

        if let Ok(metadata) = fs::metadata(path) {
            if metadata.len() > GrepTool::MAX_FILE_SIZE {
                return Ok(());
            }
        }

        if let Ok(content) = fs::read_to_string(path) {
            let search_pattern = if case_sensitive {
                pattern.to_string()
            } else {
                pattern.to_lowercase()
            };

            for (i, line) in content.lines().enumerate() {
                let search_line = if case_sensitive {
                    line.to_string()
                } else {
                    line.to_lowercase()
                };

                if search_line.contains(&search_pattern) {
                    let prefix = if show_filenames {
                        format!("{}:", path.display())
                    } else {
                        String::new()
                    };
                    
                    let line_prefix = if show_line_numbers {
                        format!("{}:", i + 1)
                    } else {
                        String::new()
                    };

                    // Truncate long lines
                    let display_line = if line.len() > 200 {
                        format!("{}...", &line[..200])
                    } else {
                        line.to_string()
                    };

                    results.add(&format!("{}{} {}", prefix, line_prefix, display_line));
                    
                    if results.count >= results.max_results * 2 {
                        break;
                    }
                }
            }
        }
    } else if path.is_dir() {
        // Skip unwanted directories
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if GrepTool::SKIP_DIRS.contains(&name) {
                return Ok(());
            }
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                search_recursive(
                    &entry.path(), 
                    pattern, 
                    case_sensitive,
                    show_line_numbers,
                    show_filenames,
                    results
                )?;
            }
        }
    }

    Ok(())
}

fn should_skip_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();
        for skip_ext in GrepTool::SKIP_EXTENSIONS {
            if ext_lower.ends_with(&skip_ext[1..]) {
                return true;
            }
        }
    }
    
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        for skip_ext in GrepTool::SKIP_EXTENSIONS {
            if name.ends_with(skip_ext) {
                return true;
            }
        }
    }
    
    false
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        if let Some(home) = std::env::var("HOME").ok() {
            return path.replacen('~', &home, 1);
        }
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_pattern() {
        let tool = GrepTool;
        let result = tool.execute("");
        assert!(result.is_err());
    }
}
