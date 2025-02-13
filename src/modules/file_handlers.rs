use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{stdout, Error, Read},
};

use pdf_extract::OutputError;
use pyo3::{ffi::c_str, prelude::*};

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

pub fn division_pdf(file_name: &str) -> Result<bool, Error> {
    let file_path = format!("books-pdf/{}.pdf", file_name);
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/pdf_handler.py"
    )));

    let call_result = Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("pdf_handler"), c_str!("pdf_handler")).unwrap();
        let function = module.getattr("split_pdf").unwrap();

        // Nombre del archivo PDF de entrada
        //let input_pdf = "books-pdf/tallerads.pdf";

        // Llamar a la función split_pdf en Python
        let result = function.call1((file_path,));

        match result {
            Ok(_) => {
                println!("Division PDF exitosa");
                return true;
            }
            Err(err) => {
                println!("Error al dividir PDF: {:?}", err);
                return false;
            }
        }
    });

    Ok(call_result)
}

pub fn get_files_from_folder() -> Result<Vec<(String, String)>, Error> {
    let paths = fs::read_dir("./books-pdf/").unwrap();
    let mut files_and_extensions_tuple: Vec<(String, String)> = Vec::new();
    for path in paths {
        let file = path.unwrap().path();
        let file_trim = file.strip_prefix("./books-pdf/").unwrap();
        let file_name = file_trim.file_stem().and_then(OsStr::to_str).unwrap();
        let file_extension = file_trim.extension().and_then(OsStr::to_str).unwrap();
        files_and_extensions_tuple.push((file_name.to_string(), file_extension.to_string()));
    }

    Ok(files_and_extensions_tuple)
}

pub fn read_tet_document_pdf(file_name: &str) -> Result<String, Error> {
    let file_path = format!("./books-fracts/{}.pdf", file_name);

    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/pdf_handler.py"
    )));

    let call_result = Python::with_gil(|py| {
        let relative_library_path = "./python/tetlib/bind/python";
        let os = py.import("os").unwrap();
        let os_path = os.getattr("path").unwrap();
        let abs_library_path = os_path
            .call_method1("abspath", (relative_library_path,))
            .unwrap();
        let sys = py.import("sys").unwrap();
        let sys_path = sys.getattr("path").unwrap();

        sys_path
            .call_method1("insert", (0, abs_library_path))
            .unwrap();

        let module = PyModule::from_code(py, code, c_str!("pdf_handler"), c_str!("TET")).unwrap();
        let function = module.getattr("open_and_read_pdf").unwrap();

        let result = function.call1((file_path,)).unwrap();
        let text: String = result.extract().unwrap();
        return text;
    });

    Ok(call_result)
}
