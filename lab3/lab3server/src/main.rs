// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Main

use std::io::{stderr, stdout, Write};

use crate::lab3::server::Server;
use crate::lab3::return_wrapper::ReturnWrapper;

pub mod lab3;

fn main() -> ReturnWrapper {
    writeln!(stdout().lock(), "Starting the Lab3 Server...").unwrap();

    let mut server = Server::new();

    // Open the server on the specified address
    if let Err(_) = server.open("127.0.0.1:8080") {
        writeln!(stderr().lock(), "Error: Could not open the server.").unwrap();
        return ReturnWrapper::new(Err(1));
    }

    writeln!(stdout().lock(), "Server successfully opened. Listening for connections...").unwrap();
    
    // Run the server
    server.run();

    writeln!(stdout().lock(), "Server shut down gracefully.").unwrap();
    ReturnWrapper::new(Ok(()))
}