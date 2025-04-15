use crate::errors::Error;
use crate::http_method::HttpMethod;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn get_request_line(buf_reader: BufReader<&TcpStream>) -> Vec<String> {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}

pub fn parse_line(request_line: &String) -> Result<(HttpMethod, &str), Error> {
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
