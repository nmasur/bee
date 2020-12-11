use std::io;
use std::io::Write;

fn main() {
    // Add dictionary of words
    let all_words = include_str!("words.list");

    // Gather letters as input
    let letters = read_letters("All letters");
    let required_letters = read_letters("Required letters");

    // Make sure required letters are in all letters and deduplicate
    let letters = combine_letters(letters, &required_letters);

    eprintln!(
        "Letters are: {} (must contain {})",
        letters, required_letters,
    );

    match_words(all_words, letters, required_letters);
}

fn read_letters(message: &str) -> String {
    let mut letters = String::new();
    eprint!("{}: ", message);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut letters)
        .expect("Failed to read line");
    let letters = letters.trim();
    let mut letters: Vec<char> = letters.chars().collect();
    letters.dedup();
    letters.into_iter().collect()
}

fn combine_letters(letters: String, required_letters: &str) -> String {
    let mut combined_letters = format!("{}{}", letters, required_letters)
        .chars()
        .collect::<Vec<char>>();
    combined_letters.dedup();
    combined_letters.into_iter().collect()
}

fn match_words(all_words: &str, letters: String, required_letters: String) {
    // Match words
    let mut matched_words = Vec::new();
    for word in all_words.lines() {
        let mut word_matches = true;

        // Words must be longer than three characters
        if word.chars().count() < 4 {
            continue;
        }

        // Words must only contain necessary letters
        for letter in word.chars() {
            if !letters.contains(letter) {
                word_matches = false;
                break;
            }
        }

        // Words must contain required letter
        for letter in required_letters.chars() {
            if letter == '\n' {
                continue;
            }
            if !word.contains(letter) {
                word_matches = false;
                break;
            }
        }

        // Add successful match to list
        if word_matches {
            matched_words.push(word);
        }
    }

    eprintln!("Found {} matching words!", matched_words.len());

    for word in matched_words {
        println!("{}", word);
    }
}
