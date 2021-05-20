pub mod server;
pub mod tasks;

use serde::{Deserialize, Serialize};

use self::tasks::{DnsTask, HttpTask, PingTask, StatusPageTask};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigHeader {
    pub page_id: String,
    pub http_timeout_ms: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskList {
    pub dns: DnsTask,
    pub http: HttpTask,
    pub ping: PingTask,
    pub statuspage: StatusPageTask,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub config: ConfigHeader,
    pub tasks: TaskList,
}

