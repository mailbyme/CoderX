use super::{Tool, ToolError};
use std::process::Command;

pub struct BashTool;

impl Tool for BashTool {
    fn name(&self) -> &str { "bash" }
    fn description(&self) -> &str { "Execute shell commands" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let output = if cfg!(windows) {
            Command::new("powershell")
                .arg("-Command")
                .arg(args)
                .output()
                .map_err(ToolError::IoError)?
        } else {
            Command::new("bash")
                .arg("-c")
                .arg(args)
                .output()
                .map_err(ToolError::IoError)?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(ToolError::ExecutionError(format!("{}: {}", stdout, stderr)))
        }
    }
}
