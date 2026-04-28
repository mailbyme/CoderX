pub struct Box {
    title: String,
    content: String,
    border_style: BorderStyle,
    padding: usize,
    width: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
    None,
}

impl Box {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            border_style: BorderStyle::Single,
            padding: 1,
            width: None,
        }
    }

    pub fn with_border_style(mut self, style: BorderStyle) -> Self {
        self.border_style = style;
        self
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn render(&self) -> String {
        let (h, v, tl, tr, bl, br) = match self.border_style {
            BorderStyle::Single => ('─', '│', '┌', '┐', '└', '┘'),
            BorderStyle::Double => ('═', '║', '╔', '╗', '╚', '╝'),
            BorderStyle::Rounded => ('─', '│', '╭', '╮', '╰', '╯'),
            BorderStyle::None => return self.content.clone(),
        };

        let content_lines: Vec<&str> = self.content.lines().collect();
        let max_content_width = content_lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let title_width = self.title.len() + 2;

        let inner_width = self.width
            .map(|w| w.saturating_sub(2))
            .unwrap_or(max_content_width.max(title_width));

        let padding_str = " ".repeat(self.padding);

        let mut result = String::new();

        if self.title.is_empty() {
            let line = h.to_string().repeat(inner_width + self.padding * 2);
            result.push_str(&format!("{}{}{}\n", tl, line, tr));
        } else {
            let title_line = format!(" {} ", self.title);
            let remaining = inner_width + self.padding * 2 - title_line.len();
            let h_line = h.to_string().repeat(remaining);
            result.push_str(&format!("{}{}{}{}\n", tl, title_line, h_line, tr));
        }

        for _ in 0..self.padding {
            let spaces = " ".repeat(inner_width + self.padding * 2);
            result.push_str(&format!("{}{}{}\n", v, spaces, v));
        }

        for line in &content_lines {
            let line_padded = format!("{}{}", padding_str, line);
            let extra_padding = inner_width + self.padding * 2 - line.len();
            let spaces = " ".repeat(extra_padding);
            result.push_str(&format!("{}{}{}{}\n", v, line_padded, spaces, v));
        }

        for _ in 0..self.padding {
            let spaces = " ".repeat(inner_width + self.padding * 2);
            result.push_str(&format!("{}{}{}\n", v, spaces, v));
        }

        let line = h.to_string().repeat(inner_width + self.padding * 2);
        result.push_str(&format!("{}{}{}\n", bl, line, br));

        result
    }
}

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    column_widths: Vec<usize>,
}

impl Table {
    pub fn new(headers: Vec<&str>) -> Self {
        let headers: Vec<String> = headers.iter().map(|s| s.to_string()).collect();
        let column_widths = headers.iter().map(|h| h.len()).collect();

        Self {
            headers,
            rows: Vec::new(),
            column_widths,
        }
    }

    pub fn add_row(&mut self, row: Vec<&str>) {
        let row: Vec<String> = row.iter().map(|s| s.to_string()).collect();

        for (i, cell) in row.iter().enumerate() {
            if i < self.column_widths.len() {
                self.column_widths[i] = self.column_widths[i].max(cell.len());
            }
        }

        self.rows.push(row);
    }

    pub fn render(&self) -> String {
        let mut result = String::new();

        let separator = self.render_separator();
        result.push_str(&separator);
        result.push_str(&self.render_row(&self.headers));
        result.push_str(&separator);

        for row in &self.rows {
            result.push_str(&self.render_row(row));
        }

        result.push_str(&separator);

        result
    }

    fn render_separator(&self) -> String {
        let mut parts = Vec::new();
        parts.push("+".to_string());
        for width in &self.column_widths {
            parts.push("-".repeat(width + 2));
            parts.push("+".to_string());
        }
        parts.join("") + "\n"
    }

    fn render_row(&self, row: &[String]) -> String {
        let mut parts = Vec::new();
        parts.push("|".to_string());
        for (i, cell) in row.iter().enumerate() {
            let width = self.column_widths.get(i).copied().unwrap_or(0);
            parts.push(format!(" {:width$} |", cell, width = width));
        }
        parts.join("") + "\n"
    }
}

pub struct List {
    items: Vec<String>,
    style: ListStyle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListStyle {
    Bullet,
    Numbered,
    Checkbox,
}

impl List {
    pub fn new(style: ListStyle) -> Self {
        Self {
            items: Vec::new(),
            style,
        }
    }

    pub fn add(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    pub fn render(&self) -> String {
        let mut result = String::new();

        for (i, item) in self.items.iter().enumerate() {
            let prefix = match self.style {
                ListStyle::Bullet => "• ".to_string(),
                ListStyle::Numbered => format!("{}. ", i + 1),
                ListStyle::Checkbox => "☐ ".to_string(),
            };
            result.push_str(&format!("{}{}\n", prefix, item));
        }

        result
    }

    pub fn render_checked(&self, checked: &[bool]) -> String {
        let mut result = String::new();

        for (i, item) in self.items.iter().enumerate() {
            let is_checked = checked.get(i).copied().unwrap_or(false);
            let prefix = match self.style {
                ListStyle::Bullet => "• ".to_string(),
                ListStyle::Numbered => format!("{}. ", i + 1),
                ListStyle::Checkbox => {
                    if is_checked { "☑ ".to_string() } else { "☐ ".to_string() }
                }
            };
            result.push_str(&format!("{}{}\n", prefix, item));
        }

        result
    }
}
