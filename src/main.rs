use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            let error_logs = extract_errors(text.as_str());

            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(error) => {
                    println!("Writing of errors.txt failed: {}", error);
                }
            }
        },
        Err(error) => println!("Failed to read file: {}", error),
    }

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    match validate_email(String::from("willy@meta.com")) {
        Ok(..) => println!("Email is valid"),
        Err(why_it_failed) => {
            println!("{}", why_it_failed)
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0"))
    } else {
        Ok(a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Email must have '@'"))
    }
}
