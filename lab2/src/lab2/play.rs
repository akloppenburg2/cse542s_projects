use crate::lab2::scene_fragment::SceneFragment;
use crate::lab2::script_gen::grab_trimmed_file_lines;
use crate::DEBUG;
use crate::lab2::declarations::GEN_SCRIPT_ERR;

pub type ScriptConfig = Vec<(bool, String)>;
pub type Fragments = Vec<SceneFragment>;

pub struct Play {
    fragments: Fragments,
}

impl Play {
    pub fn new() -> Play {
        Play {
            fragments: Vec::new(),
        }
    }

    pub fn add_fragment(&mut self, fragment: SceneFragment) {
        self.fragments.push(fragment);
    }

    fn add_config(&self, line: &String, script_config: &mut ScriptConfig, part_files_dir: String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        
        if tokens.is_empty() {
            return;
        }

        if tokens[0] == "[scene]" {
            if tokens.len() == 1 {
                if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                    eprintln!("Warning: Missing scene title after [scene]");
                }
            } else {
                let title = tokens[1..].join(" ");
                script_config.push((true, title));
            }
        } else {
            script_config.push((false, part_files_dir.clone() + tokens[0]));
            if tokens.len() > 1 && DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                eprintln!("Warning: Extra tokens after file name in config: {}", line);
            }
        }
    }

    pub fn read_config(&self, config_file: &String, script_config: &mut ScriptConfig) -> Result<(), u8> {
        let mut lines = Vec::new();
        if grab_trimmed_file_lines(config_file, &mut lines).is_err() {
            eprintln!("Error: Failed to process file: '{}'", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        if lines.is_empty() {
            eprintln!("Error: Script file '{}' is empty.", config_file);
            return Err(GEN_SCRIPT_ERR);
        }

        for line in lines {
            self.add_config(&line, script_config, String::new());
        }
        Ok(())
    }

    pub fn process_config(&mut self, script_config: &ScriptConfig) -> Result<(), u8> {
        let mut title = String::new();

        for (is_new_scene, text) in script_config {
            if *is_new_scene {
                title = text.clone();
            } else {
                let mut fragment = SceneFragment::new(&title, &text);
                title.clear();
                
                if fragment.prepare(&text).is_err() {
                    eprintln!("Error: Failed to prepare fragment with '{}'", text);
                    return Err(GEN_SCRIPT_ERR);
                }
                
                self.add_fragment(fragment);
            }
        }
        Ok(())
    }

    pub fn prepare(&mut self, config_file: &String) -> Result<(), u8> {
        let mut script_config = ScriptConfig::new();
        self.read_config(config_file, &mut script_config)?;
        self.process_config(&script_config)?;

        if !self.fragments.is_empty() && !self.fragments[0].title().is_empty() {
            Ok(())
        } else {
            eprintln!("Error: No fragments with a title in the script.");
            Err(GEN_SCRIPT_ERR)
        }
    }

    pub fn recite(&mut self) {
        for i in 0..self.fragments.len() {
            if i == 0 {
                self.fragments[i].enter_all();
            } else {
                let previous_fragment = &self.fragments[i - 1];
                self.fragments[i].enter(previous_fragment);
            }

            self.fragments[i].recite();

            if i == self.fragments.len() - 1 {
                self.fragments[i].exit_all();
            } else {
                let next_fragment = &self.fragments[i + 1];
                self.fragments[i].exit(next_fragment);
            }
        }
    }
}