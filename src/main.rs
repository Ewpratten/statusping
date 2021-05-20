mod config;
mod tasks;

use std::{io::Read, thread, time::Duration};

use autojson::structify;
use clap::{value_t, App, Arg};
use config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use tasks::TaskResult;

fn set_component_status(
    progress_bar: &ProgressBar,
    dry_run: bool,
    page_id: &String,
    component_id: &String,
    status: &TaskResult,
) {
    // Log
    progress_bar.println(format!(
        "Task: {}::{} is {:?}",
        page_id, component_id, status
    ));

    // Only send data if real execution
    if !dry_run {
        // Send data

        // Sleep past the rate limit
        thread::sleep(Duration::from_secs(1));
    }
}

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
        .arg(
            Arg::with_name("dry_run")
                .long("dry-run")
                .takes_value(false)
                .help("Run, but do not send any data")
                .required(false),
        )
        .get_matches();

    // Get data
    let config = matches.value_of("config_file").unwrap();
    let oauth = matches.value_of("oauth_file").unwrap();
    let dry_run = matches.is_present("dry_run");

    // Load the config
    let config: Config = structify(config).unwrap();
    println!("Loaded config file");

    // Load the oauth data
    let oauth = std::fs::read_to_string(oauth).unwrap();
    println!("Loaded oauth");

    // Set up progress bar
    let pb = ProgressBar::new(
        (config.tasks.dns.len()
            + config.tasks.http.len()
            + config.tasks.ping.len()
            + config.tasks.statuspage.len()) as u64,
    );
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] ({eta})")
            .progress_chars("#>-"),
    );

    // Perform dns checks
    // TODO

    // Perform http checks
    for http_task in config.tasks.http.iter() {
        let result = tasks::http::check_host_http(config.config.http_timeout_ms, http_task);
        set_component_status(
            &pb,
            dry_run,
            &config.config.page_id,
            &http_task.component_id,
            &result,
        );
        pb.inc(1);
    }

    // Perform ping checks
    // TODO

    // Perform statuspage checks
    // TODO

    // Finish the progress bar
    pb.finish_and_clear();
}
