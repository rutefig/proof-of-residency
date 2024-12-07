use bytes::BufMut;
use futures::{StreamExt, TryStreamExt};
use prover_script::hyle::{Hyle, HyleNetwork};
use sp1_sdk::{
    include_elf, network::proto::network::ProofMode, utils, HashableKey, NetworkProverV1,
    ProverClient, SP1ProofWithPublicValues, SP1Stdin,
};
use std::convert::Infallible;
use warp::{
    filters::multipart::FormData, http::StatusCode, reject::Rejection, reply::Reply, Filter,
};

/// The ELF we want to execute inside the zkVM.
const REGEX_IO_ELF: &[u8] = include_elf!("prover-program");

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let mut parts = form.into_stream();

    while let Some(Ok(p)) = parts.next().await {
        if p.name() == "file" {
            let content_type = p.content_type();
            match content_type {
                // for now only pdf files are supported
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        let value = p
                            .stream()
                            .try_fold(Vec::new(), |mut vec, data| {
                                vec.put(data);
                                async move { Ok(vec) }
                            })
                            .await
                            .map_err(|e| {
                                eprintln!("reading file error: {}", e);
                                warp::reject::reject()
                            })?;

                        // Create a new stdin with d the input for the program.
                        let mut stdin = SP1Stdin::new();

                        let mut home_dir = std::env::home_dir().unwrap();
                        // TODO: change to configurable path
                        home_dir.push("Documents/Projects/Hyle/hyle-cosmos/hyled");

                        let hyle = Hyle::new(HyleNetwork::Devnet, &home_dir);

                        let null_state = 0u32.to_be_bytes().to_vec();

                        let hyle_input = hyle
                            .publish_payload("default", "residency", "Portugal", &null_state, &value)
                            .unwrap();

                        stdin.write(&hyle_input);

                        // LOCAL PROVER
                        // Generate the proof for the given program and input.
                        let client = ProverClient::new();
                        println!("Prover client created");
                        let (pk, vk) = client.setup(REGEX_IO_ELF);
                        println!("vk: {:?}", vk.bytes32());
                        let mut proof = client.prove(&pk, stdin).run().expect("proving failed");
                        
                        // NETWORK PROVER
                        // let network = NetworkProverV1::new();
                        // let mut proof = network
                        //     .prove(REGEX_IO_ELF, stdin, ProofMode::Core, None)
                        //     .await
                        //     .expect("proving failed");

                        // Read the output.
                        let res = proof.public_values.read::<bool>();
                        println!("res: {}", res);

                        // Verify proof.
                        // client.verify(&proof, &vk).expect("verification failed");

                        // Test a round trip of proof serialization and deserialization. - LATER
                        proof
                            .save("proof-with-pis.bin")
                            .expect("saving proof failed");

                        // // read proof file and return it
                        let proof = std::fs::read("proof-with-pis.bin").unwrap();

                        let response_data = serde_json::json!({
                            "success": true,
                            "result": res,  // The boolean verification result
                            "proof": proof,
                        });

                        return Ok(warp::reply::json(&response_data));
                    }

                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }
        }
    }

    Ok(warp::reply::json(&"success"))
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

#[tokio::main]
async fn main() {
    // Setup a tracer for logging.
    utils::setup_logger();

    // Run the server.
    let upload_route = warp::path("upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST", "GET"]);

    let router = upload_route.recover(handle_rejection).with(cors);
    println!("Server started at localhost:8080");
    warp::serve(router).run(([0, 0, 0, 0], 8080)).await;
}
