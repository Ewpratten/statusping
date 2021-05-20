pub mod ping;
pub mod http;

#[derive(Debug)]
pub enum TaskResult{
    Up,
    Down,
    Degraded
}