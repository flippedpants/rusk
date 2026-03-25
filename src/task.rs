use uuid::Uuid;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum Status{
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum Priority{
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task{
    id: Uuid,
    name: String,
    priority: Priority,
    status: Status,
    // due_date: Option<DateTime<Local>>,
}

impl Task{
    pub fn new(id: Uuid, name: String, priority: Priority, status: Status) -> Self{
        Self{
            id,
            name,
            priority,
            status,
        }
    }
}