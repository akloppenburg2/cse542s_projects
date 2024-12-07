// script_gen.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab 3
use std::fs::File;
use std::net::TcpStream;
use std::result::Result;
use std::io::{stderr, Write, Read, BufReader, BufRead};

use super::declarations::{GEN_SCRIPT_ERR, OK_RESULT};

// Function to grab and trim lines from a file
pub fn grab_trimmed_file_lines(file_name: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // Initialize a BufReader and a String to hold each line
    let mut reader = get_buffered_reader(file_name)?;
    let mut line = String::new();

    // Loop to read each line from the file
    loop {
        line.clear();  // Clear the buffer before reading the next line

        match reader.read_line(&mut line) {
            Ok(OK_RESULT) => return Ok(()),  // End of file reached, return success
            Ok(_) => {
                // Trim the line and push it into the vector if it's not empty
                let trimmed_line = line.trim().to_string();
                if !trimmed_line.is_empty() {
                    lines.push(trimmed_line);
                }
            }
            Err(_) => {
                writeln!(stderr().lock(), "Error: Could not read line from file '{}'", file_name).unwrap();
                return Err(GEN_SCRIPT_ERR);  // Return error if reading fails
            }
        }
    }
}

// Function to create a buffered reader based on a token (file or network source)
pub fn get_buffered_reader(token: &String) -> Result<BufReader<Box<dyn Read>>, u8> {

    // If the token is a network address, split it and gather the tokens
    if token.contains("net:") {
        // Since we prepend the path to the token to allow for scripts in different directories, we need to start looking after that portion of the string
        let mut addr_index = 0;
        match token.find("net:")
        {
            Some(index) => addr_index = index + 4,  // We add 4 since find returns the index of the beginning of the match, and we want the index of the end
            None        => writeln!(stderr().lock(), "Error: Invalid network address format '{}'", token).unwrap(),
        }
        let parts: Vec<&str> = token[addr_index..].split(':').collect();
        if parts.len() < 3 {
            writeln!(stderr().lock(), "Error: Invalid network address format '{}'", token).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        let address = format!("{}:{}", parts[0], parts[1]);
        let file_name = parts[2].to_string();

        let mut stream = TcpStream::connect(&address).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not connect to server '{}'", address).unwrap();
            GEN_SCRIPT_ERR
        })?;

        writeln!(stream, "{}", file_name).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not send file request to server").unwrap();
            GEN_SCRIPT_ERR
        })?;

        Ok(BufReader::new(Box::new(stream)))
    }
    // If it's not, treat it normally
    else {
        let file = File::open(token).map_err(|_| {
            writeln!(stderr().lock(), "Error: Could not open file '{}'", token).unwrap();
            GEN_SCRIPT_ERR
        })?;
        Ok(BufReader::new(Box::new(file)))
    }
}
