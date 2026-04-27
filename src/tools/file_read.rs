use super::{Tool, ToolError};
use std::fs::File;
use std::io::Read;

pub struct FileReadTool;

impl Tool for FileReadTool {
    fn name(&self) -> &str { "read_file" }
    fn description(&self) -> &str { "Read file contents" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let path = args.trim();
        if path.is_empty() {
            return Err(ToolError::InvalidArgs("Path is required".to_string()));
        }

        let mut file = File::open(path).map_err(ToolError::IoError)?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(ToolError::IoError)?;
        Ok(content)
    }
}
