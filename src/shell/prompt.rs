use std::io::Write;

use crate::builtin::cd::DirManager;

use super::boosh_command::{BooshCommand, Executable, Parse};

/// Customizable prompt which can echo program output

pub struct Prompt {
    // Raw string for prompt from config
    raw: String,
    // // Go through tokens, if token starts with $, process the rest of the token as a command, else simply print
    // tokens: Vec<&'a str>,

    // Final string after command execution to print as prompt
    output: String,
}

impl Prompt {
    pub fn parse(self: &mut Self, dir_manager: &mut DirManager) {
        self.output.clear();
        let tokens: Vec<&str> = self.raw.split_whitespace().collect();

        for token in tokens.iter() {
            match token.split_at(1).0 {
                "$" => {
                    // Exec command and save to output
                    let program = token[1..].to_string();
                    let command: BooshCommand = BooshCommand::from_input(&program);
                    // print!("{:?} {:?}", command.program, command.args);

                    let out = command
                        .execute(dir_manager)
                        .unwrap_or_else(|| "".to_string());
                    self.output.push_str(out.as_str());
                }
                _ => {
                    self.output.push_str(token);
                }
            }
        }
    }

    /// Print the prompt output after execution of any commands
    pub fn print(self: &Self) {
        print!("{}", self.output);
        std::io::stdout().flush().unwrap();
    }

    pub fn new(raw: &String) -> Self {
        return Prompt {
            raw: raw.to_string(),
            output: String::new(),
        };
    }
}
