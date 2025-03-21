use crate::search_types::SearchType;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn grep_by_search_type(
    buf_reader: BufReader<File>,
    pattern: &String,
    search_type: SearchType,
) -> Vec<String> {
    // TODO: move this behaviour to a trait, using a struct

    match search_type {
        SearchType::Sequential => sequential_grep(buf_reader, pattern),
        SearchType::Concurrent => concurrent_grep(buf_reader, pattern),
        SearchType::ChunkConcurrent => chunk_concurrent_grep(buf_reader, pattern),
    }
}

fn chunk_concurrent_grep(buf_reader: BufReader<File>, pattern: &String) -> Vec<String> {
    todo!()
}

fn concurrent_grep(buf_reader: BufReader<File>, pattern: &String) -> Vec<String> {
    todo!()
}

fn sequential_grep(buf_reader: BufReader<File>, pattern: &String) -> Vec<String> {
    buf_reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .filter(|line| line.contains(pattern))
        .collect::<Vec<String>>()
}
