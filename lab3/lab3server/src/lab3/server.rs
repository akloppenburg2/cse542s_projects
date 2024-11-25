// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Implementation

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub fn start_server() -> Result<(), u8> {
    // Bind the server to localhost and a specific port
    let listener = TcpListener::bind("127.0.0.1:7878").map_err(|_| {
        eprintln!("Error: Could not bind to the port.");
        1
    })?;

    println!("Server is running on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
            Err(_) => {
                eprintln!("Error: Failed to accept connection.");
                return Err(1);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<(), u8> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).map_err(|_| {
        eprintln!("Error reading from stream.");
        1
    })?;

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    // Respond to the client
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
    stream.write(response.as_bytes()).map_err(|_| {
        eprintln!("Error writing to stream.");
        1
    })?;
    stream.flush().map_err(|_| {
        eprintln!("Error flushing stream.");
        1
    })?;

    Ok(())
}