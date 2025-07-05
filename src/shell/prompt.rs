use super::boosh_command::{BooshCommand, Executable, Parse};

/// Customizable prompt which can echo program output

pub struct Prompt<'a> {
    // Raw string for prompt from config
    raw: &'a String,
    // Go through tokens, if token starts with $, process the rest of the token as a command, else simply print
    tokens: Vec<&'a str>,

    // Final string after command execution to print as prompt
    output: &'a str,
}

impl Prompt<'_> {
    fn parse(self: &Self) {
        let tokens: Vec<&str> = self.raw.split_whitespace().collect();

        for token in tokens.iter() {
            match token.split_at(1).0 {
                "$" => {
                    // Exec command and save to output
                    let command: BooshCommand = BooshCommand::from_input(&self.raw);
                }
                _ => {
                    todo!();
                }
            }
        }
    }

    /// Print the prompt output after execution of any commands
    fn print(self: &Self) {
        todo!()
    }
}
