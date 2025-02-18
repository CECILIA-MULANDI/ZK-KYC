#![no_main]
sp1_zkvm::entrypoint!(main);

use zk_kyc_lib::{hash_doc, verify_document, Document};

pub fn main() {
    // Read the document fields
    let id = sp1_zkvm::io::read::<String>();
    let issuer = sp1_zkvm::io::read::<String>();
    let expiry_date = sp1_zkvm::io::read::<u64>();
    let issue_date = sp1_zkvm::io::read::<u64>();

    // Construct the document
    let doc = Document {
        id,
        issuer,
        expiry_date,
        issue_date,
    };

    // Read current time
    let current_time = sp1_zkvm::io::read::<u64>();

    // Compute the hash for privacy
    let hashed_doc = hash_doc(&doc);

    let is_valid_res = verify_document(&doc, current_time);
    sp1_zkvm::io::commit(&hashed_doc);
    sp1_zkvm::io::commit(&is_valid_res);
}
