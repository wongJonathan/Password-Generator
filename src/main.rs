use std::env;
use std::process;

use password_generator::Config;
use password_generator::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Trouble with arugments: {}", err);
        println!("Usage: ./password_generator length");
        println!("\tlength: Length of password (integer)");
        process::exit(1);
    });
    run(config).expect("Error creating password");
}
