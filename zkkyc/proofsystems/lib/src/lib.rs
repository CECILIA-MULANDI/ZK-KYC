// use alloy_sol_types::sol;

// sol! {
//     /// The public values encoded as a struct that can be easily deserialized inside Solidity.
//     struct PublicValuesStruct {
//         uint32 n;
//         uint32 a;
//         uint32 b;
//     }
// }
// for the proof of concept, we take in a string as the doc<input>

// use std::io;

pub fn submit_docs(doc: u32) -> bool {
    // assume we are submitting an id
    // parse the doc
    // calculate age
    // if > 18 then legal age
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Could not read the file");
    // doc = input
    //     .trim()
    //     .parse()
    //     .expect("The value passed should be an integer");
    doc > 18
}
