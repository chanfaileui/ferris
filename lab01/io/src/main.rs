use std::io::{self, Write};

fn main() {
    // https://stackoverflow.com/a/34993933

    print!("What is your name? ");
    let _ = io::stdout().flush();

    let mut name = String::new(); // new, empty, mutable string to store user input
    match io::stdin().read_line(&mut name) {
        Ok(_) => (),
        Err(err) => println!("Could not parse input: {}", err),
    }

    if name.trim().is_empty() {
        println!("No name entered :(, goodbye.");
        return;
    } else {
        println!("Hello, {}, nice to meet you!", name.trim());
    }
}
