use crate::http_method::HttpMethod;
use crate::log_analyzer::analyze_logs;
use crate::request::{ContentType, Request};
use crate::response::Response;
use crate::server::Server;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;

pub fn handle_connection(mut stream: TcpStream, server: Arc<Server>) {
    let request = Request::from_stream(&mut stream);

    let response: Response = match (&request.method, request.uri.as_str()) {
        (HttpMethod::GET, "stats") => get_stats(server),
        (HttpMethod::POST, "upload") => process_file(request, server),
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
    let stats = server.file_stats.try_read().unwrap();

    let body = format!(
        "Total exceptions: {exceptions}\nFiles processed: {}\nPer file:{}\n",
        stats.len(),
        format_map(&stats)
    );
    Response::new(200, body)
}

fn process_file(request: Request, server: Arc<Server>) -> Response {
    if request.content_type != ContentType::MultipartFormData {
        return Response::from_status(400);
    }

    if request.body.is_empty() {
        return Response::new(400, "File not found or empty".to_string());
    }

    // Should acquire, or explode from acquiring
    let acquire_result = server.file_semaphore.try_acquire();

    if acquire_result.is_err() {
        // Server is full
        return Response::from_status(429);
    }

    let mut files = server.file_stats.write().unwrap();

    files.insert(
        get_file_name(&request.body.join("")),
        analyze_logs("exception".to_string(), &request.body),
    );

    // Everything went right
    Response::from_status(200)
}

fn get_file_name(headers: &str) -> String {
    headers
        .lines()
        .find(|line| line.to_lowercase().starts_with("content-disposition"))
        .and_then(|line| {
            line.split(';')
                .map(str::trim)
                .find(|part| part.to_lowercase().starts_with("filename="))
                .and_then(|part| {
                    part.splitn(2, '=')
                        .nth(1)
                        .map(str::trim)
                        .map(|val| val.trim_matches('"').to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                        .split('"')
                        .nth(0)
                })
        })
        .unwrap_or_else(|| "unknown").to_string()
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
