use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};

pub struct RecallManager {
    memory_dir: PathBuf,
    min_sessions: usize,
    min_hours: u64,
    last_consolidation: u64,
}

impl RecallManager {
    pub fn new() -> Self {
        Self {
            memory_dir: Self::get_default_memory_dir(),
            min_sessions: 5,
            min_hours: 24,
            last_consolidation: 0,
        }
    }

    fn get_default_memory_dir() -> PathBuf {
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            PathBuf::from(home).join(".coderex").join("memory")
        } else {
            PathBuf::from(".coderex").join("memory")
        }
    }

    pub fn set_memory_dir(&mut self, path: PathBuf) {
        self.memory_dir = path;
    }

    pub fn should_consolidate(&self, session_count: usize) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let hours_since_last = (now - self.last_consolidation) / 3600;

        session_count >= self.min_sessions && hours_since_last >= self.min_hours
    }

    pub fn consolidate(&mut self, sessions: &[String]) -> io::Result<ConsolidationResult> {
        let mut result = ConsolidationResult::default();

        for session in sessions {
            if let Ok(content) = fs::read_to_string(session) {
                let extracted = self.extract_memories_from_session(&content);
                result.memories_extracted += extracted.len();
                
                for memory in extracted {
                    let path = self.memory_dir.join(format!(
                        "{}.md",
                        memory.name.replace(' ', "_").to_lowercase()
                    ));
                    fs::create_dir_all(&self.memory_dir)?;
                    let mut file = fs::File::create(path)?;
                    file.write_all(memory.to_markdown().as_bytes())?;
                }
            }
        }

        self.last_consolidation = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        result.sessions_processed = sessions.len();
        result.success = true;

        Ok(result)
    }

    fn extract_memories_from_session(&self, content: &str) -> Vec<super::memory::Memory> {
        let mut memories = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];

            if line.contains("TODO:") || line.contains("FIXME:") || line.contains("NOTE:") {
                let note = line.split(':').last().unwrap_or("").trim();
                if !note.is_empty() {
                    let mut memory = super::memory::Memory::new(
                        format!("note_{}", i),
                        super::memory::MemoryType::Project,
                    );
                    memory.description = note.to_string();
                    memory.content = line.to_string();
                    memories.push(memory);
                }
            }

            if line.contains("error") || line.contains("Error") || line.contains("ERROR") {
                if i + 1 < lines.len() {
                    let mut memory = super::memory::Memory::new(
                        format!("error_{}", i),
                        super::memory::MemoryType::Feedback,
                    );
                    memory.description = "Error encountered".to_string();
                    memory.content = format!("{}\n{}", line, lines[i + 1]);
                    memories.push(memory);
                }
            }

            if line.contains("important") || line.contains("Important") || line.contains("IMPORTANT") {
                let mut memory = super::memory::Memory::new(
                    format!("important_{}", i),
                    super::memory::MemoryType::User,
                );
                memory.description = "Important information".to_string();
                memory.content = line.to_string();
                memories.push(memory);
            }

            i += 1;
        }

        memories
    }

    pub fn prune_old_memories(&mut self, max_age_days: u64) -> io::Result<usize> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let max_age_seconds = max_age_days * 24 * 3600;
        let mut pruned = 0;

        if !self.memory_dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.memory_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(memory) = super::memory::Memory::from_markdown(&content) {
                        if now - memory.updated_at > max_age_seconds {
                            fs::remove_file(&path)?;
                            pruned += 1;
                        }
                    }
                }
            }
        }

        Ok(pruned)
    }

    pub fn get_consolidation_status(&self) -> ConsolidationStatus {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let hours_since = (now - self.last_consolidation) / 3600;

        ConsolidationStatus {
            last_consolidation: self.last_consolidation,
            hours_since_last: hours_since,
            min_sessions_required: self.min_sessions,
            min_hours_required: self.min_hours,
            ready: hours_since >= self.min_hours,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConsolidationResult {
    pub success: bool,
    pub sessions_processed: usize,
    pub memories_extracted: usize,
}

#[derive(Debug, Clone)]
pub struct ConsolidationStatus {
    pub last_consolidation: u64,
    pub hours_since_last: u64,
    pub min_sessions_required: usize,
    pub min_hours_required: u64,
    pub ready: bool,
}

impl Default for RecallManager {
    fn default() -> Self {
        Self::new()
    }
}
