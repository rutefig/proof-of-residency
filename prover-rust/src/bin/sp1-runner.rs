use clap::{Parser, Subcommand};
use hyle_contract_sdk::HyleOutput;

use sp1_sdk::{include_elf, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey};

pub const ELF: &[u8] = include_elf!("proof-of-residency-guest-sp1");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[clap(long, short)]
    reproducible: bool,
}

#[derive(Subcommand)]
enum Commands {
    Next { input: u32 },
    Reset { input: u32 },
}

fn main() {
    let cli = Cli::parse();

    if cli.reproducible {
        println!("Running with reproducible ELF binary.");
    } else {
        println!("Running non-reproducibly");
    }

    let (mut receipt, vk) = match &cli.command {
        Commands::Next { input } => prove(cli.reproducible, *input, 0),
        Commands::Reset { input } => prove(cli.reproducible, 1, *input),
    };

    // Write to proof.bin
    receipt.save("proof.bin").unwrap();

    let hyle_output: HyleOutput = receipt.public_values.read();

    let initial_state = u32::from_be_bytes(hyle_output.initial_state.0.try_into().unwrap());
    let next_state = u32::from_be_bytes(hyle_output.next_state.0.try_into().unwrap());

    println!("{}", "-".repeat(20));

    serde_json::to_writer(std::fs::File::create("vkey.json").unwrap(), &vk).unwrap();

    println!("Method ID saved to vkey.json");
    println!(
        "proof.bin written, transition from {} to {}",
        initial_state, next_state
    );
}

fn prove(
    _reproducible: bool,
    initial_state: u32,
    suggested_number: u32,
) -> (SP1ProofWithPublicValues, SP1VerifyingKey) {
    // Setup the prover client.
    let client = ProverClient::new();
    // Setup the program for proving.
    let (pk, vk) = client.setup(ELF);

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&(initial_state, suggested_number));

    (
        client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof"),
        vk,
    )
}