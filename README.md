# Proof of Residency

**Proof of Residency** enables users to generate a verifiable proof of residency from a pdf with an ATCud-signed utility bill (available in Portugal). The project leverages cryptographic techniques to preserve privacy, ensuring that while the country of residence is publicly verifiable, sensitive details like the specific address remain private.

By relying on trusted email sources—such as government communications, utility bills, or legal contracts, this project minimizes reliance on traditional and invasive verification methods.

## Features

- **Privacy preservation:** Keeps specific address details private while publicly verifying only the country of residence.
- **Recipient verification:** Ensures the email recipient is the individual proving residency.
- **Onchain:** Supports cryptographic proofs and decentralized validation.

---

## How it works

### Components

The Proof of Residency project employs the following technologies:

- **[ATCud](https://info.portaldasfinancas.gov.pt/pt/apoio_contribuinte/Faturacao/Comunicacao_Series_ATCUD/Paginas/default.aspx)** signature for documents
- **[SP1](https://docs.succinct.xyz/)**: Extracts and verifies document signatures from emails with attached PDFs; generates proof.
- **[Hyli](https://docs.hyli.org/)**: Adds verifiability to residency proofs.

### Step by step

1. **Submit a valid bill**: Upload an valid pdf (e.g., a utility bill or government communication) as proof of residence.
2. **Generate proof**: Use the system to parse attached PDFs, extract necessary data, and generate a privacy-preserving proof of residency.
3. **Share the proof**: Provide verifiable proof to third parties while keeping sensitive details private.

---

## How to install and run

1. Clone [Hyli](https://github.com/hyli-org/hyli) and start a single node devnet

2. Build the prover

   ```bash
    cd prover
    cargo build --all --release
    ```

3. Run the server

   ```bash
    cd ../server
    cargo run
    ```

4. Run the client (in another terminal)

   ```bash
    cd web
    npm run dev
    ```

## Contributing

Contributions are welcome! To get started:

- Fork this repository.
- Create a feature branch for your changes.
- Submit a pull request detailing your improvements.

---

## Sponsor

<p align="left">
  <a href="https://hyli.org" target="_blank"> <img src="https://github.com/hyli-org/hyli-assets/blob/main/Logos/Logo/HYLI_WORDMARK_ORANGE.png?raw=true" width="15%", height="15%"/></a>
</p>

*This project is supported by [Hyli](https://hyli.org), the new proof-powered L1 to build the next generation of apps, as part of the [Hyli grant program](https://github.com/hyli-org/hyli/blob/main/GRANTS.md).*
