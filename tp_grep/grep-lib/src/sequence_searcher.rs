use std::ops::Div;
use std::thread;

pub fn find_sequence_in_file(buf_reader: &Vec<String>, pattern: &String) -> Vec<String> {
    buf_reader
        .clone()
        .into_iter()
        .flat_map(|line: String| line.lines().map(|l| l.to_string()).collect::<Vec<String>>())
        .map(|line| line.to_lowercase())
        .filter(|cured_line| cured_line.contains(pattern))
        .collect::<Vec<String>>()
}

pub fn find_sequence_in_file_per_chunk(
    lines: &Vec<String>,
    pattern: &String,
    chunk_size: i32,
) -> Vec<String> {
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
