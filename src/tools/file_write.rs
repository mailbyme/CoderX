use super::{Tool, ToolError};
use std::fs;

pub struct FileWriteTool;

impl Tool for FileWriteTool {
    fn name(&self) -> &str { "write_file" }
    fn description(&self) -> &str { "Write content to file" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let parts: Vec<&str> = args.splitn(2, '\n').collect();
        if parts.len() != 2 {
            return Err(ToolError::InvalidArgs("Usage: write_file <path>\n<content>".to_string()));
        }

        let path = parts[0].trim();
        let content = parts[1];

        fs::write(path, content).map_err(ToolError::IoError)?;
        Ok(format!("Wrote {} bytes to {}", content.len(), path))
    }
}
