use shell::boosh_command::{BooshCommand, Executable, Parse};
use shell::environment::BooshEnvironment;
use shell::prompt::Prompt;

use std::io;
use std::io::Write;

mod builtin;
mod shell;

const PROMPT: &str =
    "\n$(echo -e \\e[36m)$(pwd)$(echo -e \\e[0m)\n$(echo -e \\e[34mboosh\\e[0m)$(echo -e \\e[32m) $(date +%H:%M)$(echo -e \\e[0m) $(echo -e \\e[31m)gabriel$(echo -e \\e[0m) $ ";

/// DONE make cd - for last directory
/// TODO modularize code
/// TODO boosh parser
/// TODO boosh prompt config
/// TODO make boosh good and usable so that I can daily drive it
/// TODO color support
/// TODO C-l C-c etc

fn boosh_loop() {
    let mut boosh_env = BooshEnvironment::new();
    let mut prompt = Prompt::new(&PROMPT.to_owned());

    loop {
        prompt.parse();
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
                command.execute(&mut boosh_env);
            }
        }
    }
}

fn main() {
    // Load config file if found

    boosh_loop();

    // Perform shutdown/cleanup
}
