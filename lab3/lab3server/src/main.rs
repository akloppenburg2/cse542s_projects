// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Main

use crate::lab3::server::Server;
use crate::lab3::return_wrapper::ReturnWrapper;

pub mod lab3;

fn main() -> ReturnWrapper {
    println!("Starting the Lab3 Server...");

    let mut server = Server::new();

    // Open the server on the specified address
    if let Err(_) = server.open("127.0.0.1:7878") {
        eprintln!("Error: Could not open the server.");
        return ReturnWrapper::new(Err(1));
    }

    println!("Server successfully opened. Listening for connections...");
    
    // Run the server
    server.run();

    println!("Server shut down gracefully.");
    ReturnWrapper::new(Ok(()))
}