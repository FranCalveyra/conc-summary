mod errors;
mod file_processor;
mod search_types;
mod sequence_searcher;
mod grep;

#[cfg(test)]
mod tests {
    use crate::file_processor::get_file_reader;
    use crate::sequence_searcher::find_sequence_in_file;

    #[test]
    fn it_works() {
        let file_reader = get_file_reader("resources/poem.txt".to_string());
        assert_eq!(true, file_reader.is_ok());

        let lines = find_sequence_in_file(
            file_reader.unwrap(),
            &"How".to_string(),
        );
        assert_eq!("How dreary to be somebody!", &lines[0]);
        assert_eq!("How public, like a frog", &lines[1]);
    }
}
