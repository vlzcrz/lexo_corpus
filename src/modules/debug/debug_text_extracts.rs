use std::{
    fs::File,
    io::{self, Error, Read},
};

use pdf_extract::OutputError;
use pyo3::{
    ffi::c_str,
    types::{PyAnyMethods, PyModule},
    Python,
};

use crate::modules::file_handlers::{clean_folder, division_pdf, get_files_from_folder};

pub fn input_doc_file() -> Result<String, Error> {
    let mut file_path_input = String::new();
    //clear_screen();
    println!("Ingresa el nombre del archivo (Presione '0' para cancelar)");
    io::stdin().read_line(&mut file_path_input).unwrap();
    Ok(file_path_input.trim().to_string())
}

pub fn read_document_txt_debug() -> Result<String, Error> {
    let input_file_name = input_doc_file().unwrap();
    if input_file_name == "0" {
        return Ok("0".to_string());
    }

    let file_path = format!("books-txt/{}.txt", input_file_name);
    let mut f = File::open(file_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_document_pdf_debug() -> Result<String, OutputError> {
    let input_file_name = input_doc_file().unwrap();
    if input_file_name == "0" {
        return Ok("0".to_string());
    }

    let file_path = format!("./books-pdf/{}.pdf", input_file_name);
    let bytes = std::fs::read(file_path).map_err(|er| {
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

pub fn read_tet_document_pdf_debug(file_name: &str) -> Result<String, Error> {
    let file_path = format!("./books-fracts/{}.pdf", file_name);

    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/pdf_handler.py"
    )));

    let content = Python::with_gil(|py| {
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

    Ok(content)
}

pub fn read_pdf_tet() -> Result<String, Error> {
    let input_file_name = input_doc_file().unwrap();
    if input_file_name == "0" {
        return Ok("0".to_string());
    }
    division_pdf(input_file_name.as_str()).unwrap();

    let content = match get_files_from_folder("books-fracts") {
        Ok(filename_extension_tuples) => {
            let mut combined_content = String::new();
            for (file, _) in filename_extension_tuples.iter() {
                if let Ok(content) = read_tet_document_pdf_debug(file) {
                    combined_content.push_str(&content);
                    combined_content.push(' ');
                }
            }
            clean_folder("books-fracts");
            combined_content
        }
        Err(e) => {
            println!("Error en get_files_from_folder: {:?}", e);
            String::new()
        }
    };

    Ok(content)
}
