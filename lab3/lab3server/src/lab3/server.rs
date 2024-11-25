// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Implementation

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

pub struct Server {
    listener: Option<TcpListener>,
    listening_addr: String,
}

pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

impl Server {
    pub fn new() -> Self {
        Self {
            listener: None,
            listening_addr: String::new(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.listener.is_some()
    }

    pub fn open(&mut self, addr: &str) -> Result<(), u8> {
        let listener = TcpListener::bind(addr).map_err(|_| {
            eprintln!("Error: Could not bind to address '{}'", addr);
            1
        })?;
        self.listener = Some(listener);
        self.listening_addr = addr.to_string();
        println!("Server listening on {}", addr);
        Ok(())
    }

    pub fn run(&self) {
        if self.listener.is_none() {
            eprintln!("Error: Server is not open. Call open() before run().");
            return;
        }

        let listener = self.listener.as_ref().unwrap(); // Safe unwrap since we checked

        while !CANCEL_FLAG.load(Ordering::SeqCst) {
            // Accept a connection
            match listener.accept() {
                Ok((socket, addr)) => {
                    println!("Accepted connection from {}", addr);

                    // Spawn a thread to handle the connection
                    thread::spawn(move || {
                        if let Err(e) = handle_connection(socket) {
                            eprintln!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error: Failed to accept connection: {}", e);
                    if CANCEL_FLAG.load(Ordering::SeqCst) {
                        break; // Exit if cancel flag is set
                    }
                }
            }
        }

        println!("Server shutting down...");
    }
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