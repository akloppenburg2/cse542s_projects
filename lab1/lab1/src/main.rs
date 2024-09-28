// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab1 main

include!("declarations.rs"); // Include the declarations.rs file
include!("script_gen.rs");  // Include the script_gen.rs file

use std::env;
use std::sync::atomic::Ordering;

// mod declarations;  // Import the declarations module

fn main() -> Result<(), u8> {
    // Declare a mutable variable for the configuration file name
    let mut config = String::new();

    // Call parse_args and handle errors
    if let Err(err) = parse_args(&mut config) {
        eprintln!("Error parsing arguments!");
        return Err(err);  // Return error for bad command line arguments
    }

    // Declare mutable variables for the play title and play structure
    let mut play_title = String::new();
    let mut play: Play = Play(Vec::new());

    // Call the script_gen function directly (no need for script_gen:: prefix)
    if let Err(err) = script_gen(&config, &mut play_title, &mut play) {
        eprintln!("Error generating script!");
        return Err(err);  // Return error if script generation failed
    }

    // Sort the play by line number
    play.0.sort_by_key(|line| line.0);

    // Recite the play (assuming recite is implemented in main.rs)
    recite(&play_title, &play);

    // Indicate successful completion
    Ok(())
}

fn usage(name: &str) {
    println!("usage: {} <configuration_file> [whinge]", name);
}

fn parse_args(config: &mut String) -> Result<(), u8> {
    let mut args = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < 2 || args.len() > 3 || (args.len() == 3 && args[2] != "whinge") {
        usage(&args[0]);
        return Err(CMD_LINE_ERR); // CMD_LINE_ERR should be defined in declarations.rs
    }

    // Set the config file name
    *config = args[1].clone();

    // Check if the third argument is "whinge"
    if args.len() == 3 && args[2] == "whinge" {
        DEBUG.store(true, Ordering::SeqCst); // DEBUG should be an AtomicBool defined in declarations.rs
    }

    Ok(())
}

fn recite(title: &String, play: &Play) {
    println!("{}", title);
    let mut current_character: Option<String> = None;

    for line in &play.0 {
        match line {
            (_, character, text) => {
                if Some(character.clone()) != current_character {
                    // New character speaking
                    println!();
                    println!("{}.", character);
                    current_character = Some(character.clone());
                } else {
                    println!("{text}");
                }
            }
        }
    }
}