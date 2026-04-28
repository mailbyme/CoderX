use super::types::{Plan, PlanStep};

pub struct BlueprintEngine {
    max_steps: usize,
    max_tokens_per_step: usize,
}

impl BlueprintEngine {
    pub fn new() -> Self {
        Self {
            max_steps: 30,
            max_tokens_per_step: 5000,
        }
    }

    pub fn create_plan(&self, title: &str, description: &str) -> Plan {
        let mut plan = Plan::new(title);
        plan.description = description.to_string();
        plan
    }

    pub fn decompose_task(&self, task: &str) -> Plan {
        let mut plan = Plan::new(&format!("Plan for: {}", task));

        let phases = self.analyze_task_phases(task);

        for (i, phase) in phases.iter().enumerate() {
            let step = PlanStep::new(
                &format!("step_{}", i + 1),
                &phase.name,
            )
            .with_description(&phase.description)
            .with_estimated_tokens(phase.estimated_tokens);

            plan.add_step(step);
        }

        self.add_dependencies(&mut plan);
        plan
    }

    fn analyze_task_phases(&self, task: &str) -> Vec<PhaseInfo> {
        let mut phases = Vec::new();
        let task_lower = task.to_lowercase();

        if task_lower.contains("implement") || task_lower.contains("create") || task_lower.contains("build") {
            phases.push(PhaseInfo {
                name: "Research & Analysis".to_string(),
                description: "Analyze requirements and existing codebase".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Design".to_string(),
                description: "Design the solution architecture".to_string(),
                estimated_tokens: 1500,
            });
            phases.push(PhaseInfo {
                name: "Implementation".to_string(),
                description: "Implement the core functionality".to_string(),
                estimated_tokens: 4000,
            });
            phases.push(PhaseInfo {
                name: "Testing".to_string(),
                description: "Write and run tests".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Review & Refine".to_string(),
                description: "Review and refine the implementation".to_string(),
                estimated_tokens: 1500,
            });
        } else if task_lower.contains("fix") || task_lower.contains("bug") {
            phases.push(PhaseInfo {
                name: "Investigate".to_string(),
                description: "Investigate the issue and identify root cause".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Fix".to_string(),
                description: "Implement the fix".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Verify".to_string(),
                description: "Verify the fix works correctly".to_string(),
                estimated_tokens: 1500,
            });
        } else if task_lower.contains("refactor") {
            phases.push(PhaseInfo {
                name: "Analyze".to_string(),
                description: "Analyze current code structure".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Plan Refactoring".to_string(),
                description: "Plan the refactoring approach".to_string(),
                estimated_tokens: 1500,
            });
            phases.push(PhaseInfo {
                name: "Refactor".to_string(),
                description: "Execute the refactoring".to_string(),
                estimated_tokens: 3000,
            });
            phases.push(PhaseInfo {
                name: "Validate".to_string(),
                description: "Validate the refactored code".to_string(),
                estimated_tokens: 1500,
            });
        } else if task_lower.contains("review") {
            phases.push(PhaseInfo {
                name: "Read Code".to_string(),
                description: "Read and understand the code".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Analyze".to_string(),
                description: "Analyze code quality and issues".to_string(),
                estimated_tokens: 2000,
            });
            phases.push(PhaseInfo {
                name: "Report".to_string(),
                description: "Generate review report".to_string(),
                estimated_tokens: 1500,
            });
        } else {
            phases.push(PhaseInfo {
                name: "Understand".to_string(),
                description: "Understand the task requirements".to_string(),
                estimated_tokens: 1500,
            });
            phases.push(PhaseInfo {
                name: "Execute".to_string(),
                description: "Execute the main task".to_string(),
                estimated_tokens: 3000,
            });
            phases.push(PhaseInfo {
                name: "Verify".to_string(),
                description: "Verify the results".to_string(),
                estimated_tokens: 1500,
            });
        }

        phases.truncate(self.max_steps);
        phases
    }

    fn add_dependencies(&self, plan: &mut Plan) {
        for i in 1..plan.steps.len() {
            let prev_id = plan.steps[i - 1].id.clone();
            plan.steps[i].dependencies.push(prev_id);
        }
    }

    pub fn create_parallel_plan(&self, task: &str, parallel_units: usize) -> Plan {
        let mut plan = Plan::new(&format!("Parallel Plan for: {}", task));
        let units = parallel_units.min(self.max_steps).max(1);

        plan.add_step(PlanStep::new("setup", "Setup & Preparation")
            .with_description("Prepare environment and gather context")
            .with_estimated_tokens(1000));

        for i in 0..units {
            plan.add_step(PlanStep::new(
                &format!("unit_{}", i + 1),
                &format!("Parallel Unit {}", i + 1),
            )
            .with_description(&format!("Execute independent unit {} of {}", i + 1, units))
            .with_dependency("setup")
            .with_estimated_tokens(self.max_tokens_per_step));
        }

        plan.add_step(PlanStep::new("merge", "Merge & Finalize")
            .with_description("Merge all parallel results and finalize")
            .with_estimated_tokens(2000));

        for i in 0..units {
            let unit_id = format!("unit_{}", i + 1);
            if let Some(step) = plan.get_step_mut("merge") {
                step.dependencies.push(unit_id);
            }
        }

        plan
    }

    pub fn estimate_total_tokens(&self, plan: &Plan) -> usize {
        plan.steps.iter().map(|s| s.estimated_tokens).sum()
    }

    pub fn validate_plan(&self, plan: &Plan) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if plan.steps.is_empty() {
            errors.push("Plan has no steps".to_string());
        }

        if plan.steps.len() > self.max_steps {
            errors.push(format!("Plan has too many steps (max: {})", self.max_steps));
        }

        let step_ids: Vec<&str> = plan.steps.iter().map(|s| s.id.as_str()).collect();
        for step in &plan.steps {
            for dep in &step.dependencies {
                if !step_ids.contains(&dep.as_str()) {
                    errors.push(format!("Step '{}' has invalid dependency '{}'", step.id, dep));
                }
            }
        }

        if self.has_circular_dependencies(plan) {
            errors.push("Plan has circular dependencies".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn has_circular_dependencies(&self, plan: &Plan) -> bool {
        for step in &plan.steps {
            let mut visited = vec![step.id.clone()];
            if self.check_circular(step, plan, &mut visited) {
                return true;
            }
        }
        false
    }

    fn check_circular(&self, step: &PlanStep, plan: &Plan, visited: &mut Vec<String>) -> bool {
        for dep_id in &step.dependencies {
            if visited.contains(dep_id) {
                return true;
            }
            visited.push(dep_id.clone());
            if let Some(dep_step) = plan.get_step(dep_id) {
                if self.check_circular(dep_step, plan, visited) {
                    return true;
                }
            }
            visited.pop();
        }
        false
    }
}

struct PhaseInfo {
    name: String,
    description: String,
    estimated_tokens: usize,
}

impl Default for BlueprintEngine {
    fn default() -> Self {
        Self::new()
    }
}
