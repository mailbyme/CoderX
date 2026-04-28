use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::item::{PipelineItem, PipelineItemStatus, PipelineItemPriority};
use super::pipeline::Pipeline;

pub struct PipelineScheduler {
    pipeline: Pipeline,
    scheduled_items: HashMap<String, ScheduledItem>,
    last_check: Instant,
    check_interval: Duration,
}

#[derive(Debug, Clone)]
struct ScheduledItem {
    item: PipelineItem,
    schedule: Schedule,
    last_run: Option<Instant>,
    next_run: Option<Instant>,
}

#[derive(Debug, Clone)]
pub enum Schedule {
    Once,
    Interval(Duration),
    Daily { hour: u8, minute: u8 },
    Weekly { day: u8, hour: u8, minute: u8 },
}

impl PipelineScheduler {
    pub fn new() -> Self {
        Self {
            pipeline: Pipeline::new(),
            scheduled_items: HashMap::new(),
            last_check: Instant::now(),
            check_interval: Duration::from_secs(60),
        }
    }

    pub fn with_check_interval(interval: Duration) -> Self {
        Self {
            pipeline: Pipeline::new(),
            scheduled_items: HashMap::new(),
            last_check: Instant::now(),
            check_interval: interval,
        }
    }

    pub fn add_item(&mut self, item: PipelineItem) {
        self.pipeline.push(item);
    }

    pub fn add_scheduled_item(&mut self, item: PipelineItem, schedule: Schedule) {
        let next_run = self.calculate_next_run(&schedule);

        let scheduled = ScheduledItem {
            item,
            schedule,
            last_run: None,
            next_run,
        };

        self.scheduled_items.insert(scheduled.item.id.clone(), scheduled);
    }

    fn calculate_next_run(&self, schedule: &Schedule) -> Option<Instant> {
        match schedule {
            Schedule::Once => Some(Instant::now()),
            Schedule::Interval(duration) => Some(Instant::now() + *duration),
            Schedule::Daily { hour, minute } => {
                let now = Instant::now();
                let elapsed = now.elapsed().as_secs();
                let seconds_until = self.seconds_until_time(*hour, *minute, elapsed);
                Some(Instant::now() + Duration::from_secs(seconds_until))
            }
            Schedule::Weekly { day, hour, minute } => {
                let now = Instant::now();
                let elapsed = now.elapsed().as_secs();
                let seconds_until = self.seconds_until_weekly(*day, *hour, *minute, elapsed);
                Some(Instant::now() + Duration::from_secs(seconds_until))
            }
        }
    }

    fn calculate_next_run_for_id(&self, id: &str) -> Option<Instant> {
        if let Some(scheduled) = self.scheduled_items.get(id) {
            self.calculate_next_run(&scheduled.schedule)
        } else {
            None
        }
    }

    fn seconds_until_time(&self, target_hour: u8, target_minute: u8, _elapsed: u64) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let current_hour = ((now / 3600) % 24) as u8;
        let current_minute = ((now / 60) % 60) as u8;

        let target_seconds = (target_hour as u64 * 3600) + (target_minute as u64 * 60);
        let current_seconds = (current_hour as u64 * 3600) + (current_minute as u64 * 60);

        if target_seconds > current_seconds {
            target_seconds - current_seconds
        } else {
            (24 * 3600) - (current_seconds - target_seconds)
        }
    }

    fn seconds_until_weekly(&self, target_day: u8, target_hour: u8, target_minute: u8, _elapsed: u64) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let current_day = ((now / 86400) % 7) as u8;
        let current_hour = ((now / 3600) % 24) as u8;
        let current_minute = ((now / 60) % 60) as u8;

        let current_seconds = (current_hour as u64 * 3600) + (current_minute as u64 * 60);
        let target_seconds = (target_hour as u64 * 3600) + (target_minute as u64 * 60);

        let days_until = if target_day > current_day {
            target_day - current_day
        } else if target_day == current_day && target_seconds > current_seconds {
            0
        } else {
            7 - (current_day - target_day)
        };

        let seconds_until = (days_until as u64 * 86400) + 
            if target_seconds > current_seconds {
                target_seconds - current_seconds
            } else if days_until == 0 {
                7 * 86400 - (current_seconds - target_seconds)
            } else {
                target_seconds + (86400 - current_seconds)
            };

        seconds_until
    }

    pub fn tick(&mut self) -> Vec<PipelineItem> {
        let mut ready_items = Vec::new();

        if Instant::now().duration_since(self.last_check) < self.check_interval {
            return ready_items;
        }

        self.last_check = Instant::now();

        let now = Instant::now();
        let mut to_update: Vec<String> = Vec::new();
        
        for (id, scheduled) in &self.scheduled_items {
            if let Some(next_run) = scheduled.next_run {
                if now >= next_run {
                    let mut item = scheduled.item.clone();
                    item.status = PipelineItemStatus::Pending;
                    ready_items.push(item);
                    to_update.push(id.clone());
                }
            }
        }

        for id in to_update {
            let next_run = self.calculate_next_run_for_id(&id);
            if let Some(scheduled) = self.scheduled_items.get_mut(&id) {
                scheduled.last_run = Some(now);
                scheduled.next_run = next_run;
            }
        }

        while let Some(item) = self.pipeline.pop() {
            ready_items.push(item);
        }

        ready_items
    }

    pub fn remove_scheduled(&mut self, item_id: &str) {
        self.scheduled_items.remove(item_id);
    }

    pub fn get_scheduled_items(&self) -> Vec<&PipelineItem> {
        self.scheduled_items.values().map(|s| &s.item).collect()
    }

    pub fn get_pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn get_pipeline_mut(&mut self) -> &mut Pipeline {
        &mut self.pipeline
    }

    pub fn get_status(&self) -> SchedulerStatus {
        SchedulerStatus {
            pending_items: self.pipeline.pending_count(),
            running_items: self.pipeline.running_count(),
            completed_items: self.pipeline.completed_count(),
            scheduled_items: self.scheduled_items.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SchedulerStatus {
    pub pending_items: usize,
    pub running_items: usize,
    pub completed_items: usize,
    pub scheduled_items: usize,
}

impl SchedulerStatus {
    pub fn format(&self) -> String {
        format!(
            "Scheduler: {} pending, {} running, {} completed, {} scheduled",
            self.pending_items,
            self.running_items,
            self.completed_items,
            self.scheduled_items
        )
    }
}

impl Default for PipelineScheduler {
    fn default() -> Self {
        Self::new()
    }
}
