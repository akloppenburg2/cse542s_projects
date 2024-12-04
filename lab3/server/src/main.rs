// server.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Implementation
use std::io::{stdout, stderr, Write};
use std::env;

use crate::lab3::server::Server;
use crate::lab3::return_wrapper::ReturnWrapper;

pub mod lab3;

// Command line constants
const NUM_ARGS: usize = 2;          // Number of command line arguments
const PROGRAM_NAME: usize = 0;      // Position of the name of the program in the vector produced by env::args.collect()
const ADDRESS_INDEX: usize = 1;      // Position of the network address in the vector produced by env::args.collect()

// Error constants
const CMD_LINE_ERR: u8 = 1;         // Error for command line argument issues

fn main() -> ReturnWrapper {
    writeln!(stdout().lock(), "Starting the Lab3 Server...").unwrap();

    let mut address = String::new();

    // Parse arguments from the command line
    if let Err(e) = parse_args(&mut address) {
        writeln!(stderr().lock(), "Error parsing arguments!").unwrap();
        return ReturnWrapper::new(Err(e));  // Return error for bad command line arguments
    }

    // If successful, create server, open, and run
    let mut server = Server::new();

    // Open the server on the specified address
    server.open(&address);
    
    // Run the server
    server.run();

    ReturnWrapper::new(Ok(()))
}

fn usage(name: &str) {
    println!("usage: {} <network_address>", name);
}

fn parse_args(address: &mut String) -> Result<(), u8> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() != NUM_ARGS {
        usage(&args[PROGRAM_NAME]);
        return Err(CMD_LINE_ERR);
    }
    else
    {
        *address = args[ADDRESS_INDEX].clone();
    }

    Ok(())
}