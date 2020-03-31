use std::env;
use std::process;

use password_generator::Config;
use password_generator::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Trouble with arugments: {}", err);
        println!("Usage: ./password_generator [-a amount] length");
        println!("\tlength: Length of password (integer)");
        println!("\tamount: Number of passwords (integer)");
        process::exit(1);
    });
    println!("{:?}", config);
    run(config).expect("Error creating password");
}
