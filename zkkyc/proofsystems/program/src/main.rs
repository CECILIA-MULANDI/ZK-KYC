#![no_main]
sp1_zkvm::entrypoint!(main);

use fibonacci_lib::{KycUpload,new}

pub fn main() {
    // Read an input to the program.

    let doc = sp1_zkvm::io::read::<u32>();
    sp1_zkvm::io::commit(&doc);
    // pass in the doc to the function from the lib.rs
    let res = submit_docs(doc);

    sp1_zkvm::io::commit(&res);
}
