// play.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Defines PlayLines and Player structs
use std::sync::{Arc, Mutex};
use std::io::{stdout, stderr, Write};
use std::thread;

use super::scene_fragment::SceneFragment;
use super::declarations::{DEBUG, GEN_SCRIPT_ERR, PREPEND_INDEX, INITIAL_INDEX};
use super::script_gen::grab_trimmed_file_lines;

// Define 
pub type ScriptConfig = Vec<(bool, String)>;

pub type Fragments = Vec<Arc<Mutex<SceneFragment>>>;

// Script generation constants
const CHAR_TOKEN_INDEX: usize = 0;
const CONFIG_TOKEN_INDEX: usize = 0;
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
        let mut threads = Vec::new();

        // Iterate through each tuple in ScriptConfig (bool, file name/title)
        for config in play_config {
            match config {
                (is_new, new_title) => {
                    if *is_new {
                        self.title = new_title.to_string();
                    }
                    else {
                        let title = self.title.clone();
                        let config_file = new_title.clone();
                        // Spawn a thread for each SceneFragment::prepare call
                        let thread = thread::spawn(move || {
                            let mut frag = SceneFragment::new(&title);
                            frag.prepare(&config_file).map(|_| frag)
                        });

                        threads.push(thread);
                        self.title.clear();
                    }
                }
            }
        }

        // Join all threads and handle errors
        for thread in threads {
            match thread.join()
            {
                Ok(Ok(frag)) => {
                    self.players.push(Arc::new(Mutex::new(frag)));
                }
                Ok(Err(e)) => {
                    writeln!(stderr().lock(), "Error: SceneFragment preparation failed with error code {}", e).unwrap();
                    return Err(e);
                }
                Err(_) => {
                    writeln!(stderr().lock(), "Error: A thread panicked during SceneFragment::prepare()").unwrap();
                    return Err(GEN_SCRIPT_ERR); // Return general error code
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

        if tokens[CHAR_TOKEN_INDEX] == "[scene]" && tokens.len() < NUM_TOKENS {
            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                writeln!(stderr().lock(), "Warning: missing scene title").unwrap();
            }
            return;
        }
    
        if tokens[CHAR_TOKEN_INDEX] == "[scene]" && tokens.len() >= NUM_TOKENS {
            play_config.push((true, tokens.join(" ")));
        }
        else {
            play_config.push((false, tokens[CONFIG_TOKEN_INDEX].to_string()));
            if DEBUG.load(std::sync::atomic::Ordering::SeqCst) && tokens.len() >= NUM_TOKENS {
                writeln!(stderr().lock(), "Warning: additional tokens in the line").unwrap();
            }
        }
    }
    
    pub fn read_config(file: &String, play_config: &mut ScriptConfig) -> Result<(), u8> {
        // Vector for lines
        let mut lines = Vec::new();

        // String to hold the directory the config files are in
        let path: String;

        // Call grab_trimmed_file_lines to read and trim lines from the file
        if let Err(_) = grab_trimmed_file_lines(file, &mut lines) {
            writeln!(stderr().lock(), "Error: Failed to process file: '{}'", file).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        if lines.is_empty() {
            writeln!(stderr().lock(), "Error: No lines in file: '{}'", file).unwrap();
        }

        // If config files are in a different directory we need to use the full path
        // Get that directory here so that we can prepend it to the config file names in the next step
        match file.rsplit_once('/')
        {
            None                => path = "".to_string(),
            Some((dir_name, _)) => path = dir_name.to_string() + "/",
        }

        // Add remaining elements to config - if we need to prepend path to the config file names we do so here
        for mut line in lines
        {
            // Config file name lines contain no whitespace, so search for that
            if !line.contains("[scene]")
            {
                line.insert_str(PREPEND_INDEX, &path);
            }

            Self::add_config(&line, play_config);
        }

        Ok(())
    }

    // Main script generation function
    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {

        // Initialize and then read config
        let mut play_config = ScriptConfig::new(); 
        if let Err(_) = Self::read_config(config_file, &mut play_config){
            writeln!(stderr().lock(), "Error: Failed to read config '{}'", config_file).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        // Process config into struct
        if let Err(_) = self.process_config(&play_config){
            writeln!(stderr().lock(), "Error: Failed to process config '{}'", config_file).unwrap();
            return Err(GEN_SCRIPT_ERR);
        }

        // Check for lock and if available, verify scene fragment is prepared
        if !self.players.is_empty()
        {
            match self.players[INITIAL_INDEX].lock()
            {
                Ok(ref player_ref) => if !player_ref.title.is_empty() { return Ok(()) },
                _ => writeln!(stderr().lock(), "Unable to acquire lock on list of players!").unwrap(),
            }
            
        }

        writeln!(stderr().lock(), "Error: Invalid scene").unwrap();
        return Err(GEN_SCRIPT_ERR);
    }

    pub fn recite(&mut self) {
        writeln!(stdout().lock(), "{}", self.title).unwrap();

        for idx in INITIAL_INDEX..self.players.len() {
            if idx == INITIAL_INDEX {
                match self.players[idx].lock()
                {
                    Ok(ref idx_ref) => idx_ref.enter_all(),
                    _ => writeln!(stderr().lock(), "Error acquiring lock for enter_all!").unwrap(),
                }
            }
            else {
                match self.players[idx].lock()
                {
                    Ok(ref idx_ref) => match self.players[idx-1].lock()
                                        { Ok(ref prev_ref) => idx_ref.enter(prev_ref),
                                          _ => writeln!(stderr().lock(), "Error acquiring lock 2 for enter!").unwrap(),},
                    _ => writeln!(stderr().lock(), "Error acquiring lock 1 for enter!").unwrap(),
                }
            }

            match self.players[idx].lock()
            {
                Ok(ref mut idx_ref) => idx_ref.recite(),
                _ => writeln!(stderr().lock(), "Error acquiring lock for recite!").unwrap(),
            }

            if idx == self.players.len() - 1 {
                match self.players[idx].lock()
                {
                    Ok(ref idx_ref) => idx_ref.exit_all(),
                    _ => writeln!(stderr().lock(), "Error acquiring lock for exit_all!").unwrap(),
                }
            } else {
                match self.players[idx].lock()
                {
                    Ok(ref idx_ref) => match self.players[idx+1].lock()
                                        { Ok(ref prev_ref) => idx_ref.exit(prev_ref),
                                          _ => writeln!(stderr().lock(), "Error acquiring lock 2 for exit!").unwrap(),},
                    _ => writeln!(stderr().lock(), "Error acquiring lock 1 for exit!").unwrap(),
                }
            }
            
        }
    }
}