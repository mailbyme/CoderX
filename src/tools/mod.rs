pub mod bash;
pub mod file_read;
pub mod file_write;
pub mod grep;
pub mod git;

pub use bash::BashTool;
pub use file_read::FileReadTool;
pub use file_write::FileWriteTool;
pub use grep::GrepTool;
pub use git::GitTool;

use std::collections::HashMap;

pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, args: &str) -> Result<String, ToolError>;
}

#[derive(Debug)]
pub enum ToolError {
    IoError(std::io::Error),
    InvalidArgs(String),
    ExecutionError(String),
    NotFound(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::IoError(e) => write!(f, "IO error: {}", e),
            ToolError::InvalidArgs(s) => write!(f, "Invalid arguments: {}", s),
            ToolError::ExecutionError(s) => write!(f, "Execution error: {}", s),
            ToolError::NotFound(s) => write!(f, "Tool not found: {}", s),
        }
    }
}

impl From<std::io::Error> for ToolError {
    fn from(e: std::io::Error) -> Self {
        ToolError::IoError(e)
    }
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut tools: HashMap<String, Box<dyn Tool + Send + Sync>> = HashMap::new();
        tools.insert("bash".to_string(), Box::new(BashTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("read".to_string(), Box::new(FileReadTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("read_file".to_string(), Box::new(FileReadTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("write".to_string(), Box::new(FileWriteTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("write_file".to_string(), Box::new(FileWriteTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("grep".to_string(), Box::new(GrepTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("search".to_string(), Box::new(GrepTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("git".to_string(), Box::new(GitTool) as Box<dyn Tool + Send + Sync>);
        Self { tools }
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool + Send + Sync>> {
        self.tools.get(name)
    }

    pub fn execute(&self, name: &str, args: &str) -> Result<String, ToolError> {
        if let Some(tool) = self.tools.get(name) {
            tool.execute(args)
        } else {
            Err(ToolError::NotFound(name.to_string()))
        }
    }

    pub fn list(&self) -> Vec<(&str, &str)> {
        let mut result: Vec<(&str, &str)> = self.tools.iter()
            .map(|(name, tool)| (name.as_str(), tool.description()))
            .collect();
        result.sort_by_key(|(name, _)| *name);
        result.dedup_by_key(|(name, _)| *name);
        result
    }

    pub fn has(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
