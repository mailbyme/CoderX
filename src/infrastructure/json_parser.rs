#[derive(Debug, Clone)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
    Null,
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
        let mut map = std::collections::HashMap::new();
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
            JsonValue::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(Self::serialize).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(map) => {
                let pairs: Vec<String> = map.iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, Self::serialize(v)))
                    .collect();
                format!("{{{}}}", pairs.join(","))
            }
            JsonValue::Null => "null".to_string(),
        }
    }

    pub fn parse(_s: &str) -> JsonValue {
        JsonValue::Null
    }

    pub fn extract_string(value: &JsonValue) -> Option<String> {
        match value {
            JsonValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}
