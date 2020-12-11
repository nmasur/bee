use std::io;
use std::io::Write;

static WORDS_FILE: &'static str = include_str!("words.list");

fn main() {
    // Gather letters as input
    let mut letters = String::new();
    print!("All letters: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut letters)
        .expect("Failed to read line");

    let mut required_letters = String::new();
    print!("Required letters: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut required_letters)
        .expect("Failed to read line");

    println!(
        "Letters are: {} (must contain {})",
        letters.trim(),
        required_letters.trim()
    );

    // Match words
    let mut matched_words = Vec::new();
    for word in WORDS_FILE.lines() {
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

        if word_matches {
            matched_words.push(word);
        }
    }

    println!("Found {} matching words!", matched_words.len());

    for word in matched_words {
        println!("{}", word);
    }
}
