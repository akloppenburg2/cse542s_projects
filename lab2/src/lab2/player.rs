use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::lab2::declarations::GEN_SCRIPT_ERR;
use crate::DEBUG;

pub type PlayLines = Vec<(usize, String)>;

pub struct Player {
    name: String,          // Character's name
    lines: PlayLines,      // Lines for the character
    index: usize,          // Current line index
}

impl Player {
    pub fn new(name: &String) -> Player {
        Player {
            name: name.clone(),
            lines: Vec::new(),
            index: 0,
        }
    }

    fn add_script_line(&mut self, line: &String) {
        if !line.trim().is_empty() {
            if let Some((line_num_str, rest_of_line)) = line.split_once(char::is_whitespace) {
                if let Ok(line_num) = line_num_str.trim().parse::<usize>() {
                    self.lines.push((line_num, rest_of_line.trim().to_string()));
                } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                    eprintln!("Warning: Invalid line number '{}' in line '{}'", line_num_str, line);
                }
            }
        }
    }

    pub fn prepare(&mut self, file_name: &String) -> Result<(), u8> {
        let file = File::open(file_name).map_err(|_| GEN_SCRIPT_ERR)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(content) = line {
                self.add_script_line(&content);
            } else {
                eprintln!("Error reading line from file '{}'", file_name);
                return Err(GEN_SCRIPT_ERR);
            }
        }

        // Sort lines by line number
        self.lines.sort_by_key(|line| line.0);

        Ok(())
    }

    pub fn speak(&mut self, last_speaker: &mut String) {
        if self.index >= self.lines.len() {
            return;
        }

        if &self.name != last_speaker {
            *last_speaker = self.name.clone();
            println!();
            println!("{}:", self.name);
        }

        println!("{}", self.lines[self.index].1);
        self.index += 1;
    }

    pub fn next_line(&self) -> Option<usize> {
        if self.index < self.lines.len() {
            Some(self.lines[self.index].0)
        } else {
            None
        }
    }
}