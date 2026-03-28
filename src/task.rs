use uuid::Uuid;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use strum::EnumString;

#[derive(Debug, Serialize, Deserialize, EnumString, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Status{
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, EnumString, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum Priority{
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Task{
    id: Uuid,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    due_date: DateTime<Local>,
}

impl Task{
    pub fn new(id: Uuid, title: String, priority: Priority, status: Status, due_date:DateTime<Local> ) -> Self{
        Self{
            id,
            title,
            priority,
            status,
            due_date,
        }
    }
}