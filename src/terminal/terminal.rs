use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Color {
    fn code(&self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::Reset => 0,
        }
    }
}

pub struct Terminal {
    stdout: io::Stdout,
    stdin: io::Stdin,
    width: u16,
    height: u16,
}

impl Terminal {
    pub fn new() -> Self {
        let (width, height) = Self::detect_size();
        #[cfg(windows)]
        windows_terminal::enable_vt100();
        
        Self {
            stdout: io::stdout(),
            stdin: io::stdin(),
            width,
            height,
        }
    }

    fn detect_size() -> (u16, u16) {
        #[cfg(windows)]
        {
            windows_terminal::get_terminal_size()
        }
        #[cfg(not(windows))]
        {
            let cols = std::env::var("COLUMNS")
                .unwrap_or("80".to_string())
                .parse()
                .unwrap_or(80);
            let lines = std::env::var("LINES")
                .unwrap_or("24".to_string())
                .parse()
                .unwrap_or(24);
            (cols, lines)
        }
    }

    pub fn clear(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[2J\x1B[H")?;
        self.stdout.flush()
    }

    pub fn set_cursor(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.stdout, "\x1B[{};{}H", y + 1, x + 1)?;
        self.stdout.flush()
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[?25l")?;
        self.stdout.flush()
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        write!(self.stdout, "\x1B[?25h")?;
        self.stdout.flush()
    }

    pub fn read_line(&mut self, prompt: &str) -> io::Result<String> {
        write!(self.stdout, "{}", prompt)?;
        self.stdout.flush()?;
        
        let mut line = String::new();
        self.stdin.read_line(&mut line)?;
        Ok(line.trim().to_string())
    }

    pub fn write(&mut self, text: &str) -> io::Result<()> {
        write!(self.stdout, "{}", text)?;
        self.stdout.flush()
    }

    pub fn write_color(&mut self, text: &str, color: Color) -> io::Result<()> {
        write!(self.stdout, "\x1B[{}m{}\x1B[0m", color.code(), text)?;
        self.stdout.flush()
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }
}

#[cfg(windows)]
mod windows_terminal {
    use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode, GetConsoleScreenBufferInfo};
    use winapi::um::winbase::STD_OUTPUT_HANDLE;
    use winapi::um::processenv::GetStdHandle;

    pub fn enable_vt100() {
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let mut mode = 0u32;
            if GetConsoleMode(handle, &mut mode) != 0 {
                mode |= 0x0004;
                SetConsoleMode(handle, mode);
            }
        }
    }

    pub fn get_terminal_size() -> (u16, u16) {
        unsafe {
            let mut csbi = winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO::default();
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            if GetConsoleScreenBufferInfo(handle, &mut csbi) != 0 {
                (
                    csbi.srWindow.Right - csbi.srWindow.Left + 1,
                    csbi.srWindow.Bottom - csbi.srWindow.Top + 1,
                )
            } else {
                (80, 24)
            }
        }
    }
}
