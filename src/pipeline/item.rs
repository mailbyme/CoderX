use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PipelineItemStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipelineItemPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Urgent = 4,
}

#[derive(Debug, Clone)]
pub struct PipelineItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: PipelineItemStatus,
    pub priority: PipelineItemPriority,
    pub created_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
    pub progress: f32,
    pub result: Option<String>,
    pub error: Option<String>,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl PipelineItem {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            status: PipelineItemStatus::Pending,
            priority: PipelineItemPriority::Normal,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
            progress: 0.0,
            result: None,
            error: None,
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn with_priority(mut self, priority: PipelineItemPriority) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_dependency(mut self, item_id: &str) -> Self {
        self.dependencies.push(item_id.to_string());
        self
    }

    pub fn start(&mut self) {
        self.status = PipelineItemStatus::Running;
        self.started_at = Some(Instant::now());
    }

    pub fn complete(&mut self, result: &str) {
        self.status = PipelineItemStatus::Completed;
        self.completed_at = Some(Instant::now());
        self.result = Some(result.to_string());
        self.progress = 100.0;
    }

    pub fn fail(&mut self, error: &str) {
        self.status = PipelineItemStatus::Failed;
        self.completed_at = Some(Instant::now());
        self.error = Some(error.to_string());
    }

    pub fn cancel(&mut self) {
        self.status = PipelineItemStatus::Cancelled;
        self.completed_at = Some(Instant::now());
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 100.0);
    }

    pub fn duration(&self) -> Option<Duration> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(Instant::now().duration_since(start)),
            _ => None,
        }
    }

    pub fn is_ready(&self, completed_items: &[&str]) -> bool {
        self.dependencies
            .iter()
            .all(|dep| completed_items.contains(&dep.as_str()))
    }

    pub fn format_status(&self) -> String {
        let status = match self.status {
            PipelineItemStatus::Pending => "⏳ Pending",
            PipelineItemStatus::Running => "🔄 Running",
            PipelineItemStatus::Completed => "✅ Completed",
            PipelineItemStatus::Failed => "❌ Failed",
            PipelineItemStatus::Cancelled => "🚫 Cancelled",
        };

        let progress = if self.progress > 0.0 {
            format!(" ({:.0}%)", self.progress)
        } else {
            String::new()
        };

        format!("{}{}", status, progress)
    }
}
