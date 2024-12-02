// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Main

use crate::lab3::server::Server;
use crate::lab3::return_wrapper::ReturnWrapper;
use std::env;

pub mod lab3;

fn main() -> ReturnWrapper {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure exactly two arguments: program name and address
    if args.len() != 2 {
        usage(&args[0]);
        return ReturnWrapper::new(Err(1));
    }

    // Extract the network address argument
    let address = &args[1];

    println!("Starting the Lab3 Server...");

    // Initialize the server
    let mut server = Server::new();

    // Open the server with the provided address
    if let Err(e) = server.open(address) {
        eprintln!("Error: Could not open the server.");
        return ReturnWrapper::new(Err(e));
    }

    println!("Server successfully opened. Listening for connections...");

    // Run the server
    server.run();

    println!("Server shut down gracefully.");
    ReturnWrapper::new(Ok(()))
}

fn usage(program_name: &str) {
    println!("Usage: {} <network_address>", program_name);
}