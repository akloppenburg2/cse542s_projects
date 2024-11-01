// player.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Defines PlayLines and Player structs

use {
    super::{
        declarations::{DEBUG, GEN_SCRIPT_ERR},
        script_gen::grab_trimmed_file_lines,
    }
};

// Define the PlayLines struct which holds a vector of (line number, line text) tuples
pub type PlayLines = Vec<(usize, String)>;

// Define Player struct which holds Player name
#[derive(Eq, PartialOrd)]
pub struct Player {
    pub name: String,
    pub lines: PlayLines,
    pub index: usize,
}

// Implmentation of Player
impl Player {
    pub fn new(name: &String) -> Self {
        Self {
            name: name.to_string(),
            lines: Vec::new(),
            index: 0,
        }
    }

    // The add_script_line function to process each line of the character's file
    fn add_script_line(&mut self, line: &String) {
        if !line.trim().is_empty() {
            if let Some((line_num_str, rest_of_line)) = line.split_once(char::is_whitespace) {
                let line_num_str = line_num_str.trim();
                let rest_of_line = rest_of_line.trim();

                if let Ok(line_num) = line_num_str.parse::<usize>() {
                    self.lines.push((line_num, rest_of_line.to_string()));
                } else {
                    if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                        eprintln!("Warning: Invalid line number '{}' in line '{}'", line_num_str, line);
                    }
                }
            }
        }
    }

    pub fn prepare(&mut self, part_name: &String) -> Result<(), u8> {    
        // Vector to store lines
        let mut lines = Vec::new();

        // Call grab_trimmed_file_lines to read and trim lines from the file
        if let Err(_) = grab_trimmed_file_lines(part_name, &mut lines) {
            eprintln!("Error: Failed to process file for part '{}'", part_name);
            return Err(GEN_SCRIPT_ERR);
        }

        // Add each line to the Play using add_script_line
        for line in &lines {
            self.add_script_line(line);
        }
        
        self.lines.sort();
        Ok(())
    }

    pub fn speak(&mut self, char_name: &mut String) {
        if self.index > self.lines.len() {
            return;
        }

        if char_name != &self.name {
            *char_name = self.name.clone();
            println!("");
            println!("{}", self.name);
        }

        println!("{:?}", self.lines[self.index]);
        self.index += 1;
    }

    pub fn next_line(&self) -> Option<usize> {
        if self.index < self.lines.len() {
            return Some(self.lines[self.index].0);
        }
        None
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        // Both players are silent if they have no lines
        if self.next_line().is_none() && other.next_line().is_none() {
            return true;
        }
        
        // Both players have lines, check if the first line numbers are the same
        if let (Some(self_line), Some(other_line)) = (self.next_line(), other.next_line()) {
            return self_line == other_line;
        }
        
        false
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Case 1: Check if one player is silent and the other is not
        if self.next_line().is_none() && other.next_line().is_some() {
            return std::cmp::Ordering::Less; // self is less
        }

        if self.next_line().is_some() && other.next_line().is_none() {
            return std::cmp::Ordering::Greater; // self is greater
        }

        // Case 2: Both have lines, compare first line numbers
        if let (Some(self_line), Some(other_line)) = (self.next_line(), other.next_line()) {
            if self_line > other_line {
                return std::cmp::Ordering::Less; // self is less
            } else {
                return std::cmp::Ordering::Greater; // self is greater
            }
        }

        std::cmp::Ordering::Equal
    }
}
