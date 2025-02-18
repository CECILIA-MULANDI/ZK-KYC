use lib::Document;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};

pub const ZK_KYC: &[u8] = include_elf!("zk_kyc_program");

fn main() {
    utils::setup_logger();
    dotenv::dotenv().ok();

    println!("Creating input data...");

    // 1. Create a sample input
    let doc = Document {
        id: "1".to_string(),
        issuer: "GOK".to_string(),
        expiry_date: 1672542000_u64,
        issue_date: 1672527600_u64,
    };
    let current_time = 1772527600_u64;

    println!("Document created: {:?}", doc);

    // Initialize stdin
    let mut stdin = SP1Stdin::new();

    // Write the raw document fields to stdin
    stdin.write(&doc.id.as_bytes());
    stdin.write(&doc.issuer.as_bytes());
    stdin.write(&doc.expiry_date.to_le_bytes());
    stdin.write(&doc.issue_date.to_le_bytes());

    // Write current_time as bytes
    stdin.write(&current_time.to_le_bytes());

    println!("Creating prover client...");
    // 2. Create a prover
    let client = ProverClient::from_env();

    println!("Executing program...");
    // 3. Execute the program without generating a proof
    let (_, report) = client
        .execute(ZK_KYC, &stdin)
        .run()
        .expect("Failed to execute program");

    println!(
        "The program executed in {} cycles",
        report.total_instruction_count()
    );

    // 4. Generate pkey & vkey
    println!("Generating proving and verification keys...");
    let (pk, vk) = client.setup(ZK_KYC);

    println!("Generating proof...");
    let proof = client
        .prove(&pk, &stdin)
        .compressed()
        .run()
        .expect("Failed to generate proof");
    client.verify(&proof, &vk).expect("verification failed");
    println!("Proof generated and verified  successfully!");
}
