use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::lab2::play::{Play, PlayConfig}; // Updated imports
use crate::lab2::player::Player; // Import Player
use crate::DEBUG;
use crate::lab2::declarations::GEN_SCRIPT_ERR;

// Function to grab and trim lines from a file
pub fn grab_trimmed_file_lines(file_name: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Could not open file '{}'", file_name);
            return Err(GEN_SCRIPT_ERR);
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(content) = line {
            let trimmed_line = content.trim().to_string();
            if !trimmed_line.is_empty() {
                lines.push(trimmed_line);
            }
        } else {
            eprintln!("Error: Could not read line from file '{}'", file_name);
            return Err(GEN_SCRIPT_ERR);
        }
    }
    Ok(())
}

// Function to process the PlayConfig and generate the Play script
pub fn process_config(play: &mut Play, play_config: &PlayConfig) -> Result<(), u8> {
    for (character_name, file_name) in play_config {
        let mut player = Player::new(character_name); // Create a new player for each character
        if player.prepare(file_name).is_err() {
            eprintln!("Error: Failed to prepare player '{}'", character_name);
            return Err(GEN_SCRIPT_ERR);
        }
        play.add_player(player); // Add player to the play using add_player method
    }
    Ok(())
}

// Add a config entry
pub fn add_config(line: &String, play_config: &mut PlayConfig, part_files_dir: String) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == 2 {
        play_config.push((tokens[0].to_string(), part_files_dir + tokens[1]));
    } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
        eprintln!("Warning: Badly formed line in config: {}", line);
    }
}

// Read configuration from a file
pub fn read_config(config_file: &String, part_files_dir: &String, play_title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {
    let mut lines = Vec::new();
    if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
        eprintln!("Error: Failed to process file: '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    *play_title = lines.remove(0);

    for line in lines {
        add_config(&line, play_config, part_files_dir.to_string());
    }
    Ok(())
}

// Main script generation function
pub fn script_gen(config_file: &String, part_files_dir: &String, play_title: &mut String, play: &mut Play) -> Result<(), u8> {
    let mut play_config = PlayConfig::new();
    if read_config(config_file, part_files_dir, play_title, &mut play_config).is_err() {
        eprintln!("Error: Failed to read config '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    if process_config(play, &play_config).is_err() {
        eprintln!("Error: Failed to process config '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }
    Ok(())
}