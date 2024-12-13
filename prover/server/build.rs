// prover/script/build.rs
use sp1_helper::build_program_with_args;

fn main() {
    build_program_with_args("../program", Default::default())
}
