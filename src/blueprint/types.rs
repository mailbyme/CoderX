use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanStatus {
    Draft,
    Approved,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct PlanStep {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: StepStatus,
    pub dependencies: Vec<String>,
    pub estimated_tokens: usize,
    pub actual_tokens: Option<usize>,
    pub result: Option<String>,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StepStatus {
    Pending,
    Ready,
    Running,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct Plan {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: PlanStatus,
    pub steps: Vec<PlanStep>,
    pub created_at: u64,
    pub updated_at: u64,
    pub total_steps: usize,
    pub completed_steps: usize,
}

impl Plan {
    pub fn new(title: &str) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            id: format!("plan_{}", now),
            title: title.to_string(),
            description: String::new(),
            status: PlanStatus::Draft,
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
            total_steps: 0,
            completed_steps: 0,
        }
    }

    pub fn add_step(&mut self, step: PlanStep) {
        self.steps.push(step);
        self.total_steps = self.steps.len();
        self.update_timestamp();
    }

    pub fn get_ready_steps(&self) -> Vec<&PlanStep> {
        let completed: Vec<&str> = self.steps
            .iter()
            .filter(|s| s.status == StepStatus::Completed)
            .map(|s| s.id.as_str())
            .collect();

        self.steps
            .iter()
            .filter(|s| {
                s.status == StepStatus::Pending &&
                s.dependencies.iter().all(|d| completed.contains(&d.as_str()))
            })
            .collect()
    }

    pub fn get_step(&self, id: &str) -> Option<&PlanStep> {
        self.steps.iter().find(|s| s.id == id)
    }

    pub fn get_step_mut(&mut self, id: &str) -> Option<&mut PlanStep> {
        self.steps.iter_mut().find(|s| s.id == id)
    }

    pub fn update_step_status(&mut self, step_id: &str, status: StepStatus) {
        if let Some(step) = self.get_step_mut(step_id) {
            step.status = status;
            if status == StepStatus::Completed {
                self.completed_steps += 1;
            }
        }
        self.update_timestamp();
    }

    pub fn complete_step(&mut self, step_id: &str, result: &str) {
        if let Some(step) = self.get_step_mut(step_id) {
            step.status = StepStatus::Completed;
            step.result = Some(result.to_string());
            self.completed_steps += 1;
        }
        self.update_timestamp();

        if self.completed_steps == self.total_steps {
            self.status = PlanStatus::Completed;
        }
    }

    pub fn fail_step(&mut self, step_id: &str, error: &str) {
        if let Some(step) = self.get_step_mut(step_id) {
            step.status = StepStatus::Failed;
            step.error = Some(error.to_string());
        }
        self.status = PlanStatus::Failed;
        self.update_timestamp();
    }

    pub fn get_progress(&self) -> f32 {
        if self.total_steps == 0 {
            return 0.0;
        }
        (self.completed_steps as f32 / self.total_steps as f32) * 100.0
    }

    pub fn approve(&mut self) {
        self.status = PlanStatus::Approved;
        self.update_timestamp();
    }

    pub fn start(&mut self) {
        self.status = PlanStatus::Running;
        self.update_timestamp();
    }

    pub fn cancel(&mut self) {
        self.status = PlanStatus::Cancelled;
        self.update_timestamp();
    }

    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }

    pub fn format_summary(&self) -> String {
        let status = match self.status {
            PlanStatus::Draft => "📝 Draft",
            PlanStatus::Approved => "✅ Approved",
            PlanStatus::Running => "🔄 Running",
            PlanStatus::Completed => "🎉 Completed",
            PlanStatus::Failed => "❌ Failed",
            PlanStatus::Cancelled => "🚫 Cancelled",
        };

        let mut summary = format!(
            "# {}\n\n**Status:** {}\n**Progress:** {:.0}% ({}/{})\n\n",
            self.title, status, self.get_progress(), self.completed_steps, self.total_steps
        );

        summary.push_str("## Steps\n\n");
        for step in &self.steps {
            let step_status = match step.status {
                StepStatus::Pending => "⏳",
                StepStatus::Ready => "▶️",
                StepStatus::Running => "🔄",
                StepStatus::Completed => "✅",
                StepStatus::Failed => "❌",
                StepStatus::Skipped => "⏭️",
            };
            summary.push_str(&format!("- {} {}\n", step_status, step.name));
        }

        summary
    }
}

impl PlanStep {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            status: StepStatus::Pending,
            dependencies: Vec::new(),
            estimated_tokens: 0,
            actual_tokens: None,
            result: None,
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_dependency(mut self, step_id: &str) -> Self {
        self.dependencies.push(step_id.to_string());
        self
    }

    pub fn with_estimated_tokens(mut self, tokens: usize) -> Self {
        self.estimated_tokens = tokens;
        self
    }

    pub fn start(&mut self) {
        self.status = StepStatus::Running;
    }

    pub fn complete(&mut self, result: &str) {
        self.status = StepStatus::Completed;
        self.result = Some(result.to_string());
    }

    pub fn fail(&mut self, error: &str) {
        self.status = StepStatus::Failed;
        self.error = Some(error.to_string());
    }

    pub fn skip(&mut self, reason: &str) {
        self.status = StepStatus::Skipped;
        self.result = Some(format!("Skipped: {}", reason));
    }
}
