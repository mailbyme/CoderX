use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryType {
    User,
    Feedback,
    Project,
    Reference,
}

impl MemoryType {
    pub fn as_str(&self) -> &str {
        match self {
            MemoryType::User => "user",
            MemoryType::Feedback => "feedback",
            MemoryType::Project => "project",
            MemoryType::Reference => "reference",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "user" => MemoryType::User,
            "feedback" => MemoryType::Feedback,
            "reference" => MemoryType::Reference,
            _ => MemoryType::Project,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub name: String,
    pub description: String,
    pub memory_type: MemoryType,
    pub content: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub tags: Vec<String>,
}

impl Memory {
    pub fn new(name: String, memory_type: MemoryType) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            name,
            description: String::new(),
            memory_type,
            content: String::new(),
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }

    pub fn to_markdown(&self) -> String {
        format!(
            "---\nname: {}\ndescription: {}\ntype: {}\ntags: {}\ncreated: {}\nupdated: {}\n---\n\n{}",
            self.name,
            self.description,
            self.memory_type.as_str(),
            self.tags.join(", "),
            self.created_at,
            self.updated_at,
            self.content
        )
    }

    pub fn from_markdown(content: &str) -> Option<Self> {
        let parts: Vec<&str> = content.splitn(2, "---\n").collect();
        if parts.len() < 2 {
            return None;
        }

        let frontmatter = parts.get(1)?;
        let content_start = frontmatter.find("\n---\n")?;
        let metadata = &frontmatter[..content_start];
        let body = &frontmatter[content_start + 5..];

        let mut name = String::new();
        let mut description = String::new();
        let mut memory_type = MemoryType::Project;
        let mut tags = Vec::new();
        let mut created_at = 0u64;
        let mut updated_at = 0u64;

        for line in metadata.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "name" => name = value.to_string(),
                    "description" => description = value.to_string(),
                    "type" => memory_type = MemoryType::from_str(value),
                    "tags" => tags = value.split(',').map(|s| s.trim().to_string()).collect(),
                    "created" => created_at = value.parse().unwrap_or(0),
                    "updated" => updated_at = value.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        Some(Self {
            name,
            description,
            memory_type,
            content: body.to_string(),
            created_at,
            updated_at,
            tags,
        })
    }
}

pub struct MemoryManager {
    memory_dir: PathBuf,
    memories: HashMap<String, Memory>,
    max_memories: usize,
    max_content_length: usize,
}

impl MemoryManager {
    pub fn new() -> Self {
        let memory_dir = Self::get_default_memory_dir();

        Self {
            memory_dir,
            memories: HashMap::new(),
            max_memories: 100,
            max_content_length: 2000,
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

    pub fn initialize(&mut self) -> io::Result<()> {
        fs::create_dir_all(&self.memory_dir)?;
        self.load_all()?;
        Ok(())
    }

    pub fn load_all(&mut self) -> io::Result<()> {
        self.memories.clear();

        if !self.memory_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.memory_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(memory) = Memory::from_markdown(&content) {
                        self.memories.insert(memory.name.clone(), memory);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn save(&self, memory: &Memory) -> io::Result<()> {
        let filename = format!("{}.md", memory.name.replace(' ', "_").to_lowercase());
        let path = self.memory_dir.join(filename);

        fs::create_dir_all(&self.memory_dir)?;

        let content = memory.to_markdown();
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn add(&mut self, memory: Memory) -> io::Result<()> {
        if self.memories.len() >= self.max_memories {
            self.evict_oldest();
        }

        let name = memory.name.clone();
        self.save(&memory)?;
        self.memories.insert(name, memory);

        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Memory> {
        self.memories.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Memory> {
        self.memories.get_mut(name)
    }

    pub fn remove(&mut self, name: &str) -> io::Result<()> {
        if let Some(memory) = self.memories.remove(name) {
            let filename = format!("{}.md", memory.name.replace(' ', "_").to_lowercase());
            let path = self.memory_dir.join(filename);
            if path.exists() {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    pub fn list(&self) -> Vec<&Memory> {
        self.memories.values().collect()
    }

    pub fn list_by_type(&self, memory_type: MemoryType) -> Vec<&Memory> {
        self.memories
            .values()
            .filter(|m| m.memory_type == memory_type)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Memory> {
        let query_lower = query.to_lowercase();

        self.memories
            .values()
            .filter(|m| {
                m.name.to_lowercase().contains(&query_lower)
                    || m.description.to_lowercase().contains(&query_lower)
                    || m.content.to_lowercase().contains(&query_lower)
                    || m.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    fn evict_oldest(&mut self) {
        if self.memories.is_empty() {
            return;
        }

        let oldest = self
            .memories
            .iter()
            .min_by_key(|(_, m)| m.updated_at)
            .map(|(k, _)| k.clone());

        if let Some(key) = oldest {
            let _ = self.remove(&key);
        }
    }

    pub fn update(&mut self, name: &str, content: &str) -> io::Result<()> {
        if let Some(memory) = self.memories.get_mut(name) {
            memory.content = content.chars().take(self.max_content_length).collect();
            memory.updated_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
        }
        if let Some(memory) = self.memories.get(name) {
            self.save(memory)?;
        }
        Ok(())
    }

    pub fn get_memory_count(&self) -> usize {
        self.memories.len()
    }

    pub fn build_memory_prompt(&self) -> String {
        let mut prompt = String::from("# Memory Context\n\n");

        let user_memories: Vec<_> = self.list_by_type(MemoryType::User);
        if !user_memories.is_empty() {
            prompt.push_str("## User Preferences\n");
            for m in user_memories {
                prompt.push_str(&format!("- **{}**: {}\n", m.name, m.description));
            }
            prompt.push('\n');
        }

        let project_memories: Vec<_> = self.list_by_type(MemoryType::Project);
        if !project_memories.is_empty() {
            prompt.push_str("## Project Context\n");
            for m in project_memories {
                prompt.push_str(&format!("- **{}**: {}\n", m.name, m.description));
            }
            prompt.push('\n');
        }

        let feedback_memories: Vec<_> = self.list_by_type(MemoryType::Feedback);
        if !feedback_memories.is_empty() {
            prompt.push_str("## Feedback & Learnings\n");
            for m in feedback_memories {
                prompt.push_str(&format!("- **{}**: {}\n", m.name, m.description));
            }
            prompt.push('\n');
        }

        prompt
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}
