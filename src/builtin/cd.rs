use std::env::{current_dir, set_current_dir, var};
use std::io::ErrorKind;
use std::path::{absolute, Path, PathBuf};

// TODO change this in config file
const MAX_CD_HISTORY_LEN: usize = 3;

#[derive(Clone)]
pub struct DirManager {
    history: Vec<PathBuf>,
}

impl DirManager {
    pub fn new() -> DirManager {
        DirManager {
            history: vec![current_dir().unwrap().to_path_buf()],
        }
    }

    /// Given a reference to a path, push the absolute version of it to the history vector.
    /// Removes oldest cd to stay within MAX_CD_HISTORY_LEN.
    fn push_history(self: &mut Self, path: &Path) {
        // If history length + 1 exceeds max, delete oldest
        if self.history.len() + 1 > MAX_CD_HISTORY_LEN {
            self.history.remove(0);
        }

        self.history.push(absolute(path).unwrap().to_path_buf());
    }

    /// Change current working directory.
    /// Handles special cd aliases such as ~ and -, and no args
    /// Pushes to cd history with each run.
    /// Prints errors if cd is not a directory or not found, prints generic error for all other errors.
    /// Panics if std::env::var cannot find a HOME variable
    pub fn change_directory(self: &mut Self, args: Vec<&str>) {
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
                if self.history.len() > 1 {
                    // Clone the previous directory path from history for safe use
                    &self.history[self.history.len() - 2].clone()
                } else if let Some(last_path) = self.history.last() {
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
                self.push_history(&path);
            }
        }
    }
}
