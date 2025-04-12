use crate::server::{Response, Server};
use grep_lib::sequence_searcher::find_sequence_in_file;
use std::fs::File;
use std::io::BufReader;
use std::net::TcpStream;

pub fn analyze_logs(search_term: String, buf_reader: BufReader<File>) -> usize {
    find_sequence_in_file(buf_reader, &search_term).len()
}

pub fn get_stats(server: &Server) -> Response {
    Response::new(200, "".to_string())
}

pub fn process_file(stream: TcpStream, server: &Server) -> Response {
    Response::new(200, "".to_string())
}
