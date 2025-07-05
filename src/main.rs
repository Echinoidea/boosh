use builtin::cd::DirManager;
use shell::boosh_command::{BooshCommand, Executable, Parse};

use std::io;
use std::io::Write;

use std::process::{Command, Stdio};

mod builtin;
mod shell;

const PROMPT: &str = "> ";

/// TODO make cd - for last directory
/// TODO modularize code
/// TODO boosh parser
/// TODO boosh prompt config
/// TODO make boosh good and usable so that I can daily drive it
/// TODO color support
/// TODO C-l C-c etc

fn boosh_loop() {
    let mut dir_manager = DirManager::new();

    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        let input = input.trim().to_string();

        let command: BooshCommand = BooshCommand::from_input(&input);

        match input.as_str() {
            "exit" => {
                break;
            }
            _ => {
                command.execute(&mut dir_manager);
            }
        }
    }
}

fn main() {
    // Load config file if found

    boosh_loop();

    // Perform shutdown/cleanup
}
