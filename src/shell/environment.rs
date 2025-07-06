// Instead of using things like DirManager, this is a global boosh environment that stores jump history
// and other states that I will need later

use std::{
    env::current_dir,
    path::{absolute, Path, PathBuf},
};

// TODO change this in config file
const MAX_CD_HISTORY_LEN: usize = 3;

pub struct BooshEnvironment {
    pub dir_history: Vec<PathBuf>,
}

impl BooshEnvironment {
    /// Initialize a BooshEnvironment,
    /// default dir_history is the current dir
    pub fn new() -> BooshEnvironment {
        BooshEnvironment {
            dir_history: vec![current_dir().unwrap().to_path_buf()],
        }
    }
}

// What do I want to be recordable?
// cd history
// command history
// eventually, concurrent processes maybe would also fit here
/// Describes something that can be recorded into a field
/// of the BooshEnvrionment struct. E.g. cd history
pub trait RecordDir {
    /// Push a relative path, which is converted to an absolute path,
    /// to the BooshEnvironment.dir_history vector
    fn push_history(self: &mut Self, path: &Path);
}

impl RecordDir for BooshEnvironment {
    fn push_history(self: &mut Self, path: &Path) {
        // If history length + 1 exceeds max, delete oldest
        if self.dir_history.len() + 1 > MAX_CD_HISTORY_LEN {
            self.dir_history.remove(0);
        }

        self.dir_history.push(absolute(path).unwrap().to_path_buf());
    }
}
