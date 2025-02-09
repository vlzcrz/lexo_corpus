use std::{
    fs::File,
    io::{Error, Read},
};

// una función que permita leer el documento pdf
pub fn read_document_pdf(path: &str) -> Result<String, Error> {
    let bytes = std::fs::read(path).unwrap();
    let content = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    Ok(content)
}

// una función que permita leer el documento txt
pub fn read_document_txt(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}
