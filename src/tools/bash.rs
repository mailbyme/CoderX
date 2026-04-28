use super::{Tool, ToolError};
use std::process::Command;
use std::time::Instant;

pub struct BashTool;

impl BashTool {
    const DEFAULT_TIMEOUT_SECS: u64 = 60;
    const MAX_OUTPUT_SIZE: usize = 100_000;
}

impl Tool for BashTool {
    fn name(&self) -> &str { "bash" }
    fn description(&self) -> &str { "Execute shell commands (Usage: bash <command>)" }

    fn execute(&self, args: &str) -> Result<String, ToolError> {
        let args = args.trim();
        if args.is_empty() {
            return Err(ToolError::InvalidArgs("Command is required".to_string()));
        }

        let start = Instant::now();
        
        let output = if cfg!(windows) {
            Command::new("powershell")
                .arg("-NoProfile")
                .arg("-Command")
                .arg(args)
                .output()
                .map_err(ToolError::IoError)?
        } else {
            Command::new("bash")
                .arg("-lc")
                .arg(args)
                .output()
                .map_err(ToolError::IoError)?
        };

        let duration = start.elapsed();
        let exit_code = output.status.code().unwrap_or(-1);
        
        let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if stdout.len() > Self::MAX_OUTPUT_SIZE {
            stdout = format!("{}...\n[Output truncated, {} bytes total]", 
                &stdout[..Self::MAX_OUTPUT_SIZE], stdout.len());
        }

        let mut result = String::new();
        
        if output.status.success() {
            if !stdout.is_empty() {
                result.push_str(&stdout);
            }
            if !stderr.is_empty() {
                result.push_str(&format!("\n[stderr]: {}", stderr));
            }
        } else {
            result.push_str(&format!("Exit code: {}\n", exit_code));
            if !stdout.is_empty() {
                result.push_str(&stdout);
            }
            if !stderr.is_empty() {
                result.push_str(&format!("\n[stderr]: {}", stderr));
            }
        }
        
        result.push_str(&format!("\n[Completed in {:.2}s]", duration.as_secs_f64()));

        if output.status.success() {
            Ok(result)
        } else {
            Err(ToolError::ExecutionError(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo() {
        let tool = BashTool;
        let result = tool.execute("echo hello").unwrap();
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_empty_command() {
        let tool = BashTool;
        let result = tool.execute("");
        assert!(result.is_err());
    }
}
