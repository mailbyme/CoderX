use super::{Terminal, Color};
use std::io;
use crate::i18n::{Language, translate, USER, SYSTEM, TOOL, ERROR, UNKNOWN, WELCOME_TITLE, WELCOME_HINT};

pub struct Renderer {
    terminal: Terminal,
    language: Language,
}

impl Renderer {
    pub fn new(language: Language) -> Self {
        Self {
            terminal: Terminal::new(),
            language,
        }
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.terminal.clear()
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn render_welcome(&mut self) -> io::Result<()> {
        self.clear()?;
        self.terminal.write_color("\n  ______           __  __           \n", Color::Cyan)?;
        self.terminal.write_color(" / ____/_  _______/ /_/ /____  _____\n", Color::Cyan)?;
        self.terminal.write_color("/ /   / / / / ___/ __/ __/ _ \\/ ___/\n", Color::Cyan)?;
        self.terminal.write_color("/ /___/ /_/ (__  ) /_/ /_/  __/ /\n", Color::Cyan)?;
        self.terminal.write_color("\\____/\\__,_/____/\\__/\\__/\\___/_/   \n\n", Color::Cyan)?;
        
        let title = translate(&WELCOME_TITLE, self.language);
        let hint = translate(&WELCOME_HINT, self.language);
        
        self.terminal.write_color(&format!("  {}\n\n", title), Color::Green)?;
        self.terminal.write_color(&format!("  {}\n\n", hint), Color::Yellow)?;
        Ok(())
    }

    pub fn render_message(&mut self, role: &str, content: &str) -> io::Result<()> {
        let (prefix, color) = match role {
            "user" => (translate(&USER, self.language), Color::Blue),
            "assistant" => ("[CoderX] ", Color::Green),
            "system" => (translate(&SYSTEM, self.language), Color::Yellow),
            "tool" => (translate(&TOOL, self.language), Color::Cyan),
            _ => (translate(&UNKNOWN, self.language), Color::Reset),
        };

        self.terminal.write_color(prefix, color)?;
        self.terminal.write(" ")?;
        self.terminal.write(content)?;
        self.terminal.write("\n\n")?;
        Ok(())
    }

    pub fn render_tool_use(&mut self, tool_name: &str, args: &str) -> io::Result<()> {
        self.terminal.write_color(&format!("{} ", translate(&TOOL, self.language)), Color::Cyan)?;
        self.terminal.write(tool_name)?;
        self.terminal.write(" ")?;
        self.terminal.write(args)?;
        self.terminal.write("\n")?;
        Ok(())
    }

    pub fn render_prompt(&mut self) -> io::Result<String> {
        self.terminal.write_color("> ", Color::Blue)?;
        self.terminal.read_line("")
    }

    pub fn render_status(&mut self, status: &str) -> io::Result<()> {
        let width = self.terminal.width();
        let padding = " ".repeat((width as usize) - status.len() - 2);
        self.terminal.write_color(&format!(" {} {}", status, padding), Color::Blue)?;
        self.terminal.write("\n")?;
        Ok(())
    }

    pub fn render_error(&mut self, message: &str) -> io::Result<()> {
        self.terminal.write_color(&format!("{} ", translate(&ERROR, self.language)), Color::Red)?;
        self.terminal.write(message)?;
        self.terminal.write("\n")?;
        Ok(())
    }

    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
