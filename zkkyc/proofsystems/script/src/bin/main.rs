use lib::Document;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};

pub const ZK_KYC: &[u8] = include_elf!("zk_kyc_program");

fn main() {
    utils::setup_logger();
    dotenv::dotenv().ok();

    println!("Creating input data...");

    // 1. create a sample input
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

    println!("Writing document to stdin...");
    stdin.write(&doc);

    println!("Writing current_time to stdin...");
    stdin.write(&current_time);

    println!("Creating prover client...");
    // 2. create a prover
    let client = ProverClient::from_env();

    println!("Executing program...");
    // 3. Execute the program without generating a proof
    match client.execute(ZK_KYC, &stdin).run() {
        Ok(res) => println!("The results of the execution are: {:?}", res),
        Err(e) => {
            eprintln!("Execution failed: {:?}", e);
            std::process::exit(1);
        }
    }
}
