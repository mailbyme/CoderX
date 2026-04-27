use std::net::{TcpStream, ToSocketAddrs};
use std::io::{Read, Write};

pub struct HttpClient;

impl HttpClient {
    pub fn new() -> Self {
        Self
    }

    pub fn post(
        &self,
        host: &str,
        port: u16,
        path: &str,
        body: &str,
        headers: &[(&str, &str)],
    ) -> std::io::Result<String> {
        let addr = (host, port).to_socket_addrs()?.next()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "Cannot resolve host"))?;

        let mut stream = TcpStream::connect(addr)?;

        let mut request = format!("POST {} HTTP/1.1\r\n", path);
        request.push_str(&format!("Host: {}\r\n", host));
        for (key, value) in headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }
        request.push_str(&format!("Content-Length: {}\r\n\r\n{}", body.len(), body));

        stream.write_all(request.as_bytes())?;

        let mut response = String::new();
        stream.read_to_string(&mut response)?;

        Ok(response)
    }
}
