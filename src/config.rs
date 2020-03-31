use std::fmt;


pub struct Config<'a> {
    pub length: usize,
    pub amount: Option<usize>,
    pub file: Option<Vec<&'a str>>,
}

impl Config<'_> {
    pub fn new(raw_args: &[String]) -> Result<Config, &'static str> {
        let args = &raw_args[1..];

        let mut length: usize = 0;
        let mut amount: Option<usize> = None;
        let mut file: Option<Vec<&str>> = None;

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
                "-f" | "--file" => file = Some(vec!["test", "a", "b"]),
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

impl fmt::Debug for Config<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
        .field("length", &self.length)
        .field("ammount", &self.amount)
        .finish()
    }
}