use std::env;

mod config;
mod main_loop;
mod file_reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    main_loop::start(&args).expect("Error creating password");
}
