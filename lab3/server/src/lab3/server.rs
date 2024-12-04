// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{Ordering, AtomicBool};
use std::io::{stdout, stderr, Write};
use std::thread;

// Static CANCEL_FLAG for graceful shutdown
pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

// Define the Server struct
pub struct Server {
    listener: Option<TcpListener>,
    address: String,
}

impl Server {
    // Create a new server
    pub fn new() -> Self {
        Self {
            listener: None,
            address: String::new(),
        }
    }

    // Check if the server is open
    pub fn is_open(&self) -> bool {
        self.listener.is_some()
    }

    // Open the server at the specified address
    pub fn open(&mut self, addr: &str) {
        match TcpListener::bind(addr) {
            Ok(listener) => {
                self.listener = Some(listener);
                self.address = addr.to_string();
                writeln!(stdout().lock(), "Server listening on {}", addr).unwrap();
            }
            Err(e) => {
                writeln!(stderr().lock(), "Error: Failed to bind server to address {}: {}", addr, e).unwrap();
            }
        }
    }

    // Run server 
    pub fn run(&self) {
        // Verify we have a listener first
        match &self.listener {
            Some(listener) => {
                while !CANCEL_FLAG.load(Ordering::SeqCst) {
                    // Accept a connection
                    match listener.accept() {
                        Ok((socket, addr)) => {
                            // Check cancel flag again and immediately exit if true
                            if CANCEL_FLAG.load(Ordering::SeqCst) {
                                return; // Exit if cancel flag is set
                            }
                            writeln!(stdout().lock(), "Accepted connection from {}", addr).unwrap();
                            // Spawn a thread to handle the connection
                            thread::spawn(move || {
                                Self::handle_connection(socket);
                            });
                        }
                        Err(e) => {
                            writeln!(stderr().lock(), "Error: Failed to accept connection: {}", e).unwrap();
                            // Check cancel flag again and immediately exit if true
                            if CANCEL_FLAG.load(Ordering::SeqCst) {
                                return; // Exit if cancel flag is set
                            }
                        }
                    }
                }
            },
            None => writeln!(stderr().lock(), "Error: Server is not open. Call open() before run().").unwrap(),
        }
        writeln!(stdout().lock(), "Server shutting down...").unwrap();
    }

    pub fn handle_connection(socket: TcpStream)
    {

    }
}