mod errors;
mod file_processor;
mod search_types;
mod sequence_searcher;

#[cfg(test)]
mod tests {
    use crate::file_processor::get_file_reader;
    use crate::search_types::SearchType;
    use crate::sequence_searcher::grep_by_search_type;

    #[test]
    fn it_works() {
        let file_reader = get_file_reader("resources/poem.txt".to_string());
        assert_eq!(true, file_reader.is_ok());

        let lines = grep_by_search_type(
            file_reader.unwrap(),
            &"How".to_string(),
            SearchType::Sequential,
        );
        assert_eq!("How dreary to be somebody!", &lines[0]);
        assert_eq!("How public, like a frog", &lines[1]);
    }
}
