use super::{Tool, ToolError};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub struct FileWriteTool;

impl FileWriteTool {
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
}

impl Tool for FileWriteTool {
    fn name(&self) -> &str { "write_file" }
    fn description(&self) -> &str { "Write content to file (Usage: write <path> [mode] <content>, mode: w=write, a=append)" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let args = args.trim();
        
        // Parse: write <path> [mode] <content>
        // mode can be 'w' (write/overwrite) or 'a' (append)
        // Content starts after the path (and optional mode) on the same line or next line
        
        let lines: Vec<&str> = args.lines().collect();
        if lines.is_empty() {
            return Err(ToolError::InvalidArgs("Usage: write <path> [mode]\n<content>".to_string()));
        }

        let first_line = lines[0].trim();
        let parts: Vec<&str> = first_line.splitn(3, ' ').collect();
        
        if parts.is_empty() {
            return Err(ToolError::InvalidArgs("Path is required".to_string()));
        }

        let path = expand_tilde(parts[0]);
        let (mode, content_start) = if parts.len() >= 2 && (parts[1] == "a" || parts[1] == "append") {
            ("append", if parts.len() >= 3 { parts[2] } else { "" })
        } else if parts.len() >= 2 && (parts[1] == "w" || parts[1] == "write" || parts[1] == "overwrite") {
            ("write", if parts.len() >= 3 { parts[2] } else { "" })
        } else {
            ("write", if parts.len() >= 2 { parts[1] } else { "" })
        };

        // Build content from remaining content on first line and subsequent lines
        let mut content = String::new();
        if !content_start.is_empty() {
            content.push_str(content_start);
        }
        if lines.len() > 1 {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(&lines[1..].join("\n"));
        }

        if content.len() > Self::MAX_FILE_SIZE {
            return Err(ToolError::ExecutionError(format!(
                "Content too large: {} bytes (max: {} bytes)",
                content.len(),
                Self::MAX_FILE_SIZE
            )));
        }

        let path = Path::new(&path);
        
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        let (bytes_written, action) = if mode == "append" {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;
            file.write_all(content.as_bytes())?;
            (content.len(), "Appended")
        } else {
            fs::write(path, &content)?;
            (content.len(), "Wrote")
        };

        Ok(format!("{} {} bytes to {}", action, bytes_written, path.display()))
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
    use std::fs;

    #[test]
    fn test_write_file() {
        let tool = FileWriteTool;
        let result = tool.execute("/tmp/coderx_test.txt\nHello, World!");
        assert!(result.is_ok());
        
        // Cleanup
        let _ = fs::remove_file("/tmp/coderx_test.txt");
    }
}
