use std::path::{Path, PathBuf};

pub struct PathValidator {
    working_directory: PathBuf,
    allow_symlinks: bool,
}

impl PathValidator {
    pub fn new() -> Self {
        Self {
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            allow_symlinks: true,
        }
    }

    pub fn with_working_directory(path: PathBuf) -> Self {
        Self {
            working_directory: path,
            allow_symlinks: true,
        }
    }

    pub fn set_working_directory(&mut self, path: PathBuf) {
        self.working_directory = path;
    }

    pub fn get_working_directory(&self) -> &Path {
        &self.working_directory
    }

    pub fn set_allow_symlinks(&mut self, allow: bool) {
        self.allow_symlinks = allow;
    }

    pub fn is_path_in_working_directory(&self, path: &Path) -> bool {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.working_directory.join(path)
        };

        if let Ok(canonical) = absolute_path.canonicalize() {
            if let Ok(workdir_canonical) = self.working_directory.canonicalize() {
                return canonical.starts_with(&workdir_canonical);
            }
        }

        absolute_path.starts_with(&self.working_directory)
    }

    pub fn validate_path(&self, path: &Path) -> Result<PathBuf, String> {
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.working_directory.join(path)
        };

        if !self.allow_symlinks {
            if let Ok(metadata) = absolute_path.symlink_metadata() {
                if metadata.file_type().is_symlink() {
                    return Err("Symlinks are not allowed".to_string());
                }
            }
        }

        if !self.is_path_in_working_directory(&absolute_path) {
            return Err(format!(
                "Path '{}' is outside the working directory",
                absolute_path.display()
            ));
        }

        Ok(absolute_path)
    }

    pub fn check_path_traversal(&self, path: &str) -> Result<(), String> {
        if path.contains("..") {
            let normalized = self.normalize_path(path);
            if !self.is_path_in_working_directory(&normalized) {
                return Err("Path traversal detected".to_string());
            }
        }
        Ok(())
    }

    fn normalize_path(&self, path: &str) -> PathBuf {
        let parts: Vec<&str> = path.split(|c| c == '/' || c == '\\').collect();
        let mut result = Vec::new();

        for part in parts {
            if part == ".." {
                result.pop();
            } else if part != "." && !part.is_empty() {
                result.push(part);
            }
        }

        self.working_directory.join(result.join("/"))
    }

    pub fn is_safe_filename(filename: &str) -> bool {
        if filename.is_empty() {
            return false;
        }

        let unsafe_names = [
            ".", "..", "CON", "PRN", "AUX", "NUL",
            "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];

        let upper = filename.to_uppercase();
        if unsafe_names.contains(&upper.as_str()) {
            return false;
        }

        let unsafe_chars = ['<', '>', ':', '"', '|', '?', '*', '\0'];
        for c in unsafe_chars {
            if filename.contains(c) {
                return false;
            }
        }

        true
    }

    pub fn check_windows_special_paths(path: &str) -> bool {
        let upper = path.to_uppercase();
        
        if upper.contains("::$DATA") {
            return true;
        }
        
        if upper.contains("~") && upper.len() > 2 {
            let parts: Vec<&str> = upper.split('~').collect();
            if parts.len() > 1 {
                if let Ok(num) = parts.last().unwrap().parse::<u32>() {
                    if num > 0 {
                        return true;
                    }
                }
            }
        }

        if upper.starts_with("\\\\?\\") || upper.starts_with("\\\\.\\") {
            return true;
        }

        false
    }

    pub fn resolve_path(&self, path: &str) -> PathBuf {
        let path = Path::new(path);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.working_directory.join(path)
        }
    }
}

impl Default for PathValidator {
    fn default() -> Self {
        Self::new()
    }
}
