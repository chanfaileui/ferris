use std::io::{self, Write};

fn main() {
    // https://stackoverflow.com/a/34993933
    loop {
        print!("What is your name? ");
        io::stdout().flush();

        let mut name = String::new(); // new, empty, mutable string to store user input
        match io::stdin().read_line(&mut name) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        println!("Hello, {}, nice to meet you!", name.trim());

        // let lines = io::stdin().lines();
        // for line in lines {
        //     println!("Hello, {}, nice to meet you!", line.unwrap());
        // }
    }
}
