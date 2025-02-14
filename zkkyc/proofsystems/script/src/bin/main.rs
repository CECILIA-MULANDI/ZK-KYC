use bincode; // Add this dependency

use lib::Document;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};

pub const ZK_KYC: &[u8] = include_elf!("zk-kyc");

fn main() {
    utils::setup_logger();
    dotenv::dotenv().ok();
    // 1. create a sample input
    let doc = Document {
        id: "1".to_string(),
        issuer: "GOK".to_string(),
        expiry_date: 1672542000,
        issue_date: 1672527600,
    };
    let current_time = 1772527600;
    let mut stdin = SP1Stdin::new();
    stdin.write(&doc);
    stdin.write(&current_time);

    //2. create a prover
    let client = ProverClient::from_env();

    // 3. Execute the program without generating a proof
    let res = client.execute(ZK_KYC, &stdin).run().unwrap();
    println!("The results of the execution are: {:?}", res);
}
