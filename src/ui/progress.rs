pub struct ProgressBar {
    total: usize,
    current: usize,
    width: usize,
    label: String,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            width: 40,
            label: String::new(),
        }
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn set_current(&mut self, current: usize) {
        self.current = current.min(self.total);
    }

    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
        }
    }

    pub fn get_percentage(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.current as f32 / self.total as f32) * 100.0
    }

    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }

    pub fn render(&self) -> String {
        let percentage = self.get_percentage();
        let filled = if self.total > 0 {
            (self.width as f32 * (self.current as f32 / self.total as f32)) as usize
        } else {
            0
        };

        let empty = self.width - filled;

        let bar = format!(
            "[{}{}]",
            "█".repeat(filled),
            "░".repeat(empty)
        );

        if self.label.is_empty() {
            format!("{} {:.0}%", bar, percentage)
        } else {
            format!("{} {} {:.0}%", self.label, bar, percentage)
        }
    }

    pub fn render_with_status(&self, status: &str) -> String {
        let bar = self.render();
        if status.is_empty() {
            bar
        } else {
            format!("{} - {}", bar, status)
        }
    }

    pub fn spinner_frames() -> Vec<&'static str> {
        vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    }

    pub fn render_spinner(frame: usize, message: &str) -> String {
        let frames = Self::spinner_frames();
        let frame_char = frames[frame % frames.len()];
        format!("{} {}", frame_char, message)
    }
}

pub struct Spinner {
    frame: usize,
    message: String,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            frame: 0,
            message: message.to_string(),
        }
    }

    pub fn tick(&mut self) -> String {
        let output = ProgressBar::render_spinner(self.frame, &self.message);
        self.frame = (self.frame + 1) % ProgressBar::spinner_frames().len();
        output
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }
}

pub struct StatusIndicator {
    status: Status,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Idle,
    Running,
    Success,
    Error,
    Warning,
}

impl StatusIndicator {
    pub fn new() -> Self {
        Self {
            status: Status::Idle,
            message: String::new(),
        }
    }

    pub fn set_status(&mut self, status: Status, message: &str) {
        self.status = status;
        self.message = message.to_string();
    }

    pub fn render(&self) -> String {
        let icon = match self.status {
            Status::Idle => "⚪",
            Status::Running => "🔄",
            Status::Success => "✅",
            Status::Error => "❌",
            Status::Warning => "⚠️",
        };

        if self.message.is_empty() {
            icon.to_string()
        } else {
            format!("{} {}", icon, self.message)
        }
    }
}

impl Default for StatusIndicator {
    fn default() -> Self {
        Self::new()
    }
}
