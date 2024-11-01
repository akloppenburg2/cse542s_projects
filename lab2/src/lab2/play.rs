// play.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Defines PlayLines and Player structs

use {
    super::{
        scene_fragment::SceneFragment,
        declarations::{DEBUG, GEN_SCRIPT_ERR},
        script_gen::grab_trimmed_file_lines,
    }
};

// Define 
pub type ScriptConfig = Vec<(bool, String)>;

pub type Fragments = Vec<SceneFragment>;

// Script generation constants
const CHAR_TOKEN_INDEX: usize = 0;
const NUM_TOKENS: usize = 2;

// Play struct declaration
pub struct Play {
    pub title: String,
    pub players: Fragments,
}

impl Play {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            players: Fragments::new(),
        }
    }

    // Function to process the ScriptConfig and generate the Play script
    pub fn process_config(&mut self, play_config: &ScriptConfig) -> Result<(), u8> {

        // Iterate through each tuple in ScriptConfig (character name, file name)
        for config in play_config {
            match config {
                (is_new, new_title) => {
                    if *is_new {
                        self.title = new_title.to_string();
                    } else {
                        let mut frag = SceneFragment::new(&self.title);
                        match frag.prepare(new_title) {
                            Err(e) => return Err(e),
                            _ => {}    
                        }
                        self.players.push(frag);
                        self.title.clear();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_config(line: &String, play_config: &mut ScriptConfig) {
        // Tokenize line
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.is_empty() {
            return;
        }

        if tokens[CHAR_TOKEN_INDEX] == "[scene]" && tokens.len() == 1 {
            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                eprintln!("Warning: missing scene title");
            }
            return;
        }
    
        if tokens[CHAR_TOKEN_INDEX] == "[scene]" && tokens.len() >= NUM_TOKENS {
            play_config.push((true, tokens.join(" ")));
        }
        else {
            play_config.push((false, tokens[CHAR_TOKEN_INDEX].to_string()));
            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && tokens.len() >= NUM_TOKENS {
                eprintln!("Warning: additional tokens in the line");
            }
        }
    }
    
    pub fn read_config(file: &String, play_config: &mut ScriptConfig) -> Result<(), u8> {
        // Vector for lines
        let mut lines = Vec::new();
    
        // Call grab_trimmed_file_lines to read and trim lines from the file
        if let Err(_) = grab_trimmed_file_lines(file, &mut lines) {
            eprintln!("Error: Failed to process file: '{}'", file);
            return Err(GEN_SCRIPT_ERR);
        }

        if lines.is_empty() {
            eprintln!("Error: No lines in file: '{}'", file);
        }
    
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
        let mut play_config = ScriptConfig::new(); 
        if let Err(_) = Self::read_config(config_file, &mut play_config){
            eprintln!("Error: Failed to read config '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        if let Err(_) = self.process_config(&play_config){
            eprintln!("Error: Failed to process config '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        if !self.players.is_empty() && !self.players[0].title.is_empty() {
            return Ok(())
        }

        eprintln!("Error: Invalid scene");
        return Err(GEN_SCRIPT_ERR);
    }

    pub fn recite(&mut self) {
        println!("{}", self.title);

        for idx in 0..self.players.len() {
            if idx == 0 {
                self.players[0].enter_all();
            } else {
                self.players[idx].enter(&self.players[idx-1])
            }
            self.players[idx].recite();
            if idx == self.players.len() - 1 {
                self.players[self.players.len() - 1].exit_all();
            } else {
                self.players[idx].exit(&self.players[idx+1]);
            }
            
        }
    }
}