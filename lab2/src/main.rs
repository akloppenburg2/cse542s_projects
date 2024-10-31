// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab1 main

pub mod lab2;

use std::env;
use std::sync::atomic::Ordering;
use crate::lab2::play::Play;
use crate::lab2::declarations::{DEBUG, GEN_SCRIPT_ERR, CMD_LINE_ERR};
use crate::lab2::script_gen::script_gen;

fn main() -> Result<(), u8> {
    let mut config = String::new();
    let mut part_files_dir = String::new();

    // Call parse_args and handle errors
    if let Err(err) = parse_args(&mut config, &mut part_files_dir) {
        eprintln!("Error parsing arguments!");
        return Err(err);  // Return error for bad command line arguments
    }

    // Initialize play title and play structure
    let mut play_title = String::new();
    let mut play = Play::new(&play_title); // Initialize using Play's constructor

    // Generate the play script
    if let Err(err) = script_gen(&config, &part_files_dir, &mut play_title, &mut play) {
        eprintln!("Error generating script!");
        return Err(err);  // Return error if script generation failed
    }

    // Display the play (assuming `display` method in `Play` is implemented)
    recite(&play_title, &play);

    Ok(())
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

// Updated `recite` function to match the current `Play` and `Scene` structure
fn recite(title: &String, play: &Play) {
    println!("{}", title);
    let mut current_character: Option<String> = None;

    // Loop through each scene in the play
    for scene in &play.scenes {
        for line in &scene.lines {
            // Each line is expected to be in the format "line_num (character): text"
            if let Some((_, rest)) = line.split_once(" (") {
                if let Some((character, text)) = rest.split_once("): ") {
                    let character = character.trim().to_string();
                    let text = text.trim();

                    // Print character name only if it changes
                    if Some(character.clone()) != current_character {
                        println!("\n{}:", character);
                        current_character = Some(character.clone());
                    }
                    // Print the line text
                    println!("{}", text);
                }
            }
        }
    }
}