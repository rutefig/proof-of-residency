//! A simple program that takes a regex pattern and a string and returns whether the string
//! matches the pattern.
#![no_main]

use hyle_contract::{HyleInput, HyleOutput};
use prover_lib::{Config, Country, Scope};
sp1_zkvm::entrypoint!(main);

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.

pub fn main() {
    // Read a series of bytes from the input, which should be a PDF file.
    // let pdf_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let input: HyleInput<Vec<u8>> = sp1_zkvm::io::read::<HyleInput<Vec<u8>>>();

    let pdf_bytes = input.program_inputs;

    let result = prover_lib::run(&pdf_bytes, Config {
        scope: Scope::Country,
        country: Country::Portugal,
    });

    // TODO: Improve 
    sp1_zkvm::io::commit(&HyleOutput {
        program_outputs: result,
        version: 1,
        initial_state: input.initial_state,
        next_state: "Portugal".into(), // TODO: change this to the actual next state
        identity: input.identity,
        tx_hash: input.tx_hash,
        index: 0,
        payloads: pdf_bytes,
        success: result,
    });
}
