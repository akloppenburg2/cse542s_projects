// declarations.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab1 types, constants, and static variables

pub const GEN_SCRIPT_ERR: u8 = 2; // Error for script generation failure

// Static variable for handling the "whinge" option (Debug mode)
pub static DEBUG: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);