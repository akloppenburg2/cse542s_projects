// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Test Client
use std::net::TcpStream;
use std::time::Duration;
use std::io::{stdout, stderr, Write, BufRead, BufReader};
use std::env;
use std::thread;

// Command line constants
const NUM_ARGS: usize = 3;          // Number of command line arguments
const PROGRAM_NAME: usize = 0;      // Position of the name of the program in the vector produced by env::args.collect()
const ADDRESS_INDEX: usize = 1;     // Position of the network address in the vector produced by env::args.collect()
const TOKEN_INDEX: usize = 2;       // Position of the token to be sent to the server in the vector produced by env::args.collect()

// Error constants
const CMD_LINE_ERR: u8 = 1;         // Error for command line argument issues
const CLIENT_ERROR: u8 = 3;         // Client connect/write/read error

fn main() -> Result<(), u8> {
    writeln!(stdout().lock(), "Starting the Lab3 Client...").unwrap();

    // Strings to store parsed args
    let mut address = String::new();
    let mut token   = String::new();

    // Parse arguments from the command line
    if let Err(e) = parse_args(&mut address, &mut token) {
        writeln!(stderr().lock(), "Error parsing arguments!").unwrap();
        return Err(e);  // Return error for bad command line arguments
    }

    // If successful, try to connect to address via a TcpStream
    if let Ok(mut stream) = TcpStream::connect(&address)
    {
        match stream.write_all(token.as_bytes())
        {
            Ok(()) => {
                if token == "quit"
                {
                    writeln!(stdout().lock(), "Shutting down server...").unwrap();   
                    // Sleep for one second
                    let sec = Duration::new(1, 0);
                    thread::sleep(sec);

                    // Wake the server back up
                    match TcpStream::connect(&address)
                    {
                        Ok(_) => return Ok(()),
                        Err(e) => writeln!(stderr().lock(), "Error: Failed to accept connection: {}", e).unwrap(),
                    }
                }
                else
                {
                    // Handle other tokens (read responses)
                    let reader = BufReader::new(stream);
                    for line in reader.lines() {
                        match line {
                            Ok(content) => println!("Server response: {}", content),
                            Err(e) => {
                                eprintln!("Error: Failed to read from server: {}", e);
                                return Err(CLIENT_ERROR);
                            }
                        }
                    }
                    println!("Connection closed by server.");
                }
            },
            Err(e) => writeln!(stderr().lock(), "Error: Failed to write token: {}.", e).unwrap(),
        }
    }
    else
    {
        writeln!(stderr().lock(), "Error: Failed to accept connection: {}", CLIENT_ERROR).unwrap();
    }

    Ok(())
}

// usage fn
fn usage(name: &str) {
    println!("usage: {} <network_address> <token>", name);
}

// parses arguments
fn parse_args(address: &mut String, token: &mut String) -> Result<(), u8> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != NUM_ARGS {
        usage(&args[PROGRAM_NAME]);
        return Err(CMD_LINE_ERR);
    }
    else
    {
        *address = args[ADDRESS_INDEX].clone();
        *token   = args[TOKEN_INDEX].clone();
    }

    Ok(())
}