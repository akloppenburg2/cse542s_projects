// main.rs
// Benjamin Kim, name, name
// Lab1 main

use std::env;

include!("declarations.rs");

fn main() -> Result<(), u8> {
    Ok(())
}

fn usage(name: &String) {
    println!("usage: {name} <configuration_file> [whinge]")
}

fn parse_args(config: &mut String) -> Result<(), u8> {
    let mut args = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < 2 || args.len() > 3 || (args.len() == 3 && args[2] != "whinge") {
        usage(&args[0]);
        return Err(CMD_LINE_ERR as u8);
    } else {
        *config = args[1].clone();
        if args[2] == "whinge" {
            DEBUG.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    Ok(())
}

fn recite(title: &String, play: &Play) {
    println!("{title}");

    let mut curr = 0;
    let mut current_character: Option<String> = None;

    for line in &play.0 {
        match line {
            (order, character, text) => {
                if Some(character.clone()) != current_character {
                    println!();
                    println!("{character}. {text}");
                    current_character = Some(character.clone());
                } else {
                    println!("{text}");
                }
            }       
        }
    }
}