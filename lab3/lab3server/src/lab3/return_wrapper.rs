// return_wrapper.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo

use std::process::{Termination, ExitCode};
use std::io::{stderr, Write};

pub const OK_RESULT_WRAPPER: u8 = 0; // Define OK_RESULT_WRAPPER directly here

pub struct ReturnWrapper(u8);

impl ReturnWrapper {
    pub fn new(res: Result<(), u8>) -> Self {
        match res {
            Ok(_) => Self(OK_RESULT_WRAPPER),
            Err(e) => Self(e),
        }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.0 != OK_RESULT_WRAPPER {
            writeln!(stderr().lock(), "Error: {}", self.0).unwrap();
        }
        ExitCode::from(self.0)
    }
}