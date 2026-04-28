use std::collections::HashMap;
use super::types::{Issue, IssueSeverity, IssueCategory};

pub struct SecurityScanner {
    patterns: Vec<SecurityPattern>,
}

struct SecurityPattern {
    id: String,
    name: String,
    pattern: String,
    severity: IssueSeverity,
    description: String,
    suggestion: String,
}

impl SecurityScanner {
    pub fn new() -> Self {
        let patterns = vec![
            SecurityPattern {
                id: "aws-access-key".to_string(),
                name: "AWS Access Key".to_string(),
                pattern: r#"AKIA[0-9A-Z]{16}"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "AWS Access Key ID detected".to_string(),
                suggestion: "Remove the key and rotate it immediately. Use AWS Secrets Manager or environment variables.".to_string(),
            },
            SecurityPattern {
                id: "aws-secret-key".to_string(),
                name: "AWS Secret Key".to_string(),
                pattern: r#"[A-Za-z0-9/+=]{40}"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "Potential AWS Secret Access Key detected".to_string(),
                suggestion: "Verify and remove if it's a secret key. Rotate the key immediately.".to_string(),
            },
            SecurityPattern {
                id: "github-token".to_string(),
                name: "GitHub Token".to_string(),
                pattern: r#"ghp_[0-9a-zA-Z]{36}"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "GitHub Personal Access Token detected".to_string(),
                suggestion: "Revoke the token and use environment variables instead.".to_string(),
            },
            SecurityPattern {
                id: "github-oauth".to_string(),
                name: "GitHub OAuth Token".to_string(),
                pattern: r#"gho_[0-9a-zA-Z]{36}"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "GitHub OAuth Token detected".to_string(),
                suggestion: "Revoke the token and use proper OAuth flow.".to_string(),
            },
            SecurityPattern {
                id: "anthropic-api-key".to_string(),
                name: "Anthropic API Key".to_string(),
                pattern: r#"sk-ant-api03-[a-zA-Z0-9\-_]{95}"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "Anthropic API Key detected".to_string(),
                suggestion: "Remove the key and regenerate it. Use environment variables.".to_string(),
            },
            SecurityPattern {
                id: "openai-api-key".to_string(),
                name: "OpenAI API Key".to_string(),
                pattern: r#"sk-[a-zA-Z0-9]{48}"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "OpenAI API Key detected".to_string(),
                suggestion: "Remove the key and regenerate it. Use environment variables.".to_string(),
            },
            SecurityPattern {
                id: "private-key".to_string(),
                name: "Private Key".to_string(),
                pattern: r#"-----BEGIN (?:RSA |DSA |EC |OPENSSH )?PRIVATE KEY-----"#.to_string(),
                severity: IssueSeverity::Critical,
                description: "Private key detected".to_string(),
                suggestion: "Remove the private key and use a secure key management system.".to_string(),
            },
            SecurityPattern {
                id: "jwt-secret".to_string(),
                name: "JWT Secret".to_string(),
                pattern: r#"jwt[_\-]?secret"#.to_string(),
                severity: IssueSeverity::Error,
                description: "Potential JWT secret reference detected".to_string(),
                suggestion: "Use environment variables for JWT secrets.".to_string(),
            },
            SecurityPattern {
                id: "password-field".to_string(),
                name: "Hardcoded Password".to_string(),
                pattern: r#"password\s*=\s*["'][^"']+["']"#.to_string(),
                severity: IssueSeverity::Error,
                description: "Hardcoded password detected".to_string(),
                suggestion: "Use environment variables or a secrets manager.".to_string(),
            },
            SecurityPattern {
                id: "sql-injection".to_string(),
                name: "SQL Injection Risk".to_string(),
                pattern: r#"(SELECT|INSERT|UPDATE|DELETE).*\+.*"#.to_string(),
                severity: IssueSeverity::Error,
                description: "Potential SQL injection vulnerability".to_string(),
                suggestion: "Use parameterized queries or prepared statements.".to_string(),
            },
            SecurityPattern {
                id: "xss-risk".to_string(),
                name: "XSS Risk".to_string(),
                pattern: r#"innerHTML\s*="#.to_string(),
                severity: IssueSeverity::Warning,
                description: "Potential XSS vulnerability with innerHTML".to_string(),
                suggestion: "Use textContent or sanitize HTML input.".to_string(),
            },
            SecurityPattern {
                id: "eval-usage".to_string(),
                name: "eval() Usage".to_string(),
                pattern: r#"eval\s*\("#.to_string(),
                severity: IssueSeverity::Critical,
                description: "eval() function usage detected".to_string(),
                suggestion: "Avoid eval() as it can execute arbitrary code.".to_string(),
            },
            SecurityPattern {
                id: "debug-code".to_string(),
                name: "Debug Code".to_string(),
                pattern: r#"(console\.log|print\(|debugger|var_dump)"#.to_string(),
                severity: IssueSeverity::Info,
                description: "Debug code detected".to_string(),
                suggestion: "Remove debug code before production deployment.".to_string(),
            },
        ];

        Self { patterns }
    }

