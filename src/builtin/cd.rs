use std::env::{current_dir, set_current_dir, var};
use std::io::ErrorKind;
use std::path::{absolute, Path, PathBuf};

use crate::shell::environment::{BooshEnvironment, RecordDir};

/// Change current working directory.
/// Handles special cd aliases such as ~ and -, and no args
/// Pushes to cd history with each run.
/// Prints errors if cd is not a directory or not found, prints generic error for all other errors.
/// Panics if std::env::var cannot find a HOME variable
pub fn change_directory(env: &mut BooshEnvironment, args: Vec<&str>) {
    let key = "HOME";
    let home_path: String = match var(key) {
        Ok(val) => val,
        Err(_) => {
            panic!("Cannot find $HOME");
        }
    };

    // No args alias, cd home
    if args.len() < 1 {
        set_current_dir(home_path.as_str()).unwrap();
        return;
    }

    // Handle aliases
    let path: &Path = match args[0] {
        "~" => Path::new(&home_path),
        "-" => {
            if env.dir_history.len() > 1 {
                // Clone the previous directory path from history for safe use
                &env.dir_history[env.dir_history.len() - 2].clone()
            } else if let Some(last_path) = env.dir_history.last() {
                &last_path.clone()
            } else {
                &current_dir().unwrap().clone()
            }
        }
        _ => Path::new(args[0]),
    };

    // Change cwd
    match set_current_dir(&path) {
        Err(e) => match e.kind() {
            ErrorKind::NotADirectory => {
                eprintln!("{:?} is not a directory", path);
            }
            ErrorKind::NotFound => {
                eprintln!("{:?} not found", path);
            }
            _ => {
                eprintln!("Error occurred");
            }
        },
        Ok(_) => {
            // Add to cd history
            env.push_history(&path);
        }
    }
}
