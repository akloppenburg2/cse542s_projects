// declarations.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 types, constants, and static variables

use std::sync::atomic::AtomicBool;

// Error codes for script generation and other operations
pub const GEN_SCRIPT_ERR: u8 = 2;           // Error code for script generation failure
pub const CMD_LINE_ERR: u8 = 1;             // Error code for command line argument issues

// OK exit codes
pub const OK_RESULT: usize = 0;            // OK result for usize operations
pub const OK_RESULT_WRAPPER: u8 = 0;       // OK result for u8 operations

// Static variable for debug mode
pub static DEBUG: AtomicBool = AtomicBool::new(false); // Handles the "whinge" option (Debug mode)

// Constants for handling paths and player/line indices
pub const PREPEND_INDEX: usize = 0;        // Starting index for path prepending
pub const INITIAL_INDEX: usize = 0;        // Starting index for lists of players and/or lines