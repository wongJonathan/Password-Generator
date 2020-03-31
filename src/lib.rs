use std::fmt;
use std::error::Error;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub struct Config {
    pub length: usize,
    pub amount: Option<usize>,
}

impl Config {
    pub fn new(raw_args: &[String]) -> Result<Config, &'static str> {
        let args = &raw_args[1..];

        let mut length: usize = 0;
        let mut amount: Option<usize> = None;

        if args.len() < 1 {
            return Err("Not enough args");
        }

        let mut iter = IntoIterator::into_iter(args);

        while let Some(arg) = iter.next() {
            match &arg[..] {
                "-a" | "--amount" => 
                    match iter.next() {
                        Some(ammount_arg) =>
                            amount = match ammount_arg.parse::<usize>() {
                                Ok(val) => 
                                    Some(val),
                                _ => return Err("Invalid value for amount")
                            },
                        _ => return Err("Missing value for amount")
                    },
                command_arg => 
                    length = match command_arg.parse::<usize>() {
                        Ok(x) => x,
                        _ => return Err("length.\nNot a valid integer")
                    }
            }
        }

        return Ok(Config { length, amount });
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
        .field("length", &self.length)
        .field("ammount", &self.amount)
        .finish()
    }
}

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
        println!("{}", password);
    }

    Ok(())
}

pub fn generate_password(length: usize) -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .collect()
}