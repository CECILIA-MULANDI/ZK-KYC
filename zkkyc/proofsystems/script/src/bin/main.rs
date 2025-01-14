//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use chrono::NaiveDate;
use lib::KycUpload;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};
/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ZK_KYC: &[u8] = include_elf!("fibonacci-program");

fn main() {
    // set up logging
    utils::setup_logger();
    //create an input stream and write a sample document to it
    let doc: KycUpload = KycUpload {
        name: String::from("Alice Johnson"),
        date_of_birth: NaiveDate::from_ymd_opt(1990, 5, 15),
        gender: Some(String::from("Female")),
        signature: String::from("c3fcd3d76192e4007dfb496cca67e13b"), // A hashed representation
        address: String::from("123 Blockchain Lane, Crypto City, CC 54321"),
        id_number: String::from("A123456789"),
        id_type: String::from("Passport"),
        document_issue_date: NaiveDate::from_ymd_opt(2020, 1, 1),
        document_expiry_date: Some(NaiveDate::from_ymd_opt(2030, 1, 1)),
    };
    // write to the std
    let mut stdin = SP1Stdin::new();
    stdin.write(&doc);
    // create prover client
    let client = ProverClient::new();

    // Set up pk and vk
    let (pk, vk) = client.setup(ZK_KYC);

    let mut proof = client.prove(&pk, stdin).run().unwrap();

    println!("Proof generated!");

    client
        .verify(&proof, &vk)
        .expect("Failed to verify the program");
}
