use crate::builtin::cd::DirManager;
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
    pub fn parse(&mut self, _dir_manager: &mut DirManager) {
        self.output.clear();
        // Use the trait method directly
        self.output = self.expand_sub_command(&self.raw);
    }

    /// Print the prompt output after execution of any commands
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
