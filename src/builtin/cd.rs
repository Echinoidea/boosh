use std::env;
use std::env::set_current_dir;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct DirManager {
    history: Vec<PathBuf>,
    cwd: PathBuf,
}

impl DirManager {
    fn new() -> DirManager {
        DirManager {
            history: Vec::<PathBuf>::new(),
            cwd: std::env::current_dir().unwrap(),
        }
    }

    fn change_directory(self: &mut Self, args: Vec<&str>) {
        let key = "HOME";
        let home_path: String = match env::var(key) {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Cannot find $HOME");
                String::new()
            }
        };

        if args.len() < 1 {
            set_current_dir(home_path.as_str()).unwrap();
            return;
        }

        let path = match args[0] {
            "~" => Path::new(home_path.as_str()),
            "-" => {
                if self.history.len() > 1 {
                    self.history[self.history.clone().len() - 1]
                } else {
                    return;
                }
            }
            _ => Path::new(args[0]),
        };

        self.history.push(std::env::current_dir().unwrap());

        set_current_dir(path).unwrap_or_else(|error| match error.kind() {
            ErrorKind::NotADirectory => {
                eprintln!("{:?} is not a directory", path);
            }
            ErrorKind::NotFound => {
                eprintln!("{:?} not found", path);
            }
            _ => {
                eprintln!("Error occurred")
            }
        });
    }
}
