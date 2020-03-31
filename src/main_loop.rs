use std::error::Error;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use rand::distributions::Alphanumeric;
use std::process;

use crate::config::Config;

pub fn start(args: &Vec<String>) -> Result<(), Box<dyn Error>>  {
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Trouble with arugments: {}", err);
        println!("Usage: ./password_generator [-a amount] length");
        println!("\tlength: Length of password (integer)");
        println!("\tamount: Number of passwords (integer)");
        process::exit(1);
    });

    let passwords = run(config);

    for password in passwords {
        println!("{}\n", password);
    }

    Ok(())
}

fn run(config: Config) -> Vec<String> {
    let mut passwords: Vec<String> = vec![];
    let mut amount: usize = 1;

    if config.amount.is_some() {
        amount = config.amount.unwrap();
    }

    for _ in 0..amount {
        passwords.push(generate_password(config.length, &config.file));
    }

    passwords
}

pub fn generate_password(length: usize, file: &Option<Vec<String>>) -> String {
    let mut output = String::from("");
    match file {
        Some(file_words) => 
            for _ in 0..length {
                output.push_str(file_words.choose(&mut rand::thread_rng()).unwrap());
                output.push_str(" ");
            },
        None => 
        output = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect()
    };

    output.trim().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_only_once() {
        let config = Config {
            length: 4,
            amount: None,
            file: None
        };

        let results = run(config);

        assert_eq!(results.len(), 1);

    }

    #[test]
    fn runs_correct_number_of_times() {
        let config = Config {
            length: 4,
            amount: Some(3),
            file: None
        };

        let results = run(config);

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn runs_with_file() {
        let config = Config {
            length: 3,
            amount: None,
            file: Some(vec![String::from("test")])
        };
        let expected = vec![String::from("test test test")];

        let results = run(config);

        assert_eq!(results, expected);
    }
}