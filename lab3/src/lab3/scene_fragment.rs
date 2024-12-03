// scene_fragment.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab 3
use std::sync::{Arc, Mutex};
use std::io::{stdout, stderr, Write};
use std::thread;

use super::declarations::{DEBUG, GEN_SCRIPT_ERR, PREPEND_INDEX, INITIAL_INDEX};
use super::player::Player;
use super::script_gen::grab_trimmed_file_lines;

// Define PlayConfig as a vector of (character name, file name) tuples
pub type PlayConfig = Vec<(String, String)>;

// Script generation constants
const LINE_NUM_TOKEN_INDEX: usize = 0;
const LINE_TOKEN_INDEX: usize = 1;
const NUM_TOKENS: usize = 2;

// SceneFragment struct declaration
pub struct SceneFragment {
    pub title: String,
    pub players: Vec<Arc<Mutex<Player>>>,
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
                    self.players.push(Arc::new(Mutex::new(player)));
                }
            }
        }
        Ok(())
    }

    pub fn add_config(line: &String, play_config: &mut PlayConfig, path: &String) {
        // Tokenize line
        let mut tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    
        if tokens.len() >= NUM_TOKENS {

            // Prepend the token with the path, if needed
            tokens[LINE_TOKEN_INDEX].insert_str(PREPEND_INDEX, path);

            // Once modified, push tokens to the play config
            play_config.push((tokens[LINE_NUM_TOKEN_INDEX].to_string(), tokens[LINE_TOKEN_INDEX].clone()));
        }
        else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            writeln!(stderr().lock(), "Warning: Badly formed line in config: {}", line).unwrap();
        }
    }
    
    pub fn read_config(config_file: &String, play_config: &mut PlayConfig) -> Result<(), u8> {
        // Vector for lines
        let mut lines = Vec::new();

        let path: String;
        
        // Call grab_trimmed_file_lines to read and trim lines from the file
        if let Err(_) = grab_trimmed_file_lines(config_file, &mut lines) {
            writeln!(stderr().lock(), "Error: Failed to process file: '{}'", config_file).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        // If config files are in a different directory we need to use the full path
        // Get that directory here so that we can prepend it to the config file names in the next step
        match config_file.rsplit_once('/')
        {
            None                => path = "".to_string(),
            Some((dir_name, _)) => path = dir_name.to_string() + "/",
        }
    
        // Add remaining elements to config
        for line in lines
        {
            Self::add_config(&line, play_config, &path);
        }

        Ok(())
    }

    // 
    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {
        // Initialize and then read config
        let mut play_config = PlayConfig::new();
        if let Err(_) = Self::read_config(config_file, &mut play_config){
            panic!("Error: Failed to read config '{}'", config_file);
        }

        if let Err(_) = self.process_config(&play_config){
            panic!("Error: Failed to process config '{}'", config_file);
        }

        self.players.sort_by(Self::player_sort);
        Ok(())
    }

    pub fn recite(&mut self) {
        let mut last_speaker = String::new();
        let mut expected_line_num = INITIAL_INDEX;

        loop {
            let mut next_line_num = None;

            // Determine the next line to speak from all players
            for player in &self.players {
                match player.lock()
                {
                    Ok(ref player_ref) => {
                        if let Some(line_num) = player_ref.next_line() 
                        {
                            if next_line_num.is_none() || line_num < next_line_num.unwrap() 
                            {
                                next_line_num = Some(line_num);
                            }
                        }
                    },
                    _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
                }
            }

            if next_line_num.is_none() {
                break;
            }

            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && next_line_num.unwrap() > expected_line_num {
                for line in expected_line_num..next_line_num.unwrap() {
                    writeln!(stderr().lock(), "Warning: Missing line {}", line).unwrap();
                }
            }

            let mut duplicates = INITIAL_INDEX;
            for player in &mut self.players {
                match player.lock()
                {
                    Ok(ref mut player_ref) => {
                        if player_ref.next_line() == next_line_num
                        {
                            player_ref.speak(&mut last_speaker);
                            duplicates += 1;
                        }
                    },
                    _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
                }
            }

            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && duplicates > 1 {
                writeln!(stderr().lock(), "Warning: Multiple speakers at the same time").unwrap();
            }

            expected_line_num = next_line_num.unwrap() + 1;
        }
    }

    pub fn enter(&self, other: &SceneFragment) {
        if !self.title.trim().is_empty() {
            writeln!(stdout().lock(), "\n{}\n", self.title).unwrap();
        }
        for player in &self.players {
            let mut contains = false;
            match player.lock()
            {
                Ok(ref player_ref) => {
                    for other_player in &other.players 
                    {
                        match other_player.lock()
                        {
                            Ok(ref other_ref) => {
                                if other_ref.name == player_ref.name
                                {
                                    contains = true;
                                }
                            },
                            _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
                        }
                    }

                    if !contains {
                        writeln!(stdout().lock(), "[Enter {}.]", player_ref.name).unwrap();
                    }
                },
                _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
            }
        }
    }

    pub fn enter_all(&self) {
        if !self.title.trim().is_empty() {
            writeln!(stdout().lock(), "{}", self.title).unwrap();
        }
        writeln!(stdout().lock(), "").unwrap();
        for player in &self.players {
            match player.lock()
            {
                Ok(ref player_ref) => writeln!(stdout().lock(), "[Enter {}.]", player_ref.name).unwrap(),
                _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
            }
        }
    }

    pub fn exit(&self, other: &SceneFragment) {
        writeln!(stdout().lock(), "").unwrap();
        for player in self.players.iter().rev() {
            let mut contains = false;
            match player.lock()
            {
                Ok(ref player_ref) => {
                    for other_player in &other.players {
                        match other_player.lock()
                        {
                            Ok(ref other_ref) => {
                                if other_ref.name == player_ref.name {
                                    contains = true;
                                }
                            },
                            _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
                        }
                    }
                    if !contains {
                        writeln!(stdout().lock(), "[Exit {}.]", player_ref.name).unwrap();
                    }
                
                },
                _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
            }
        }
    }

    pub fn exit_all(&self) {
        writeln!(stdout().lock(), "").unwrap();
        for player in self.players.iter().rev() {
            match player.lock()
            {
                Ok(ref player_ref) => writeln!(stdout().lock(), "[Exit {}.]", player_ref.name).unwrap(),
                _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
            }
        }
    }

    pub fn player_sort(player1: &Arc<Mutex<Player>>, player2: &Arc<Mutex<Player>>) -> std::cmp::Ordering
    {
        match player1.lock()
        {
            Ok(ref player1_ref) => {
                match player2.lock()
                { 
                    Ok(ref player2_ref) => {
                        match Player::partial_cmp(player1_ref, player2_ref)
                        {
                            Some(compare) => return compare,
                            _ => return std::cmp::Ordering::Equal,
                        }
                    },
                    _ => return std::cmp::Ordering::Equal,
                }
            },
            _ => return std::cmp::Ordering::Equal,
        }
    }
}