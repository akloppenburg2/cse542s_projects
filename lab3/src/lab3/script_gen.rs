// script_gen.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3

use std::fs::File;
use std::io::{BufReader, BufRead, stderr, Write};
use std::net::TcpStream;

use super::declarations::GEN_SCRIPT_ERR;

pub fn grab_trimmed_file_lines(file_name: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // Use get_buffered_reader instead of directly opening the file
    let reader = match get_buffered_reader(file_name) {
        Ok(reader) => reader,
        Err(_) => {
            writeln!(stderr().lock(), "Error: Could not open or connect to '{}'", file_name).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }
    };

    // Loop to read each line from the reader
    for result in reader.lines() {
        match result {
            Ok(content) => {
                let trimmed = content.trim().to_string();
                if !trimmed.is_empty() {
                    lines.push(trimmed);
                }
            }
            Err(_) => {
                writeln!(stderr().lock(), "Error: Could not read line from '{}'", file_name).unwrap();
                return Err(GEN_SCRIPT_ERR);
            }
        }
    }

    Ok(())
}

pub fn get_buffered_reader(source: &String) -> Result<BufReader<Box<dyn std::io::Read>>, u8> {
    if source.starts_with("net:") {
        // Parse the source string into address and file name
        let parts: Vec<&str> = source[4..].split(':').collect();
        if parts.len() != 3 {
            writeln!(stderr().lock(), "Error: Invalid network format '{}'", source).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        let address = format!("{}:{}", parts[0], parts[1]);
        let file_name = parts[2];

        // Connect to the server
        let mut stream = TcpStream::connect(address.clone()).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not connect to server '{}'", address).unwrap();
            GEN_SCRIPT_ERR
        })?;

        // Send the file name request to the server
        writeln!(stream, "{}", file_name).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not send file request '{}'", file_name).unwrap();
            GEN_SCRIPT_ERR
        })?;

        // Return the stream wrapped in a BufReader
        Ok(BufReader::new(Box::new(stream)))
    } else {
        // Open the file locally
        let file = File::open(source).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not open file '{}'", source).unwrap();
            GEN_SCRIPT_ERR
        })?;
        Ok(BufReader::new(Box::new(file)))
    }
}