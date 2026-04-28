use std::collections::VecDeque;
use super::item::{PipelineItem, PipelineItemStatus, PipelineItemPriority};

pub struct Pipeline {
    pending: VecDeque<PipelineItem>,
    running: Vec<PipelineItem>,
    completed: Vec<PipelineItem>,
    max_concurrent: usize,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            running: Vec::new(),
            completed: Vec::new(),
            max_concurrent: 4,
        }
    }

    pub fn with_max_concurrent(max: usize) -> Self {
        Self {
            pending: VecDeque::new(),
            running: Vec::new(),
            completed: Vec::new(),
            max_concurrent: max,
        }
    }

    pub fn push(&mut self, item: PipelineItem) {
        self.pending.push_back(item);
        self.sort_by_priority();
    }

    pub fn push_front(&mut self, item: PipelineItem) {
        self.pending.push_front(item);
        self.sort_by_priority();
    }

    fn sort_by_priority(&mut self) {
        let mut items: Vec<PipelineItem> = self.pending.drain(..).collect();
        items.sort_by(|a, b| b.priority.cmp(&a.priority));
        self.pending = items.into_iter().collect();
    }

    pub fn pop(&mut self) -> Option<PipelineItem> {
        if self.running.len() >= self.max_concurrent {
            return None;
        }

        let completed_ids: Vec<&str> = self.completed
            .iter()
            .filter(|t| t.status == PipelineItemStatus::Completed)
            .map(|t| t.id.as_str())
            .collect();

        for i in 0..self.pending.len() {
            if self.pending[i].is_ready(&completed_ids) {
                let item = self.pending.remove(i).unwrap();
                return Some(item);
            }
        }

        None
    }

    pub fn start_item(&mut self, mut item: PipelineItem) {
        item.start();
        self.running.push(item);
    }

    pub fn complete_item(&mut self, item_id: &str, result: &str) {
        if let Some(pos) = self.running.iter().position(|t| t.id == item_id) {
            let mut item = self.running.remove(pos);
            item.complete(result);
            self.completed.push(item);
        }
    }

    pub fn fail_item(&mut self, item_id: &str, error: &str) {
        if let Some(pos) = self.running.iter().position(|t| t.id == item_id) {
            let mut item = self.running.remove(pos);
            item.fail(error);
            self.completed.push(item);
        }
    }

    pub fn cancel_item(&mut self, item_id: &str) {
        if let Some(pos) = self.pending.iter().position(|t| t.id == item_id) {
            let mut item = self.pending.remove(pos).unwrap();
            item.cancel();
            self.completed.push(item);
        } else if let Some(pos) = self.running.iter().position(|t| t.id == item_id) {
            let mut item = self.running.remove(pos);
            item.cancel();
            self.completed.push(item);
        }
    }

    pub fn update_progress(&mut self, item_id: &str, progress: f32) {
        if let Some(item) = self.running.iter_mut().find(|t| t.id == item_id) {
            item.set_progress(progress);
        }
    }

    pub fn get(&self, item_id: &str) -> Option<&PipelineItem> {
        self.pending.iter()
            .chain(self.running.iter())
            .chain(self.completed.iter())
            .find(|t| t.id == item_id)
    }

    pub fn get_mut(&mut self, item_id: &str) -> Option<&mut PipelineItem> {
        if let Some(item) = self.pending.iter_mut().find(|t| t.id == item_id) {
            return Some(item);
        }
        if let Some(item) = self.running.iter_mut().find(|t| t.id == item_id) {
            return Some(item);
        }
        if let Some(item) = self.completed.iter_mut().find(|t| t.id == item_id) {
            return Some(item);
        }
        None
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub fn running_count(&self) -> usize {
        self.running.len()
    }

    pub fn completed_count(&self) -> usize {self.completed.len()
    }

    pub fn total_count(&self) -> usize {
        self.pending.len() + self.running.len() + self.completed.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty() && self.running.is_empty()
    }

    pub fn has_capacity(&self) -> bool {
        self.running.len() < self.max_concurrent
    }

    pub fn list_pending(&self) -> Vec<&PipelineItem> {
        self.pending.iter().collect()
    }

    pub fn list_running(&self) -> Vec<&PipelineItem> {
        self.running.iter().collect()
    }

    pub fn list_completed(&self) -> Vec<&PipelineItem> {
        self.completed.iter().collect()
    }

    pub fn clear_completed(&mut self) {
        self.completed.clear();
    }

    pub fn get_status_summary(&self) -> String {
        format!(
            "Pipeline: {} pending, {} running, {} completed",
            self.pending_count(),
            self.running_count(),
            self.completed_count()
        )
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
