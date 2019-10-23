use std::path::PathBuf;

use clap::{App, ArgMatches};
use prettytable::{format, Table};

use crate::conf::Configuration;

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
            (@arg name: +required "name of project")
        )
        (@subcommand list =>
            (about: "Lists managed projects")
        )
        (@subcommand compose =>
            (about: "Runs a docker-compose command on a project")
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
        ("list", _) => {
            if configuration.project_len() < 1 {
                println!("No projects. Use `add` to add some");
            } else {
                let mut table = Table::new();
                table.set_format(
                    *format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table.set_titles(row!["Name", "Root"]);
                for p in configuration.projects() {
                    table.add_row(row![p.name(), p.root().to_string_lossy()]);
                }
                table.printstd();
            }
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
