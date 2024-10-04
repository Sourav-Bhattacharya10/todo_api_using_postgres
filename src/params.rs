use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    pub page: u64,
    pub page_size: u64,
    pub term: Option<String>,
    pub done_status: Option<bool>,
}
