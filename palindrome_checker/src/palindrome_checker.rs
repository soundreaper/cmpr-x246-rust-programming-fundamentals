use std::io;

fn main() {
   
   //an array of functions
   let palindrome_checkers:[fn(s:&str)->bool; 2] = [is_pal_loop, is_pal_recursion];

   println!("This is a palindrome checker program.");
   println!("A palindrome is a word, phrase, or number that reads the same forward and backward.");
   println!("Strings \"Kayak\", \"race car\",\"161\" are all palindromes.");

   //add code

   loop {
      //add code to remove non-alphanumeric characters from a give string, ... 
     for f in palindrome_checkers.iter() {
         //add code
     }
     println!("=====")  
   }
   println!("Exiting the program.")
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
