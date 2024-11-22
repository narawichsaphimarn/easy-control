use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    pub ip_addr: String,
}
