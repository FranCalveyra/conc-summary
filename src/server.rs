use crate::leibniz_adder::get_term;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let line = &http_request[0];

    let leibniz_term = parse_line(&line);
    if leibniz_term == -1f64 {
        &stream
            .write_all("Error calculating series".as_bytes())
            .unwrap();
    }
    let response_body = leibniz_term.to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    &stream.write(response.as_bytes()).unwrap();

    println!("Request: {line:#?}")
}

fn parse_line(request_line: &String) -> f64 {
    println!("Request Line: {}", request_line);

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() > 1 {
        let path = parts[1];

        if let Some(captured_params) = extract_path_param(path, "/pi/:i") {
            if let Some(term) = captured_params.get("i") {
                println!("Extracted Term: {}", &term);
                let leibniz_term = get_term(term.parse::<i32>().unwrap());
                return leibniz_term;
            }
        }
    }
    -1f64
}

fn extract_path_param(url: &str, pattern: &str) -> Option<HashMap<String, String>> {
    let url_parts: Vec<&str> = url.trim_matches('/').split('/').collect();
    let pattern_parts: Vec<&str> = pattern.trim_matches('/').split('/').collect();

    if url_parts.len() != pattern_parts.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (url_part, pattern_part) in url_parts.iter().zip(pattern_parts.iter()) {
        if pattern_part.starts_with(':') {
            let key = pattern_part.trim_start_matches(':').to_string();
            params.insert(key, url_part.to_string());
        } else if pattern_part != url_part {
            return None; // Mismatch in static segments
        }
    }

    Some(params)
}
