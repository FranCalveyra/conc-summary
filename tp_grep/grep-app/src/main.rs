use grep_lib::search_types::SearchType;
use grep_lib::grep::grep;

use std::env;
use std::time::Instant;

fn main () {
    let current_time = Instant::now();
    terminal_grep();
    println!("\nElapsed time in millis: {}", current_time.elapsed().as_millis());
}

fn terminal_grep() {
    let args: Vec<String> = env::args().collect();
    // let args: Vec<String> = Vec::from(&["", "c-chunk", "man", "resources/bible.txt"].map(|s| s.to_string()));
    if args.len() < 4 {
        eprintln!("Usage: cargo run -- <search_type> <search_term> <file1> <file2> ...");
        std::process::exit(1);
    }

    let search_type = get_search_type(&args[1]);
    let search_term = &args[2];
    let files = &args[3..];

    grep(search_type, search_term.to_string(), files.to_vec());
}

fn get_search_type(search_type: &String) -> SearchType {
    match search_type.as_str() {
        "seq" => SearchType::Sequential,
        "conc" => SearchType::Concurrent,
        "c-chunk" => SearchType::ChunkConcurrent,
        _ => {
            eprintln!("Invalid search type: {}", search_type);
            std::process::exit(1);
        }
    }
}