use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueCategory {
    Correctness,
    Security,
    Performance,
    Maintainability,
    Style,
    Documentation,
    Testing,
    BestPractice,
}

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub title: String,
    pub description: String,
    pub suggestion: Option<String>,
    pub rule: String,
    pub metadata: HashMap<String, String>,
}

impl Issue {
    pub fn new(file: &str, line: usize, severity: IssueSeverity, category: IssueCategory, title: &str) -> Self {
        Self {
            id: format!("issue_{}_{}", file.replace('/', "_"), line),
            file: file.to_string(),
            line,
            column: 0,
            severity,
            category,
            title: title.to_string(),
            description: String::new(),
            suggestion: None,
            rule: String::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    pub fn with_rule(mut self, rule: &str) -> Self {
        self.rule = rule.to_string();
        self
    }

    pub fn format(&self) -> String {
        let severity_str = match self.severity {
            IssueSeverity::Info => "ℹ️ INFO",
            IssueSeverity::Warning => "⚠️ WARN",
            IssueSeverity::Error => "❌ ERROR",
            IssueSeverity::Critical => "🚨 CRITICAL",
        };

        let category_str = match self.category {
            IssueCategory::Correctness => "Correctness",
            IssueCategory::Security => "Security",
            IssueCategory::Performance => "Performance",
            IssueCategory::Maintainability => "Maintainability",
            IssueCategory::Style => "Style",
            IssueCategory::Documentation => "Documentation",
            IssueCategory::Testing => "Testing",
            IssueCategory::BestPractice => "Best Practice",
        };

        let mut output = format!(
            "{} [{}] {}:{} - {}\n",
            severity_str, category_str, self.file, self.line, self.title
        );

        if !self.description.is_empty() {
            output.push_str(&format!("  {}\n", self.description));
        }

        if let Some(ref suggestion) = self.suggestion {
            output.push_str(&format!("  💡 Suggestion: {}\n", suggestion));
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub file: String,
    pub issues: Vec<Issue>,
    pub score: f32,
    pub summary: String,
    pub stats: ReviewStats,
}

#[derive(Debug, Clone, Default)]
pub struct ReviewStats {
    pub total_lines: usize,
    pub code_lines: usize,
    pub comment_lines: usize,
    pub blank_lines: usize,
    pub functions: usize,
    pub classes: usize,
    pub complexity: usize,
}

impl ReviewResult {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_string(),
            issues: Vec::new(),
            score: 100.0,
            summary: String::new(),
            stats: ReviewStats::default(),
        }
    }

    pub fn add_issue(&mut self, issue: Issue) {
        let penalty = match issue.severity {
            IssueSeverity::Info => 0.0,
            IssueSeverity::Warning => 5.0,
            IssueSeverity::Error => 15.0,
            IssueSeverity::Critical => 30.0,
        };
        self.score = (self.score - penalty).max(0.0);
        self.issues.push(issue);
    }

    pub fn get_issues_by_severity(&self, severity: IssueSeverity) -> Vec<&Issue> {
        self.issues.iter().filter(|i| i.severity == severity).collect()
    }

    pub fn get_issues_by_category(&self, category: IssueCategory) -> Vec<&Issue> {
        self.issues.iter().filter(|i| i.category == category).collect()
    }

    pub fn has_critical_issues(&self) -> bool {
        self.issues.iter().any(|i| i.severity == IssueSeverity::Critical)
    }

    pub fn has_security_issues(&self) -> bool {
        self.issues.iter().any(|i| i.category == IssueCategory::Security)
    }

    pub fn format_report(&self) -> String {
        let mut report = format!("# Code Review: {}\n\n", self.file);
        
        report.push_str(&format!("**Score:** {:.0}/100\n\n", self.score));

        report.push_str("## Statistics\n\n");
        report.push_str(&format!("- Total Lines: {}\n", self.stats.total_lines));
        report.push_str(&format!("- Code Lines: {}\n", self.stats.code_lines));
        report.push_str(&format!("- Comment Lines: {}\n", self.stats.comment_lines));
        report.push_str(&format!("- Functions: {}\n", self.stats.functions));
        report.push_str(&format!("- Classes: {}\n", self.stats.classes));
        report.push_str(&format!("- Complexity: {}\n\n", self.stats.complexity));

        if !self.issues.is_empty() {
            report.push_str("## Issues\n\n");

            let critical: Vec<_> = self.get_issues_by_severity(IssueSeverity::Critical);
            let errors: Vec<_> = self.get_issues_by_severity(IssueSeverity::Error);
            let warnings: Vec<_> = self.get_issues_by_severity(IssueSeverity::Warning);
            let infos: Vec<_> = self.get_issues_by_severity(IssueSeverity::Info);

            if !critical.is_empty() {
                report.push_str("### 🚨 Critical\n\n");
                for issue in critical {
                    report.push_str(&issue.format());
                }
                report.push('\n');
            }

            if !errors.is_empty() {
                report.push_str("### ❌ Errors\n\n");
                for issue in errors {
                    report.push_str(&issue.format());
                }
                report.push('\n');
            }

            if !warnings.is_empty() {
                report.push_str("### ⚠️ Warnings\n\n");
                for issue in warnings {
                    report.push_str(&issue.format());
                }
                report.push('\n');
            }

            if !infos.is_empty() {
                report.push_str("### ℹ️ Info\n\n");
                for issue in infos {
                    report.push_str(&issue.format());
                }
                report.push('\n');
            }
        }

        if !self.summary.is_empty() {
            report.push_str("## Summary\n\n");
            report.push_str(&self.summary);
        }

        report
    }
}
