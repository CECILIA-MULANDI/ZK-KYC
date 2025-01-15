use bincode; // Add this dependency
use chrono::NaiveDate;
use lib::KycUpload;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};

pub const ZK_KYC: &[u8] = include_elf!("fibonacci-program");

fn main() {
    utils::setup_logger();

    let docs: Vec<KycUpload> = vec![KycUpload {
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
    }];

    // Use bincode for serialization instead of JSON
    let serialized_docs = bincode::serialize(&docs).expect("Failed to serialize");
    println!("Serialized document size: {} bytes", serialized_docs.len());

    let mut stdin = SP1Stdin::new();
    stdin.write(&serialized_docs);
    println!("Input data written to stdin.");

    let client = ProverClient::new();

    let (output, report) = client
        .execute(ZK_KYC, stdin.clone())
        .run()
        .expect("Execution failed");
    println!(
        "Program executed successfully with {} cycles",
        report.total_instruction_count()
    );

    let (pk, vk) = client.setup(ZK_KYC);

    let proof = client
        .prove(&pk, stdin)
        .run()
        .expect("Proof generation failed");
    println!("Proof generated successfully!");

    client
        .verify(&proof, &vk)
        .expect("Proof verification failed");
    println!("Proof verified successfully!");
}
