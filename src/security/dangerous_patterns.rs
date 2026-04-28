use std::collections::HashSet;

pub struct DangerousPatternChecker {
    dangerous_commands: HashSet<String>,
    zsh_dangerous_commands: HashSet<String>,
}

impl DangerousPatternChecker {
    pub fn new() -> Self {
        let mut dangerous_commands = HashSet::new();
        dangerous_commands.insert("eval".to_string());
        dangerous_commands.insert("exec".to_string());
        dangerous_commands.insert("source".to_string());
        dangerous_commands.insert("sudo".to_string());
        dangerous_commands.insert("su".to_string());
        dangerous_commands.insert("chmod".to_string());
        dangerous_commands.insert("chown".to_string());
        dangerous_commands.insert("rm".to_string());
        dangerous_commands.insert("mkfs".to_string());
        dangerous_commands.insert("dd".to_string());
        dangerous_commands.insert("shutdown".to_string());
        dangerous_commands.insert("reboot".to_string());
        dangerous_commands.insert("halt".to_string());
        dangerous_commands.insert("poweroff".to_string());
        dangerous_commands.insert("init".to_string());
        dangerous_commands.insert("systemctl".to_string());
        dangerous_commands.insert("service".to_string());

        let mut zsh_dangerous_commands = HashSet::new();
        zsh_dangerous_commands.insert("zmodload".to_string());
        zsh_dangerous_commands.insert("emulate".to_string());
        zsh_dangerous_commands.insert("sysopen".to_string());
        zsh_dangerous_commands.insert("sysread".to_string());
        zsh_dangerous_commands.insert("syswrite".to_string());
        zsh_dangerous_commands.insert("sysseek".to_string());
        zsh_dangerous_commands.insert("zpty".to_string());
        zsh_dangerous_commands.insert("ztcp".to_string());
        zsh_dangerous_commands.insert("zf_rm".to_string());
        zsh_dangerous_commands.insert("zf_mv".to_string());
        zsh_dangerous_commands.insert("zf_chmod".to_string());

        Self {
            dangerous_commands,
            zsh_dangerous_commands,
        }
    }

    pub fn check_command(&self, command: &str) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if command.contains("$(") {
            errors.push("Command substitution detected: $()".to_string());
        }
        if command.contains('`') && command.matches('`').count() >= 2 {
            errors.push("Command substitution detected: backticks".to_string());
        }
        if command.contains("${") {
            errors.push("Parameter substitution detected: ${}".to_string());
        }
        if command.contains("<(") {
            errors.push("Process substitution detected: <()".to_string());
        }
        if command.contains(">(") {
            errors.push("Process substitution detected: >()".to_string());
        }

        if command.contains("$'") {
            errors.push("ANSI-C quoting detected: $'...'".to_string());
        }
        if command.contains("$\"") {
            errors.push("Locale quoting detected: $\"...\"".to_string());
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        if !parts.is_empty() {
            let base_cmd = parts[0].trim_start_matches(|c| c == '-' || c == '/');
            
            if self.dangerous_commands.contains(base_cmd) {
                errors.push(format!("Dangerous command detected: {}", base_cmd));
            }

            if self.zsh_dangerous_commands.contains(base_cmd) {
                errors.push(format!("Zsh dangerous command detected: {}", base_cmd));
            }
        }

        let lower_cmd = command.to_lowercase();
        
        if (lower_cmd.contains("wget") || lower_cmd.contains("curl")) && 
           (lower_cmd.contains("| sh") || lower_cmd.contains("| bash")) {
            errors.push("Download and execute pattern detected".to_string());
        }
        
        if lower_cmd.contains("rm -rf /") || lower_cmd.contains("rm -rf /*") {
            errors.push("Recursive root deletion detected".to_string());
        }
        
        if lower_cmd.contains("/dev/sd") && (lower_cmd.contains(">") || lower_cmd.contains("dd")) {
            errors.push("Direct disk write detected".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn is_dangerous_command(&self, cmd: &str) -> bool {
        self.dangerous_commands.contains(cmd) || self.zsh_dangerous_commands.contains(cmd)
    }

    pub fn has_command_substitution(&self, command: &str) -> bool {
        command.contains("$(") || 
        (command.contains('`') && command.matches('`').count() >= 2) ||
        command.contains("${")
    }

    pub fn has_obfuscation(&self, command: &str) -> bool {
        command.contains("$'") || command.contains("$\"")
    }
}

impl Default for DangerousPatternChecker {
    fn default() -> Self {
        Self::new()
    }
}
