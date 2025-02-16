use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    pub id: String,
    pub issuer: String,
    pub expiry_date: u64,
    pub issue_date: u64,
}
pub fn verify_document(document: &Document, current_time: u64) -> bool {
    let is_format_valid = !document.id.is_empty() && !document.issuer.is_empty();
    let dates_valid = document.issue_date < document.expiry_date;
    let not_expired = document.expiry_date > current_time;

    is_format_valid && dates_valid && not_expired
}
