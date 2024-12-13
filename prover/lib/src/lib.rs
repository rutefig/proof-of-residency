// prover/lib/src/lib.rs
mod portugal;

pub enum Scope {
    Country,
    City,
}

pub enum Country {
    Portugal,
}

pub struct Config {
    pub scope: Scope,
    pub country: Country,
}

pub fn run(pdf_bytes: &[u8], config: Config) -> bool {
    use pdf_extract;
    use portugal;
    // TODO: If the file is encoded in base64, decode it

    // from the bytes of the file, extract the text
    let pdf = pdf_extract::extract_text_from_mem(&pdf_bytes).unwrap();

    println!(r#"{:#?}"#, pdf);

    match config.country {
        Country::Portugal => portugal::validate(pdf),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file_bytes = std::fs::read("../../examples/sample_invoice.pdf").unwrap();
        let result = run(&file_bytes, Config {
            scope: Scope::Country,
            country: Country::Portugal,
        });
        assert_eq!(result, true);
    }
}
