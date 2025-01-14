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
use serde_json;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};
/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ZK_KYC: &[u8] = include_elf!("fibonacci-program");
fn main() {
    // Set up logging
    utils::setup_logger();

    // Create an input stream and write a sample document to it
    let docs: Vec<KycUpload> = vec![
        KycUpload {
            name: String::from("Alice Johnson"),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 5, 15).expect("Invalid date of birth"),
            gender: Some(String::from("Female")),
            signature: String::from("c3fcd3d76192e4007dfb496cca67e13b"),
            address: String::from("123 Blockchain Lane, Crypto City, CC 54321"),
            id_number: String::from("A123456789"),
            id_type: String::from("Passport"),
            document_issue_date: NaiveDate::from_ymd_opt(2020, 1, 1).expect("Invalid issue date"),
            document_expiry_date: Some(
                NaiveDate::from_ymd_opt(2030, 1, 1).expect("Invalid expiry date"),
            ),
        },
        KycUpload {
            name: String::from("Bob Smith"),
            date_of_birth: NaiveDate::from_ymd_opt(1985, 3, 20).expect("Invalid date of birth"),
            gender: Some(String::from("Male")),
            signature: String::from("e2fc714c4727ee9395f324cd2e7f331f"),
            address: String::from("456 Blockchain Ave, Crypto Town, CT 12345"),
            id_number: String::from("B987654321"),
            id_type: String::from("Driver's License"),
            document_issue_date: NaiveDate::from_ymd_opt(2018, 7, 1).expect("Invalid issue date"),
            document_expiry_date: Some(
                NaiveDate::from_ymd_opt(2028, 7, 1).expect("Invalid expiry date"),
            ),
        },
    ];

    // Serialize the documents to JSON
    let serialized_docs = serde_json::to_string(&docs).expect("Failed to serialize documents");
    println!("Serialized Docs: {}", serialized_docs); // Debugging output

    // Convert the serialized string to bytes
    let serialized_docs_bytes = serialized_docs.as_bytes();
    if serialized_docs_bytes.is_empty() {
        panic!("Serialized input data is empty");
    }

    // Log the byte size and the first few bytes to ensure they are being serialized correctly
    println!("Serialized Bytes Length: {}", serialized_docs_bytes.len());
    println!("First few bytes: {:?}", &serialized_docs_bytes[0..10]);

    // Write the serialized bytes to stdin (Check for data format expected by zkVM)
    let mut stdin = SP1Stdin::new();

    // Add additional checks to ensure the data is correctly formatted or padded if necessary
    stdin.write(&serialized_docs_bytes);
    println!("Input data written to stdin.");

    // Create prover client
    let client = ProverClient::new();

    // Execute the zkVM program with the input data
    let (_, report) = client.execute(ZK_KYC, stdin.clone()).run().unwrap();
    println!(
        "Executed program with {} cycles",
        report.total_instruction_count()
    );

    // Setup pk and vk
    let (pk, vk) = client.setup(ZK_KYC);

    // Generate the proof for the zkVM execution
    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("Proof generation failed");

    println!("Proof generated!");

    // Verify the proof using the verification key
    client
        .verify(&proof, &vk)
        .expect("Failed to verify the proof");
}
