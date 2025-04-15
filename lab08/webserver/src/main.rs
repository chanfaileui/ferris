//! A simple web server
//! which serves some html at index.html
//! and replaces triple curly braces with the given variable
mod test;
use std::io::{Read, Write};

use std::net::{TcpListener, TcpStream};
// hint, hint
use std::sync::{Arc, Mutex};
use std::thread;

struct State {
    counter: i32,
}

fn handle_client(state: Arc<Mutex<State>>, mut stream: TcpStream) {
    // setup buffer, and read from stream into buffer
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // convert request payload to string
    let string = String::from_utf8_lossy(&buffer);

    // extract header
    let mut split = string.split("\r\n");
    let header = split.next().unwrap();

    if header == "POST /counter HTTP/1.1" {
        //TODO: increment the counter
        let mut state = state.lock().unwrap();
        state.counter += 1;
    }

    let file = include_bytes!("../index.html");

    // TODO: replace triple brackets in file with the counter in state (array of bytes)
    //      - you should make sure your resulting content is still called file
    //      - or the below code will not work
    // Look into std::str::from_utf8 to convert bytes to string
    // Then you can use string methods like replace to find and replace "{{{}}}}"
    // Finally, convert back to bytes with .as_bytes()

    // let file_string = std::str::from_utf8(file).unwrap();
    // let replaced_string = file_string.replace("{{{}}}", &counter_value.to_string());
    // let file = replaced_string.as_bytes();

    let file = String::from_utf8_lossy(file).to_string();
    let state = state.lock().unwrap();
    let file = file.replace("{{{ counter }}}", &state.counter.to_string());

    // DONT CHANGE ME
    let response = format!(
        "HTTP/1.1 200 OK\r\nContentType: text/html; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n",
        file.len()
    );

    // converts response to &[u8], and writes them to the stream
    stream.write_all(response.as_bytes()).unwrap();
    stream.write_all(file.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let port = std::env::args().nth(1).unwrap_or("8081".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;

    println!("Server running on port {}", port);
    // TODO: create new state, so that it can be safely
    //      shared between threads
    let state = Arc::new(Mutex::new(State { counter: 0 }));

    // accept connections and process them serially
    for stream in listener.incoming() {
        // TODO: spawn a thread for each connection
        // TODO: pass the state to the thread (and the handle_client fn)
        let state = Arc::clone(&state);
        thread::spawn(move || {
            // println!("hi");
            handle_client(state, stream.unwrap());
        });
    }
    Ok(())
}
