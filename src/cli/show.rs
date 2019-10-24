use std::collections::HashSet;
use std::iter::empty;

use prettytable::{format, Table};

use crate::conf::Configuration;
use crate::pros::Project;

fn as_table(projects: Vec<&Project>) -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Name", "Root"]);
    for p in projects {
        table.add_row(row![p.name(), p.root().to_string_lossy()]);
    }
    table
}

pub fn run(
    configuration: &Configuration,
    names: Option<HashSet<&str>>,
    as_list: bool,
) {
    let projects: Vec<_> = configuration.projects()
        .filter(|p| names.as_ref().map_or(true, |n| n.contains(p.name())))
        .collect();

    if as_list {
        as_table(projects).printstd();
        return;
    }

    for p in projects.iter() {
        if projects.len() > 1 {
            println!("* {} ({})", p.name(), p.root().display());
        }
        p.compose("ps", empty());
        println!("");
    }
}
