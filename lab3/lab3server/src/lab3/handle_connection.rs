// handle_connection.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo

use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;
use crate::lab3::server::CANCEL_FLAG;

pub fn handle_connection(mut stream: TcpStream) -> Result<(), u8> {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).map_err(|_| {
        eprintln!("Error: Failed to read data from client.");
        1
    })?;

    // Parse the received token
    let token = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
    println!("Received token: '{}'", token);

    // Handle "quit" command
    if token == "quit" {
        println!("Received 'quit' command. Shutting down server...");
        CANCEL_FLAG.store(true, Ordering::SeqCst);
        return Ok(());
    }

    // Check for insecure or invalid tokens
    if token.contains('/') || token.contains('\\') || token.contains("..") || token.contains('$') {
        let response = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid file name.";
        stream.write_all(response.as_bytes()).map_err(|_| {
            eprintln!("Error: Failed to write bad request response.");
            1
        })?;
        println!("Invalid file name: {}", token);
        return Ok(());
    }

    // Attempt to open the file
    let file_path = Path::new(&token);
    if !file_path.exists() || !file_path.is_file() {
        let response = "HTTP/1.1 404 Not Found\r\n\r\nFile not found.";
        stream.write_all(response.as_bytes()).map_err(|_| {
            eprintln!("Error: Failed to write not found response.");
            1
        })?;
        println!("File not found: {}", token);
        return Ok(());
    }

    // Open the file and stream its contents to the client
    let file = File::open(&file_path).map_err(|_| {
        eprintln!("Error: Failed to open file '{}'.", token);
        1
    })?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let response_header = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n";
    stream.write_all(response_header.as_bytes()).map_err(|_| {
        eprintln!("Error: Failed to write response header.");
        1
    })?;

    while reader.read_line(&mut line).map_err(|_| {
        eprintln!("Error: Failed to read from file '{}'.", token);
        1
    })? > 0 {
        stream.write_all(line.as_bytes()).map_err(|_| {
            eprintln!("Error: Failed to write to client.");
            1
        })?;
        line.clear();
    }

    Ok(())
}