//! A simple program that takes a regex pattern and a string and returns whether the string
//! matches the pattern.
#![no_main]
sp1_zkvm::entrypoint!(main);

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};
// use regex::Regex;

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.

pub fn main() {
    // Read two inputs from the prover: a regex pattern and a target string.
    // let pattern = sp1_zkvm::io::read::<String>();
    // let target_string = sp1_zkvm::io::read::<String>();
    // let pdf_base64 = sp1_zkvm::io::read::<String>();
    let pdf_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    // let pdf_text = pdf_extract::extract_text_from_mem(&pdf_bytes).unwrap();

    // let result = BASE64_STANDARD.decode(pdf_base64.as_bytes()).unwrap();

    // // Try to compile the regex pattern. If it fails, write `false` as output and return.
    // let regex = match Regex::new(&pattern) {
    //     Ok(regex) => regex,
    //     Err(_) => {
    //         panic!("Invalid regex pattern");
    //     }
    // };

    // // Perform the regex search on the target string.
    // let result = regex.is_match(&target_string);
    // let result = pdf_text.contains("Lisboa");

    // Write the result (true or false) to the output.
    sp1_zkvm::io::commit(&true);
}
