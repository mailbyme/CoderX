use super::{Tool, ToolError};
use std::fs;
use std::path::Path;

pub struct GrepTool;

impl Tool for GrepTool {
    fn name(&self) -> &str { "grep" }
    fn description(&self) -> &str { "Search for patterns in files" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let parts: Vec<&str> = args.splitn(2, ' ').collect();
        if parts.len() != 2 {
            return Err(ToolError::InvalidArgs("Usage: grep <pattern> <path>".to_string()));
        }

        let (pattern, path) = (parts[0], parts[1]);
        let mut results = String::new();

        search_recursive(Path::new(path), pattern, &mut results)?;
        Ok(results)
    }
}

fn search_recursive(path: &Path, pattern: &str, results: &mut String) -> Result<(), ToolError> {
    if path.is_file() {
        if let Ok(content) = fs::read_to_string(path) {
            for (i, line) in content.lines().enumerate() {
                if line.contains(pattern) {
                    results.push_str(&format!(
                        "{}:{}: {}\n",
                        path.display(),
                        i + 1,
                        line
                    ));
                }
            }
        }
    } else if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                search_recursive(&entry.path(), pattern, results)?;
            }
        }
    }
    Ok(())
}
