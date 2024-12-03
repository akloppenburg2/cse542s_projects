// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server
use std::net::TcpListener;
use std::sync::atomic::AtomicBool;
use std::io::{stdout, stderr, Write};

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
}