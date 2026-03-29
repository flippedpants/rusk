// use uuid::Uuid;

mod task;
mod commands;
mod storage;
mod logic;
use commands::start;

fn main() {
    start();
}
