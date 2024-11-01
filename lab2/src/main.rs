// main.rs

pub mod lab2;

use std::env;
use std::sync::atomic::Ordering;
use crate::lab2::play::Play;
use crate::lab2::declarations::{DEBUG, CMD_LINE_ERR};
use crate::lab2::return_wrapper::ReturnWrapper; // Import ReturnWrapper

fn main() -> ReturnWrapper {
    // Declare two mutable variables for the configuration file name and the part files directory (if one is provided)
    let mut config = String::new();
    let mut part_files_dir = String::new();

    // Call parse_args and handle errors
    if let Err(err) = parse_args(&mut config, &mut part_files_dir) {
        eprintln!("Error parsing arguments!");
        return ReturnWrapper::new(err);  // Wrap with ReturnWrapper for errors
    }

    // Create a new Play instance and prepare it using the config file
    let mut play = Play::new();
    if let Err(err) = play.prepare(&config, &part_files_dir) {
        eprintln!("Error generating script!");
        return ReturnWrapper::new(err);  // Wrap with ReturnWrapper for errors
    }

    // Call recite to print the play
    play.recite();

    ReturnWrapper::new(0) // Success wrapped with ReturnWrapper
}


fn usage(name: &str) {
    println!("usage: {} <configuration_file> [part_files_dir] [whinge/nowhinge]", name);
}

fn parse_args(config: &mut String, part_files_dir: &mut String) -> Result<(), u8> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 4 {
        usage(&args[0]);
        return Err(CMD_LINE_ERR); // CMD_LINE_ERR should be defined in declarations.rs
    }

    // Set the config file name
    *config = args[1].clone();

    // Set part_files_dir if provided and not "whinge"/"nowhinge"
    if args.len() > 2 {
        for arg in &args[2..] {
            if arg != "whinge" && arg != "nowhinge" {
                *part_files_dir = arg.to_string();
                break;
            }
        }
    }

    // Set part_files_dir to config file's directory if not provided
    if part_files_dir.is_empty() {
        if let Some(index) = config.rfind('/') {
            *part_files_dir = config[..index + 1].to_string();
        }
    }

    // Set debug flag based on "whinge" or "nowhinge"
    if args.contains(&"whinge".to_string()) {
        DEBUG.store(true, Ordering::SeqCst); // DEBUG should be an AtomicBool defined in declarations.rs
    } else {
        DEBUG.store(false, Ordering::SeqCst);
    }

    Ok(())
}