// play.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Defines PlayLines and Player structs

use super::scene_fragment::SceneFragment;
use std::sync::{Arc, Mutex};

// Define
pub type ScriptConfig = Vec<(String, String)>;
pub type Fragments = Vec<Arc<Mutex<SceneFragment>>>;

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
        for (new_title, file_name) in play_config {
            if new_title == "[scene]" {
                self.title = file_name.clone();
            } else {
                let mut frag = SceneFragment::new(&self.title);
                if let Err(e) = frag.prepare(file_name) {
                    return Err(e);
                }
                self.players.push(Arc::new(Mutex::new(frag)));
                self.title.clear();
            }
        }
        Ok(())
    }

    // Function to prepare the play based on the given configuration file
    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {
        let mut play_config = ScriptConfig::new();

        // Read the configuration file
        if let Err(_) = SceneFragment::read_config(config_file, &mut play_config) {
            eprintln!("Error: Failed to read config '{}'", config_file);
            return Err(1);
        }

        // Process the configuration to generate the play
        if let Err(_) = self.process_config(&play_config) {
            eprintln!("Error: Failed to process config '{}'", config_file);
            return Err(1);
        }

        // Ensure there are players and their titles are valid
        if !self.players.is_empty() {
            if let Ok(player) = self.players[0].lock() {
                if !player.title.is_empty() {
                    return Ok(());
                }
            } else {
                eprintln!("Error: Failed to lock SceneFragment during preparation.");
            }
        }

        eprintln!("Error: Invalid scene");
        Err(1)
    }

    // Function to recite the play
    pub fn recite(&mut self) {
        println!("{}", self.title);

        // Define dummy fragments outside the loop for longer lifetime
        let dummy_prev = Arc::new(Mutex::new(SceneFragment::new(&self.title)));
        let dummy_next = Arc::new(Mutex::new(SceneFragment::new(&self.title)));

        for idx in 0..self.players.len() {
            let current = self.players[idx].lock();
            let previous = if idx > 0 {
                self.players[idx - 1].lock()
            } else {
                dummy_prev.lock()
            };
            let next = if idx < self.players.len() - 1 {
                self.players[idx + 1].lock()
            } else {
                dummy_next.lock()
            };

            if let Ok(mut current_fragment) = current {
                if idx == 0 {
                    current_fragment.enter_all();
                } else if let Ok(ref prev_fragment) = previous {
                    current_fragment.enter(prev_fragment);
                }

                current_fragment.recite();

                if idx == self.players.len() - 1 {
                    current_fragment.exit_all();
                } else if let Ok(ref next_fragment) = next {
                    current_fragment.exit(next_fragment);
                }
            } else {
                eprintln!("Error: Failed to lock SceneFragment during recitation.");
            }
        }
    }
}