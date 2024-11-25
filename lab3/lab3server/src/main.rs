// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 Server Main

use crate::lab3::server::start_server;
use crate::lab3::return_wrapper::ReturnWrapper;

pub mod lab3;

fn main() -> ReturnWrapper {
    println!("Starting the Lab3 Server...");

    // Start the server
    if let Err(e) = start_server() {
        eprintln!("Error starting the server: {}", e);
        return ReturnWrapper::new(Err(e));
    }

    println!("Server shut down gracefully.");
    ReturnWrapper::new(Ok(()))
}