// player.rs

pub type PlayLines = Vec<(usize, String)>;

pub struct Player {
    name: String,           // Character's name
    lines: PlayLines,       // Lines for the character
    index: usize,           // Index of the current line
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