use std::collections::HashMap;
use std::io;

const MOVIE_TITLES: &[&str] = &[
    "Love Actually", "STAR WARS", "From Russia With love", "Dr. Strangelove",
    "Bourne Ultimatum", "The fault in our stars", "Bourne supremacy", "A star is born",
    "Starsky and Hutch", "Star Trek", "Lover's Paradise", "A Christmas Star",
    "Chitty Chitty Bang Bang", "Ernest Saves Christmas", "A CHRISTMAS CAROL",
    "The Muppet Christmas Carol", "White Christmas", "Fahrenheit 451"
];

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const SMILEY_FACE: &str = "\u{1F642}";
const SAD_FACE: &str = "\u{1F61E}";
const THINKING_FACE: &str = "\u{1F914}";

fn main() {
    loop {
        let input = promp_input();
        let input_trimmed = input.trim();
        
        if input_trimmed.eq_ignore_ascii_case("E") {
            println!("\nExiting the program.");
            break;
        }

        if input_trimmed.is_empty() {
            println!("Your input \"{}\" had no words to search for.{}", input, THINKING_FACE);
            continue;
        }
    }
}

fn promp_input() -> String {
    println!("\nEnter your search words(s) separated with one or more spaces/tabs, or E (to exit), and hit the Enter key:");
    
    // we'll read in the input and throw an error if we couldn't read the line
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("[ERROR] Failed to read line.");
    // this let's us keep the original string that was read in without the newline character from .read_line()
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);
    input.to_string()
}