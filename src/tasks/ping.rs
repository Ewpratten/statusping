use oping::Ping;
use std::{net::IpAddr, time::Duration};
use std::time;

use crate::config::tasks::PingTask;

use super::TaskResult;

pub struct PingResult {
    pub is_up: bool,
    pub latency: time::Duration,
}

fn ping(address: &String, timeout: time::Duration) -> Option<PingResult> {
    // Create an ICMP ping packet
    let mut packet = Ping::new();

    // Set the packet's timeout
    let _ = packet.set_timeout(timeout.as_secs_f64()).unwrap();

    // Specify the remote host
    let _ = packet.add_host(address).unwrap();

    // Send the ping packet
    return match packet.send() {
        Ok(mut result) => match result.next() {
            Some(item) => Some(PingResult {
                is_up: item.dropped == 0,
                latency: time::Duration::from_millis(item.latency_ms as u64),
            }),
            None => None,
        },
        Err(_e) => None,
    };
}

/// Perform an Ping check on a host
pub fn check_host_ping(task: &PingTask) -> TaskResult {
    return match ping(&task.server.host, Duration::from_secs(1)) {
        Some(result) => match result.is_up {
            true => TaskResult::Up,
            false => TaskResult::Down
        }
        None => TaskResult::Down
    }
}
