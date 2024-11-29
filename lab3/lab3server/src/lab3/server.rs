// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Implementation

use std::net::{TcpListener, TcpStream};
use std::io::{stdout, stderr, BufRead, BufReader, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::fs::File;

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
            writeln!(stderr().lock(), "Error: Could not bind to address '{}'", addr).unwrap();
            1
        })?;
        self.listener = Some(listener);
        self.listening_addr = addr.to_string();
        writeln!(stdout().lock(), "Server listening on {}", addr).unwrap();
        Ok(())
    }

    pub fn run(&self) {
        if self.listener.is_none() {
            writeln!(stderr().lock(), "Error: Server is not open. Call open() before run().").unwrap();
            return;
        }

        let listener = self.listener.as_ref().unwrap(); // Safe unwrap since we checked

        while !CANCEL_FLAG.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((socket, addr)) => {
                    writeln!(stdout().lock(), "Accepted connection from {}", addr).unwrap();

                    thread::spawn(move || {
                        if let Err(e) = handle_connection(socket) {
                            writeln!(stderr().lock(), "Error handling connection: {}", e).unwrap();
                        }
                    });
                }
                Err(e) => {
                    writeln!(stderr().lock(), "Error: Failed to accept connection: {}", e).unwrap();
                    if CANCEL_FLAG.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
        }

        writeln!(stdout().lock(), "Server shutting down...").unwrap();
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), u8> {
    let mut reader = BufReader::new(&mut stream);
    let mut token = String::new();

    // Read the token from the client
    reader.read_line(&mut token).map_err(|_| {
        writeln!(stderr().lock(), "Error reading from stream.").unwrap();
        1
    })?;
    token = token.trim().to_string();

    if token.eq_ignore_ascii_case("quit") {
        writeln!(stdout().lock(), "Quit command received. Setting cancel flag...").unwrap();
        CANCEL_FLAG.store(true, Ordering::SeqCst);
        return Ok(());
    }

    if token.contains('/') || token.contains('\\') || token.contains("..") || token.contains('$') {
        writeln!(stream, "Invalid file name: {}", token).map_err(|_| {
            writeln!(stderr().lock(), "Error writing to stream.").unwrap();
            1
        })?;
        return Err(1);
    }

    let file_path = format!("./{}", token);
    let file = File::open(&file_path).map_err(|_| {
        writeln!(stderr().lock(), "Error: Could not open file '{}'", token).unwrap();
        1
    })?;

    let mut file_reader = BufReader::new(file);
    let mut line = String::new();
    while file_reader.read_line(&mut line).map_err(|_| {
        writeln!(stderr().lock(), "Error reading from file '{}'", token).unwrap();
        1
    })? > 0
    {
        stream.write_all(line.as_bytes()).map_err(|_| {
            writeln!(stderr().lock(), "Error writing to stream.").unwrap();
            1
        })?;
        line.clear();
    }

    writeln!(stream, "\nEOF").map_err(|_| {
        writeln!(stderr().lock(), "Error writing EOF to stream.").unwrap();
        1
    })?;

    Ok(())
}
