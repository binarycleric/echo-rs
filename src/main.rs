use std::env;

fn main() {
    let input = env::args().nth(1);

    match input {
        Some(value) => { 
            println!("{}", value); 
        },
        None => { /* do nothing */ },
    }
}
