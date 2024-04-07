// Uncomment this block to pass the first stage
// use std::net::TcpListener;
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => { 
                // Connection failed
                println!("Error: {}", e);
            }
        }
    }

    // Uncomment this block to pass the first stage
    //
    // let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(_stream) => {
    //             println!("accepted new connection");
    //         }
    //         Err(e) => {
    //             println!("error: {}", e);
    //         }
    //     }
    // }
}

fn handle_connection(stream: TcpStream) {
    // do nothing....so far
}