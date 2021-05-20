use serde::{Deserialize, Serialize};

use super::server::ServerConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct DnsTask {
    pub component_id: String,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpTask {
    pub component_id: String,
    pub secure: bool,
    pub subpath: Option<String>,
    pub expect_code: Option<u32>,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PingTask {
    pub component_id: String,
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatusPageTask {
    pub component_id: String,
    pub server: ServerConfig,
}
