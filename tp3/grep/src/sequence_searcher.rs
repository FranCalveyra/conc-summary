use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Div;
use std::ptr::read;
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

    // let mut chunks:i32 = (lines.len() as i32).div(chunk_size);

    // println!("Initial chunks: {chunks}");
    //
    // let mut result: Vec<Vec<String>> = Vec::new();
    //
    // while chunks>0{
    //     thread::spawn(|| {
    //         result.push(lines[ (chunk_size*(chunks-1)) as usize..(chunk_size*chunks) as usize]
    //             .iter()
    //             .filter(|line| line.contains(pattern))
    //             .cloned()
    //             .collect::<Vec<String>>());
    //     });
    //
    //
    //     chunks -=1;
    //
    //     println!("Current chunks: {chunks}");
    // }
    //
    // result.into_iter().flatten().collect()

    Vec::new()
}
