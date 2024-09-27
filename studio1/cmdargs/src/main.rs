use std::env;
use std::process::ExitCode;

const BAD_ARGS: u8 = 1;

fn main() -> Result<(), u8>
{
    // Create vector
    let mut args = Vec::new();
    
    // Gather arguments
    for arg in env::args()
    {
        args.push(arg);
    }

    // Check length, print usage if no input and error out
    if args.len() == 1
    {
        eprintln!("usage: {} <arg1> [<arg2> ...]", args[args.len()]);
        return Err(BAD_ARGS);
    }
    // otherwise,  print args
    else
    {
        println!("arguments passed to {} were {:?}", args.first().unwrap(), &args[1..]);
    }

    // If no issues, return success
    return Ok(());
}
