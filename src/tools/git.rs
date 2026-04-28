use std::process::{Command, Stdio};
use crate::tools::{Tool, ToolError};

pub struct GitTool;

impl GitTool {
    pub fn new() -> Self {
        GitTool
    }

    fn execute_internal(&self, args: &str) -> Result<String, String> {
        let args: Vec<&str> = args.split_whitespace().collect();
        
        if args.is_empty() {
            return Ok("Usage: git <command> [args]\nAvailable commands: status, add, commit, push, pull, init, branch, checkout, merge, log, diff, reset, stash".to_string());
        }
        
        let output = Command::new("git")
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| format!("Failed to execute git: {}", e))?;
        
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        
        if output.status.success() {
            Ok(result)
        } else {
            Err(format!("Git error: {}{}", result, error))
        }
    }
}

impl Tool for GitTool {
    fn name(&self) -> &str {
        "git"
    }

    fn description(&self) -> &str {
        "Execute Git commands"
    }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        self.execute_internal(args)
            .map_err(|e| ToolError::ExecutionError(e))
    }
}

