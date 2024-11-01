// scene_fragment.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// 

use {
    super::{
        declarations::{DEBUG, GEN_SCRIPT_ERR},
        player::Player,
        script_gen::grab_trimmed_file_lines,
    }
};

// Define PlayConfig as a vector of (character name, file name) tuples
pub type PlayConfig = Vec<(String, String)>;

// Script generation constants
const TITLE_INDEX: usize = 0;
const CHAR_TOKEN_INDEX: usize = 0;
const FILE_TOKEN_INDEX: usize = 1;
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
            play_config.push((tokens[CHAR_TOKEN_INDEX].to_string(), tokens[FILE_TOKEN_INDEX].to_string()));
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
        *play_title = lines.remove(TITLE_INDEX);
    
        // Add remaining elements to config
        for line in lines
        {
            Self::add_config(&line, play_config);
        }

        Ok(())
    }

    // Main script generation function
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

    // Method to get the title of the SceneFragment
    pub fn title(&self) -> &String {
        &self.title
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
            println!("{}", self.title);
        }

        for player in &self.players {
            if !other.players.contains(player) {
                println!("[Enter {}.]", player.name);
            }
        }
    }

    pub fn enter_all(&self) {
        if !self.title.trim().is_empty() {
            println!("{}", self.title);
        }

        for player in &self.players {
            println!("[Enter {}.]", player.name);
        }
    }

    pub fn exit(&self, other: &SceneFragment) {

        for idx in self.players.len()..0 {
            if !other.players.contains(&self.players[idx - 1]) {
                println!("[Exit {}.]", &self.players[idx - 1].name);
            }
        }
    }

    pub fn exit_all(&self) {
        for idx in self.players.len()..0 {
            println!("[Exit {}.]", &self.players[idx - 1].name);
        }
    }
}