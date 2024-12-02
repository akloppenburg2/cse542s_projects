// main.rs
// Lab3 Test Client

use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Validate arguments (program name, address, token)
    if args.len() != 3 {
        usage(&args[0]);
        exit(1);
    }

    let server_address = &args[1];
    let token = &args[2];

    // Try to connect to the server
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to the server at {}", server_address);

            // Send the token to the server
            if let Err(e) = writeln!(stream, "{}", token) {
                eprintln!("Error: Failed to send token: {}", e);
                exit(1);
            }

            if token.eq_ignore_ascii_case("quit") {
                // Handle "quit" token
                println!("Sent 'quit' token to the server. Waiting briefly before reconnecting...");

                // Sleep for one second
                thread::sleep(Duration::from_secs(1));

                // Reconnect to wake up the server
                match TcpStream::connect(server_address) {
                    Ok(_) => println!("Reconnected to the server to wake it up."),
                    Err(e) => eprintln!("Error: Failed to reconnect: {}", e),
                }

                return;
            } else {
                // Handle other tokens (read responses)
                let reader = BufReader::new(stream);
                for line in reader.lines() {
                    match line {
                        Ok(content) => println!("Server response: {}", content),
                        Err(e) => {
                            eprintln!("Error: Failed to read from server: {}", e);
                            exit(1);
                        }
                    }
                }

                println!("Server closed the connection.");
            }
        }
        Err(e) => {
            eprintln!("Error: Could not connect to the server at {}: {}", server_address, e);
            exit(1);
        }
    }
}

fn usage(program_name: &str) {
    println!("Usage: {} <server_address> <token>", program_name);
}