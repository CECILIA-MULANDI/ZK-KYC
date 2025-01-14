#![no_main]
sp1_zkvm::entrypoint!(main);

use fibonacci_lib::{KycUpload,new}

pub fn main() {
    // Read an input to the program.

    let name: = sp1_zkvm::io::read::<String>();
    let dob: = sp1_zkvm::io::read::<String >();
    let gender: = sp1_zkvm::io::read::< Option<String>>();
    let signature: = sp1_zkvm::io::read::<String>();
    let address: = sp1_zkvm::io::read::<String>();
    let id_number:  = sp1_zkvm::io::read::<String>();
    let id_type:  = sp1_zkvm::io::read::<String>();
    let issue_date:= sp1_zkvm::io::read::<String >();
    let expiry_date: = sp1_zkvm::io::read::<Option<String>>();
   let doc =KycUpload::new(
    name,dob,gender,signature,address,id_number,id_type,issue_date,expiry_date
   );

    sp1_zkvm::io::commit(&doc);
}
