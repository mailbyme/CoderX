use std::collections::HashSet;
use std::path::Path;

pub struct Sandbox {
    enabled: bool,
    auto_allow: bool,
    excluded_commands: HashSet<String>,
    allowed_paths: Vec<String>,
}

impl Sandbox {
    pub fn new() -> Self {
        let mut excluded_commands = HashSet::new();
        excluded_commands.insert("cd".to_string());
        excluded_commands.insert("pwd".to_string());
        excluded_commands.insert("echo".to_string());
        excluded_commands.insert("ls".to_string());
        excluded_commands.insert("cat".to_string());
        excluded_commands.insert("git".to_string());
        
        Self {
            enabled: false,
            auto_allow: false,
            excluded_commands,
            allowed_paths: Vec::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_auto_allow(&mut self, value: bool) {
        self.auto_allow = value;
    }

    pub fn is_auto_allow_enabled(&self) -> bool {
        self.auto_allow
    }

    pub fn add_allowed_path(&mut self, path: &str) {
        if !self.allowed_paths.contains(&path.to_string()) {
            self.allowed_paths.push(path.to_string());
        }
    }

    pub fn should_use_sandbox(&self, command: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let cmd_parts: Vec<&str> = command.split_whitespace().collect();
        if cmd_parts.is_empty() {
            return false;
        }

        let base_cmd = cmd_parts[0];
        !self.excluded_commands.contains(base_cmd)
    }

    pub fn is_path_allowed(&self, path: &Path) -> bool {
        if self.allowed_paths.is_empty() {
            return true;
        }

        let path_str = path.to_string_lossy();
        self.allowed_paths.iter().any(|allowed| {
            path_str.starts_with(allowed) || path_str == *allowed
        })
    }

    pub fn validate_command(&self, command: &str) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }

        let dangerous_patterns = [
            "rm -rf /",
            "rm -rf /*",
            ":(){ :|:& };:",
            "mkfs",
            "dd if=",
            "> /dev/sd",
            "chmod -R 777 /",
            "chown -R",
            "wget | sh",
            "curl | sh",
            "curl | bash",
            "wget | bash",
            "/dev/null",
            "shutdown",
            "reboot",
            "init 0",
            "init 6",
            "halt",
            "poweroff",
        ];

        let lower_cmd = command.to_lowercase();
        for pattern in dangerous_patterns.iter() {
            if lower_cmd.contains(&pattern.to_lowercase()) {
                return Err(format!("Dangerous command pattern detected: {}", pattern));
            }
        }

        Ok(())
    }

    pub fn wrap_command(&self, command: &str, working_dir: &Path) -> String {
        if !self.enabled {
            return command.to_string();
        }

        let working_dir_str = working_dir.to_string_lossy();
        format!(
            "cd '{}' && {}",
            working_dir_str,
            command
        )
    }
}

impl Default for Sandbox {
    fn default() -> Self {
        Self::new()
    }
}
