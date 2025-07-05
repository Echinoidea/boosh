use builtin::cd::DirManager;
use shell::boosh_command::{BooshCommand, Executable, Parse};
use shell::prompt::{self, Prompt};

use std::io;
use std::io::Write;

use std::process::{Command, Stdio};

mod builtin;
mod shell;

const PROMPT: &str = "$date > ";

/// TODO make cd - for last directory
/// TODO modularize code
/// TODO boosh parser
/// TODO boosh prompt config
/// TODO make boosh good and usable so that I can daily drive it
/// TODO color support
/// TODO C-l C-c etc

fn boosh_loop() {
    let mut dir_manager = DirManager::new();
    let mut prompt = Prompt::new(&PROMPT.to_owned());

    loop {
        prompt.parse(&mut dir_manager);
        prompt.print();
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

        println!("");
    }
}

fn main() {
    // Load config file if found

    boosh_loop();

    // Perform shutdown/cleanup
}
