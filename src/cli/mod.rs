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
            (@arg args: +allow_hyphen_values ...
                "arguments for docker-compose")
        )
        (@subcommand start =>
            (about: "Runs docker-compose start on a project")
            (@arg project: +required "name of project")
            (@arg args: +allow_hyphen_values ...
                "additional arguments for docker-compose")
        )
        (@subcommand stop =>
            (about: "Runs docker-compose stop on a project")
            (@arg project: +required "name of project")
            (@arg args: +allow_hyphen_values ...
                "additional arguments for docker-compose")
        )
    )
}

macro_rules! compose {
    ($configuration: expr, $matches: expr) => {
        let command = $matches.value_of("command").unwrap();
        compose!(command, $configuration, $matches);
    };
    ($command: expr, $configuration: expr, $matches: expr) => {
        let project = $matches.value_of("project").unwrap();
        let args = $matches.values_of("args")
            .map(|v| v.collect())
            .unwrap_or_else(|| vec![]);
        $configuration.project(project).unwrap().compose($command, args);
    };
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
                HashSet::from_iter(v)
            });
            let as_list = matches.is_present("list");
            show::run(&configuration, names, as_list);
        },
        ("compose", Some(matches)) => {
            compose!(configuration, matches);
        },
        ("start", Some(matches)) => {
            compose!("start", configuration, matches);
        },
        ("stop", Some(matches)) => {
            compose!("stop", configuration, matches);
        },

        _ => { panic!("should not happen"); },
    }
}
