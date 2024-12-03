// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 main

use crate::lab3::{declarations::DEBUG, play::Play, return_wrapper::ReturnWrapper};
use std::env;
use std::sync::atomic::Ordering;
use std::io::{stderr, stdout, Write};

pub mod lab3; // declare lab3 module

// Command line argument constants
const MAX_ARGS: usize = 3;          // Maximum number of command line arguments
const MIN_ARGS: usize = 2;          // Minimum number of command line arguments
const CMD_LINE_ERR: u8 = 1;         // Error code for command line issues

fn main() -> ReturnWrapper {
    // Declare a mutable variable for the configuration file name
    let mut config = String::new();

    // Parse command line arguments
    if let Err(e) = parse_args(&mut config) {
        writeln!(stderr().lock(), "Error: Failed to parse arguments!").unwrap();
        return ReturnWrapper::new(Err(e)); // Return error for bad arguments
    }

    writeln!(stdout().lock(), "Running script from config file: {}", config).unwrap();

    // Create a Play instance and prepare the play
    let mut play = Play::new();
    if let Err(e) = play.prepare(&config) {
        writeln!(stderr().lock(), "Error: Failed to prepare play: {}", e).unwrap();
        return ReturnWrapper::new(Err(e));
    }

    // Recite the play if preparation is successful
    play.recite();

    // Indicate successful completion
    writeln!(stdout().lock(), "Play recitation completed successfully.").unwrap();
    ReturnWrapper::new(Ok(()))
}

fn usage(program_name: &str) {
    writeln!(
        stdout().lock(),
        "Usage: {} <script_file_name> [whinge]",
        program_name
    )
    .unwrap();
}

fn parse_args(config: &mut String) -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is valid
    if args.len() < MIN_ARGS || args.len() > MAX_ARGS {
        usage(&args[0]);
        return Err(CMD_LINE_ERR);
    }

    // Set the configuration file name
    *config = args[1].clone();

    // Enable debugging mode if "whinge" argument is provided
    if args.len() == MAX_ARGS && args[2] == "whinge" {
        DEBUG.store(true, Ordering::SeqCst);
        writeln!(stdout().lock(), "Debug mode enabled.").unwrap();
    } else if args.len() == MAX_ARGS {
        writeln!(stderr().lock(), "Error: Invalid optional argument '{}'", args[2]).unwrap();
        usage(&args[0]);
        return Err(CMD_LINE_ERR);
    }

    Ok(())
}