// declarations.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab1 and Lab2: types, constants, and static variables shared across modules

use std::sync::atomic::AtomicBool;

// Error codes (use u8 type for compatibility with main.rs)
pub const CMD_LINE_ERR: u8 = 1;        // Error for command line argument issues
pub const GEN_SCRIPT_ERR: u8 = 2;      // Error for script generation failure

// Command line argument constants
#[allow(dead_code)]
pub const MAX_ARGS: usize = 3;
#[allow(dead_code)]
pub const MIN_ARGS: usize = 2;
#[allow(dead_code)]
pub const PROGRAM_NAME: usize = 0;
#[allow(dead_code)]
pub const CONFIG_FILE: usize = 1;
#[allow(dead_code)]
pub const OPT: usize = 2;

// Static variable for handling the "whinge" option (Debug mode)
pub static DEBUG: AtomicBool = AtomicBool::new(false);

// Type alias for play configuration
pub type PlayConfig = Vec<(String, String)>; // Stores (character name, file name)

// Define a Play struct for Lab 1 (if still used in other parts of Lab 2, otherwise remove or update as needed)
pub struct Play(pub Vec<(usize, String, String)>); // Holds (line number, character, line text) tuples
