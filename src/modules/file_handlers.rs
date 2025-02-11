use std::{
    collections::HashMap,
    fs::File,
    io::{stdout, Read},
};

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

// Función que crea los listado de palabras totales no ordenadas
pub fn create_csv_unordered(words: &HashMap<String, u32>) {
    let mut word_list = csv::Writer::from_writer(stdout());

    for (word, frequency) in words.iter() {
        word_list
            .write_record([word, &frequency.to_string()])
            .unwrap();
    }
    word_list.flush().unwrap();
}

// FUnción que crea los listados de palabras totales y las n50 palabras mas frecuentes de manera ordenada

pub fn create_csv_ordered(keys: &Vec<String>, values: &Vec<u32>, file_path: &str) {
    let file_path_nall = format!("books-data/{}.csv", file_path);
    let file_path_n50 = format!("books-data/{}-n50.csv", file_path);
    let mut word_list = csv::Writer::from_path(file_path_nall).unwrap();
    let mut word_list_n50 = csv::Writer::from_path(file_path_n50).unwrap();
    for (index, word) in keys.iter().enumerate() {
        word_list
            .write_record([word, &values[index].to_string()])
            .unwrap();
    }
    word_list.flush().unwrap();

    for index in 0..50 {
        word_list_n50
            .write_record([keys[index].to_string(), values[index].to_string()])
            .unwrap();
    }
    word_list_n50.flush().unwrap();
}
