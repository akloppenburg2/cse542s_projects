// scene_fragment.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo

use super::{
    declarations::{DEBUG, GEN_SCRIPT_ERR, PREPEND_INDEX, INITIAL_INDEX},
    player::Player,
    script_gen::grab_trimmed_file_lines,
};
use std::sync::{Arc, Mutex};
use std::cmp::Ordering;
use std::io::{stderr, stdout, Write};
use std::thread;

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

    // Multithreaded process_config method
    pub fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        let mut thread_handles = Vec::new();

        for (char_name, file_name) in play_config {
            let char_name = char_name.clone();
            let file_name = file_name.clone();

            // Spawn a thread for each Player::prepare call
            let handle = thread::spawn(move || {
                let mut player = Player::new(&char_name);
                player.prepare(&file_name).map(|_| player)
            });

            thread_handles.push(handle);
        }

        // Join all threads and handle errors
        for handle in thread_handles {
            match handle.join() {
                Ok(Ok(player)) => {
                    self.players.push(Arc::new(Mutex::new(player)));
                }
                Ok(Err(e)) => {
                    writeln!(stderr().lock(), "Error: Failed to prepare player with error code {}", e).unwrap();
                    return Err(e);
                }
                Err(_) => {
                    writeln!(stderr().lock(), "Error: A thread panicked during Player::prepare.").unwrap();
                    return Err(GEN_SCRIPT_ERR);
                }
            }
        }

        self.players.sort_by(SceneFragment::compare_players);
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
            tokens[LINE_TOKEN_INDEX].insert_str(PREPEND_INDEX, path);
            play_config.push((
                tokens[LINE_NUM_TOKEN_INDEX].to_string(),
                tokens[LINE_TOKEN_INDEX].clone(),
            ));
        } else {
            writeln!(stderr().lock(), "Warning: Badly formed line in config: {}", line).unwrap();
        }
    }

    pub fn read_config(config_file: &String, play_config: &mut PlayConfig) -> Result<(), u8> {
        let mut lines = Vec::new();
        let path: String;

        if let Err(_) = grab_trimmed_file_lines(config_file, &mut lines) {
            writeln!(stderr().lock(), "Error: Failed to process file: '{}'", config_file).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

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
            panic!("Error: Failed to read config '{}'", config_file);
        }

        if let Err(_) = self.process_config(&play_config) {
            panic!("Error: Failed to process config '{}'", config_file);
        }

        Ok(())
    }

    pub fn recite(&mut self) {
        let mut last_speaker = String::new();
        let mut expected_line_num = INITIAL_INDEX;

        loop {
            let mut next_line_num = None;

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
                    writeln!(stderr().lock(), "Warning: Missing line {}", line).unwrap();
                }
            }

            let mut duplicates = INITIAL_INDEX;
            for player_arc in &mut self.players {
                if let Ok(mut player) = player_arc.lock() {
                    if player.next_line() == next_line_num {
                        player.speak(&mut last_speaker);
                        duplicates += 1;
                    }
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
        for player_arc in &self.players {
            if let Ok(player) = player_arc.lock() {
                if other.players.iter().all(|p| p.lock().map_or(true, |other_player| other_player.name != player.name)) {
                    writeln!(stdout().lock(), "[Enter {}.]", player.name).unwrap();
                }
            }
        }
    }

    pub fn enter_all(&self) {
        if !self.title.trim().is_empty() {
            writeln!(stdout().lock(), "{}", self.title).unwrap();
        }
        writeln!(stdout().lock()).unwrap();
        for player_arc in &self.players {
            if let Ok(player) = player_arc.lock() {
                writeln!(stdout().lock(), "[Enter {}.]", player.name).unwrap();
            }
        }
    }

    pub fn exit(&self, other: &SceneFragment) {
        writeln!(stdout().lock()).unwrap();
        for idx in INITIAL_INDEX..self.players.len() {
            if other.players.iter().all(|p| p.lock().map_or(true, |other_player| other_player.name != self.players[self.players.len() - 1 - idx].lock().unwrap().name)) {
                writeln!(stdout().lock(), "[Exit {}.]", self.players[self.players.len() - 1 - idx].lock().unwrap().name).unwrap();
            }
        }
    }

    pub fn exit_all(&self) {
        writeln!(stdout().lock()).unwrap();
        for idx in INITIAL_INDEX..self.players.len() {
            writeln!(stdout().lock(), "[Exit {}.]", self.players[self.players.len() - 1 - idx].lock().unwrap().name).unwrap();
        }
    }
}