// scene_fragment.rs

use crate::lab2::player::Player;
use crate::lab2::script_gen::grab_trimmed_file_lines;
use crate::DEBUG;
use crate::lab2::declarations::GEN_SCRIPT_ERR;
use std::collections::HashSet;

pub type PlayConfig = Vec<(String, String)>;

pub struct SceneFragment {
    title: String,
    players: Vec<Player>,
}

impl SceneFragment {
    pub fn new(title: &String) -> SceneFragment {
        SceneFragment {
            title: title.clone(),
            players: Vec::new(),
        }
    }

    // Public method to add a player to the scene fragment
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn prepare(&mut self, config_file: &String, part_files_dir: &String) -> Result<(), u8> {
        let play_config = self.read_config(config_file, part_files_dir)?;
        self.process_config(&play_config)?;
        self.players.sort(); // Sort players by their first line for proper order
        Ok(())
    }

    pub fn recite(&mut self) {
        if !self.title.trim().is_empty() {
            println!("{}", self.title);
        }
        let mut last_speaker = String::new();
        let mut expected_line_num = 0;

        loop {
            let mut next_line_num = None;

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

            let next_line_num = next_line_num.unwrap();

            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && next_line_num > expected_line_num {
                eprintln!("Warning: Missing line {}", expected_line_num);
            }
            expected_line_num = next_line_num + 1;

            for player in &mut self.players {
                if player.next_line() == Some(next_line_num) {
                    player.speak(&mut last_speaker);
                }
            }
        }
    }

    // Enter method to announce new players entering
    pub fn enter(&self, previous_scene: &SceneFragment) {
        let previous_players: HashSet<_> = previous_scene.players.iter().map(|p| &p.name).collect();
        for player in &self.players {
            if !previous_players.contains(&player.name) {
                println!("[Enter {}.]", player.name);
            }
        }
    }

    // Announce all players in the current scene
    pub fn enter_all(&self) {
        for player in &self.players {
            println!("[Enter {}.]", player.name);
        }
    }

    // Exit method to announce players exiting
    pub fn exit(&self, next_scene: &SceneFragment) {
        let next_players: HashSet<_> = next_scene.players.iter().map(|p| &p.name).collect();
        for player in self.players.iter().rev() {
            if !next_players.contains(&player.name) {
                println!("[Exit {}.]", player.name);
            }
        }
    }

    // Announce all players exiting the current scene
    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Exit {}.]", player.name);
        }
    }

    fn read_config(&mut self, config_file: &String, part_files_dir: &String) -> Result<PlayConfig, u8> {
        let mut lines = Vec::new();
        if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
            eprintln!("Error: Failed to process file: '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        self.title = lines.remove(0);
        let mut play_config = PlayConfig::new();

        for line in lines {
            self.add_config(&line, &mut play_config, part_files_dir.to_string());
        }
        Ok(play_config)
    }

    fn process_config(&mut self, play_config: &PlayConfig) -> Result<(), u8> {
        for (character_name, file_name) in play_config {
            let mut player = Player::new(character_name);
            if player.prepare(file_name).is_err() {
                eprintln!("Error preparing player {}", character_name);
                return Err(GEN_SCRIPT_ERR);
            }
            self.add_player(player);
        }
        Ok(())
    }

    fn add_config(&self, line: &String, play_config: &mut PlayConfig, part_files_dir: String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() == 2 {
            play_config.push((tokens[0].to_string(), part_files_dir + tokens[1]));
        } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
            eprintln!("Warning: Badly formed line in config: {}", line);
        }
    }
}