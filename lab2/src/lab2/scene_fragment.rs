use crate::lab2::player::Player;
use crate::DEBUG;
use std::collections::HashSet;

pub struct SceneFragment {
    title: String,
    text: String,
    players: Vec<Player>,
}

impl SceneFragment {
    pub fn new(title: &String, text: &String) -> SceneFragment {
        SceneFragment {
            title: title.clone(),
            text: text.clone(),
            players: Vec::new(),
        }
    }

    // Public method to add a player to the scene fragment
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    // Updates the fragment's text; expects `text` argument as `&String`
    pub fn prepare(&mut self, text: &String) -> Result<(), u8> {
        self.text = text.clone();
        Ok(())
    }

    // Method to get the title of the SceneFragment
    pub fn title(&self) -> &String {
        &self.title
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
}