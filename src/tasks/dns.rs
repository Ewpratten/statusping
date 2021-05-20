use trust_dns_client::{
    client::{Client, SyncClient},
    rr::{DNSClass, Name, RData, Record, RecordType},
    udp::UdpClientConnection,
};

use crate::config::tasks::DnsTask;
use std::{net::Ipv4Addr, str::FromStr};

use super::TaskResult;

/// Perform an DNS check on a host
pub fn check_host_dns(task: &DnsTask) -> TaskResult {
    // Open a connection to the dns server
    let dns = UdpClientConnection::new(
        format!(
            "{}:{}",
            task.server.host,
            match task.server.port {
                Some(port) => port,
                None => 53,
            }
        )
        .parse()
        .unwrap(),
    )
    .unwrap();

    // Build a client
    let dns = SyncClient::new(dns);

    // Request "example.com" from the server
    let response = dns.query(
        &Name::from_str("www.example.com.").unwrap(),
        DNSClass::IN,
        RecordType::A,
    );

    // Handle result
    return match response {
        Ok(res) => {
            // Get the answers from the server
            let answers: &[Record] = res.answers();

            // Handle the answer
            if let &RData::A(ref ip) = answers[0].rdata() {
                // Ensure the record is correct
                if *ip == Ipv4Addr::new(93, 184, 216, 34) {
                    TaskResult::Up
                } else {
                    TaskResult::Degraded
                }
            } else {
                TaskResult::Degraded
            }
        }
        Err(_) => TaskResult::Down,
    };
}
