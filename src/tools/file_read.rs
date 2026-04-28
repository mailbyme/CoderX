use super::{Tool, ToolError};
use std::fs::File;
use std::io::{Read, BufReader};
use std::path::Path;

pub struct FileReadTool;

impl FileReadTool {
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
    const DEFAULT_MAX_LINES: usize = 2000;
}

impl Tool for FileReadTool {
    fn name(&self) -> &str { "read_file" }
    fn description(&self) -> &str { "Read file contents (Usage: read <path> [start_line] [end_line])" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let parts: Vec<&str> = args.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(ToolError::InvalidArgs("Path is required. Usage: read <path> [start_line] [end_line]".to_string()));
        }

        let path = parts[0];
        let start_line = parts.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
        let end_line = parts.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(start_line + Self::DEFAULT_MAX_LINES);

        let path = expand_tilde(path);
        let path = Path::new(&path);

        if !path.exists() {
            return Err(ToolError::InvalidArgs(format!("File not found: {}", path.display())));
        }

        if !path.is_file() {
            return Err(ToolError::InvalidArgs(format!("Not a file: {}", path.display())));
        }

        let metadata = std::fs::metadata(path)?;
        if metadata.len() > Self::MAX_FILE_SIZE {
            return Err(ToolError::ExecutionError(format!(
                "File too large: {} bytes (max: {} bytes)",
                metadata.len(),
                Self::MAX_FILE_SIZE
            )));
        }

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();

        let start = start_line.saturating_sub(1).min(total_lines);
        let end = end_line.min(total_lines);

        let mut result = String::new();
        result.push_str(&format!("File: {} ({} lines)\n", path.display(), total_lines));
        result.push_str(&format!("Showing lines {}-{}\n\n", start + 1, end));

        for (i, line) in lines[start..end].iter().enumerate() {
            result.push_str(&format!("{:6} | {}\n", start + i + 1, line));
        }

        if end < total_lines {
            result.push_str(&format!("\n... {} more lines", total_lines - end));
        }

        Ok(result)
    }
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
    fn test_empty_path() {
        let tool = FileReadTool;
        let result = tool.execute("");
        assert!(result.is_err());
    }
}
