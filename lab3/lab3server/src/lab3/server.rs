// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Implementation

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

// Static CANCEL_FLAG for graceful shutdown
pub static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

// Define the Server struct
pub struct Server {
    listener: Option<TcpListener>,
    listening_addr: String,
}

impl Server {
    // Create a new server
    pub fn new() -> Self {
        Self {
            listener: None,
            listening_addr: String::new(),
        }
    }

    // Check if the server is open
    pub fn is_open(&self) -> bool {
        self.listener.is_some()
    }

    // Open the server at the specified address
    pub fn open(&mut self, addr: &str) -> io::Result<()> {
        match TcpListener::bind(addr) {
            Ok(listener) => {
                self.listener = Some(listener);
                self.listening_addr = addr.to_string();
                writeln!(io::stdout().lock(), "Server listening on {}", addr).unwrap();
                Ok(())
            }
            Err(e) => {
                writeln!(
                    io::stderr().lock(),
                    "Error: Failed to bind server to address {}: {}",
                    addr, e
                )
                .unwrap();
                Err(e)
            }
        }
    }

    // Start listening for incoming connections
    pub fn start(&mut self) -> Result<(), u8> {
        if let Some(listener) = &self.listener {
            writeln!(
                io::stdout().lock(),
                "Server is running on {}",
                self.listening_addr
            )
            .unwrap();

            for stream in listener.incoming() {
                if CANCEL_FLAG.load(Ordering::SeqCst) {
                    writeln!(io::stdout().lock(), "Server shutting down gracefully.").unwrap();
                    break;
                }

                match stream {
                    Ok(stream) => {
                        thread::spawn(move || {
                            if let Err(e) = handle_connection(stream) {
                                eprintln!("Error handling connection: {}", e);
                            }
                        });
                    }
                    Err(_) => {
                        writeln!(io::stderr().lock(), "Error: Failed to accept connection.").unwrap();
                        return Err(1);
                    }
                }
            }
        } else {
            writeln!(io::stderr().lock(), "Error: Server is not open.").unwrap();
            return Err(1);
        }

        Ok(())
    }
}

// Handle a single connection
fn handle_connection(mut stream: TcpStream) -> Result<(), u8> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).map_err(|_| {
        eprintln!("Error reading from stream.");
        1
    })?;

    writeln!(io::stdout().lock(), "Request: {}", String::from_utf8_lossy(&buffer)).unwrap();

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
