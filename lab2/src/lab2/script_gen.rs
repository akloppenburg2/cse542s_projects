use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::lab2::play::{Play, ScriptConfig}; // Updated to use ScriptConfig
use crate::lab2::scene_fragment::SceneFragment;
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

// Function to process the ScriptConfig and generate the Play script
pub fn process_config(play: &mut Play, script_config: &ScriptConfig) -> Result<(), u8> {
    let mut title = String::new();

    for (is_new_scene, text) in script_config {
        if *is_new_scene {
            title = text.clone();
        } else {
            let mut fragment = SceneFragment::new(&title, &text); // Updated to pass both title and text
            title.clear();
            
            // Updated prepare call with the correct single argument
            if fragment.prepare(&text).is_err() {
                eprintln!("Error: Failed to prepare fragment with '{}'", text);
                return Err(GEN_SCRIPT_ERR);
            }
            
            play.add_fragment(fragment); // Ensure Play struct has add_fragment method
        }
    }
    Ok(())
}

// Add a config entry
pub fn add_config(line: &String, script_config: &mut ScriptConfig, part_files_dir: String) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.is_empty() {
        return;  // Ignore blank lines
    }

    if tokens[0] == "[scene]" {
        if tokens.len() == 1 {
            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                eprintln!("Warning: Missing scene title after [scene]");
            }
        } else {
            let title = tokens[1..].join(" ");
            script_config.push((true, title));
        }
    } else {
        script_config.push((false, part_files_dir.clone() + tokens[0]));
        if tokens.len() > 1 && DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            eprintln!("Warning: Extra tokens after file name in config: {}", line);
        }
    }
}

// Read configuration from a file
pub fn read_config(config_file: &String, script_config: &mut ScriptConfig) -> Result<(), u8> {
    let mut lines = Vec::new();
    if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
        eprintln!("Error: Failed to process file: '{}'", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    if lines.is_empty() {
        eprintln!("Error: Script file '{}' is empty.", config_file);
        return Err(GEN_SCRIPT_ERR);
    }

    for line in lines {
        add_config(&line, script_config, String::new());
    }
    Ok(())
}