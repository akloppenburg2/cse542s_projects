use std::fs::File;
use std::io::{BufReader, BufRead};

// Define PlayConfig as a vector of (character name, file name) tuples
pub type PlayConfig = Vec<(String, String)>;

// Script generation constants
pub const TITLE_IDX: usize = 0;
pub const CHAR_NAME_LINE: usize = 0;
pub const FILE_LINE: usize = 1;
pub const CHAR_TOKEN_IDK: usize = 0;
pub const FILE_TOKEN_IDK: usize = 1;
pub const TOKENS: usize = 2;

// Function to grab and trim lines from a file
pub fn grab_trimmed_file_lines(file_name: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // Try to open the file, and if it fails, return an error
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Could not open file '{}'", file_name);
            return Err(GEN_SCRIPT_ERR);
        }
    };

    // Initialize a BufReader and a String to hold each line
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    // Loop to read each line from the file
    loop {
        line.clear();  // Clear the buffer before reading the next line

        match reader.read_line(&mut line) {
            Ok(0) => return Ok(()),  // End of file reached, return success
            Ok(_) => {
                // Trim the line and push it into the vector if it's not empty
                let trimmed_line = line.trim().to_string();
                if !trimmed_line.is_empty() {
                    lines.push(trimmed_line);
                }
            }
            Err(_) => {
                eprintln!("Error: Could not read line from file '{}'", file_name);
                return Err(GEN_SCRIPT_ERR);  // Return error if reading fails
            }
        }
    }
}

// Function to process the PlayConfig and generate the Play script
pub fn process_config(play: &mut Play, play_config: &PlayConfig) -> Result<(), u8> {

    // Iterate through each tuple in PlayConfig (character name, file name)
    for config in play_config {
        match config {
            (part_name, file_name) => {
                // Vector to store lines
                let mut lines = Vec::new();

                // Call grab_trimmed_file_lines to read and trim lines from the file
                if let Err(_) = grab_trimmed_file_lines(file_name, &mut lines) {
                    eprintln!("Error: Failed to process file for part '{}'", part_name);
                    return Err(GEN_SCRIPT_ERR);
                }

                // Add each line to the Play using add_script_line
                for line in &lines {
                    add_script_line(play, line, part_name);
                }
            }
        }
    }

    Ok(())
}

pub fn add_config(line: &String, play_config: &mut PlayConfig) {

    // Tokenize line
    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.len() == 2 {
        play_config.push((tokens[0].to_string(), tokens[1].to_string()));
    }
    else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
        eprintln!("Warning: Badly formed line in config: {}", line);
    }

}

pub fn read_config(config_file: &String, play_title: &mut String, play_config: &mut PlayConfig) -> Result<(), u8> {

    // Vector for lines
    let mut lines = Vec::new();

    // Call grab_trimmed_file_lines to read and trim lines from the file
    if let Err(_) = grab_trimmed_file_lines(config_file, &mut lines) {
        eprintln!("Error: Failed to process file: '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    // Set title to first element
    *play_title = lines.remove(0);

    // Add remaining elements to config
    for line in lines
    {
        add_config(&line, play_config);
    }

    Ok(())

}

// Main script generation function
pub fn script_gen(config_file: &String, play_title: &mut String, play: &mut Play) -> Result<(), u8> {
    
    // Initialize and then read config
    let mut play_config = PlayConfig::new(); 
    if let Err(_) = read_config(config_file, play_title, &mut play_config){
        eprintln!("Error: Failed to read config '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    if let Err(_) = process_config(play, &play_config){
        eprintln!("Error: Failed to process config '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    Ok(())
}

// The add_script_line function to process each line of the character's file
pub fn add_script_line(play: &mut Play, line: &String, character: &String) {
    if !line.trim().is_empty() {
        if let Some((line_num_str, rest_of_line)) = line.split_once(char::is_whitespace) {
            let line_num_str = line_num_str.trim();
            let rest_of_line = rest_of_line.trim();

            if let Ok(line_num) = line_num_str.parse::<usize>() {
                play.0.push((line_num, character.clone(), rest_of_line.to_string()));
            } else {
                if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                    eprintln!("Warning: Invalid line number '{}' in line '{}'", line_num_str, line);
                }
            }
        }
    }
}