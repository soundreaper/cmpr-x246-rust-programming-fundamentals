use std::io;

fn main() {
    // start by stating what the program does
    println!("This is a temperature conversion calculator.");
    println!("It converts from/to Celsius, Fahrenheit, and Kelvin.");

    loop {
        // prompt the user for temperature input here
        let temperature = prompt_temperature();
        // we check here if "E" is inputted so we can exit
        if temperature == "E" {
            println!("\n\nExiting the temperature conversion calculator.");
            break;
        }

        // parse the temperature into a floating-point number (it's read in as a string since we need to check for exit with "E")
        let temperature: f64 = match temperature.parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("[ERROR] Invalid temperature value.");
                // if the temperature isn't a number, throw an error BUT don't exit the program
                continue;
            }
        };

        // prompt the user for the FROM and TO units here; we pass in the input statement and a string with which unit we're asking for
        let from_unit = prompt_unit("Enter FROM temperature unit:[Cc/Ff/Kk]", "FROM");
        let to_unit = prompt_unit("Enter TO temperature unit:[Cc/Ff/Kk]", "TO");

        // pass in all the values we've gathered and convert, the output is an Option just in case there was some error in conversion
        let result = convert_temperature(temperature, &from_unit, &to_unit);
        match result {
            // if the return is a float value, great! we can display the before and after to 2 decimal places!
            Some(converted_temp) => {
                println!("\n{:.2}{} = {:.2}{}", temperature, from_unit, converted_temp, to_unit);
            }
            // as stated below, we already check for errors in user input so we should never run into this error but it's there just in case
            None => {
                println!("\n[ERROR] There was an error in this conversion.");
            }
        }
    }
}

fn prompt_temperature() -> String {
    println!("\n\nEnter a temperature (number) or E to exit:");

    let mut input = String::new();
    // we'll read in the input and throw an error if we couldn't read the line
    io::stdin().read_line(&mut input).expect("[ERROR] Failed to read line.");
    input.trim().to_string()
}

fn prompt_unit(prompt: &str, from_or_to: &str) -> String {
    // we loop here so that we keep prompting the user even if they make an invalid input
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("[ERROR] Failed to read line.");
        let input = input.trim().to_string();

        // if the user inputs a correct unit then we return the unit as uppercase since the output always uses uppercase for the unit
        if input == "C" || input == "c" || input == "F" || input == "f" || input == "K" || input == "k" {
            return input.to_uppercase();
        } else {
            println!("[ERROR] Invalid {} temperature unit {}", from_or_to, input);
            println!("Valid choices are:[Cc/Ff/Kk]\n");
        }
    }
}

fn convert_temperature(value: f64, from: &str, to: &str) -> Option<f64> {
    // we'll use a match function to figure out which conversion formula we need; since we check user input above, we should never run into the last case but it's there just in case
    match (from, to) {
        ("C", "F") => Some(value * 9.0 / 5.0 + 32.0),
        ("C", "K") => Some(value + 273.15),
        ("F", "C") => Some((value - 32.0) * 5.0 / 9.0),
        ("F", "K") => Some((value + 459.67) * 5.0 / 9.0),
        ("K", "F") => Some(value * 9.0 / 5.0 - 459.67),
        ("K", "C") => Some(value - 273.15),
        _ => None,
    }
}