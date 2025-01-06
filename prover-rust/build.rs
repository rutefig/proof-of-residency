fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "risc0")]
    {
        // For risc0, we simply recompile the crate with the risc0-guest target,
        // which will compile the contrat to run in the Risc0 ZKVM.
        use risc0_build::GuestOptions;
        use std::collections::HashMap;

        let mut options = HashMap::new();
        options.insert(
            "hyle-collatz-conjecture",
            GuestOptions {
                features: vec!["risc0-guest".to_owned()],
                use_docker: None,
            },
        );
        risc0_build::embed_methods_with_options(options);
    }
    #[cfg(feature = "sp1")]
    {
        use sp1_helper::build_program_with_args;
        use sp1_helper::BuildArgs;

        build_program_with_args(
            ".",
            BuildArgs {
                features: vec!["sp1-guest".to_owned()],
                ..Default::default()
            },
        )
    }
}