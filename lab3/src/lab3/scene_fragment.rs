// scene_fragment.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo

use super::{
    declarations::{DEBUG, GEN_SCRIPT_ERR, PREPEND_INDEX, INITIAL_INDEX},
    player::Player,
    script_gen::grab_trimmed_file_lines,
};

use std::sync::{Arc, Mutex};
use std::cmp::Ordering;

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
            title: title.clone(),
            players: Vec::new(),
        }
    }

    // Function to process the PlayConfig and generate the SceneFragment script
    pub fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        for (char_name, file_name) in play_config {
            let mut player = Player::new(char_name); // Ensure `player` is mutable
            if let Err(e) = player.prepare(file_name) {
                return Err(e);
            }
            self.players.push(Arc::new(Mutex::new(player)));
        }
        Ok(())
    }

    pub fn compare_players(a: &Arc<Mutex<Player>>, b: &Arc<Mutex<Player>>) -> Ordering {
        let lock_a = a.lock();
        let lock_b = b.lock();

        if let (Ok(ref player_a), Ok(ref player_b)) = (lock_a, lock_b) {
            if let Some(order) = player_a.partial_cmp(player_b) {
                return order;
            }
        }
        Ordering::Equal
    }

    pub fn add_config(line: &String, play_config: &mut PlayConfig, path: &String) {
        let mut tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        if tokens.len() >= NUM_TOKENS {
            // Prepend the token with the path, if needed
            tokens[LINE_TOKEN_INDEX].insert_str(PREPEND_INDEX, path);

            // Once modified, push tokens to the play config
            play_config.push((
                tokens[LINE_NUM_TOKEN_INDEX].to_string(),
                tokens[LINE_TOKEN_INDEX].clone(),
            ));
        } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            eprintln!("Warning: Badly formed line in config: {}", line);
        }
    }

    pub fn read_config(config_file: &String, play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines = Vec::new();
        let path: String;

        if let Err(_) = grab_trimmed_file_lines(config_file, &mut lines) {
            eprintln!("Error: Failed to process file: '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        // Determine the directory path
        match config_file.rsplit_once('/') {
            None => path = "".to_string(),
            Some((dir_name, _)) => path = format!("{}/", dir_name),
        }

        for line in lines {
            Self::add_config(&line, play_config, &path);
        }

        Ok(())
    }

    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {
        let mut play_config = PlayConfig::new();
        if let Err(_) = Self::read_config(config_file, &mut play_config) {
            eprintln!("Error: Failed to read config '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        if let Err(_) = self.process_config(&play_config) {
            eprintln!("Error: Failed to process config '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        self.players.sort_by(SceneFragment::compare_players);
        Ok(())
    }

    pub fn recite(&mut self) {
        let mut last_speaker = String::new();
        let mut expected_line_num = INITIAL_INDEX;

        loop {
            let mut next_line_num = None;

            // Determine the next line to speak from all players
            for player_arc in &self.players {
                if let Ok(player) = player_arc.lock() {
                    if let Some(line_num) = player.next_line() {
                        if next_line_num.is_none() || line_num < next_line_num.unwrap() {
                            next_line_num = Some(line_num);
                        }
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

            let mut duplicates = INITIAL_INDEX;
            for player_arc in &mut self.players {
                if let Ok(mut player) = player_arc.lock() { // Ensure `player` is mutable
                    if player.next_line() == next_line_num {
                        player.speak(&mut last_speaker);
                        duplicates += 1;
                    }
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
            println!("\n{}\n", self.title);
        }
        for player_arc in &self.players {
            if let Ok(player) = player_arc.lock() {
                if other.players.iter().all(|p| p.lock().map_or(true, |other_player| other_player.name != player.name)) {
                    println!("[Enter {}.]", player.name);
                }
            }
        }
    }

    pub fn enter_all(&self) {
        if !self.title.trim().is_empty() {
            println!("{}", self.title);
        }
        println!();
        for player_arc in &self.players {
            if let Ok(player) = player_arc.lock() {
                println!("[Enter {}.]", player.name);
            }
        }
    }

    pub fn exit(&self, other: &SceneFragment) {
        println!();
        for idx in INITIAL_INDEX..self.players.len() {
            if other.players.iter().all(|p| p.lock().map_or(true, |other_player| other_player.name != self.players[self.players.len() - 1 - idx].lock().unwrap().name)) {
                println!("[Exit {}.]", self.players[self.players.len() - 1 - idx].lock().unwrap().name);
            }
        }
    }

    pub fn exit_all(&self) {
        println!();
        for idx in INITIAL_INDEX..self.players.len() {
            println!("[Exit {}.]", self.players[self.players.len() - 1 - idx].lock().unwrap().name);
        }
    }
}