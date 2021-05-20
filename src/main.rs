mod config;
mod tasks;

use std::io::Read;

use autojson::structify;
use clap::{value_t, App, Arg};
use config::Config;

fn main() {
    let matches = App::new("statusping")
        .author("Evan Pratten <ewpratten@gmail.com>")
        .arg(
            Arg::with_name("config_file")
                .takes_value(true)
                .help("Path to the config JSON file")
                .required(true),
        )
        .arg(
            Arg::with_name("oauth_file")
                .takes_value(true)
                .help("Path to a file containing your statuspage OAuth key")
                .required(true),
        )
        .get_matches();

    // Get data
    let config = matches.value_of("config_file").unwrap();
    let oauth = matches.value_of("oauth_file").unwrap();

    // Load the config
    let config: Config = structify(config).unwrap();
    println!("Loaded config file");

    // Load the oauth data
    let oauth = std::fs::read_to_string(oauth).unwrap();
    println!("Loaded oauth");



}
