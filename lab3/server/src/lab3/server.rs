// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{Ordering, AtomicBool};
use std::io::{stdout, stderr, Read, Write, BufRead, BufReader};
use std::thread;
use std::path::Path;
use std::fs::File;

// Server connection error
const SERVER_ERROR: u8 = 3;

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
                                if let Err(e) = Self::handle_connection(socket) {
                                    writeln!(stderr().lock(), "Error handling connection: {}", e).unwrap();
                                }
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

    fn handle_connection(mut stream: TcpStream) -> Result<(), u8> {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer).map_err(|_| {
            writeln!(stderr().lock(), "Error: Failed to read data from client.").unwrap();
            SERVER_ERROR
        })?;
    
        // Parse the received token
        let token = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
    
        // Handle "quit" command
        if token == "quit" {
            CANCEL_FLAG.store(true, Ordering::SeqCst);
            return Ok(());
        }
    
        // Check for insecure or invalid tokens
        if token.contains('/') || token.contains('\\') || token.contains("..") || token.contains('$') {
            let response = "Invalid file name.";
            stream.write_all(response.as_bytes()).map_err(|_| {
                writeln!(stderr().lock(), "Error: Failed to write bad request response.").unwrap();
                SERVER_ERROR
            })?;
            writeln!(stderr().lock(), "Invalid file name: {}", token).unwrap();
            return Ok(());
        }
    
        // Attempt to open the file
        let file_path = Path::new(&token);
        if !file_path.exists() || !file_path.is_file() {
            let response = "File not found.";
            stream.write_all(response.as_bytes()).map_err(|_| {
                writeln!(stderr().lock(), "Error: Failed to write not found response.").unwrap();
                SERVER_ERROR
            })?;
            writeln!(stderr().lock(), "File not found: {}", token).unwrap();
            return Ok(());
        }
    
        // Open the file and stream its contents to the client
        let file = File::open(&file_path).map_err(|_| {
            writeln!(stderr().lock(), "Error: Failed to open file '{}'.", token).unwrap();
            SERVER_ERROR
        })?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
    
        while reader.read_line(&mut line).map_err(|_| {
            writeln!(stderr().lock(), "Error: Failed to read from file '{}'.", token).unwrap();
            SERVER_ERROR
        })? > 0 {
            stream.write_all(line.as_bytes()).map_err(|_| {
                writeln!(stderr().lock(), "Error: Failed to write to client.").unwrap();
                SERVER_ERROR
            })?;
            line.clear();
        }
    
        Ok(())  
    }
}