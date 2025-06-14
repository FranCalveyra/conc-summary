use grep_lib::sequence_searcher::find_sequence_in_file;

pub fn analyze_logs(search_term: String, buf_reader: &Vec<String>) -> usize {
    find_sequence_in_file(buf_reader, &search_term).len()
}
