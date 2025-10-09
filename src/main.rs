mod lexer;

use std::fs;

fn main() {
    let contents =
        fs::read_to_string("testfile.txt").expect("Should have been able to read the file");

    let delimiters = ['"', ' '].to_vec();

    let known_keywords = ["chaine".to_string(), "afficher".to_string()].to_vec();
    let known_operators = [":".to_string(), "+".to_string(), "-".to_string()].to_vec();

    let mut result = lexer::TokensLines::from_string(contents, delimiters);

    println!("{:?}", result.to_vec());

    result.merge_within_delimiters('"', false);

    println!("{:?}", result.to_vec());
}