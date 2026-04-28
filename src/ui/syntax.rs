use std::collections::HashMap;

pub struct SyntaxHighlighter {
    language_configs: HashMap<String, LanguageConfig>,
}

struct LanguageConfig {
    keywords: Vec<&'static str>,
    types: Vec<&'static str>,
    strings: bool,
    comments: bool,
    numbers: bool,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut configs = HashMap::new();

        configs.insert("rust".to_string(), LanguageConfig {
            keywords: vec![
                "fn", "let", "mut", "const", "static", "pub", "mod", "use", "crate",
                "self", "super", "struct", "enum", "impl", "trait", "type", "where",
                "if", "else", "match", "loop", "while", "for", "in", "return", "break",
                "continue", "async", "await", "move", "ref", "as", "unsafe", "extern",
            ],
            types: vec![
                "i8", "i16", "i32", "i64", "i128", "isize",
                "u8", "u16", "u32", "u64", "u128", "usize",
                "f32", "f64", "bool", "char", "str", "String",
                "Vec", "Option", "Result", "Box", "Rc", "Arc",
                "Some", "None", "Ok", "Err", "true", "false",
            ],
            strings: true,
            comments: true,
            numbers: true,
        });

        configs.insert("python".to_string(), LanguageConfig {
            keywords: vec![
                "def", "class", "if", "elif", "else", "for", "while", "try",
                "except", "finally", "with", "as", "import", "from", "return",
                "yield", "raise", "pass", "break", "continue", "lambda", "and",
                "or", "not", "in", "is", "None", "True", "False", "async", "await",
            ],
            types: vec![
                "int", "float", "str", "bool", "list", "dict", "set", "tuple",
                "bytes", "bytearray", "memoryview", "range", "frozenset",
            ],
            strings: true,
            comments: true,
            numbers: true,
        });

        configs.insert("javascript".to_string(), LanguageConfig {
            keywords: vec![
                "function", "var", "let", "const", "if", "else", "for", "while",
                "do", "switch", "case", "break", "continue", "return", "try",
                "catch", "finally", "throw", "new", "delete", "typeof", "instanceof",
                "in", "void", "async", "await", "class", "extends", "import", "export",
            ],
            types: vec![
                "undefined", "null", "true", "false", "NaN", "Infinity",
                "Object", "Array", "String", "Number", "Boolean", "Function",
                "Promise", "Map", "Set", "Symbol",
            ],
            strings: true,
            comments: true,
            numbers: true,
        });

        configs.insert("go".to_string(), LanguageConfig {
            keywords: vec![
                "package", "import", "func", "return", "var", "const", "type",
                "struct", "interface", "map", "chan", "if", "else", "for", "range",
                "switch", "case", "default", "break", "continue", "goto", "fallthrough",
                "defer", "go", "select",
            ],
            types: vec![
                "int", "int8", "int16", "int32", "int64",
                "uint", "uint8", "uint16", "uint32", "uint64", "uintptr",
                "float32", "float64", "complex64", "complex128",
                "bool", "string", "byte", "rune", "error",
                "true", "false", "nil",
            ],
            strings: true,
            comments: true,
            numbers: true,
        });

        Self { language_configs: configs }
    }

    pub fn highlight(&self, code: &str, language: &str) -> String {
        let config = self.language_configs.get(language);

        let mut result = String::new();
        let mut in_string = false;
        let mut in_comment = false;
        let mut string_char = ' ';

        for line in code.lines() {
            let highlighted = self.highlight_line(line, config, &mut in_string, &mut in_comment, &mut string_char);
            result.push_str(&highlighted);
            result.push('\n');
        }

        result
    }

    fn highlight_line(
        &self,
        line: &str,
        config: Option<&LanguageConfig>,
        in_string: &mut bool,
        in_comment: &mut bool,
        string_char: &mut char,
    ) -> String {
        let mut result = String::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if *in_comment {
                result.push_str(&Self::color_comment(&chars[i..].iter().collect::<String>()));
                break;
            }

            let c = chars[i];

            if *in_string {
                result.push_str(&Self::color_string(&c.to_string()));
                if c == *string_char && (i == 0 || chars[i - 1] != '\\') {
                    *in_string = false;
                }
                i += 1;
                continue;
            }

            if c == '"' || c == '\'' || c == '`' {
                *in_string = true;
                *string_char = c;
                result.push_str(&Self::color_string(&c.to_string()));
                i += 1;
                continue;
            }

            if i + 1 < chars.len() && c == '/' && chars[i + 1] == '/' {
                result.push_str(&Self::color_comment(&chars[i..].iter().collect::<String>()));
                break;
            }

            if c == '#' && config.map(|c| c.comments).unwrap_or(true) {
                result.push_str(&Self::color_comment(&chars[i..].iter().collect::<String>()));
                break;
            }

            if c.is_digit(10) {
                let mut num = String::new();
                while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.' || chars[i] == 'x' || (chars[i] >= 'a' && chars[i] <= 'f') || (chars[i] >= 'A' && chars[i] <= 'F')) {
                    num.push(chars[i]);
                    i += 1;
                }
                result.push_str(&Self::color_number(&num));
                continue;
            }

            if c.is_alphabetic() || c == '_' {
                let mut word = String::new();
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    word.push(chars[i]);
                    i += 1;
                }

                if let Some(cfg) = config {
                    if cfg.keywords.contains(&word.as_str()) {
                        result.push_str(&Self::color_keyword(&word));
                    } else if cfg.types.contains(&word.as_str()) {
                        result.push_str(&Self::color_type(&word));
                    } else {
                        result.push_str(&word);
                    }
                } else {
                    result.push_str(&word);
                }
                continue;
            }

            result.push(c);
            i += 1;
        }

        result
    }

    fn color_keyword(text: &str) -> String {
        format!("\x1b[35m{}\x1b[0m", text)
    }

    fn color_type(text: &str) -> String {
        format!("\x1b[33m{}\x1b[0m", text)
    }

    fn color_string(text: &str) -> String {
        format!("\x1b[32m{}\x1b[0m", text)
    }

    fn color_comment(text: &str) -> String {
        format!("\x1b[90m{}\x1b[0m", text)
    }

    fn color_number(text: &str) -> String {
        format!("\x1b[36m{}\x1b[0m", text)
    }

    pub fn detect_language(filename: &str) -> Option<&'static str> {
        let ext = filename.rsplit('.').next()?;
        match ext {
            "rs" => Some("rust"),
            "py" => Some("python"),
            "js" | "jsx" | "mjs" => Some("javascript"),
            "ts" | "tsx" => Some("typescript"),
            "go" => Some("go"),
            "java" => Some("java"),
            "c" | "h" => Some("c"),
            "cpp" | "hpp" | "cc" => Some("cpp"),
            "rb" => Some("ruby"),
            "php" => Some("php"),
            "swift" => Some("swift"),
            "kt" => Some("kotlin"),
            "sh" | "bash" => Some("bash"),
            "json" => Some("json"),
            "yaml" | "yml" => Some("yaml"),
            "toml" => Some("toml"),
            "md" => Some("markdown"),
            "html" => Some("html"),
            "css" => Some("css"),
            "sql" => Some("sql"),
            _ => None,
        }
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
