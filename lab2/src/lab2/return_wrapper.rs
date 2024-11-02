// return_wrapper.rs
// Benjamin Kim, Alex Kloppenburg, Sam Yoo
// 

use std::process::{Termination, ExitCode};

pub struct ReturnWrapper(u8);

impl ReturnWrapper {
    pub fn new(res: Result<(), u8>) -> Self {
        match res {
            Ok(_) => return Self(0),
            Err(e) => return Self(e),
        }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.0 != 0 {
            eprintln!("Error: {}", self.0);
        }
        ExitCode::from(self.0)
    }
}