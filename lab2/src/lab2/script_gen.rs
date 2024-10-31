use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::lab2::play::Play;
use crate::lab2::add_script_line::add_script_line;
use crate::{DEBUG, GEN_SCRIPT_ERR};

// Define PlayConfig as a vector of (character name, file name) tuples
pub type PlayConfig = Vec<(String, String)>;

// Script generation constants
pub const TITLE_INDEX: usize = 0;
pub const NUM_TOKENS: usize = 2;

// Define the Scene struct to represent each scene in the play
pub struct Scene {
    pub lines: Vec<String>, // Holds lines in the scene
}

impl Scene {
    // Constructor for a new scene
    pub fn new() -> Scene {
        Scene { lines: Vec::new() }
    }

    // Add a line to the scene
    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    // Display the scene's content
    pub fn display(&self) {
        println!("Scene:");
        for line in &self.lines {
            println!("{}", line);
        }
    }
}

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
    for (part_name, file_name) in play_config {
        let mut lines = Vec::new();
        if grab_trimmed_file_lines(file_name, &mut lines).is_err() {
            eprintln!("Error: Failed to process file for part '{}'", part_name);
            return Err(GEN_SCRIPT_ERR);
        }

        let mut scene = Scene::new();
        for line in &lines {
            add_script_line(play, line, part_name, &mut scene);
        }
        play.add_scene(scene); // Adds the scene to the play
    }
    Ok(())
}

// Add a config entry
pub fn add_config(line: &String, play_config: &mut PlayConfig, part_files_dir: String) {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() == NUM_TOKENS {
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

    *play_title = lines.remove(TITLE_INDEX);

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