use crate::request_handler::{get_request_line, parse_line};
use std::collections::HashMap;
use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;
use crate::http_method::HttpMethod;
use crate::response::Response;
use crate::server::Server;

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

fn get_stats(server: Arc<Server>) -> Response {
    let exceptions: i64 = server.clone().get_exceptions();
    let body = format!(
        "Total exceptions: {exceptions}\nFiles processed: {}\nPer file:{}\n",
        server.file_stats.try_read().unwrap().len(),
        format_map(&server.file_stats.try_read().unwrap())
    );
    Response::new(200, body)
}

fn process_file(stream: &mut TcpStream, server: Arc<Server>) -> Response {
    // TODO: check border cases (server is full, file not specified)
    Response::new(200, "".to_string())
}

fn format_map(map: &HashMap<String, usize>) -> String {
    let mut formatted = String::from("{");

    let entries = map
        .iter()
        .map(|(key, value)| format!("\"{}\": {}", key, value))
        .collect::<Vec<_>>();

    formatted.push_str(&entries.join(", "));
    formatted.push('}');

    formatted
}

fn write_response(stream: &mut TcpStream, response: Response) {
    let _ = &stream.write(response.to_string().as_bytes()).unwrap();
}
