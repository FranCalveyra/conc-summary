use crate::server::{Response, Server};
use grep_lib::sequence_searcher::find_sequence_in_file;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::net::TcpStream;
use std::sync::Arc;

pub fn analyze_logs(search_term: String, buf_reader: BufReader<File>) -> usize {
    find_sequence_in_file(buf_reader, &search_term).len()
}

pub fn get_stats(server: Arc<Server>) -> Response {
    let exceptions: i64 = server.clone().get_exceptions();
    let body = format!(
        "Total exceptions: {exceptions}\nFiles processed: {}\nPer file:{}\n",
        server.file_stats.try_read().unwrap().len(),
        format_map(&server.file_stats.try_read().unwrap())
    );
    Response::new(200, body)
}

pub fn process_file(stream: &mut TcpStream, server: Arc<Server>) -> Response {
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
