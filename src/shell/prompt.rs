use std::io::Write;

use super::parser::Expandable;

/// Customizable prompt which can echo program output
pub struct Prompt {
    // Raw string for prompt from config
    raw: String,
    // Final string after command execution to print as prompt
    output: String,
}

impl Expandable for Prompt {}

impl Prompt {
    pub fn parse(&mut self) {
        self.output.clear();
        self.output = self.expand_sub_command(&self.raw);
    }

    pub fn print(&self) {
        print!("{}", self.output);
        std::io::stdout().flush().unwrap();
    }

    pub fn new(raw: &str) -> Self {
        Prompt {
            raw: raw.to_string(),
            output: String::new(),
        }
    }
}
