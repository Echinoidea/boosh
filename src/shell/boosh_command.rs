use super::environment::BooshEnvironment;
use super::parser::{Expandable, Tokenizable};
use crate::builtin::cd::change_directory;
use std::process::{Command, Stdio};

/// Struct storing a single command, as in a single program with args. Can be piped.
pub struct BooshCommand {
    pub program: String,
    pub args: Vec<String>,
}

impl Expandable for BooshCommand {}
impl Tokenizable for BooshCommand {}

pub trait Parse {
    /// Construct a new BooshCommand from a raw string input
    fn from_input(input: &str) -> BooshCommand;
}

impl Parse for BooshCommand {
    /// Take a raw string, tokenize it, and parse it into a BooshCommand instance
    fn from_input(input: &str) -> BooshCommand {
        // Create a temporary instance to use the trait methods
        let temp = BooshCommand {
            program: String::new(),
            args: Vec::new(),
        };

        let tokens = temp.parse_tokens(input);
        let (program, args) = match tokens.split_first() {
            Some((first, rest)) => (first.clone(), rest.to_vec()),
            None => ("".to_string(), Vec::new()),
        };

        BooshCommand { program, args }
    }
}

pub trait Executable {
    fn execute(&self, boosh_env: &mut BooshEnvironment) -> Option<String>;
}

impl Executable for BooshCommand {
    fn execute(&self, boosh_env: &mut BooshEnvironment) -> Option<String> {
        // TODO: Match to any member of builtin enum, execute accordingly
        match self.program.as_str() {
            "cd" => {
                // Convert String args to &str for compatibility
                let str_args: Vec<&str> = self.args.iter().map(|s| s.as_str()).collect();
                change_directory(boosh_env, str_args);
                None
            }
            _ => {
                match Command::new(&self.program)
                    .args(&self.args)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                {
                    Ok(child_process) => {
                        Some(String::from_utf8_lossy(&child_process.stdout).to_string())
                    }
                    Err(e) => {
                        eprintln!("Failed to execute command: {}", e);
                    }
                }
            }
        }
    }
}
