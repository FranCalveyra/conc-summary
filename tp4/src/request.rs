use crate::errors::Error;
use crate::http_method::HttpMethod;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub struct Request {
    pub uri: String,
    pub parts: usize,
    pub headers: String,
    pub body: String,
    pub method: HttpMethod,
    pub content_type: ContentType,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);
        let lines: Vec<String> = get_request_line(buf_reader);

        let (method, uri) = parse_line(&lines[0]).unwrap();
        let (headers, body) = Self::split_headers_and_body(&lines[1..]);
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

    fn split_headers_and_body(lines: &[String]) -> (String, String) {
        let mut headers = String::new();
        let mut body = String::new();
        let mut is_body = false;

        for line in lines {
            if line.is_empty() {
                is_body = true;
                continue;
            }

            if is_body {
                body.push_str(line);
                body.push('\n');
            } else {
                headers.push_str(line);
                headers.push('\n');
            }
        }

        (headers, body)
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

fn get_request_line(buf_reader: BufReader<&mut TcpStream>) -> Vec<String> {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}

fn parse_line(request_line: &String) -> Result<(HttpMethod, &str), Error> {
    println!("Request Line: {}", request_line);

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
