use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Ensure exactly three arguments: program name, server address, and token
    if args.len() != 3 {
        usage(&args[0]);
        return;
    }

    let server_address = &args[1];
    let token = &args[2];

    println!("Connecting to server at {}...", server_address);

    if token == "quit" {
        // Handle the "quit" token case
        println!("Sending 'quit' token to the server...");
        if let Err(e) = TcpStream::connect(server_address)
            .and_then(|mut stream| writeln!(stream, "{}", token))
        {
            eprintln!("Error sending 'quit' token: {}", e);
            return;
        }

        println!("Waiting briefly before reconnecting...");
        thread::sleep(Duration::from_secs(1));

        if let Err(e) = TcpStream::connect(server_address) {
            eprintln!("Error reconnecting to the server: {}", e);
        } else {
            println!("Reconnected to the server to wake it up.");
        }

        return;
    }

    // Handle other tokens
    match TcpStream::connect(server_address) {
        Ok(mut stream) => {
            println!("Connected to the server at {}", server_address);

            if let Err(e) = writeln!(stream, "{}", token) {
                eprintln!("Error sending token: {}", e);
                return;
            }

            let reader = BufReader::new(stream);

            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(e) => {
                        eprintln!("Error reading from server: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error connecting to the server: {}", e);
        }
    }
}

fn usage(program_name: &str) {
    println!("Usage: {} <server_address> <token>", program_name);
}
