// return_wrapper.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// Lab3
use std::process::{Termination, ExitCode};
use std::io::{stderr, Write};

use super::declarations::OK_RESULT_WRAPPER;

pub struct ReturnWrapper(u8);

impl ReturnWrapper {
    pub fn new(res: Result<(), u8>) -> Self {
        match res {
            Ok(_) => return Self(OK_RESULT_WRAPPER),
            Err(e) => return Self(e),
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