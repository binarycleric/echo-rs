use std::env;
use std::io;
use std::io::Read;

fn from_stdin() {
    let mut buffer = String::new();

    match io::stdin().read_to_string(&mut buffer) {
        Err(error) => println!("error: {}", error),
        _ => {
            println!("{}", buffer);
        }
    }
}

fn main() {
    let input = env::args().nth(1);

    match input {
        Some(value) => { 
            println!("{}", value); 
        },
        None => from_stdin()
    }
}
