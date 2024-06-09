use std::collections::HashMap;
use std::io;

// declaring constants at the top
// list of movie titles as a constant slice of string slices
const MOVIE_TITLES: &[&str] = &[
    "Love Actually", "STAR WARS", "From Russia With love", "Dr. Strangelove",
    "Bourne Ultimatum", "The fault in our stars", "Bourne supremacy", "A star is born",
    "Starsky and Hutch", "Star Trek", "Lover's Paradise", "A Christmas Star",
    "Chitty Chitty Bang Bang", "Ernest Saves Christmas", "A CHRISTMAS CAROL",
    "The Muppet Christmas Carol", "White Christmas", "Fahrenheit 451"
];

// ANSI escape codes for text coloring
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

// Unicode strings for emojis
const SMILEY_FACE: &str = "\u{1F642}";
const SAD_FACE: &str = "\u{1F61E}";
const THINKING_FACE: &str = "\u{1F914}";

fn main() {
    loop {
        // promp the user for search terms
        let input = promp_input();
        // remove the whitespace from the input and check if its "E" to quit the program
        let input_trimmed = input.trim();
        if input_trimmed.eq_ignore_ascii_case("E") {
            println!("\nExiting the program.");
            break;
        }

        // check if the input is empty or whitespace only
        if input_trimmed.is_empty() {
            println!("Your input \"{}\" had no words to search for.{}", input, THINKING_FACE);
            continue;
        }

        // split the input into words and collect them into vector
        let words: Vec<&str> = input_trimmed.split_whitespace().collect();
        
        // use a HashMap to collect unique words since we don't care if a search term is inputted twice
        let mut unique_words = HashMap::new();
        for &word in &words {
            let word_lower = word.to_lowercase();
            unique_words.entry(word_lower).or_insert(0);
        }

        // use a HashMap to count occurrences of each word in the movie titles
        let mut word_counts = HashMap::new();

        for &title in MOVIE_TITLES {
            // convert movie titles to lowercase
            let title_lower = title.to_lowercase();
            // split title into words
            let title_words: Vec<&str> = title_lower.split_whitespace().collect();
            for word in unique_words.keys() {
                if title_words.contains(&word.as_str()) {
                    *word_counts.entry(word.clone()).or_insert(0) += 1;
                }
            }
        }

        // collect word counts into a vector and sort alphabetically
        let mut word_counts_vec: Vec<(&String, &usize)> = word_counts.iter().collect();
        word_counts_vec.sort();

        // if the word count vector is empty, none of the search terms were found
        if word_counts.is_empty() {
            println!("\nSorry, none of the words in your input \"{}\" were found in the movie titles.{}", input, SAD_FACE);
        } else {
            // prints a new line after user input
            println!("");
            // iterate through vector and output each search term that matched and a red asterisk for the number of matches
            for (word, count) in word_counts_vec {
                println!("{:<10} {}{}{}", word, RED, "*".repeat(*count), RESET);
            }
            // can't forget the smiley face :)
            println!("{}", SMILEY_FACE);
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