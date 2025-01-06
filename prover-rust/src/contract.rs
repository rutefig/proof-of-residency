#![no_main]
#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::vec;

use hyle_contract_sdk::guest;
use hyle_contract_sdk::BlobIndex;
use hyle_contract_sdk::HyleOutput;
use hyle_contract_sdk::Identity;
use hyle_contract_sdk::StateDigest;
use hyle_contract_sdk::TxHash;

use lib::{Config, Scope, Country};

#[cfg(feature = "risc0-guest")]
risc0_zkvm::guest::entry!(main);

#[cfg(feature = "sp1-guest")]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let (pdf_bytes, tx_hash): (Vec<u8>, String) = guest::env::read();

    let result = prover_lib::run(
        &pdf_bytes,
        Config {
            scope: Scope::Country,
            country: Country::Portugal,
        },
    );

    // TODO: Improve the state on Hyle to be more meaningful and useful (using timestamps and scoped location)
    guest::env::commit(&HyleOutput {
        version: 1,
        initial_state: StateDigest("".to_be_bytes().to_vec()),
        next_state: StateDigest("Portugal".as_bytes().to_vec()),
        identity: Identity("".to_owned()),
        tx_hash: TxHash(tx_hash),
        index: BlobIndex(0),
        blobs: vec![],
        success: result,
        program_outputs: vec![],
    });
}
