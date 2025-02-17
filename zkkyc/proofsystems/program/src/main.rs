#![no_main]
sp1_zkvm::entrypoint!(main);

use zk_kyc_lib::{hash_doc, verify_document, Document};

pub fn main() {
    // Read an input to the program.
    let doc = sp1_zkvm::io::read::<Document>();
    let current_time = sp1_zkvm::io::read::<u64>();
    // compute the hash for privacy
    let hashed_doc = hash_doc(&doc);

    let is_valid_res = verify_document(&doc, current_time);
    sp1_zkvm::io::commit(&hashed_doc);
    sp1_zkvm::io::commit(&is_valid_res);
}
