// add_script_line.rs
use std::io::{stderr, stdout, Write};

use crate::lab3::player::Player; // Ensure correct import path for Player
use crate::DEBUG;

pub fn add_script_line(player: &mut Player, line: &String) {
    if !line.trim().is_empty() {
        if let Some((line_num_str, rest_of_line)) = line.split_once(char::is_whitespace) {
            if let Ok(line_num) = line_num_str.trim().parse::<usize>() {
                player.add_line(line_num, rest_of_line.trim().to_string());
            } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                writeln!(stderr().lock(), "Warning: Invalid line number '{}' in line '{}'", line_num_str, line).unwrap();
            }
        }
    }
}