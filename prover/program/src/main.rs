// prover/program/src/main.rs
//! A simple program that takes a regex pattern and a string and returns whether the string
//! matches the pattern.
#![no_main]

use hyle_contract_sdk::{BlobIndex, HyleOutput, Identity, StateDigest, TxHash};
use prover_lib::{Config, Country, Scope};
sp1_zkvm::entrypoint!(main);

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.

pub fn main() {
    // Read a series of bytes from the input, which should be a PDF file.
    let pdf_bytes = sp1_zkvm::io::read::<Vec<u8>>();
    let tx_hash = sp1_zkvm::io::read::<String>();

    let result = prover_lib::run(&pdf_bytes, Config {
        scope: Scope::Country,
        country: Country::Portugal,
    });

    // TODO: Improve the state on Hyle to be more meaningful and useful (using timestamps and scoped location)
    sp1_zkvm::io::commit(&HyleOutput {
        program_outputs: vec![],
        version: 1,
        initial_state: StateDigest("".as_bytes().to_vec()),
        next_state: StateDigest("Portugal".as_bytes().to_vec()), // TODO: change this to the actual next state
        identity: Identity::default(),
        tx_hash: TxHash(tx_hash),
        index: BlobIndex(0),
        blobs: vec![], // TODO: change this to the actual payloads
        success: result,
    });
}
