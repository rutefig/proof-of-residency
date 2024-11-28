use pdf_extract;
use portugal::{extract_postal_code, is_valid_atcud};
use regex::Regex;
mod portugal;

fn is_base64(s: &str) -> bool {
    // Basic check if string looks like base64
    let base64_regex = Regex::new(r"^[A-Za-z0-9+/]*={0,2}$").unwrap();
    base64_regex.is_match(s.trim()) && s.len() % 4 == 0
}

pub fn run(pdf_bytes: &[u8]) -> bool {
    // TODO: If the file is encoded in base64, decode it

    // from the bytes of the file, extract the text
    let pdf = pdf_extract::extract_text_from_mem(&pdf_bytes).unwrap();
    // let pdf = pdf_extract::extract_text("fatura_net.pdf").unwrap();


    
    println!(r#"{:#?}"#, pdf);

    // check the text contains a valid Portuguese postal code
    let postal_code = extract_postal_code(&pdf);

    // check the text contains a valid ATCUD
    is_valid_atcud(&pdf).is_some() && postal_code.is_some()

    // TODO: Validate Proof of Residency Scope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file_bytes = std::fs::read("FaturaIberdrola.pdf").unwrap();
        let result = run(&file_bytes);
        assert_eq!(result, true);
    }
}
