use std::process::{Command, Stdio};

use crate::builtin::cd::DirManager;

/// Struct storing a single command, as in a single program with args. Can be piped.
pub struct BooshCommand<'a> {
    pub program: &'a str,
    pub args: Vec<&'a str>,
}

pub trait Parse {
    /// Construct a new BooshCommand from a raw string input
    fn from_input(input: &String) -> BooshCommand;
}

impl<'a> Parse for BooshCommand<'_> {
    fn from_input(input: &String) -> BooshCommand {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let (program, args) = match tokens.split_first() {
            Some((&first, rest)) => (first, rest.to_vec()),
            None => (":", Vec::new()),
        };

        BooshCommand { program, args }
    }
}

pub trait Executable {
    /// TODO: dir manager needs to be handled differently
    fn execute(self: &Self, dir_manager: &mut DirManager);
}

impl<'a> Executable for BooshCommand<'_> {
    fn execute(self: &Self, dir_manager: &mut DirManager) {
        match self.program {
            "cd" => {
                dir_manager.change_directory(self.args.clone());
            }
            _ => {
                let child = Command::new(self.program)
                    .args(self.args.clone())
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn();

                match child {
                    Ok(mut child_process) => {
                        // wait for child to finish
                        let _ = child_process.wait();
                    }
                    Err(e) => {
                        eprintln!("Failed to execute command: {}", e);
                    }
                }
            }
        }
    }
}
