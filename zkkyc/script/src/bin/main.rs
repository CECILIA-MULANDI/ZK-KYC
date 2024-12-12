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

// use alloy_sol_types::SolType;
use clap::Parser;
use sp1_sdk::{include_elf, utils, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ZK_KYC: &[u8] = include_elf!("fibonacci-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    doc: u32,
}

fn main() {
    // set up logging
    utils::setup_logger();
    // parse the arguments
    let args = Args::parse();
    // write to the std
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.doc);

    // create prover client
    let client = ProverClient::new();
    let (_, report) = client.execute(ZK_KYC, stdin.clone()).run().unwrap();
    println!(
        "The program executed with {} cycles ",
        report.total_instruction_count()
    );
    // Set up pk and vk
    let (pk, vk) = client.setup(ZK_KYC);

    let proof = client.prove(&pk, stdin).run().unwrap();

    println!("Proof generated!");

    client
        .verify(&proof, &vk)
        .expect("Failed to verify the program");
}
