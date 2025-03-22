use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Div;
use std::thread;

pub fn find_sequence_in_file(
    buf_reader: BufReader<File>,
    pattern: &String,
) -> Vec<String> {

    buf_reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .filter(|line| line.contains(pattern))
        .collect::<Vec<String>>()

}

pub fn find_sequence_in_file_per_chunk(
    buf_reader: BufReader<File>,
    pattern: &String,
    chunk_size: i32
) -> Vec<String> {

    let lines: Vec<String> = buf_reader.lines().map(|line| line.unwrap()).collect();

    let chunks: i32 = (lines.len() as i32).div(chunk_size);

    let mut threads = vec![];

    for chunk in 0..chunks {
        let pattern_copy = pattern.clone();
        let lines_copy = lines.clone();
        let handle = thread::spawn(move || {
            lines_copy[(chunk_size * chunk) as usize..(chunk_size * (chunk + 1)) as usize]
                .iter()
                .filter(|line| line.contains(&pattern_copy))
                .cloned()
                .collect::<Vec<String>>()
        });
        threads.push(handle);
    }

    let mut result = vec![];
    for thread in threads {
        result.extend(thread.join().unwrap());
    }

    result
}
