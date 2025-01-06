use clap::{Parser, Subcommand};
use hyle_contract_sdk::HyleOutput;
use risc0_zkvm::sha::Digestible;

include!(concat!(env!("OUT_DIR"), "/methods.rs"));

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

    let receipt = match &cli.command {
        Commands::Next { input } => prove(cli.reproducible, *input, 0),
        Commands::Reset { input } => prove(cli.reproducible, 1, *input),
    };

    // Write to proof.bin
    borsh::to_writer(&std::fs::File::create("proof.bin").unwrap(), &receipt).unwrap();

    let hyle_output = receipt.journal.decode::<HyleOutput>().unwrap();

    let initial_state = u32::from_be_bytes(hyle_output.initial_state.0.try_into().unwrap());
    let next_state = u32::from_be_bytes(hyle_output.next_state.0.try_into().unwrap());

    println!("{}", "-".repeat(20));
    println!(
        "Method ID: {:?} (hex)",
        receipt.inner.claim().unwrap().digest()
    );
    println!(
        "proof.bin written, transition from {} to {}",
        initial_state, next_state
    );
}

fn prove(reproducible: bool, initial_state: u32, suggested_number: u32) -> risc0_zkvm::Receipt {
    let env = risc0_zkvm::ExecutorEnv::builder()
        .write(&(initial_state, suggested_number))
        .unwrap()
        .build()
        .unwrap();

    let prover = risc0_zkvm::default_prover();
    let binary = if reproducible {
        std::fs::read("target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/method/method")
            .expect("Could not read ELF binary at target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/method/method")
    } else {
        PROOF_OF_RESIDENCY_GUEST_RISC0_ELF.to_vec()
    };
    prover.prove(env, &binary).unwrap().receipt
}
