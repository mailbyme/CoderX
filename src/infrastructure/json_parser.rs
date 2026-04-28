use std::collections::HashMap;
use std::char;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
    Null,
}

impl JsonValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(obj) => obj.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(arr) => arr.get(index),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, JsonValue::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, JsonValue::Number(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, JsonValue::Bool(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, JsonValue::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Object(_))
    }
}

#[derive(Debug)]
pub struct JsonParseError {
    pub message: String,
    pub position: usize,
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON parse error at position {}: {}", self.position, self.message)
    }
}

pub struct JsonParser;

impl JsonParser {
    pub fn string(s: &str) -> JsonValue {
        JsonValue::String(s.to_string())
    }

    pub fn number(n: f64) -> JsonValue {
        JsonValue::Number(n)
    }

    pub fn bool(b: bool) -> JsonValue {
        JsonValue::Bool(b)
    }

    pub fn array(items: &[JsonValue]) -> JsonValue {
        JsonValue::Array(items.to_vec())
    }

    pub fn object(pairs: &[(&str, JsonValue)]) -> JsonValue {
        let mut map = HashMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.clone());
        }
        JsonValue::Object(map)
    }

    pub fn null() -> JsonValue {
        JsonValue::Null
    }

    pub fn serialize(value: &JsonValue) -> String {
        match value {
            JsonValue::String(s) => Self::escape_string(s),
            JsonValue::Number(n) => {
                if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(Self::serialize).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("{}:{}", Self::escape_string(k), Self::serialize(v)))
                    .collect();
                format!("{{{}}}", pairs.join(","))
            }
            JsonValue::Null => "null".to_string(),
        }
    }

    fn escape_string(s: &str) -> String {
        let mut result = String::from("\"");
        for c in s.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                c if c.is_control() => {
                    result.push_str(&format!("\\u{:04x}", c as u32));
                }
                c => result.push(c),
            }
        }
        result.push('"');
        result
    }

    pub fn parse(s: &str) -> Result<JsonValue, JsonParseError> {
        let mut parser = Parser::new(s);
        parser.parse_value()
    }

    pub fn parse_str(s: &str) -> JsonValue {
        Self::parse(s).unwrap_or(JsonValue::Null)
    }

    pub fn extract_string(value: &JsonValue) -> Option<String> {
        match value {
            JsonValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn extract_number(value: &JsonValue) -> Option<f64> {
        match value {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn extract_bool(value: &JsonValue) -> Option<bool> {
        match value {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn parse_value(&mut self) -> Result<JsonValue, JsonParseError> {
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Err(JsonParseError {
                message: "Unexpected end of input".to_string(),
                position: self.pos,
            });
        }

        let c = self.current_char();
        let value = match c {
            '"' => self.parse_string()?,
            '0'..='9' | '-' => self.parse_number()?,
            't' | 'f' => self.parse_bool()?,
            'n' => self.parse_null()?,
            '[' => self.parse_array()?,
            '{' => self.parse_object()?,
            _ => {
                return Err(JsonParseError {
                    message: format!("Unexpected character: '{}'", c),
                    position: self.pos,
                })
            }
        };

        self.skip_whitespace();
        Ok(value)
    }

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            let c = self.current_char();
            self.pos += c.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.current_char() {
                ' ' | '\t' | '\n' | '\r' => self.advance(),
                _ => break,
            }
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue, JsonParseError> {
        if self.current_char() != '"' {
            return Err(JsonParseError {
                message: "Expected '\"'".to_string(),
                position: self.pos,
            });
        }
        self.advance();

        let mut result = String::new();
        while self.pos < self.input.len() {
            match self.current_char() {
                '"' => {
                    self.advance();
                    return Ok(JsonValue::String(result));
                }
                '\\' => {
                    self.advance();
                    if self.pos >= self.input.len() {
                        return Err(JsonParseError {
                            message: "Unexpected end of string escape".to_string(),
                            position: self.pos,
                        });
                    }
                    let escaped = match self.current_char() {
                        '"' => '"',
                        '\\' => '\\',
                        '/' => '/',
                        'b' => '\x08',
                        'f' => '\x0c',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        'u' => {
                            self.advance();
                            if self.pos + 4 > self.input.len() {
                                return Err(JsonParseError {
                                    message: "Invalid unicode escape".to_string(),
                                    position: self.pos,
                                });
                            }
                            let hex = &self.input[self.pos..self.pos + 4];
                            let code = u32::from_str_radix(hex, 16).map_err(|_| JsonParseError {
                                message: "Invalid unicode escape".to_string(),
                                position: self.pos,
                            })?;
                            self.pos += 4;
                            char::from_u32(code).unwrap_or('\u{fffd}')
                        }
                        c => {
                            return Err(JsonParseError {
                                message: format!("Invalid escape character: '{}'", c),
                                position: self.pos,
                            })
                        }
                    };
                    result.push(escaped);
                    self.advance();
                }
                c => {
                    result.push(c);
                    self.advance();
                }
            }
        }

        Err(JsonParseError {
            message: "Unterminated string".to_string(),
            position: self.pos,
        })
    }

    fn parse_number(&mut self) -> Result<JsonValue, JsonParseError> {
        let start = self.pos;
        
        if self.current_char() == '-' {
            self.advance();
        }

        if self.current_char() == '0' {
            self.advance();
        } else if self.current_char().is_ascii_digit() {
            while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        } else {
            return Err(JsonParseError {
                message: "Invalid number".to_string(),
                position: self.pos,
            });
        }

        if self.pos < self.input.len() && self.current_char() == '.' {
            self.advance();
            while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        }

        if self.pos < self.input.len() && (self.current_char() == 'e' || self.current_char() == 'E') {
            self.advance();
            if self.pos < self.input.len() && (self.current_char() == '+' || self.current_char() == '-') {
                self.advance();
            }
            while self.pos < self.input.len() && self.current_char().is_ascii_digit() {
                self.advance();
            }
        }

        let num_str = &self.input[start..self.pos];
        let num: f64 = num_str.parse().map_err(|_| JsonParseError {
            message: format!("Invalid number: {}", num_str),
            position: start,
        })?;

        Ok(JsonValue::Number(num))
    }

    fn parse_bool(&mut self) -> Result<JsonValue, JsonParseError> {
        if self.input[self.pos..].starts_with("true") {
            self.pos += 4;
            Ok(JsonValue::Bool(true))
        } else if self.input[self.pos..].starts_with("false") {
            self.pos += 5;
            Ok(JsonValue::Bool(false))
        } else {
            Err(JsonParseError {
                message: "Expected 'true' or 'false'".to_string(),
                position: self.pos,
            })
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue, JsonParseError> {
        if self.input[self.pos..].starts_with("null") {
            self.pos += 4;
            Ok(JsonValue::Null)
        } else {
            Err(JsonParseError {
                message: "Expected 'null'".to_string(),
                position: self.pos,
            })
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, JsonParseError> {
        if self.current_char() != '[' {
            return Err(JsonParseError {
                message: "Expected '['".to_string(),
                position: self.pos,
            });
        }
        self.advance();

        let mut items = Vec::new();
        self.skip_whitespace();

        if self.current_char() == ']' {
            self.advance();
            return Ok(JsonValue::Array(items));
        }

        loop {
            let value = self.parse_value()?;
            items.push(value);

            self.skip_whitespace();
            match self.current_char() {
                ',' => {
                    self.advance();
                    self.skip_whitespace();
                }
                ']' => {
                    self.advance();
                    return Ok(JsonValue::Array(items));
                }
                _ => {
                    return Err(JsonParseError {
                        message: "Expected ',' or ']'".to_string(),
                        position: self.pos,
                    })
                }
            }
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonParseError> {
        if self.current_char() != '{' {
            return Err(JsonParseError {
                message: "Expected '{'".to_string(),
                position: self.pos,
            });
        }
        self.advance();

        let mut map = HashMap::new();
        self.skip_whitespace();

        if self.current_char() == '}' {
            self.advance();
            return Ok(JsonValue::Object(map));
        }

        loop {
            self.skip_whitespace();
            
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => unreachable!(),
            };

            self.skip_whitespace();
            if self.current_char() != ':' {
                return Err(JsonParseError {
                    message: "Expected ':'".to_string(),
                    position: self.pos,
                });
            }
            self.advance();

            let value = self.parse_value()?;
            map.insert(key, value);

            self.skip_whitespace();
            match self.current_char() {
                ',' => {
                    self.advance();
                }
                '}' => {
                    self.advance();
                    return Ok(JsonValue::Object(map));
                }
                _ => {
                    return Err(JsonParseError {
                        message: "Expected ',' or '}'".to_string(),
                        position: self.pos,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let result = JsonParser::parse(r#""hello""#).unwrap();
        assert_eq!(result.as_str(), Some("hello"));
    }

    #[test]
    fn test_parse_number() {
        let result = JsonParser::parse("42").unwrap();
        assert_eq!(result.as_number(), Some(42.0));

        let result = JsonParser::parse("-3.14").unwrap();
        assert_eq!(result.as_number(), Some(-3.14));

        let result = JsonParser::parse("1e10").unwrap();
        assert_eq!(result.as_number(), Some(1e10));
    }

    #[test]
    fn test_parse_bool() {
        let result = JsonParser::parse("true").unwrap();
        assert_eq!(result.as_bool(), Some(true));

        let result = JsonParser::parse("false").unwrap();
        assert_eq!(result.as_bool(), Some(false));
    }

    #[test]
    fn test_parse_null() {
        let result = JsonParser::parse("null").unwrap();
        assert!(result.is_null());
    }

    #[test]
    fn test_parse_array() {
        let result = JsonParser::parse("[1, 2, 3]").unwrap();
        let arr = result.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_number(), Some(1.0));
    }

    #[test]
    fn test_parse_object() {
        let result = JsonParser::parse(r#"{"name": "test", "value": 42}"#).unwrap();
        assert_eq!(result.get("name").unwrap().as_str(), Some("test"));
        assert_eq!(result.get("value").unwrap().as_number(), Some(42.0));
    }

    #[test]
    fn test_serialize() {
        let value = JsonParser::object(&[
            ("name", JsonParser::string("test")),
            ("count", JsonParser::number(42)),
        ]);
        let serialized = JsonParser::serialize(&value);
        assert!(serialized.contains("\"name\":\"test\""));
        assert!(serialized.contains("\"count\":42"));
    }

    #[test]
    fn test_roundtrip() {
        let json = r#"{"key":"value","number":123,"bool":true,"null":null,"array":[1,2,3]}"#;
        let parsed = JsonParser::parse(json).unwrap();
        let serialized = JsonParser::serialize(&parsed);
        let reparsed = JsonParser::parse(&serialized).unwrap();
        
        assert_eq!(parsed.get("key").unwrap().as_str(), reparsed.get("key").unwrap().as_str());
    }
}
