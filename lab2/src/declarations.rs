// declarations.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab1 types, constants, and static variables

// Define the Play struct which holds a vector of (line number, character, line text) tuples
pub struct Play(pub Vec<(usize, String, String)>);

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

// Error codes (use u8 type for compatibility with main.rs)
pub const CMD_LINE_ERR: u8 = 1;        // Error for command line argument issues
pub const GEN_SCRIPT_ERR: u8 = 2;      // Error for script generation failure

// Static variable for handling the "whinge" option (Debug mode)
pub static DEBUG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);