use crate::errors::Error;
use crate::http_method::HttpMethod;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

pub struct Request {
    pub uri: String,
    pub parts: usize,
    pub headers: String,
    pub body: Vec<String>,
    pub method: HttpMethod,
    pub content_type: ContentType,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);
        let request: Vec<String> = get_request(buf_reader);
        let (method, uri) = parse_line(&request[0]).unwrap();
        let (headers, body) = Self::split_headers_and_body(&request[1..]);
        let content_type = Self::get_content_type(&headers);

        Request {
            uri: uri.to_string(),
            parts: uri.trim_matches('/').split('/').count(),
            headers,
            body,
            method,
            content_type,
        }
    }

    fn split_headers_and_body(lines: &[String]) -> (String, Vec<String>) {
        let mut headers = String::new();
        let mut body_lines = Vec::new();
        let mut is_body = false;

        for line in lines {
            if line.is_empty() {
                is_body = true;
                continue;
            }

            if is_body {
                body_lines.push(line.clone());
            } else {
                headers.push_str(line);
                headers.push('\n');
            }
        }

        let boundary = headers
            .lines()
            .find(|line| line.starts_with("Content-Type: multipart/form-data; boundary="))
            .and_then(|line| line.split("boundary=").nth(1))
            .map(|b| format!("--{}", b.trim()));

        let mut body_parts = Vec::new();
        if let Some(boundary_marker) = boundary {
            let mut current_part = Vec::new();
            for line in body_lines {
                if line.starts_with(&boundary_marker) {
                    if !current_part.is_empty() {
                        body_parts.extend(current_part.iter().cloned());
                        current_part = Vec::new();
                    }
                    continue;
                }
                current_part.push(line);
            }

            if !current_part.is_empty() {
                body_parts.push(current_part.join("\n"));
            }
        } else {
            body_parts.push(body_lines.join("\n"));
        }

        (headers, body_parts)
    }

    fn get_content_type(headers: &str) -> ContentType {
        match headers {
            h if h.contains("Content-Type: multipart/form-data") => ContentType::MultipartFormData,
            h if h.contains("Content-Type: application/json") => {
                ContentType::Application("json".to_string())
            }
            _ => ContentType::Text("plain".to_string()),
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum ContentType {
    MultipartFormData,
    Text(String),
    Application(String),
}

fn get_request(mut buf_reader: BufReader<&mut TcpStream>) -> Vec<String> {
    let mut request_lines = Vec::new();
    let mut content_length: Option<usize> = None;

    loop {
        let mut line = String::new();
        let bytes_read = buf_reader.read_line(&mut line).unwrap();

        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim_end().to_string();

        if trimmed.is_empty() {
            request_lines.push(String::new());
            break;
        }

        if let Some(cl) = trimmed.strip_prefix("Content-Length:") {
            content_length = cl.trim().parse::<usize>().ok();
        }

        request_lines.push(trimmed);
    }

    if let Some(length) = content_length {
        let mut body = vec![0u8; length];
        buf_reader.read_exact(&mut body).unwrap();

        let body_text = String::from_utf8_lossy(&body);
        request_lines.extend(body_text.lines().map(|s| s.to_string()));
    }

    request_lines
}

fn parse_line(request_line: &String) -> Result<(HttpMethod, &str), Error> {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() > 1 {
        let method_string = parts[0];
        let method = HttpMethod::from_str(method_string);
        let path = parts[1];

        let segments: Vec<&str> = path.trim_matches('/').split('/').collect();
        if segments.len() == 1 {
            Ok((method, segments[0]))
        } else {
            Err(Error::InvalidRoute)
        }
    } else {
        Err(Error::InvalidRoute)
    }
}
