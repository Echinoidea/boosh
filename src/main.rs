use std::env::set_current_dir;
use std::ffi::CString;
use std::io;
use std::io::Write;
use std::path::Path;

const PROMPT: &str = "> ";

/// TODO make cd - for last directory
/// TODO modularize code
/// TODO boosh parser
/// TODO boosh prompt config
/// TODO make boosh good and usable so that I can daily drive it
/// TODO color support

fn change_directory(args: Vec<&str>) -> Result<(), std::io::Error> {
    let path = match args[1] {
        "~" => Path::new("/home/gabriel/"),
        _ => Path::new(args[1]),
    };

    match set_current_dir(path) {
        Err(e) => {
            eprintln!("Directory not found");
            Err(e)
        }
        Ok(_) => Ok(()),
    }
}

fn parse_args(command: &String) -> Vec<&str> {
    return command.split_whitespace().collect();
}

fn boosh_run(args: Vec<&str>) {
    match args[0] {
        "cd" => match change_directory(args) {
            Err(_) => {}
            Ok(_) => {}
        },
        _ => {
            unsafe {
                let child_pid = libc::fork();

                match child_pid {
                    -1 => {
                        eprintln!("Error forking");
                    }
                    0 => {
                        // We are the child process
                        // execve
                        // command => ls where args is -lah
                        let c_command = CString::new(args[0]).unwrap();
                        let c_args: Vec<CString> =
                            args.iter().map(|&arg| CString::new(arg).unwrap()).collect();

                        let mut c_args_ptrs: Vec<*const libc::c_char> =
                            c_args.iter().map(|c_str| c_str.as_ptr()).collect();

                        c_args_ptrs.push(std::ptr::null());

                        libc::execvp(c_command.as_ptr(), c_args_ptrs.as_ptr());
                    }
                    _ => {
                        // We are the parent
                        let mut status: libc::c_int = 0;
                        libc::waitpid(child_pid, &mut status, 0);
                        // Return back to main loop
                    }
                }
            }
        }
    }
}

fn boosh_loop() {
    loop {
        print!("{}", PROMPT);
        std::io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read command");

        let input = input.trim().to_string();

        match input.as_str() {
            "exit" => {
                break;
            }
            _ => {
                boosh_run(parse_args(&input));
            }
        }
    }
}

fn main() {
    // Load config file if found

    boosh_loop();

    // Perform shutdown/cleanup
}
