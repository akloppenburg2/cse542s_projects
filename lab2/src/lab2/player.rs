// player.rs

use std::cmp::Ordering;

pub type PlayLines = Vec<(usize, String)>;

pub struct Player {
    pub name: String,           // Character's name, made public for access in other modules
    lines: PlayLines,           // Lines for the character
    index: usize,               // Index of the current line
}

impl Player {
    pub fn new(name: &String) -> Player {
        Player {
            name: name.clone(),
            lines: Vec::new(),
            index: 0,
        }
    }

    // Public method to add a line to the player's lines
    pub fn add_line(&mut self, line_num: usize, text: String) {
        self.lines.push((line_num, text));
    }

    // Prepare method with unused `_file_name` parameter to suppress warnings
    pub fn prepare(&mut self, _file_name: &String) -> Result<(), u8> {
        // Add file handling code here if needed, or leave as is to suppress warnings
        Ok(())
    }

    pub fn next_line(&self) -> Option<usize> {
        if self.index < self.lines.len() {
            Some(self.lines[self.index].0)
        } else {
            None
        }
    }

    pub fn speak(&mut self, last_speaker: &mut String) {
        if self.index >= self.lines.len() {
            return;
        }

        if &self.name != last_speaker {
            *last_speaker = self.name.clone();
            println!();
            println!("{}:", self.name);
        }

        println!("{}", self.lines[self.index].1);
        self.index += 1;
    }
}

// Implement PartialEq and Eq for Player based on the specified criteria
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        // Both players are silent
        if self.lines.is_empty() && other.lines.is_empty() {
            return true;
        }
        // Both have lines and share the same first line number
        if !self.lines.is_empty() && !other.lines.is_empty() {
            return self.lines[0].0 == other.lines[0].0;
        }
        false
    }
}

impl Eq for Player {}

// Implement PartialOrd and Ord for Player based on the specified criteria
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.lines.is_empty(), other.lines.is_empty()) {
            // Both players are silent (equal)
            (true, true) => Ordering::Equal,
            // Self has no lines, so it’s less than the other player
            (true, false) => Ordering::Less,
            // Other player has no lines, so self is greater
            (false, true) => Ordering::Greater,
            // Both players have lines; compare based on the first line number
            (false, false) => self.lines[0].0.cmp(&other.lines[0].0),
        }
    }
}