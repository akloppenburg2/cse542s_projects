// play.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Defines PlayLines and Player structs

use super::scene_fragment::SceneFragment;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{stderr, stdout, Write};

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

    // Multithreaded process_config method
    pub fn process_config(&mut self, play_config: &ScriptConfig) -> Result<(), u8> {
        let mut thread_handles = Vec::new();

        for (new_title, file_name) in play_config {
            if new_title == "[scene]" {
                self.title = file_name.clone();
            } else {
                let title = self.title.clone();
                let config_file = file_name.clone();

                // Spawn a thread for each SceneFragment::prepare call
                let handle = thread::spawn(move || {
                    let mut fragment = SceneFragment::new(&title);
                    fragment.prepare(&config_file).map(|_| fragment)
                });

                thread_handles.push(handle);
                self.title.clear();
            }
        }

        // Join all threads and handle errors
        for handle in thread_handles {
            match handle.join() {
                Ok(Ok(fragment)) => {
                    self.players.push(Arc::new(Mutex::new(fragment)));
                }
                Ok(Err(e)) => {
                    writeln!(stderr().lock(), "Error: SceneFragment preparation failed with error code {}", e).unwrap();
                    return Err(e);
                }
                Err(_) => {
                    writeln!(stderr().lock(), "Error: A thread panicked during SceneFragment::prepare.").unwrap();
                    return Err(1); // Return general error code
                }
            }
        }

        Ok(())
    }

    // Prepare the play based on the given configuration file
    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {
        let mut play_config = ScriptConfig::new();

        // Read the configuration file
        if let Err(_) = SceneFragment::read_config(config_file, &mut play_config) {
            writeln!(stderr().lock(), "Error: Failed to read config '{}'", config_file).unwrap();
            return Err(1);
        }

        // Process the configuration to generate the play
        if let Err(_) = self.process_config(&play_config) {
            writeln!(stderr().lock(), "Error: Failed to process config '{}'", config_file).unwrap();
            return Err(1);
        }

        // Ensure there are players and their titles are valid
        if !self.players.is_empty() {
            if let Ok(player) = self.players[0].lock() {
                if !player.title.is_empty() {
                    return Ok(());
                }
            } else {
                writeln!(stderr().lock(), "Error: Failed to lock SceneFragment during preparation.").unwrap();
            }
        }

        writeln!(stderr().lock(), "Error: Invalid scene").unwrap();
        Err(1)
    }

    // Function to recite the play
    pub fn recite(&mut self) {
        writeln!(stdout().lock(), "{}", self.title).unwrap();

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
                writeln!(stderr().lock(), "Error: Failed to lock SceneFragment during recitation.").unwrap();
            }
        }
    }
}