use super::{Terminal, Color};
use std::io;

pub struct Renderer {
    terminal: Terminal,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(),
        }
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.terminal.clear()
    }

    pub fn render_welcome(&mut self) -> io::Result<()> {
        self.clear()?;
        self.terminal.write_color("\n  ______           __  __           \n", Color::Cyan)?;
        self.terminal.write_color(" / ____/_  _______/ /_/ /____  _____\n", Color::Cyan)?;
        self.terminal.write_color("/ /   / / / / ___/ __/ __/ _ \\/ ___/\n", Color::Cyan)?;
        self.terminal.write_color("/ /___/ /_/ (__  ) /_/ /_/  __/ /\n", Color::Cyan)?;
        self.terminal.write_color("\\____/\\__,_/____/\\__/\\__/\\___/_/   \n\n", Color::Cyan)?;
        self.terminal.write_color("  AI-Powered Coding Assistant\n\n", Color::Green)?;
        self.terminal.write_color("  Type /help for available commands\n\n", Color::Yellow)?;
        Ok(())
    }

    pub fn render_message(&mut self, role: &str, content: &str) -> io::Result<()> {
        let (prefix, color) = match role {
            "user" => ("[USER] ", Color::Blue),
            "assistant" => ("[CoderX] ", Color::Green),
            "system" => ("[SYS] ", Color::Yellow),
            "tool" => ("[TOOL] ", Color::Cyan),
            _ => ("[UNKNOWN] ", Color::Reset),
        };

        self.terminal.write_color(prefix, color)?;
        self.terminal.write(content)?;
        self.terminal.write("\n\n")?;
        Ok(())
    }

    pub fn render_tool_use(&mut self, tool_name: &str, args: &str) -> io::Result<()> {
        self.terminal.write_color("[TOOL] ", Color::Cyan)?;
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
        self.terminal.write_color("[ERROR] ", Color::Red)?;
        self.terminal.write(message)?;
        self.terminal.write("\n")?;
        Ok(())
    }

    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }
}
