use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum Permission {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone)]
pub struct PermissionRule {
    pub pattern: String,
    pub permission: Permission,
    pub description: Option<String>,
}

pub struct PermissionManager {
    rules: Vec<PermissionRule>,
    dangerous_files: HashSet<String>,
    dangerous_directories: HashSet<String>,
    safe_env_vars: HashSet<String>,
    allowed_commands: HashMap<String, CommandConfig>,
}

#[derive(Debug, Clone)]
pub struct CommandConfig {
    pub safe_flags: HashMap<String, String>,
    pub allow_any_args: bool,
}

impl PermissionManager {
    pub fn new() -> Self {
        let mut dangerous_files = HashSet::new();
        dangerous_files.insert(".gitconfig".to_string());
        dangerous_files.insert(".gitmodules".to_string());
        dangerous_files.insert(".bashrc".to_string());
        dangerous_files.insert(".bash_profile".to_string());
        dangerous_files.insert(".zshrc".to_string());
        dangerous_files.insert(".zprofile".to_string());
        dangerous_files.insert(".profile".to_string());
        dangerous_files.insert(".ssh/config".to_string());
        dangerous_files.insert(".ssh/authorized_keys".to_string());
        dangerous_files.insert(".mcp.json".to_string());
        dangerous_files.insert(".claude.json".to_string());

        let mut dangerous_directories = HashSet::new();
        dangerous_directories.insert(".git".to_string());
        dangerous_directories.insert(".ssh".to_string());
        dangerous_directories.insert(".gnupg".to_string());

        let mut safe_env_vars = HashSet::new();
        safe_env_vars.insert("HOME".to_string());
        safe_env_vars.insert("PWD".to_string());
        safe_env_vars.insert("OLDPWD".to_string());
        safe_env_vars.insert("USER".to_string());
        safe_env_vars.insert("LOGNAME".to_string());
        safe_env_vars.insert("SHELL".to_string());
        safe_env_vars.insert("PATH".to_string());
        safe_env_vars.insert("HOSTNAME".to_string());
        safe_env_vars.insert("LANG".to_string());
        safe_env_vars.insert("TERM".to_string());
        safe_env_vars.insert("EDITOR".to_string());
        safe_env_vars.insert("VISUAL".to_string());
        safe_env_vars.insert("RUST_BACKTRACE".to_string());
        safe_env_vars.insert("RUST_LOG".to_string());
        safe_env_vars.insert("NODE_ENV".to_string());
        safe_env_vars.insert("PYTHONUNBUFFERED".to_string());
        safe_env_vars.insert("GOOS".to_string());
        safe_env_vars.insert("GOARCH".to_string());
        safe_env_vars.insert("CGO_ENABLED".to_string());

        let mut allowed_commands = HashMap::new();
        
        let git_config = CommandConfig {
            safe_flags: HashMap::new(),
            allow_any_args: false,
        };
        allowed_commands.insert("git".to_string(), git_config);

        let ls_config = CommandConfig {
            safe_flags: HashMap::new(),
            allow_any_args: true,
        };
        allowed_commands.insert("ls".to_string(), ls_config.clone());
        allowed_commands.insert("cat".to_string(), ls_config.clone());
        allowed_commands.insert("head".to_string(), ls_config.clone());
        allowed_commands.insert("tail".to_string(), ls_config.clone());
        allowed_commands.insert("wc".to_string(), ls_config.clone());
        allowed_commands.insert("echo".to_string(), ls_config.clone());
        allowed_commands.insert("pwd".to_string(), ls_config.clone());
        allowed_commands.insert("which".to_string(), ls_config.clone());
        allowed_commands.insert("whoami".to_string(), ls_config.clone());
        allowed_commands.insert("date".to_string(), ls_config.clone());
        allowed_commands.insert("uname".to_string(), ls_config.clone());

        Self {
            rules: Vec::new(),
            dangerous_files,
            dangerous_directories,
            safe_env_vars,
            allowed_commands,
        }
    }

    pub fn add_rule(&mut self, rule: PermissionRule) {
        self.rules.push(rule);
    }

    pub fn check_command_permission(&self, command: &str) -> Permission {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Permission::Allow;
        }

        let base_cmd = parts[0];
        
        for rule in &self.rules {
            if base_cmd == rule.pattern || command.contains(&rule.pattern) {
                return rule.permission.clone();
            }
        }

        if self.allowed_commands.contains_key(base_cmd) {
            return Permission::Allow;
        }

        let dangerous_commands = [
            "rm", "rmdir", "dd", "mkfs", "fdisk", "shutdown", "reboot",
            "init", "halt", "poweroff", "systemctl", "service",
            "chmod", "chown", "passwd", "useradd", "userdel", "usermod",
            "groupadd", "groupdel", "visudo", "su", "sudo",
        ];

        for dangerous in dangerous_commands.iter() {
            if base_cmd == *dangerous {
                return Permission::Ask;
            }
        }

        Permission::Allow
    }

    pub fn check_file_permission(&self, path: &str, is_write: bool) -> Permission {
        let path_lower = path.to_lowercase();
        
        for file in &self.dangerous_files {
            if path_lower.ends_with(&format!("/{}", file.to_lowercase())) || 
               path_lower == file.to_lowercase() {
                return Permission::Deny;
            }
        }

        for dir in &self.dangerous_directories {
            if path_lower.contains(&format!("/{}/", dir.to_lowercase())) ||
               path_lower.ends_with(&format!("/{}", dir.to_lowercase())) {
                if is_write {
                    return Permission::Ask;
                }
            }
        }

        if is_write {
            for rule in &self.rules {
                if path.contains(&rule.pattern) {
                    return rule.permission.clone();
                }
            }
        }

        Permission::Allow
    }

    pub fn is_env_var_safe(&self, var: &str) -> bool {
        self.safe_env_vars.contains(var)
    }

    pub fn is_dangerous_file(&self, filename: &str) -> bool {
        self.dangerous_files.contains(filename)
    }

    pub fn is_dangerous_directory(&self, dirname: &str) -> bool {
        self.dangerous_directories.contains(dirname)
    }

    pub fn get_safe_env_vars(&self) -> &HashSet<String> {
        &self.safe_env_vars
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}
