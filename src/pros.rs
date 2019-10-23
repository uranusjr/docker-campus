use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    root: PathBuf,
}

impl Project {
    pub fn new(name: &str, root: &Path) -> Self {
        Project {
            name: name.into(),
            root: root.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn set_root(&mut self, path: &Path) {
        self.root = path.into();
    }

    pub fn compose(&self, command: &str, args: Vec<&str>) {
        let returncode = Command::new("docker-compose")
            .arg(command)
            .args(&args)
            .current_dir(&self.root)
            .spawn()
            .expect("failed to execute docker-compose")
            .wait()
            .expect("failed to wait for docker-compose");
        if !returncode.success() {
            panic!("docker-compose failed {:?}", returncode);
        }
    }
}
