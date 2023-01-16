use serde::Serialize;

#[derive(Serialize)]
pub struct Transaction {
    pub to: String,
    pub from: String,
    pub data: String,
    pub chain_id: u64,
}
