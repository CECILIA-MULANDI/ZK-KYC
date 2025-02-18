use bincode;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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

pub fn hash_doc(doc: &Document) -> [u8; 32] {
    let serialized_doc = bincode::serialize(doc).expect("Failed to serialize this document!");
    let mut hasher = Sha256::new();
    hasher.update(&serialized_doc);
    let result = hasher.finalize();
    let mut hash_array = [0u8; 32];
    hash_array.copy_from_slice(&result[..32]);
    hash_array
}
