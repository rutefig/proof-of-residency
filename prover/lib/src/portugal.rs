use regex::Regex;

pub(crate) fn validate(pdf_text: String) -> bool {
     // check the text contains a valid Portuguese postal code
     let postal_code = extract_postal_code(&pdf_text);

     // check the text contains a valid ATCUD
     is_valid_atcud(&pdf_text).is_some() && postal_code.is_some()
}

pub(super) fn is_valid_atcud(text: &str) -> Option<String> {
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

        // TODO: validate from the government's API that the ATCUD is valid
        valid_atcud
}

pub(super) fn extract_postal_code(text: &str) -> Option<String> {
    // Portuguese postal code pattern: XXXX-XXX
    let postal_code_re = Regex::new(r"\b\d{4}-\d{3}\b").unwrap();
    
    postal_code_re.find(text)
        .map(|m| m.as_str().to_string())
}