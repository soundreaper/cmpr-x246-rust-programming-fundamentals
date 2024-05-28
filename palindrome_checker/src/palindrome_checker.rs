use std::io;

fn main() {
    //an array of functions
    let palindrome_checkers:[fn(s:&str)->bool; 2] = [is_pal_loop, is_pal_recursion];

    // boilerplate at the top
    println!("This is a palindrome checker program.");
    println!("A palindrome is a word, phrase, or number that reads the same forward and backward.");
    println!("Strings \"Kayak\", \"race car\",\"161\" are all palindromes.");

    loop {
        // prompt user for string here
        println!("\nEnter a string, or E (to exit)");
        // we'll read in the input and throw an error if we couldn't read the line
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("[ERROR] Failed to read line.");
        // this let's us keep the original string that was read in without the newline character from .read_line()
        let len = input.trim_end_matches(&['\r', '\n'][..]).len();
        input.truncate(len);
        // we'll remove the whitespace from the input here and check if its "E" to quit the program
        let trimmed_input = input.trim();
        if trimmed_input.eq_ignore_ascii_case("E") {
            println!("\nExiting the program.");
            break;
        }
    
        // add code to remove non-alphanumeric characters from a given string
        let cleaned_input = remove_non_alphanumeric(trimmed_input);
        // if the input is empty then there were no alphanumeric characters
        if cleaned_input.is_empty() {
            println!("[WARNING]: string \"{}\" does not have any alphanumeric characters; ignored.", input);
            println!("=====");
            continue;
        }
        
        let mut is_palindrome = true;
        // run the cleaned input through both palindrome check functions 
        for f in palindrome_checkers.iter() {
            // if either of the functions reutrn false then we set is_palindrome to false and break
            if !f(&cleaned_input) {
                is_palindrome = false;
                break;
            }
        }

        // if is_palindrome is true then we print the entire string otherwise we check for palindromic words in the string
        if is_palindrome {
            println!("String \"{}\" is a palindrome.", input);
            println!("String \"{}\" is a palindrome.", input);
            println!("=====");    
        } else {
            let mut palindrome_words = String::new();
            // we filter out each word in the string using .split_whitespace()
            for word in trimmed_input.split_whitespace() {
                // we remove non-alphanumeric characters from a word
                let cleaned_word = remove_non_alphanumeric(word);
                // check to make sure there is a word
                if !cleaned_word.is_empty() {
                    for f in palindrome_checkers.iter() {
                        // if either functions return true, the word is a palindrome
                        if f(&cleaned_word) {
                            // this just adds a comma after each word assuming there are words in the string already
                            if !palindrome_words.is_empty() {
                                palindrome_words.push_str(", ");
                            }
                            // add the palindromic word
                            palindrome_words.push_str(&cleaned_word);
                            break;
                        }
                    }
                }
            }
            // if there are no words then the string is not a palindrome
            if palindrome_words.is_empty() {
                println!("String \"{}\" is NOT a palindrome.", input);
                println!("String \"{}\" is NOT a palindrome.", input);
                println!("=====");
            } else {
                // print the string and the palindromic words
                println!("String \"{}\" is NOT a palindrome, but has the following palindrome item(s) in it:\n  {}.", input, palindrome_words);
                println!("String \"{}\" is NOT a palindrome, but has the following palindrome item(s) in it:\n  {}.", input, palindrome_words);
                println!("=====");
            }
        }
    }
}

// function to remove non-alphanumeric characters and convert to lowercase
fn remove_non_alphanumeric(s: &str) -> String {
   let mut result = String::new();
   for c in s.chars() {
    if c.is_alphanumeric() {
        result.push(c.to_ascii_lowercase());
    }
   }
   result
}

// function to check if a string is a palindrome using a loop
fn is_pal_loop(s: &str) -> bool {
   let chars: &[u8] = s.as_bytes();
   let len = chars.len();
   for i in 0..len/2 {
    if chars[i] != chars[len - 1 - i] {
        return false;
    }
   }
   true
}

// function to check if a string is a palindrome using recursion
fn is_pal_recursion(s: &str) -> bool {
   fn helper(chars: &[u8], start: usize, end: usize) -> bool {
    if start >= end {
        return true;
    }
    if chars[start] != chars[end] {
        return false;
    }
    helper(chars, start + 1, end - 1)
   }

   let chars: &[u8] = s.as_bytes();
   helper(chars, 0, chars.len() - 1)
}
