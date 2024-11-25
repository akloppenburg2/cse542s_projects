// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Main

use crate::lab3::{server::Server, return_wrapper::ReturnWrapper};

pub mod lab3;

fn main() -> ReturnWrapper {
    println!("Starting the Lab3 Server...");

    // Create a new server instance
    let mut server = Server::new();

    // Open the server on a specific address
    if let Err(_) = server.open("127.0.0.1:7878") {
        eprintln!("Failed to open the server on the specified address.");
        return ReturnWrapper::new(Err(1));
    }

    // Start the server
    if let Err(_) = server.start() {
        eprintln!("Server encountered an error while running.");
        return ReturnWrapper::new(Err(1));
    }

    println!("Server shut down gracefully.");
    ReturnWrapper::new(Ok(()))
}
