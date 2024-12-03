// script_gen.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3

use std::fs::File;
use std::io::{stderr, BufRead, BufReader, Write};
use std::net::TcpStream;

use super::declarations::{GEN_SCRIPT_ERR, OK_RESULT};

// Function to grab and trim lines from a file
pub fn grab_trimmed_file_lines(
    file_name: &String,
    lines: &mut Vec<String>,
) -> std::result::Result<(), u8> {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            writeln!(stderr().lock(), "Error: Could not open file '{}'", file_name).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }
    };

    let mut reader = BufReader::new(file);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(OK_RESULT) => return Ok(()), // End of file reached
            Ok(_) => {
                let trimmed_line = line.trim().to_string();
                if !trimmed_line.is_empty() {
                    lines.push(trimmed_line);
                }
            }
            Err(_) => {
                writeln!(stderr().lock(), "Error: Could not read line from file '{}'", file_name).unwrap();
                return Err(GEN_SCRIPT_ERR);
            }
        }
    }
}

// Function to create a buffered reader based on a token (file or network source)
pub fn get_buffered_reader(token: &String) -> std::result::Result<BufReader<Box<dyn std::io::Read>>, u8> {
    if token.starts_with("net:") {
        let parts: Vec<&str> = token[4..].split(':').collect();
        if parts.len() < 3 {
            writeln!(stderr().lock(), "Error: Invalid network format '{}'", token).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        let address = format!("{}:{}", parts[0], parts[1]);
        let file_name = parts[2].to_string();

        // Clone `address` for error reporting to avoid moving it
        let mut stream = TcpStream::connect(&address).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not connect to server '{}'", address).unwrap();
            GEN_SCRIPT_ERR
        })?;

        writeln!(stream, "{}", file_name).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not send file request to server").unwrap();
            GEN_SCRIPT_ERR
        })?;

        Ok(BufReader::new(Box::new(stream)))
    } else {
        let file = File::open(token).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not open file '{}'", token).unwrap();
            GEN_SCRIPT_ERR
        })?;
        Ok(BufReader::new(Box::new(file)))
    }
}