    pub fn scan(&self, content: &str, file_path: &str) -> Vec<Issue> {
        let mut issues = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (line_idx, line) in lines.iter().enumerate() {
            for pattern in &self.patterns {
                if self.simple_match(line, &pattern.pattern) {
                    let issue = Issue::new(
                        file_path,
                        line_idx + 1,
                        pattern.severity.clone(),
                        IssueCategory::Security,
                        &pattern.name,
                    )
                    .with_description(&pattern.description)
                    .with_suggestion(&pattern.suggestion)
                    .with_rule(&pattern.id);

                    issues.push(issue);
                }
            }
        }

        issues
    }

    fn simple_match(&self, text: &str, pattern: &str) -> bool {
        let text_lower = text.to_lowercase();
        let pattern_lower = pattern.to_lowercase();

        if pattern.contains(r"\s") {
            let parts: Vec<&str> = pattern.split(r"\s*").collect();
            if parts.len() == 2 {
                return text_lower.contains(&parts[0].to_lowercase()) && 
                       text_lower.contains(&parts[1].to_lowercase());
            }
        }

        if pattern.starts_with('(') && pattern.contains('|') {
            let inner = &pattern[1..pattern.rfind(')').unwrap_or(pattern.len())];
            let options: Vec<&str> = inner.split('|').collect();
            for opt in options {
                if text_lower.contains(&opt.trim().to_lowercase()) {
                    return true;
                }
            }
            return false;
        }

        if pattern.contains('+') && !pattern.starts_with('+') {
            let parts: Vec<&str> = pattern.split('+').collect();
            if parts.len() == 2 {
                let prefix = parts[0].trim();
                let suffix = parts[1].trim();
                return text_lower.contains(&prefix.to_lowercase()) && 
                       text_lower.contains(&suffix.to_lowercase());
            }
        }

        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                return text_lower.starts_with(&parts[0].to_lowercase()) &&
                       text_lower.ends_with(&parts[1].to_lowercase());
            }
        }

        text_lower.contains(&pattern_lower)
    }

    pub fn scan_for_secrets(&self, content: &str) -> HashMap<String, Vec<String>> {
        let mut secrets = HashMap::new();

        for line in content.lines() {
            if line.contains("AKIA") && line.len() > 20 {
                secrets.entry("aws_access_key".to_string())
                    .or_insert_with(Vec::new)
                    .push(line.to_string());
            }
            if line.contains("ghp_") {
                secrets.entry("github_token".to_string())
                    .or_insert_with(Vec::new)
                    .push(line.to_string());
            }
            if line.contains("sk-ant-") || line.contains("sk-") {
                secrets.entry("api_key".to_string())
                    .or_insert_with(Vec::new)
                    .push(line.to_string());
            }
            if line.contains("BEGIN") && line.contains("PRIVATE KEY") {
                secrets.entry("private_key".to_string())
                    .or_insert_with(Vec::new)
                    .push(line.to_string());
            }
        }

        secrets
    }

    pub fn has_secrets(&self, content: &str) -> bool {
        let secret_indicators = [
            "AKIA", "ghp_", "gho_", "sk-ant-", "sk-",
            "BEGIN PRIVATE KEY", "BEGIN RSA PRIVATE KEY",
            "password=", "secret=", "api_key=",
        ];

        for indicator in secret_indicators.iter() {
            if content.contains(indicator) {
                return true;
            }
        }

        false
    }
}

impl Default for SecurityScanner {
    fn default() -> Self {
        Self::new()
    }
}
