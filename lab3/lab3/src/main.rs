// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 main
use std::env;
use std::sync::atomic::Ordering;

use crate::lab3::{declarations::DEBUG, play::Play, return_wrapper::ReturnWrapper};

pub mod lab3; // declare lab3 module

// Command line constants
const MAX_ARGS: usize = 3;          // Maximum number of command line arguments
const MIN_ARGS: usize = 2;          // Minimum number of command line arguments
const PROGRAM_NAME: usize = 0;      // Position of the name of the program in the vector produced by env::args.collect()
const CONFIG_FILE: usize = 1;       // Position of the name of the config file in the vector produced by env::args.collect()
const OPT: usize = 2;               // Position of the optional whinge/nowhinge argument in the vector produced by env::args.collect()
const CMD_LINE_ERR: u8 = 1;         // Error for command line argument issues

fn main() -> ReturnWrapper {
    // Declare a mutable variable for the configuration file name
    let mut config = String::new();

    // Call parse_args and handle errors
    if let Err(e) = parse_args(&mut config) {
        eprintln!("Error parsing arguments!");
        return ReturnWrapper::new(Err(e));  // Return error for bad command line arguments
    }
    
    let mut play = Play::new();
    if let Err(e) = play.prepare(&config) {
        return ReturnWrapper::new(Err(e));
    } else {
        play.recite();
    }

    // Indicate successful completion
    ReturnWrapper::new(Ok(()))
}

fn usage(name: &str) {
    println!("usage: {} <script_file_name> [whinge]", name);
}

fn parse_args(config: &mut String) -> Result<(), u8> {
    
    let args: Vec<String> = env::args().collect();

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS || (args.len() == MAX_ARGS && args[OPT] != "whinge") {
        usage(&args[PROGRAM_NAME]);
        return Err(CMD_LINE_ERR); // CMD_LINE_ERR should be defined in declarations.rs
    }

    // Set the config file name
    *config = args[CONFIG_FILE].clone();

    // Check if the third argument is "whinge"
    if args.len() == MAX_ARGS && args[OPT] == "whinge" {
        DEBUG.store(true, Ordering::SeqCst); // DEBUG should be an AtomicBool defined in declarations.rs
    }

    Ok(())
}
