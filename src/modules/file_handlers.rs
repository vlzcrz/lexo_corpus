use std::{fs::File, io::Read};

use pdf_extract::{Error, OutputError};

// una función que permita leer el documento pdf
pub fn read_document_pdf(path: &str) -> Result<String, OutputError> {
    let bytes = std::fs::read(path).map_err(|er| {
        eprintln!("Error al leer el documeto pdf, asegurese de que el nombre del archivo coincida con el valor ingresado. Error: {}", er);
        er
    })?;
    let content = pdf_extract::extract_text_from_mem(&bytes).map_err(|er| {
        eprintln!(
            "Error al extraer el contenido del pdf, pdf no compatible. Error: {}",
            er
        );
        er
    })?;

    Ok(content)
}

// una función que permita leer el documento txt
pub fn read_document_txt(path: &str) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}
