use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::slice::Iter;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::pros::Project;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    projects: Vec<Project>,
}

impl Configuration {
    pub fn load() -> Self {
        let path = configuration_path();
        if !path.is_file() {
            return Self { projects: vec![] };
        }
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).unwrap()
    }

    pub fn persist(&self) {
        let path = configuration_path();
        if let Some(p) = path.parent() {
            create_dir_all(p).unwrap();
        }
        let mut file = File::create(path).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes()).unwrap();
    }

    pub fn projects(&self) -> Iter<Project> {
        self.projects.iter()
    }

    pub fn insert_project(&mut self, name: &str, root: &Path) {
        match self.project_mut(name) {
            Some(p) => { p.set_root(root); },
            None => { self.projects.push(Project::new(name, root)); },
        }
    }

    pub fn remove_project(&mut self, name: &str) -> Option<Project> {
        self.projects.iter()
            .position(move |p| p.name() == name)
            .map(|i| self.projects.remove(i))
    }

    pub fn project(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name() == name)
    }

    pub fn project_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.iter()
            .position(move |p| p.name() == name)
            .map(move |i| &mut self.projects[i])
    }
}

fn configuration_path() -> PathBuf {
    let dirs = ProjectDirs::from("", "", crate_name!()).unwrap();
    dirs.data_dir().join("campus.toml").to_owned()
}
