use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

use crate::infrastructure::json_parser::JsonParser;
use super::message_store::{Message, SharedMessageStore};

pub struct HistoryManager {
    history_dir: PathBuf,
}

impl HistoryManager {
    pub fn new() -> io::Result<Self> {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
        let history_dir = match home {
            Ok(home) => PathBuf::from(home).join(".config").join("coderex").join("history"),
            Err(_) => env::current_dir()?.join(".coderex").join("history"),
        };
        
        fs::create_dir_all(&history_dir)?;
        
        Ok(Self { history_dir })
    }

    pub fn save_history(&self, session_id: &str, messages: &SharedMessageStore) -> io::Result<()> {
        let history_file = self.history_dir.join(format!("{}.json", session_id));
        let all_messages = messages.get_all();
        
        let mut message_arrays = Vec::new();
        for msg in all_messages {
            let msg_obj = JsonParser::object(&[
                ("id", JsonParser::string(&msg.id)),
                ("role", JsonParser::string(&msg.role)),
                ("content", JsonParser::string(&msg.content)),
                ("timestamp", JsonParser::number(msg.timestamp as f64)),
            ]);
            message_arrays.push(msg_obj);
        }
        
        let root = JsonParser::object(&[
            ("session_id", JsonParser::string(session_id)),
            ("messages", JsonParser::array(&message_arrays)),
            ("created_at", JsonParser::number(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as f64)),
        ]);
        
        let json = JsonParser::serialize(&root);
        let mut file = File::create(history_file)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }

    pub fn load_history(&self, session_id: &str) -> io::Result<Vec<Message>> {
        let history_file = self.history_dir.join(format!("{}.json", session_id));
        
        if !history_file.exists() {
            return Ok(Vec::new());
        }
        
        let mut file = File::open(history_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let value = JsonParser::parse(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("JSON parse error: {}", e))
        })?;
        let mut messages = Vec::new();
        
        if let Some(obj) = value.as_object() {
            if let Some(msg_array) = obj.get("messages").and_then(|v| v.as_array()) {
                for msg_val in msg_array {
                    if let Some(msg_obj) = msg_val.as_object() {
                        let id = msg_obj.get("id").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                        let role = msg_obj.get("role").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                        let content = msg_obj.get("content").and_then(|v| v.as_str()).unwrap_or_default().to_string();
                        let timestamp = msg_obj.get("timestamp").and_then(|v| v.as_number()).unwrap_or(0.0) as u64;
                        
                        messages.push(Message {
                            id,
                            role,
                            content,
                            timestamp,
                        });
                    }
                }
            }
        }
        
        Ok(messages)
    }

    pub fn list_sessions(&self) -> io::Result<Vec<String>> {
        let mut sessions = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.history_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "json" {
                        if let Some(file_name) = path.file_stem() {
                            if let Some(session_id) = file_name.to_str() {
                                sessions.push(session_id.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        sessions.sort();
        Ok(sessions)
    }

    pub fn delete_session(&self, session_id: &str) -> io::Result<()> {
        let history_file = self.history_dir.join(format!("{}.json", session_id));
        if history_file.exists() {
            fs::remove_file(history_file)?;
        }
        Ok(())
    }

    pub fn get_session_info(&self, session_id: &str) -> io::Result<Option<SessionInfo>> {
        let history_file = self.history_dir.join(format!("{}.json", session_id));
        
        if !history_file.exists() {
            return Ok(None);
        }
        
        let mut file = File::open(history_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        
        let value = JsonParser::parse(&content).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("JSON parse error: {}", e))
        })?;
        
        if let Some(obj) = value.as_object() {
            let message_count = obj.get("messages")
                .and_then(|v| v.as_array())
                .map(|arr| arr.len())
                .unwrap_or(0);
            
            let created_at = obj.get("created_at")
                .and_then(|v| v.as_number())
                .unwrap_or(0.0) as u64;
            
            let first_message = obj.get("messages")
                .and_then(|v| v.as_array())
                .and_then(|arr| arr.first())
                .and_then(|v| v.as_object())
                .and_then(|o| o.get("content"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            return Ok(Some(SessionInfo {
                session_id: session_id.to_string(),
                message_count,
                created_at,
                preview: first_message.unwrap_or_default(),
            }));
        }
        
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub message_count: usize,
    pub created_at: u64,
    pub preview: String,
}

impl SessionInfo {
    pub fn format_time(&self) -> String {
        let seconds = self.created_at;
        let minutes = seconds / 60;
        let hours = minutes / 60;
        let days = hours / 24;
        
        let year = 1970 + days / 365;
        let day_of_year = days % 365;
        
        let months = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut month = 0;
        let mut day = day_of_year;
        for (i, &days_in_month) in months.iter().enumerate() {
            if day < days_in_month {
                month = i;
                break;
            }
            day -= days_in_month;
        }
        day += 1;
        
        let hour = hours % 24;
        let minute = minutes % 60;
        let second = seconds % 60;
        
        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", 
            year, month + 1, day, hour, minute, second)
    }
}
