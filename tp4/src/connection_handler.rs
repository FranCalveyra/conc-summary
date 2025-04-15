use crate::http_method::HttpMethod;
use crate::request::{ContentType, Request};
use crate::response::Response;
use crate::server::Server;
use grep_lib::sequence_searcher::find_sequence_in_file;
use std::collections::HashMap;
use std::io::{BufReader, Write};
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
    let body = format!(
        "Total exceptions: {exceptions}\nFiles processed: {}\nPer file:{}\n",
        server.file_stats.try_read().unwrap().len(),
        format_map(&server.file_stats.try_read().unwrap())
    );
    Response::new(200, body)
}

fn process_file(request: Request, server: Arc<Server>) -> Response {
    // TODO: check border cases (server is full, file not specified)
    if request.content_type != ContentType::MultipartFormData {
        return Response::from_status(400);
    }

    // Should acquire, or explode from acquiring
    let acquire_result = server.file_semaphore.try_acquire();

    if acquire_result.is_err() { // Server is full
        return Response::from_status(429);
    }

    let mut files = server.file_stats.write().unwrap();

    let file = BufReader::new(&request.body);
    files.insert(
        get_file_name(request.headers),
        find_sequence_in_file(*file, &"exception".to_string()),
    );

    Response::new(200, "".to_string())
}

fn get_file_name(headers: String) -> String {
    todo!()
}

fn format_map(map: &HashMap<String, usize>) -> String {
    let mut formatted = String::from("{");

    let entries = map
        .iter()
        .map(|(key, value)| format!("\"{}\": {}", key, value))
        .collect::<Vec<_>>();

    formatted.push_str(&entries.join(", "));
    formatted = (&formatted[..formatted.len() - 2]).to_string(); // removes last comma and space
    formatted.push('}');

    formatted
}

fn write_response(stream: &mut TcpStream, response: Response) {
    let _ = &stream.write(response.to_string().as_bytes()).unwrap();
}
