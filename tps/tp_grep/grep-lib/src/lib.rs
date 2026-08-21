pub mod file_processor;
pub mod sequence_searcher;
pub mod errors;
pub mod search_types;
pub mod grep;

#[cfg(test)]
mod tests {
    // TODO: add more tests

    use crate::file_processor;
    use crate::sequence_searcher;
    #[test]
    fn it_works() {
        let file_reader = file_processor::get_file_reader("resources/poem.txt".to_string());
        assert_eq!(true, file_reader.is_ok());

        let lines = sequence_searcher::find_sequence_in_file(
            file_reader.unwrap(),
            &"How".to_string(),
        );
        assert_eq!("How dreary to be somebody!", &lines[0]);
        assert_eq!("How public, like a frog", &lines[1]);
    }
}