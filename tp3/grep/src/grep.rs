use crate::search_types::SearchType;
use crate::file_processor::get_file_reader;
use crate::sequence_searcher::find_sequence_in_file;

pub fn grep (search_type: SearchType, search_term: String, file_paths: Vec<String>) {
    // TODO: differ behaviour with a trait

    /*
    match search_type{
        Sequential => read one by one
        Concurrent => use threads for each file
        C-Chunk => use N threads simultaneously (with N being chunk size)
    }
     */


    let lines: Vec<_> =
        file_paths.iter()
            .map(|path| get_file_reader(path.to_string()).unwrap())
            .map(|reader| find_sequence_in_file(reader, &search_term))
            .collect();
    for line in lines {
        for l in line {
            println!("{}", l);
        }
    }
}