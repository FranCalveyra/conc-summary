use grep_lib::sequence_searcher::find_sequence_in_file;
use std::fs::File;
use std::io::BufReader;

pub fn analyze_logs(search_term: String, buf_reader: BufReader<File>) -> usize {
    find_sequence_in_file(buf_reader, &search_term).len()
}
