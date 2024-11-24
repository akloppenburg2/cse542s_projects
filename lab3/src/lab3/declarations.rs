// declarations.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3 types, constants, and static variables

// Error for script generation failure
pub const GEN_SCRIPT_ERR: u8 = 2;

// OK exit code
pub const OK_RESULT: usize = 0;

// Static variable for handling the "whinge" option (Debug mode)
pub static DEBUG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

// Path handling constant
pub const PREPEND_INDEX: usize = 0;

// Starting index for lists of players and/or lines
pub const INITIAL_INDEX: usize = 0;
