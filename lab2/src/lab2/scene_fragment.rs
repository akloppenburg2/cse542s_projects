// scene_fragment.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// 

use super::{
    declarations::{DEBUG, GEN_SCRIPT_ERR},
    player::Player,
    script_gen::grab_trimmed_file_lines,
};

// Define PlayConfig as a vector of (character name, file name) tuples
pub type PlayConfig = Vec<(String, String)>;

// Script generation constants
const LINE_NUM_TOKEN_INDEX: usize = 0;
const LINE_TOKEN_INDEX: usize = 1;
const NUM_TOKENS: usize = 2;

// SceneFragment struct declaration
pub struct SceneFragment {
    pub title: String,
    pub players: Vec<Player>,
}

impl SceneFragment {
    pub fn new(title: &String) -> Self {
        Self {
            title: title.to_string(),
            players: Vec::new(),
        }
    }

    // Function to process the PlayConfig and generate the SceneFragment script
    pub fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {

        // Iterate through each tuple in PlayConfig (character name, file name)
        for config in play_config {
            match config {
                (char_name, file_name) => {
                    let mut player = Player::new(char_name);
                    if let Err(e) = player.prepare(file_name) {
                        return Err(e);
                    }
                    self.players.push(player);
                }
            }
        }
        Ok(())
    }

    pub fn add_config(line: &String, play_config: &mut PlayConfig) {
        // Tokenize line
        let tokens: Vec<&str> = line.split_whitespace().collect();
    
        if tokens.len() >= NUM_TOKENS {
            play_config.push((tokens[LINE_NUM_TOKEN_INDEX].to_string(), tokens[LINE_TOKEN_INDEX..].join(" ")));
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
    
        // Add remaining elements to config
        for line in lines
        {
            Self::add_config(&line, play_config);
        }

        Ok(())
    }

    // 
    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {
        // Initialize and then read config
        let mut play_config = PlayConfig::new(); 
        if let Err(_) = Self::read_config(config_file, &mut self.title, &mut play_config){
            eprintln!("Error: Failed to read config '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        if let Err(_) = self.process_config(&play_config){
            eprintln!("Error: Failed to process config '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        self.players.sort();
        Ok(())
    }

    pub fn recite(&mut self) {
        let mut last_speaker = String::new();
        let mut expected_line_num = 0;

        loop {
            let mut next_line_num = None;

            // Determine the next line to speak from all players
            for player in &self.players {
                if let Some(line_num) = player.next_line() {
                    if next_line_num.is_none() || line_num < next_line_num.unwrap() {
                        next_line_num = Some(line_num);
                    }
                }
            }

            if next_line_num.is_none() {
                break;
            }

            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && next_line_num.unwrap() > expected_line_num {
                for line in expected_line_num..next_line_num.unwrap() {
                    eprintln!("Warning: Missing line {}", line);
                }
            }

            let mut duplicates = 0;
            for player in &mut self.players {
                if player.next_line() == next_line_num {
                    player.speak(&mut last_speaker);
                    duplicates += 1;
                }
            }

            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && duplicates > 1 {
                eprintln!("Warning: Multiple speakers at the same time");
            }

            expected_line_num = next_line_num.unwrap() + 1;
        }
    }

    pub fn enter(&self, other: &SceneFragment) {
        if !self.title.trim().is_empty() {
            println!{""};
            println!("{}", self.title);
            println!{""};
        }
        for player in &self.players {
            let mut contains = false;
            for other_player in &other.players {
                if other_player.name == player.name {
                    contains = true;
                }
            }

            if !contains {
                println!("[Enter {}.]", player.name);
            }
        }
    }

    pub fn enter_all(&self) {
        if !self.title.trim().is_empty() {
            println!("{}", self.title);
        }
        println!{""};
        for player in &self.players {
            println!("[Enter {}.]", player.name);
        }
    }

    pub fn exit(&self, other: &SceneFragment) {
        println!{""};
        for idx in 0..self.players.len() {
            let mut contains = (false, idx);
            for other_player in &other.players {
                if other_player.name == (&self.players[self.players.len()-1-idx]).name {
                    contains = (true, 0);
                }
            }
            if !contains.0 {
                println!("[Exit {}.]", &self.players[self.players.len()-1-contains.1].name);
            }
        }
    }

    pub fn exit_all(&self) {
        println!{""};
        for idx in 0..self.players.len() {
            println!("[Exit {}.]", &self.players[self.players.len()-1-idx].name);
        }
    }
}