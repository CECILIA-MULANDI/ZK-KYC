#![no_main]
sp1_zkvm::entrypoint!(main);

use fibonacci_lib::KycUpload;

pub fn main() {
    // Read an input to the program.

    let docs = sp1_zkvm::io::read::<Vec<u8>>();

    // Deserialize using bincode
    let kyc_docs: Vec<KycUpload> =
        bincode::deserialize(&docs).expect("Failed to deserialize KYC documents");

    // Process each document
    for doc in kyc_docs {
        sp1_zkvm::io::commit(&doc);
    }
}
