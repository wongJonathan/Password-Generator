use std::fmt;

use crate::file_reader;

pub struct Config{
    pub length: usize,
    pub amount: Option<usize>,
    pub file: Option<Vec<String>>,
}

impl Config {
    pub fn new(raw_args: &[String]) -> Result<Config, &'static str> {
        let args = &raw_args[1..];

        let mut length: usize = 0;
        let mut amount: Option<usize> = None;
        let mut file: Option<Vec<String>> = None;

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
                "-f" | "--file" =>
                    match iter.next() {
                        Some(filename) => 
                            file = match file_reader::read_file(filename) {
                                Ok(file_content) => 
                                    Some(file_reader::create_word_list(file_content)),
                                _ => None
                            },
                        _ => return Err("Missing value for file name")
                    },
                command_arg => 
                    length = match command_arg.parse::<usize>() {
                        Ok(x) => x,
                        _ => return Err("length.\nNot a valid integer")
                    }
            }
        }

        return Ok(Config { length, amount, file });
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