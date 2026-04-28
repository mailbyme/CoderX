use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};
use std::time::Duration;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    pub fn is_error(&self) -> bool {
        self.status_code >= 400
    }
}

#[derive(Debug)]
pub enum HttpError {
    ConnectionError(String),
    ParseError(String),
    TimeoutError,
    TlsError(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpError::ConnectionError(s) => write!(f, "Connection error: {}", s),
            HttpError::ParseError(s) => write!(f, "Parse error: {}", s),
            HttpError::TimeoutError => write!(f, "Request timeout"),
            HttpError::TlsError(s) => write!(f, "TLS error: {}", s),
            HttpError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<std::io::Error> for HttpError {
    fn from(e: std::io::Error) -> Self {
        HttpError::IoError(e)
    }
}

pub struct HttpClient {
    timeout: Duration,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(timeout: Duration) -> Self {
        Self { timeout }
    }

    pub fn get(&self, url: &str) -> Result<HttpResponse, HttpError> {
        let (host, port, path, _use_tls) = Self::parse_url(url)?;
        self.request("GET", &host, port, &path, "", &[])
    }

    pub fn post(
        &self,
        url: &str,
        body: &str,
        headers: &[(&str, &str)],
    ) -> Result<HttpResponse, HttpError> {
        let (host, port, path, _use_tls) = Self::parse_url(url)?;
        self.request("POST", &host, port, &path, body, headers)
    }

    fn parse_url(url: &str) -> Result<(String, u16, String, bool), HttpError> {
        let url = url.trim();
        
        let (scheme, rest) = if url.starts_with("https://") {
            ("https", &url[8..])
        } else if url.starts_with("http://") {
            ("http", &url[7..])
        } else {
            return Err(HttpError::ParseError("Invalid URL scheme".to_string()));
        };

        let use_tls = scheme == "https";
        let default_port = if use_tls { 443 } else { 80 };

        let (host_port, path) = if let Some(slash_pos) = rest.find('/') {
            (&rest[..slash_pos], &rest[slash_pos..])
        } else {
            (rest, "/")
        };

        let (host, port) = if let Some(colon_pos) = host_port.find(':') {
            let host = &host_port[..colon_pos];
            let port: u16 = host_port[colon_pos + 1..]
                .parse()
                .map_err(|_| HttpError::ParseError("Invalid port".to_string()))?;
            (host.to_string(), port)
        } else {
            (host_port.to_string(), default_port)
        };

        Ok((host, port, path.to_string(), use_tls))
    }

    fn request(
        &self,
        method: &str,
        host: &str,
        port: u16,
        path: &str,
        body: &str,
        headers: &[(&str, &str)],
    ) -> Result<HttpResponse, HttpError> {
        let addr = format!("{}:{}", host, port);
        let addr = addr.to_socket_addrs()?
            .next()
            .ok_or_else(|| HttpError::ConnectionError("Cannot resolve host".to_string()))?;

        let mut stream = TcpStream::connect_timeout(&addr, self.timeout)?;
        stream.set_read_timeout(Some(self.timeout))?;
        stream.set_write_timeout(Some(self.timeout))?;

        let mut request = format!("{} {} HTTP/1.1\r\n", method, path);
        request.push_str(&format!("Host: {}\r\n", host));
        request.push_str("Connection: close\r\n");
        request.push_str("User-Agent: CoderX/1.0\r\n");
        
        for (key, value) in headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        if !body.is_empty() {
            request.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }
        
        request.push_str("\r\n");
        
        if !body.is_empty() {
            request.push_str(body);
        }

        stream.write_all(request.as_bytes())?;
        stream.flush()?;

        let mut response_str = String::new();
        stream.read_to_string(&mut response_str)?;

        Self::parse_response(&response_str)
    }

    fn parse_response(response: &str) -> Result<HttpResponse, HttpError> {
        let mut lines = response.lines();
        
        let status_line = lines.next()
            .ok_or_else(|| HttpError::ParseError("Empty response".to_string()))?;

        let mut parts = status_line.splitn(3, ' ');
        let _http_version = parts.next()
            .ok_or_else(|| HttpError::ParseError("Invalid status line".to_string()))?;
        let status_code: u16 = parts.next()
            .ok_or_else(|| HttpError::ParseError("Missing status code".to_string()))?
            .parse()
            .map_err(|_| HttpError::ParseError("Invalid status code".to_string()))?;
        let status_text = parts.next().unwrap_or("").to_string();

        let mut headers = HashMap::new();
        let mut body_start = 0;

        for (i, line) in response.lines().enumerate() {
            if line.is_empty() {
                body_start = response.find("\r\n\r\n")
                    .map(|pos| pos + 4)
                    .unwrap_or(0);
                break;
            }
            
            if i > 0 {
                if let Some(colon_pos) = line.find(':') {
                    let key = line[..colon_pos].trim().to_lowercase();
                    let value = line[colon_pos + 1..].trim().to_string();
                    headers.insert(key, value);
                }
            }
        }

        let body = if body_start > 0 && body_start < response.len() {
            response[body_start..].to_string()
        } else {
            String::new()
        };

        Ok(HttpResponse {
            status_code,
            status_text,
            headers,
            body,
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url() {
        let (host, port, path, tls) = HttpClient::parse_url("https://api.anthropic.com/v1/messages").unwrap();
        assert_eq!(host, "api.anthropic.com");
        assert_eq!(port, 443);
        assert_eq!(path, "/v1/messages");
        assert!(tls);

        let (host, port, path, tls) = HttpClient::parse_url("http://localhost:8080/api").unwrap();
        assert_eq!(host, "localhost");
        assert_eq!(port, 8080);
        assert_eq!(path, "/api");
        assert!(!tls);
    }

    #[test]
    fn test_parse_response() {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 13\r\n\r\n{\"key\":\"value\"}";
        let parsed = HttpClient::parse_response(response).unwrap();
        assert_eq!(parsed.status_code, 200);
        assert_eq!(parsed.status_text, "OK");
        assert_eq!(parsed.body, "{\"key\":\"value\"}");
    }
}
