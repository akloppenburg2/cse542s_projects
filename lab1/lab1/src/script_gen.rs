use std::fs::File;
use std::io::{BufRead, BufReader};  // Remove the `self` part since it's unused
use crate::declarations::{Play, GEN_SCRIPT_ERR, DEBUG};

pub fn script_gen(config_file: &String, play_title: &mut String, play: &mut Play) -> Result<(), u8> {
    // Open the configuration file and map the io::Error to u8 error using GEN_SCRIPT_ERR
    let file = File::open(config_file).map_err(|_| GEN_SCRIPT_ERR)?;
    let reader = BufReader::new(file);
    
    let mut lines = reader.lines();
    
    // Read the title of the play (first line)
    if let Some(Ok(title)) = lines.next() {
        *play_title = title;
    } else {
        // If there's no valid title line, return GEN_SCRIPT_ERR
        return Err(GEN_SCRIPT_ERR);
    }

    // Iterate through the rest of the configuration file
    for line in lines {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 2 {
                let character_name = parts[0].to_string();
                let character_file = parts[1];

                // Attempt to read the character's script file and map errors to GEN_SCRIPT_ERR
                if let Err(_) = read_character_file(character_file, &character_name, play) {
                    return Err(GEN_SCRIPT_ERR);
                }
            } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                eprintln!("Warning: Badly formed line in config: {}", line);
            }
        }
    }

    Ok(())
}

// Function to read lines from a character's file and append them to the Play structure
pub fn read_character_file(file_name: &str, character_name: &str, play: &mut Play) -> Result<(), u8> {
    let file = File::open(file_name).map_err(|_| GEN_SCRIPT_ERR)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if let Some((line_num_str, line_text)) = trimmed.split_once(' ') {
                    if let Ok(line_num) = line_num_str.parse::<usize>() {
                        play.0.push((line_num, character_name.to_string(), line_text.to_string()));
                    } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                        eprintln!("Warning: Invalid line number: {}", line_num_str);
                    }
                }
            }
        }
    }

    Ok(())
}