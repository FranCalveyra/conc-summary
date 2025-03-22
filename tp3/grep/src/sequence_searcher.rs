use crate::search_types::SearchType;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_sequence_in_file(
    buf_reader: BufReader<File>,
    pattern: &String,
) -> Vec<String> {

    sequential_grep(buf_reader, pattern)

}

fn sequential_grep(buf_reader: BufReader<File>, pattern: &String) -> Vec<String> {
    buf_reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .filter(|line| line.contains(pattern))
        .collect::<Vec<String>>()
}
