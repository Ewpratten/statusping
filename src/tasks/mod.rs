pub mod ping;
pub mod http;

pub enum TaskResult{
    Up,
    Down,
    Degraded
}