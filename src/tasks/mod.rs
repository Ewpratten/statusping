pub mod ping;
pub mod http;
pub mod dns;

#[derive(Debug)]
pub enum TaskResult{
    Up,
    Down,
    Degraded
}