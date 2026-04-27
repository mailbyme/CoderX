pub struct CommandParser;

pub enum ParseResult {
    Empty,
    Command(String, Vec<String>),
    Message(String),
}

impl CommandParser {
    pub fn parse(input: &str) -> ParseResult {
        let trimmed = input.trim();
        
        if trimmed.is_empty() {
            return ParseResult::Empty;
        }

        if trimmed.starts_with('/') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            let cmd_name = parts[0].to_string();
            let args = parts[1..].iter().map(|s| s.to_string()).collect();
            ParseResult::Command(cmd_name, args)
        } else {
            ParseResult::Message(trimmed.to_string())
        }
    }
}
