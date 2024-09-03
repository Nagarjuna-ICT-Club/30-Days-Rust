use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on http://127.0.0.1:7878");

    // Loop to handle incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Buffer to store the data received
    let mut buffer = [0; 1024];

    // Read data from the stream
    stream.read(&mut buffer).unwrap();

    // Simple HTTP GET request parsing
    let get = b"GET / HTTP/1.1\r\n";

    // Respond with a simple HTML page
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    // Send the response back to the client
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}