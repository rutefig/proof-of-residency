mod portugal;

pub fn run(pdf_bytes: &[u8]) -> bool {
    use pdf_extract;
    use portugal;
    // TODO: If the file is encoded in base64, decode it

    // from the bytes of the file, extract the text
    let pdf = pdf_extract::extract_text_from_mem(&pdf_bytes).unwrap();

    println!(r#"{:#?}"#, pdf);

   portugal::validate(pdf)
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
