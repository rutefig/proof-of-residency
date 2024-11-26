use pdf_extract;
use regex::Regex;

fn is_valid_atcud(text: &str) -> Option<String> {
    // ATCUD format: XXXXXXXX-Y+
    // where X is an 8-character series and Y is one or more digits
    let re = Regex::new(r"([A-Z0-9]{8}-\d+)").unwrap();
    
    // Find all matches and return the first valid one
    let valid_atcud = re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .find(|atcud| {
            // Additional validation can be added here if needed
            let parts: Vec<&str> = atcud.split('-').collect();
            if parts.len() != 2 {
                return false;
            }
            
            let series = parts[0];
            let sequence = parts[1];
            
            // Verify series is exactly 8 characters
            series.len() == 8 && 
            // Verify sequence has at least 1 digit and all characters are digits
            !sequence.is_empty() && sequence.chars().all(|c| c.is_digit(10))
        });
        valid_atcud
}

pub fn run(pdf_bytes: &[u8]) -> bool {
    // TODO: If the file is encoded in base64, decode it

    // from the bytes of the file, extract the text
    let pdf = pdf_extract::extract_text_from_mem(&pdf_bytes).unwrap();
    // let pdf = pdf_extract::extract_text("fatura_net.pdf").unwrap();


    
    println!(r#"{:#?}"#, pdf);

    // check the text contains a valid ATCUD
    is_valid_atcud(&pdf).is_some()

    // TODO: Validate Proof of Residency Scope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let file_bytes = std::fs::read("fatura_net.pdf").unwrap();
        let result = run(&file_bytes);
        assert_eq!(result, true);
    }
}
