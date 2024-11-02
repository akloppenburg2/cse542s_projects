// main.rs

pub mod lab2;

use std::env;
use std::sync::atomic::Ordering;
use crate::lab2::play::Play;
use crate::lab2::declarations::{DEBUG, CMD_LINE_ERR}; // Import only needed constants

fn main() -> Result<(), u8> {
    // Declare a mutable variable for the configuration file name
    let mut config = String::new();

    // Call parse_args and handle errors
    if let Err(err) = parse_args(&mut config) {
        eprintln!("Error parsing arguments!");
        return Err(err);  // Return error for bad command line arguments
    }

    // Create a new Play instance and prepare it using the config file
    let mut play = Play::new();
    if let Err(err) = play.prepare(&config) {
        eprintln!("Error generating script!");
        return Err(err);  // Return error if script generation failed
    }

    // Call recite to print the play
    play.recite();

    Ok(())
}


fn usage(name: &str) {
    println!("usage: {} <configuration_file> [whinge/nowhinge]", name);
}

fn parse_args(config: &mut String) -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        usage(&args[0]);
        return Err(CMD_LINE_ERR);
    }

    // Set the config file name
    *config = args[1].clone();

    // Set debug flag based on "whinge" or "nowhinge"
    if args.contains(&"whinge".to_string()) {
        DEBUG.store(true, Ordering::SeqCst); // DEBUG should be an AtomicBool defined in declarations.rs
    } else {
        DEBUG.store(false, Ordering::SeqCst);
    }

    Ok(())
}