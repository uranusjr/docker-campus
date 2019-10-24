use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;

use clap::{App, ArgMatches};

use crate::conf::Configuration;

mod show;

pub fn app<'a, 'b>() -> App<'a, 'b> {
    clap_app!(app =>
        (name: crate_name!())
        (author: crate_authors!())
        (version: crate_version!())
        (@setting ArgRequiredElseHelp)
        (@subcommand add =>
            (about: "Adds a project for management")
            (@arg name: +required "name of project")
            (@arg root: +required "path to project root")
        )
        (@subcommand remove =>
            (about: "Removes a project from management")
            (alias: "rm")
            (@arg name: +required "name of project")
        )
        (@subcommand show =>
            (about: "Show status of managed projects")
            (@arg names: ... "filter projects to display")
            (@arg list: --list "only list project name and roots")
        )
        (@subcommand compose =>
            (about: "Runs a docker-compose command on a project")
            (alias: "c")
            (@arg project: +required "name of project")
            (@arg command: +required "command to run")
            (@arg args: ... "arguments passed to command")
        )
    )
}

pub fn dispatch(app_matches: &ArgMatches) {
    let mut configuration = Configuration::load();

    match app_matches.subcommand() {
        ("add", Some(matches)) => {
            let name = matches.value_of("name").unwrap();
            let root = PathBuf::from(matches.value_of("root").unwrap());
            configuration.insert_project(name, &root);
            configuration.persist();
        },
        ("remove", Some(matches)) => {
            let name = matches.value_of("name").unwrap();
            configuration.remove_project(name);
            configuration.persist();
        },
        ("show", Some(matches)) => {
            let names = matches.values_of("names").map(|v| {
                HashSet::from_iter(v.into_iter())
            });
            let as_list = matches.is_present("list");
            show::run(&configuration, names, as_list);
        },
        ("compose", Some(matches)) => {
            let project = matches.value_of("project").unwrap();
            let command = matches.value_of("command").unwrap();
            let args = matches.values_of("args")
                .map(|v| v.collect())
                .unwrap_or(vec![]);
            configuration.project(project).unwrap().compose(command, args);
        },

        _ => { panic!("should not happen"); },
    }
}
