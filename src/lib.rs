use std::fmt;
use std::error::Error;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub struct Config {
    pub length: usize,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough args");
        }

        let length: usize = match args[1].parse::<usize>() {
            Ok(x) => x,
            _ => return Err("length.\nNot a valid integer")
        };


        return Ok(Config { length });
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
        .field("length", &self.length)
        .finish()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let passwords = generate_password(config.length);

    for password in passwords {
        println!("{}", password);
    }

    Ok(())
}

pub fn generate_password(length: usize) -> Vec<String> {
    let s: String = thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .collect();
    vec![s]
}