use std::time::Duration;

use reqwest::blocking::ClientBuilder;

use crate::config::tasks::HttpTask;

use super::TaskResult;

/// Perform an HTTP check on a host
pub fn check_host_http(timeout_ms: u32, task: &HttpTask) -> TaskResult {
    // Build and configure an http client
    let client = ClientBuilder::new()
        .timeout(Some(Duration::from_millis(timeout_ms as u64)))
        .build()
        .unwrap();

    // Build and send request
    let response = client
        .get(format!(
            "{}://{}{}/{}",
            match task.secure {
                true => "https",
                false => "http",
            },
            task.server.host,
            match task.server.port {
                Some(port) => format!(":{}", port),
                None => "".to_string(),
            },
            match &task.subpath {
                Some(path) => path.clone(),
                None => "".to_string(),
            }
        ))
        .send();

    // Handle errors
    if response.is_err() {
        let error = response.unwrap_err();
        if error.is_timeout() {
            return TaskResult::Degraded;
        } else {
            return TaskResult::Down;
        }
    } else {
        // Handle success

        // Determine the correct status code
        let success_status = match task.expect_code {
            Some(code) => code,
            None => 200,
        } as u16;

        if response.unwrap().status().as_u16() == success_status {
            return TaskResult::Up;
        } else {
            return TaskResult::Down;
        }
    }
}
