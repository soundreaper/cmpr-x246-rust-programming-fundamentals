use clap::{Arg, ArgAction, Command};
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

// Constants for version and app name
const VERSION: &str = "1.0.0";
const APP_NAME: &str = "cwl";

// Entry point of the program
fn main() {
    // Setting up the command-line argument parser
    let matches = Command::new(APP_NAME)
        .version(VERSION)
        .about("Count lines, words, and characters in files or directories")
        .arg(Arg::new("path")
            .required(true)
            .help("Path to a file or a directory"))
        .arg(Arg::new("chars")
            .short('c')
            .long("chars")
            .action(ArgAction::SetTrue)
            .help("Get the character count"))
        .arg(Arg::new("words")
            .short('w')
            .long("words")
            .action(ArgAction::SetTrue)
            .help("Get the word count"))
        .arg(Arg::new("lines")
            .short('l')
            .long("lines")
            .action(ArgAction::SetTrue)
            .help("Get the line count"))
        .get_matches();

    // Retrieve the path argument and the flags for counting chars, words, and lines
    let path = matches.get_one::<String>("path").unwrap();
    let count_chars = *matches.get_one::<bool>("chars").unwrap_or(&false) || (!matches.contains_id("chars") && !matches.contains_id("words") && !matches.contains_id("lines"));
    let count_words = *matches.get_one::<bool>("words").unwrap_or(&false) || (!matches.contains_id("chars") && !matches.contains_id("words") && !matches.contains_id("lines"));
    let count_lines = *matches.get_one::<bool>("lines").unwrap_or(&false) || (!matches.contains_id("chars") && !matches.contains_id("words") && !matches.contains_id("lines"));

    // Process the given path
    if let Err(e) = process_path(Path::new(path), count_chars, count_words, count_lines) {
        eprintln!("[ERROR] {}", e);
    }
}

// Function to process the given path (file or directory)
fn process_path(path: &Path, count_chars: bool, count_words: bool, count_lines: bool) -> io::Result<()> {
    if path.is_file() {
        process_file(path, count_chars, count_words, count_lines)?;
    } else if path.is_dir() {
        process_directory(path, count_chars, count_words, count_lines)?;
    } else {
        eprintln!("[ERROR] Invalid path \"{}\"", path.display());
    }
    Ok(())
}

// Function to process a single file
fn process_file(path: &Path, count_chars: bool, count_words: bool, count_lines: bool) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    if metadata.len() == 0 {
        println!("File \"{}\" is empty", path.file_name().unwrap().to_string_lossy());
        return Ok(());
    }

    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("File name:\"{}\", error: {}", path.file_name().unwrap().to_string_lossy(), e);
            return Ok(());
        }
    };

    let reader = io::BufReader::new(file);

    let mut char_count = 0;
    let mut word_count = 0;
    let mut line_count = 0;

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("File name:\"{}\", error: {}", path.file_name().unwrap().to_string_lossy(), e);
                return Ok(());
            }
        };

        if count_lines {
            line_count += 1;
        }
        if count_chars {
            char_count += line.chars().count();
        }
        if count_words {
            word_count += line.split_whitespace().count();
        }
    }

    print!("File name:\"{}\"", path.file_name().unwrap().to_string_lossy());
    if count_chars {
        print!(", char count:{}", char_count);
    }
    if count_words {
        print!(", word count:{}", word_count);
    }
    if count_lines {
        print!(", line count:{}", line_count);
    }
    println!(",");
    Ok(())
}

// Function to process a directory
fn process_directory(path: &Path, count_chars: bool, count_words: bool, count_lines: bool) -> io::Result<()> {
    let mut is_empty = true;
    let mut has_subdirectories = false;

    for entry in WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            if is_empty {
                println!("\nDirectory name: \"{}\"", path.display());
                is_empty = false;
            }
            process_file(entry_path, count_chars, count_words, count_lines)?;
        } else if entry_path.is_dir() {
            has_subdirectories = true;
            process_directory(entry_path, count_chars, count_words, count_lines)?;
        }
    }

    if is_empty && !has_subdirectories {
        println!("\nDirectory \"{}\" is empty", path.display());
    }
    Ok(())
}