//! A simple program that takes a regex pattern and a string and returns whether the string
//! matches the pattern.
#![no_main]
sp1_zkvm::entrypoint!(main);

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.

pub fn main() {
    // Read a series of bytes from the input, which should be a PDF file.
    let pdf_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    let result = prover_lib::run(&pdf_bytes);

    println!("result: {}", result);

    // Write the result (true or false) to the output.
    sp1_zkvm::io::commit(&result);
}
