use std::env;
use std::env::set_current_dir;
use std::io::ErrorKind;
use std::path::Path;

pub fn change_directory(args: Vec<&str>) {
    let key = "HOME";
    let home_path: String = match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Cannot find $HOME");
            String::new()
        }
    };

    if args.len() < 2 {
        set_current_dir(home_path.as_str()).unwrap();
        return;
    }

    let path = match args[1] {
        "~" => Path::new("/home/gabriel/"),
        _ => Path::new(args[1]),
    };

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
