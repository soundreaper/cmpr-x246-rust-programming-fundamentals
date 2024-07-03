use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    match validate_phone_list_file() {
        Ok(file_path) => {
            process_phone_numbers(file_path);
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

// Validate the phone list file
fn validate_phone_list_file() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("[ERROR] Expected one argument got {}.", args.len() - 1));
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("[ERROR] Invalid path, \"{}\" not found.", file_path));
    }

    if path.is_dir() {
        return Err(format!("[ERROR] Invalid file, \"{}\" is not a file.", file_path));
    }

    match fs::File::open(path) {
        Ok(file) => {
            let metadata = fs::metadata(path).unwrap();
            if metadata.len() == 0 {
                return Err(format!("[ERROR] File \"{}\" is empty.", file_path));
            }

            let reader = io::BufReader::new(file);
            if reader.lines().next().is_none() {
                return Err(format!("[ERROR] File \"{}\" is empty.", file_path));
            }
        }
        Err(e) => {
            return Err(format!("[ERROR] {}", e));
        }
    }

    Ok(file_path.clone())
}

// Process the phone numbers
fn process_phone_numbers(file_path: String) {
    let path = Path::new(&file_path);
    let file = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    // Updated regex to handle leading/trailing whitespaces correctly
    let phone_re = Regex::new(r"^\s*(?:(\(\d{3}\) \d{3}-\d{4})|(\d{3}-\d{3}-\d{4})|(\d{3}\.\d{3}\.\d{4})|(\d{3} \d{3} \d{4})|(\d{10}))\s*$").unwrap();
    let toll_free_re = Regex::new(r"^\s*(?:\(?8(?:00|33|44|55|66|77|88)\)?[-.\s]?\d{3}[-.\s]?\d{4})\s*$").unwrap();

    let mut valid_numbers = Vec::new();
    let mut invalid_numbers = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }

        if let Some(caps) = phone_re.captures(&line) {
            let phone_number = caps.get(0).unwrap().as_str().to_string().trim().to_string();

            match validate_phone_number(&phone_number) {
                Ok(_) => {
                    let cleaned_number = clean_phone_number(&phone_number);
                    if toll_free_re.is_match(&phone_number) {
                        valid_numbers.push(format!("{} \x1b[36mis a toll-free phone number\x1b[0m", phone_number));
                    } else {
                        valid_numbers.push(phone_number.clone());
                    }

                    let international_number = format!("+1{}", cleaned_number);
                    valid_numbers.push(format!("{} international format: {}", "*".repeat(phone_number.len()), international_number));
                }
                Err(e) => {
                    invalid_numbers.push(format!("[ERROR] Phone number {} {}", phone_number.trim(), e));
                }
            }
        } else {
            invalid_numbers.push(format!("[ERROR] Phone number {} has invalid format/digit(s)", line.trim()));
        }
    }

    // Output the valid phone numbers
    println!("The output for valid phone numbers is:");
    for number in valid_numbers {
        println!("{}", number);
    }

    // Output the invalid phone numbers
    println!("\n\nThe output for invalid phone numbers is:");
    for number in invalid_numbers {
        if number.contains("Exchange Code has '1' in both 2nd & 3rd digits") {
            eprintln!("\x1b[34m{}\x1b[0m", number);
        } else {
            eprintln!("{}", number);
        }
    }
}

// Validate the phone number according to the specified rules
fn validate_phone_number(phone_number: &str) -> Result<(), String> {
    let clean_number = clean_phone_number(phone_number);

    if clean_number.len() != 10 {
        return Err("has invalid length".to_string());
    }

    let area_code = &clean_number[0..3];
    let exchange_code = &clean_number[3..6];

    let area_code_re = Regex::new(r"^[2-9][0-8][0-9]$").unwrap();

    if !area_code_re.is_match(area_code) {
        return Err("has invalid format/digit(s)".to_string());
    }

    let exchange_code_re = Regex::new(r"^[2-9][0-9][0-9]$").unwrap();

    if !exchange_code_re.is_match(exchange_code) {
        return Err("has invalid format/digit(s)".to_string());
    }

    // Manually check that the 2nd and 3rd digits of the exchange code are not both '1'
    if &exchange_code[1..3] == "11" {
        return Err("Exchange Code has '1' in both 2nd & 3rd digits".to_string());
    }

    Ok(())
}

// Clean the phone number by removing non-digit characters
fn clean_phone_number(phone_number: &str) -> String {
    phone_number.chars().filter(|c| c.is_digit(10)).collect()
}