mod config;
mod tasks;

use std::{collections::HashMap, io::Read, thread, time::Duration};

use autojson::structify;
use clap::{value_t, App, Arg};
use config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{header::HeaderValue, Method};
use tasks::TaskResult;
use trust_dns_client::udp::UdpClientConnection;

fn set_component_status(
    progress_bar: &ProgressBar,
    dry_run: bool,
    api_key: &String,
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

        // Build form data
        let mut form = HashMap::new();
        form.insert("component[status]", match status {
            TaskResult::Up => "operational",
            TaskResult::Down => "major_outage",
            TaskResult::Degraded => "degraded_performance"
        });

        // Send data
        let client = reqwest::blocking::Client::new();
        let request = client.patch(format!(
            "https://api.statuspage.io/v1/pages/{}/components/{}",
            page_id, component_id
        )).header("Authorization", HeaderValue::from_str(&format!("OAuth {}", api_key)).unwrap()).form(&form);
        let response = request.send();

        // Sleep past the rate limit
        thread::sleep(Duration::from_secs(1));
    }else{
        progress_bar.println("--dry-run mode. Not sending data");
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
    let oauth = oauth.strip_suffix("\n").unwrap_or(&oauth).to_string();
    println!("Loaded oauth");

    // Ensure the user is root (raw sockets cannot be opened by regular users)
    let _ = sudo::escalate_if_needed().unwrap();

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
    for dns_task in config.tasks.dns.iter() {
        let result = tasks::dns::check_host_dns(dns_task);
        set_component_status(
            &pb,
            dry_run,
            &oauth,
            &config.config.page_id,
            &dns_task.component_id,
            &result,
        );
        pb.inc(1);
    }

    // Perform http checks
    for http_task in config.tasks.http.iter() {
        let result = tasks::http::check_host_http(config.config.http_timeout_ms, http_task);
        set_component_status(
            &pb,
            dry_run,
            &oauth,
            &config.config.page_id,
            &http_task.component_id,
            &result,
        );
        pb.inc(1);
    }

    // Perform ping checks
    for ping_task in config.tasks.ping.iter() {
        let result = tasks::ping::check_host_ping(ping_task);
        set_component_status(
            &pb,
            dry_run,
            &oauth,
            &config.config.page_id,
            &ping_task.component_id,
            &result,
        );
        pb.inc(1);
    }

    // Perform statuspage checks
    // TODO

    // Finish the progress bar
    pb.finish_and_clear();
}
