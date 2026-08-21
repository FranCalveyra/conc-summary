use crate::file_processor::get_file_reader;
use crate::search_types::SearchType;
use crate::sequence_searcher::{find_sequence_in_file, find_sequence_in_file_per_chunk};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;
use std::thread::JoinHandle;

const CHUNK_SIZE: i32 = 20000; // Line amount

pub trait Grep {
    fn find_sequence_in_file(&self, pattern: &String, file_paths: Vec<String>) -> Vec<String>;
}

impl Grep for SearchType {
    fn find_sequence_in_file(&self, pattern: &String, file_paths: Vec<String>) -> Vec<String> {
        match self {
            SearchType::Sequential => sequential_grep(&pattern, file_paths),
            SearchType::Concurrent => concurrent_grep(&pattern, file_paths),
            SearchType::ChunkConcurrent => chunk_concurrent_grep(&pattern, file_paths),
        }
    }
}

pub fn grep(search_type: SearchType, search_term: String, file_paths: Vec<String>) {
    let lines = search_type.find_sequence_in_file(&search_term, file_paths);

    lines.iter().for_each(|line| println!("{}", line))
}

fn sequential_grep(search_term: &String, file_paths: Vec<String>) -> Vec<String> {
    file_paths
        .into_iter()
        .map(|path| get_file_reader(path.to_string()).unwrap())
        .map(|reader| find_sequence_in_file(&get_vector(reader), &search_term))
        .flatten()
        .collect()
}

fn get_vector(reader: BufReader<File>) -> Vec<String> {
    reader.lines().filter_map(|line| line.ok()).collect()
}

fn concurrent_grep(search_term: &String, file_paths: Vec<String>) -> Vec<String> {
    let mut handles: Vec<JoinHandle<Vec<String>>> = Vec::new();
    file_paths
        .into_iter()
        .map(|path| {
            let value = search_term.clone();
            thread::spawn(move || {
                find_sequence_in_file(
                    &get_vector(get_file_reader(path.to_string()).unwrap()),
                    &value,
                )
            })
        })
        .for_each(|handle| handles.push(handle));
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .flatten()
        .collect()
}

fn chunk_concurrent_grep(search_term: &String, file_paths: Vec<String>) -> Vec<String> {
    let mut handles: Vec<JoinHandle<Vec<String>>> = Vec::new();
    file_paths
        .into_iter()
        .map(|path| {
            let value = search_term.clone();
            thread::spawn(move || {
                find_sequence_in_file_per_chunk(
                    &get_vector(get_file_reader(path.to_string()).unwrap()),
                    &value,
                    CHUNK_SIZE,
                )
            })
        })
        .for_each(|handle| handles.push(handle));
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .flatten()
        .collect()
}
