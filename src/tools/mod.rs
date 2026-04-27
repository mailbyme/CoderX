pub mod bash;
pub mod file_read;
pub mod file_write;
pub mod grep;

pub use bash::BashTool;
pub use file_read::FileReadTool;
pub use file_write::FileWriteTool;
pub use grep::GrepTool;

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
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut tools: HashMap<String, Box<dyn Tool + Send + Sync>> = HashMap::new();
        tools.insert("bash".to_string(), Box::new(BashTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("read_file".to_string(), Box::new(FileReadTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("write_file".to_string(), Box::new(FileWriteTool) as Box<dyn Tool + Send + Sync>);
        tools.insert("grep".to_string(), Box::new(GrepTool) as Box<dyn Tool + Send + Sync>);
        Self { tools }
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool + Send + Sync>> {
        self.tools.get(name)
    }

    pub fn list(&self) -> Vec<(&str, &str)> {
        self.tools.iter()
            .map(|(name, tool)| (name.as_str(), tool.description()))
            .collect()
    }
}
