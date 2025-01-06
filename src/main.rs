use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);

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
