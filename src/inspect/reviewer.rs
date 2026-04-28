use std::fs;
use std::path::Path;
use super::types::{ReviewResult, Issue, IssueSeverity, IssueCategory, ReviewStats};

pub struct InspectCore {
    max_file_size: usize,
    check_security: bool,
    check_performance: bool,
    check_style: bool,
}

impl InspectCore {
    pub fn new() -> Self {
        Self {
            max_file_size: 1_000_000,
            check_security: true,
            check_performance: true,
            check_style: true,
        }
    }

    pub fn review_file(&self, path: &Path) -> Result<ReviewResult, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if content.len() > self.max_file_size {
            return Err(format!("File too large: {} bytes (max: {})", content.len(), self.max_file_size));
        }

        let file_str = path.to_string_lossy();
        let mut result = ReviewResult::new(&file_str);

        result.stats = self.calculate_stats(&content);
        
        self.check_correctness(&content, &mut result);
        
        if self.check_security {
            self.check_security_issues(&content, &mut result);
        }
        
        if self.check_performance {
            self.check_performance_issues(&content, &mut result);
        }
        
        if self.check_style {
            self.check_style_issues(&content, &mut result);
        }

        self.generate_summary(&mut result);

        Ok(result)
    }

    fn calculate_stats(&self, content: &str) -> ReviewStats {
        let mut stats = ReviewStats::default();
        let lines: Vec<&str> = content.lines().collect();
        stats.total_lines = lines.len();

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                stats.blank_lines += 1;
            } else if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
                stats.comment_lines += 1;
            } else {
                stats.code_lines += 1;
            }
        }

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.starts_with("fn ") || trimmed.starts_with("def ") || trimmed.starts_with("function ") || trimmed.starts_with("pub fn ") {
                stats.functions += 1;
            }
            if trimmed.starts_with("struct ") || trimmed.starts_with("class ") || trimmed.starts_with("interface ") || trimmed.starts_with("enum ") {
                stats.classes += 1;
            }
        }

        stats.complexity = self.calculate_complexity(content);

        stats
    }

    fn calculate_complexity(&self, content: &str) -> usize {
        let mut complexity = 1;
        
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("if ") || line.starts_with("if(") || line.contains(" else ") {
                complexity += 1;
            }
            if line.starts_with("for ") || line.starts_with("for(") || line.starts_with("while ") || line.starts_with("while(") {
                complexity += 1;
            }
            if line.starts_with("match ") || line.contains("switch ") {
                complexity += 1;
            }
            if line.contains("&&") || line.contains("||") {
                complexity += line.matches("&&").count() + line.matches("||").count();
            }
        }

        complexity
    }

    fn check_correctness(&self, content: &str, result: &mut ReviewResult) {
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            if line.contains("unwrap()") && !line.trim().starts_with("//") {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Warning, IssueCategory::Correctness, "Potential panic with unwrap()")
                        .with_description("Using unwrap() can cause panics. Consider using expect() with a message or proper error handling.")
                        .with_suggestion("Use `ok_or()` or `?` operator for better error handling")
                        .with_rule("correctness-no-unwrap")
                );
            }

            if line.contains("expect(\"") && line.contains("\")") {
                let expect_msg = line.split("expect(\"").nth(1).and_then(|s| s.split("\")").next());
                if let Some(msg) = expect_msg {
                    if msg.is_empty() || msg.len() < 5 {
                        result.add_issue(
                            Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::BestPractice, "Vague expect message")
                                .with_description("The expect message is too short or vague.")
                                .with_suggestion("Provide a more descriptive error message")
                                .with_rule("best-practice-expect-message")
                        );
                    }
                }
            }

            if line.contains("todo!()") || line.contains("TODO") || line.contains("FIXME") {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Maintainability, "TODO/FIXME found")
                        .with_description("This code has a TODO or FIXME that should be addressed.")
                        .with_rule("maintainability-todo")
                );
            }
        }
    }

    fn check_security_issues(&self, content: &str, result: &mut ReviewResult) {
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            let security_patterns = [
                ("password", "Potential hardcoded password"),
                ("secret", "Potential hardcoded secret"),
                ("api_key", "Potential hardcoded API key"),
                ("apikey", "Potential hardcoded API key"),
                ("token", "Potential hardcoded token"),
                ("private_key", "Potential hardcoded private key"),
                ("access_key", "Potential hardcoded access key"),
            ];

            for (pattern, message) in security_patterns.iter() {
                if line.to_lowercase().contains(pattern) && line.contains("=") && line.contains("\"") {
                    result.add_issue(
                        Issue::new(&result.file, line_num, IssueSeverity::Critical, IssueCategory::Security, message)
                            .with_description("Hardcoded credentials detected. This is a security risk.")
                            .with_suggestion("Use environment variables or a secure secrets manager")
                            .with_rule("security-no-hardcoded-secrets")
                    );
                }
            }

            if line.contains("eval(") || line.contains("eval (") {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Critical, IssueCategory::Security, "Use of eval()")
                        .with_description("eval() can execute arbitrary code and is a security risk.")
                        .with_suggestion("Use safer alternatives or validate input strictly")
                        .with_rule("security-no-eval")
                );
            }

            if line.contains("sql") && line.contains("+") && line.contains("\"") {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Error, IssueCategory::Security, "Potential SQL injection")
                        .with_description("String concatenation in SQL query detected.")
                        .with_suggestion("Use parameterized queries or prepared statements")
                        .with_rule("security-sql-injection")
                );
            }

            if line.contains("exec(") || line.contains("system(") || line.contains("shell_exec(") {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Error, IssueCategory::Security, "Command execution")
                        .with_description("Dynamic command execution detected.")
                        .with_suggestion("Validate and sanitize all inputs, use allowlists")
                        .with_rule("security-command-execution")
                );
            }
        }
    }

    fn check_performance_issues(&self, content: &str, result: &mut ReviewResult) {
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            if line.contains(".clone()") && line.matches(".clone()").count() > 1 {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Performance, "Multiple clones")
                        .with_description("Multiple clone operations may impact performance.")
                        .with_suggestion("Consider using references where possible")
                        .with_rule("performance-multiple-clones")
                );
            }

            if line.contains("for ") && line.contains(".collect()") {
                let next_line = lines.get(idx + 1).unwrap_or(&"");
                if next_line.contains("for ") {
                    result.add_issue(
                        Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Performance, "Nested iteration")
                            .with_description("Nested loops over collected data may be inefficient.")
                            .with_suggestion("Consider using iterators directly or optimizing the algorithm")
                            .with_rule("performance-nested-iteration")
                    );
                }
            }

            if line.contains("String::from(") && line.contains("\"") && line.len() < 50 {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Performance, "String allocation")
                        .with_description("Using String::from for simple literals.")
                        .with_suggestion("Consider using &str for string literals")
                        .with_rule("performance-string-allocation")
                );
            }
        }
    }

    fn check_style_issues(&self, content: &str, result: &mut ReviewResult) {
        let lines: Vec<&str> = content.lines().collect();

        for (idx, line) in lines.iter().enumerate() {
            let line_num = idx + 1;

            if line.len() > 120 {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Style, "Line too long")
                        .with_description(&format!("Line is {} characters (max 120)", line.len()))
                        .with_suggestion("Break the line into multiple lines for better readability")
                        .with_rule("style-line-length")
                );
            }

            if line.trim().len() > 0 && !line.starts_with(|c: char| c.is_whitespace()) && line.contains("  ") {
                result.add_issue(
                    Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Style, "Multiple spaces")
                        .with_description("Multiple consecutive spaces detected.")
                        .with_suggestion("Use single spaces or tabs for indentation")
                        .with_rule("style-multiple-spaces")
                );
            }

            if idx > 0 && line.trim().is_empty() {
                let prev_line = lines[idx - 1].trim();
                if prev_line.is_empty() {
                    result.add_issue(
                        Issue::new(&result.file, line_num, IssueSeverity::Info, IssueCategory::Style, "Multiple blank lines")
                            .with_description("Multiple consecutive blank lines.")
                            .with_suggestion("Use single blank line for separation")
                            .with_rule("style-blank-lines")
                    );
                }
            }
        }
    }

    fn generate_summary(&self, result: &mut ReviewResult) {
        let total_issues = result.issues.len();
        let critical = result.get_issues_by_severity(IssueSeverity::Critical).len();
        let errors = result.get_issues_by_severity(IssueSeverity::Error).len();
        let warnings = result.get_issues_by_severity(IssueSeverity::Warning).len();

        let mut summary = format!(
            "Found {} issue(s): {} critical, {} errors, {} warnings. ",
            total_issues, critical, errors, warnings
        );

        if result.score >= 90.0 {
            summary.push_str("Excellent code quality!");
        } else if result.score >= 70.0 {
            summary.push_str("Good code quality with some improvements needed.");
        } else if result.score >= 50.0 {
            summary.push_str("Moderate code quality. Several issues should be addressed.");
        } else {
            summary.push_str("Poor code quality. Significant improvements required.");
        }

        result.summary = summary;
    }

    pub fn set_check_security(&mut self, check: bool) {
        self.check_security = check;
    }

    pub fn set_check_performance(&mut self, check: bool) {
        self.check_performance = check;
    }

    pub fn set_check_style(&mut self, check: bool) {
        self.check_style = check;
    }
}

impl Default for InspectCore {
    fn default() -> Self {
        Self::new()
    }
}
