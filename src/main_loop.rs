use std::error::Error;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::config::Config;



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut passwords: Vec<String> = vec![];

    match config.amount {
        Some(amount) => 
            for _ in 0..amount {
                passwords.push(generate_password(config.length));
            },
        None => passwords.push(generate_password(config.length))
    }

    for password in passwords {
        println!("{}\n", password);
    }

    Ok(())
}

pub fn generate_password(length: usize) -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .collect()
}
