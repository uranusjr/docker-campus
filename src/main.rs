#[macro_use] extern crate clap;
#[macro_use] extern crate prettytable;

extern crate directories;
extern crate serde;
extern crate toml;

mod cli;
mod conf;
mod pros;

fn main() {
    cli::dispatch(&cli::app().get_matches());
}
