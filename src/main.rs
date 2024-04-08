// Uncomment this block to pass the first stage
// use std::net::TcpListener;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
                println!("Good connection!");
            }
            Err(e) => { 
                // Connection failed
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    // Handle request
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // Handle response
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}