use serde::{Deserialize, Serialize};

use super::todo_dto::TodoDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedTodo {
    pub todos: Vec<TodoDto>,
    pub page: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    #[serde(rename = "totalPages")]
    pub total_pages: u64,
    #[serde(rename = "totalRecords")]
    pub total_records: u64,
    pub order: String,
}
