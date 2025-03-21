use crate::errors::Errors;
use crate::leibniz_adder::get_leibniz_term;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

pub fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // Silliest approach (?
        thread::spawn(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let lines = get_request_line(buf_reader);

    let line = &lines[0];

    let current_time = Instant::now(); // Time before processing

    let leibniz_term = parse_line(&line);

    let elapsed_time = current_time.elapsed(); // Processing duration time

    if leibniz_term.is_err() {
        let mut error_message: String = String::new();
        if let Some(err) = leibniz_term.err() {
            println!("Got an error");
            error_message = err.to_string();
        }

        &stream.write_all(error_message.as_bytes()).unwrap();
        return;
    }

    let unwrapped_term = leibniz_term.unwrap();

    write_response(&mut stream, elapsed_time, unwrapped_term);
}

fn write_response(stream: &mut TcpStream, elapsed_time: Duration, unwrapped_term: f64) {
    let response_body = build_response(unwrapped_term, elapsed_time);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    &stream.write(response.as_bytes()).unwrap();
}

fn build_response(leibniz_term: f64, elapsed_time: Duration) -> String {
    format!(
        "Calculated PI value: {} \n Elapsed time in milliseconds: {}",
        leibniz_term.to_string(),
        elapsed_time.as_millis()
    )
}

fn parse_line(request_line: &String) -> Result<f64, Errors> {
    println!("Request Line: {}", request_line);

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() > 1 {
        let path = parts[1];

        if let Some(captured_params) = extract_path_param(path, "/pi/:i") {
            if let Some(term) = captured_params.get("i") {
                println!("Extracted Term: {}", &term);

                let parsed_number = term.parse::<i32>();
                if parsed_number.is_err() {
                    return Err(Errors::ParseNumberError);
                }

                let number = parsed_number.unwrap();
                if number > i32::MAX {
                    return Err(Errors::OverflowError);
                }

                let leibniz_term = get_leibniz_term(0, number);
                return Ok(leibniz_term);
            }
        }
    }
    Err(Errors::MessageError(String::from("Unable to read line")))
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

fn get_request_line(buf_reader: BufReader<&TcpStream>) -> Vec<String> {
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    http_request
}
