use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TodoTask {
    #[serde(rename = "taskName")]
    pub task_name: String,
}
