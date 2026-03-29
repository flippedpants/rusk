use uuid::Uuid;
// use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use strum::EnumString;
use std::fmt;

#[derive(Clone,Debug, Serialize, Deserialize, EnumString, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Status{
    Pending,
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, EnumString, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Priority{
    Low,
    Medium,
    High,
}

#[derive(Clone,Debug, Serialize, Deserialize, PartialEq)]
pub struct Task{
    id: Uuid,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    // due_date: DateTime<Local>,
}

impl Task{
    pub fn new(id: Uuid, title: String, priority: Priority, status: Status) -> Self{
        Self{
            id,
            title,
            priority,
            status,
        }
    }
}

impl fmt::Display for Priority{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low"
        };
        return write!(f, "{}", value);
    }
}

impl fmt::Display for Status{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Status::Completed => "Completed",
            Status::Pending => "Pending"
        };
        return write!(f, "{}", value);
    }
}