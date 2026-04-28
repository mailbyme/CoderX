use super::types::{Plan, PlanStep, PlanStatus, StepStatus};

pub struct BlueprintExecutor {
    current_plan: Option<Plan>,
    execution_log: Vec<ExecutionLogEntry>,
}

#[derive(Debug, Clone)]
pub struct ExecutionLogEntry {
    pub timestamp: u64,
    pub step_id: String,
    pub action: String,
    pub details: String,
}

impl BlueprintExecutor {
    pub fn new() -> Self {
        Self {
            current_plan: None,
            execution_log: Vec::new(),
        }
    }

    pub fn load_plan(&mut self, plan: Plan) {
        self.current_plan = Some(plan);
        self.execution_log.clear();
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.current_plan.is_none() {
            return Err("No plan loaded".to_string());
        }

        let plan = self.current_plan.as_mut().unwrap();

        if plan.status != PlanStatus::Approved {
            return Err("Plan must be approved before execution".to_string());
        }

        plan.start();
        
        for step in &mut plan.steps {
            if step.dependencies.is_empty() {
                step.status = StepStatus::Ready;
            }
        }

        self.log("plan", "start", "Plan execution started");

        Ok(())
    }

    pub fn tick(&mut self) -> Vec<ExecutionAction> {
        let mut actions = Vec::new();

        let plan = match &mut self.current_plan {
            Some(p) if p.status == PlanStatus::Running => p,
            _ => return actions,
        };

        let ready_steps: Vec<String> = plan.get_ready_steps()
            .iter()
            .map(|s| s.id.clone())
            .collect();

        for step_id in ready_steps {
            if let Some(step) = plan.get_step_mut(&step_id) {
                let name = step.name.clone();
                let description = step.description.clone();
                
                step.status = StepStatus::Running;

                actions.push(ExecutionAction::ExecuteStep {
                    step_id: step_id.clone(),
                    name,
                    description,
                });
            }
        }

        actions
    }

    pub fn complete_step(&mut self, step_id: &str, result: &str) {
        if let Some(plan) = &mut self.current_plan {
            plan.complete_step(step_id, result);
            
            if plan.status == PlanStatus::Completed {
                self.log("plan", "complete", "All steps completed");
            }
        }
        self.log(step_id, "complete", result);
    }

    pub fn fail_step(&mut self, step_id: &str, error: &str) {
        if let Some(plan) = &mut self.current_plan {
            plan.fail_step(step_id, error);
        }
        self.log(step_id, "fail", error);
    }

    pub fn get_progress(&self) -> Option<(usize, usize)> {
        self.current_plan.as_ref().map(|p| (p.completed_steps, p.total_steps))
    }

    pub fn get_progress_percentage(&self) -> f32 {
        self.current_plan.as_ref().map(|p| p.get_progress()).unwrap_or(0.0)
    }

    pub fn is_complete(&self) -> bool {
        self.current_plan.as_ref().map(|p| p.status == PlanStatus::Completed).unwrap_or(false)
    }

    pub fn is_failed(&self) -> bool {
        self.current_plan.as_ref().map(|p| p.status == PlanStatus::Failed).unwrap_or(false)
    }

    pub fn get_current_step(&self) -> Option<&PlanStep> {
        self.current_plan.as_ref().and_then(|plan| {
            plan.steps.iter().find(|s| s.status == StepStatus::Running)
        })
    }

    pub fn get_plan_summary(&self) -> Option<String> {
        self.current_plan.as_ref().map(|p| p.format_summary())
    }

    pub fn cancel(&mut self) {
        if let Some(plan) = &mut self.current_plan {
            plan.cancel();
            self.log("plan", "cancel", "Plan cancelled");
        }
    }

    fn log(&mut self, step_id: &str, action: &str, details: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        self.execution_log.push(ExecutionLogEntry {
            timestamp,
            step_id: step_id.to_string(),
            action: action.to_string(),
            details: details.to_string(),
        });
    }

    pub fn get_execution_log(&self) -> &[ExecutionLogEntry] {
        &self.execution_log
    }
}

#[derive(Debug, Clone)]
pub enum ExecutionAction {
    ExecuteStep {
        step_id: String, name: String, description: String,
    },
    WaitForDependencies {
        step_id: String, pending_deps: Vec<String>,
    },
    PlanCompleted {
        total_steps: usize, success: bool,
    },
}

impl Default for BlueprintExecutor {
    fn default() -> Self {
        Self::new()
    }
}
