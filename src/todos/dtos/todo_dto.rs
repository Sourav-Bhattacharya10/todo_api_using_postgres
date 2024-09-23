use serde::{Deserialize, Serialize};

use crate::entities::todo;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TodoDto {
    #[serde(rename = "id")]
    pub task_id: i32,
    #[serde(rename = "taskName")]
    pub task_name: String,
    #[serde(rename = "doneStatus")]
    pub done_status: bool,
}

impl From<&todo::Model> for TodoDto {
    fn from(value: &todo::Model) -> Self {
        Self {
            task_id: value.task_id,
            task_name: value.task_name.to_owned(),
            done_status: value.done_status,
        }
    }
}
