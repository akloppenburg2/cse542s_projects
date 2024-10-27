// main.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab1 main

include!("declarations.rs"); // Include the declarations.rs file
include!("script_gen.rs");  // Include the script_gen.rs file

use std::env;
use std::sync::atomic::Ordering;

// TODO: document functions

fn main() -> Result<(), u8> {
    // Declare two mutable variables for the configuration file name and the part files directory (if one is provided)
    let mut config = String::new();
    let mut part_files_dir = String::new();

    // Call parse_args and handle errors
    if let Err(err) = parse_args(&mut config, &mut part_files_dir) {
        eprintln!("Error parsing arguments!");
        return Err(err);  // Return error for bad command line arguments
    }

    // Declare mutable variables for the play title and play structure
    let mut play_title = String::new();
    let mut play: Play = Play(Vec::new());

    // Call the script_gen function directly (no need for script_gen:: prefix)
    if let Err(err) = script_gen(&config, &part_files_dir, &mut play_title, &mut play) {
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
    println!("usage: {} <configuration_file> [part_files_dir] [whinge/nowhinge]", name);
}

fn parse_args(config: &mut String, part_files_dir: &mut String) -> Result<(), u8> {
    let mut args = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < 2 || args.len() > 4
    {
        usage(&args[0]);
        return Err(CMD_LINE_ERR); // CMD_LINE_ERR should be defined in declarations.rs
    }

    // Set the config file name
    *config = args[1].clone();

    // If an optional argument is provided that is NOT whinge/nowhinge, it's our part files directory
    if args.len() > 2
    {
        for arg in args[2..].iter()
        {
            if arg != "whinge" && arg != "nowhinge"
            {
                *part_files_dir = arg.to_string();
                break;
            }
        }
    }

    // If no part files directory is provided, use the directory the config file is in
    if part_files_dir.is_empty()
    {
        // Get the location of the last '/' character in the string
        match config.rfind('/')
        {
            Some(index) => *part_files_dir = config[..index + 1].to_string(),   // If there is a / found, get the substring up to that point (i.e. the directory the files are in)
            None             => {}, // If no / is found, the files are in the currect directory and we leave it as ""
        }
        
    }

    // If "whinge" or "nowhinge" are provided as arguments, set the debug falg accordingly (off by default)
    if args.contains(&"whinge".to_string())
    {
        DEBUG.store(true, Ordering::SeqCst); // DEBUG should be an AtomicBool defined in declarations.rs
    }
    else if args.contains(&"nowhinge".to_string()) || !args.contains(&"whinge".to_string())
    {
        DEBUG.store(false, Ordering::SeqCst); // DEBUG should be an AtomicBool defined in declarations.rs
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
                }
                
                // Print text
                println!("{text}");
            }
        }
    }
}