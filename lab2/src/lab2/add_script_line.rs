use crate::{Play, DEBUG};
use super::script_gen::Scene;

pub fn add_script_line(_play: &mut Play, line: &String, character: &String, scene: &mut Scene) {
    if !line.trim().is_empty() {
        if let Some((line_num_str, rest_of_line)) = line.split_once(char::is_whitespace) {
            if let Ok(line_num) = line_num_str.trim().parse::<usize>() {
                scene.add_line(format!("{} ({}): {}", line_num, character, rest_of_line.trim()));
            } else if DEBUG.load(std::sync::atomic::Ordering::SeqCst) {
                eprintln!("Warning: Invalid line number '{}' in line '{}'", line_num_str, line);
            }
        }
    }
}