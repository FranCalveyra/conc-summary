use crate::errors::Error;
use crate::log_analyzer::{get_stats, process_file};
use crate::server::{Response, Server};
use std::cmp::PartialEq;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;

pub fn handle_connection(mut stream: TcpStream, server: Arc<Server>) {
    let buf_reader = BufReader::new(&stream);

    let lines = get_request_line(buf_reader);

    let line = &lines[0];

    let parsed_line = parse_line(line);

    if parsed_line.is_err() {
        //     do something
        let err_response = Response::new(400, "".to_string());
        write_response(&mut stream, err_response);
        return;
    }

    let (method, path) = parsed_line.unwrap();

    // May move to the Server struct?
    let response: Response = match (method, path) {
        (HttpMethod::GET, "/stats") => get_stats(server),
        (HttpMethod::POST, "/upload") => process_file(&mut stream, server),
        (_, _) => invalid_route(),
    };

    write_response(&mut stream, response);
}

fn invalid_route() -> Response {
    Response::new(
        400,
        "Valid routes: \n\
        POST /upload - Upload a file for analysis\n\
        GET /stats - Show statistics"
            .to_string(),
    )
}

#[derive(Eq, PartialEq)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub fn method_handler(method: &str) -> HttpMethod {
    match method {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        _ => HttpMethod::GET, //Default => Get
    }
}

fn get_request_line(buf_reader: BufReader<&TcpStream>) -> Vec<String> {
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
        let method = method_handler(method_string);
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

fn write_response(stream: &mut TcpStream, response: Response) {
    let _ = &stream.write(response.to_string().as_bytes()).unwrap();
}
