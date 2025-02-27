use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{stdout, Error, Read},
};

use pdf_extract::OutputError;
use pyo3::{ffi::c_str, prelude::*};

// una función que permita leer el documento pdf
pub fn read_document_pdf(file_name: &str) -> Result<String, OutputError> {
    let file_path = format!("./books-pdf/{}.pdf", file_name);
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

// una función que permita leer el documento txt
pub fn read_document_txt(file_name: &str) -> Result<String, Error> {
    let file_path = format!("books-txt/{}.txt", file_name);
    let mut f = File::open(file_path)?;
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

pub fn create_csv_ordered(
    keys: &Vec<String>,
    values: &Vec<u32>,
    file_name: &str,
    folder_name: &str,
) {
    let folder_total_words = format!("{}/word-counts", folder_name);
    let folder_total_words_exist = fs::exists(&folder_total_words).unwrap();
    if !folder_total_words_exist {
        fs::create_dir(&folder_total_words).unwrap();
    }

    let folder_n50_words = format!("{}/words-n50", folder_name);
    let folder_n50_words_exist = fs::exists(&folder_n50_words).unwrap();
    if !folder_n50_words_exist {
        fs::create_dir(&folder_n50_words).unwrap();
    }
    let file_path_nall = format!("{}/{}.csv", folder_total_words, file_name);
    let file_path_n50 = format!("{}/{}-n50.csv", folder_n50_words, file_name);
    let mut word_list = csv::Writer::from_path(file_path_nall).unwrap();
    let mut word_list_n50 = csv::Writer::from_path(file_path_n50).unwrap();

    let limit = keys.len().min(50);

    //Headers
    word_list.write_record(["Word", "Frequency"]).unwrap();
    word_list_n50.write_record(["Word", "Frequency"]).unwrap();

    for (index, word) in keys.iter().enumerate() {
        word_list
            .write_record([word, &values[index].to_string()])
            .unwrap();
    }
    word_list.flush().unwrap();

    for index in 0..limit {
        word_list_n50
            .write_record([keys[index].to_string(), values[index].to_string()])
            .unwrap();
    }
    word_list_n50.flush().unwrap();
}

pub fn create_csv_ordered_dataset(
    keys: &Vec<String>,
    values: &Vec<u32>,
    file_name: &str,
    folder_name: &str,
) {
    let folder_total_words = format!("{}/data/word-counts", folder_name);
    let folder_total_words_exist = fs::exists(&folder_total_words).unwrap();
    if !folder_total_words_exist {
        fs::create_dir(&folder_total_words).unwrap();
    }

    let folder_n50_words = format!("{}/data/words-n50", folder_name);
    let folder_n50_words_exist = fs::exists(&folder_n50_words).unwrap();
    if !folder_n50_words_exist {
        fs::create_dir(&folder_n50_words).unwrap();
    }
    let file_path_nall = format!("{}/{}.csv", folder_total_words, file_name);
    let file_path_n50 = format!("{}/{}-n50.csv", folder_n50_words, file_name);
    let mut word_list = csv::Writer::from_path(file_path_nall).unwrap();
    let mut word_list_n50 = csv::Writer::from_path(file_path_n50).unwrap();

    //Headers
    word_list.write_record(["Word", "Frequency"]).unwrap();
    word_list_n50.write_record(["Word", "Frequency"]).unwrap();

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

pub fn create_csv_inter_words(
    file_name: &str,
    inter_words_hashmaps: &Vec<HashMap<u32, u32>>,
    inter_words_strings: &Vec<String>,
    folder_name: &str,
) -> Result<(Vec<Vec<u32>>, Vec<Vec<u32>>), Error> {
    for inter_word_string in inter_words_strings.iter() {
        let folder_inter_word = format!("{}/{}", folder_name, inter_word_string);
        let folder_inter_word_exists = fs::exists(&folder_inter_word).unwrap();
        if !folder_inter_word_exists {
            fs::create_dir(folder_inter_word).unwrap();
        }
    }
    let mut vec_distance: Vec<Vec<u32>> = Vec::new();
    let mut vec_frequency: Vec<Vec<u32>> = Vec::new();

    for (index, inter_word_hashmap) in inter_words_hashmaps.iter().enumerate() {
        let mut distances: Vec<u32> = Vec::new();
        let mut frecuencies: Vec<u32> = Vec::new();
        let inter_word_path = format!(
            "{}/{}/{}-interword-{}.csv",
            folder_name, inter_words_strings[index], file_name, inter_words_strings[index]
        );
        let mut inter_word_list = csv::Writer::from_path(inter_word_path).unwrap();
        // Header
        inter_word_list
            .write_record(["Distance", "Frequency"])
            .unwrap();

        for (token_distance, frequency) in inter_word_hashmap.iter() {
            distances.push(*token_distance);
            frecuencies.push(*frequency);
            inter_word_list
                .write_record([token_distance.to_string(), frequency.to_string()])
                .unwrap();
        }
        vec_distance.push(distances);
        vec_frequency.push(frecuencies);
        inter_word_list.flush().unwrap();
    }

    Ok((vec_distance, vec_frequency))
}

type DocumentFile = (String, i32);

pub fn extract_csv_labeled_data(
    file_name: &str,
    file_extension: &str,
) -> Result<Vec<(String, i32)>, Error> {
    let file_path = format!("./labeled-data/{}.{}", file_name, file_extension);
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut csv_content: Vec<(String, i32)> = Vec::new();
    for result in rdr.deserialize() {
        let record: DocumentFile = result.unwrap();
        csv_content.push(record);
    }
    Ok(csv_content)
}

pub fn division_pdf(file_name: &str) -> Result<bool, Error> {
    let file_path = format!("books-pdf/{}.pdf", file_name);
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/file_handler.py"
    )));

    let call_result = Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("file_handler"), c_str!("file_handler")).unwrap();
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

pub fn get_files_from_folder(folder_name: &str) -> Result<Vec<(String, String)>, Error> {
    let folder_path = format!("./{}/", folder_name);
    let paths = fs::read_dir(&folder_path).unwrap();
    let mut files_and_extensions_tuple: Vec<(String, String)> = Vec::new();
    for path in paths {
        let file = path.unwrap().path();
        let file_trim = file.strip_prefix(&folder_path).unwrap();
        let file_name = file_trim.file_stem().and_then(OsStr::to_str).unwrap();
        let file_extension = file_trim.extension().and_then(OsStr::to_str).unwrap();
        files_and_extensions_tuple.push((file_name.to_string(), file_extension.to_string()));
    }

    files_and_extensions_tuple.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files_and_extensions_tuple)
}

pub fn read_tet_document_pdf(file_name: &str) -> Result<String, Error> {
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

pub fn document_extract_content(file_name: &str, file_extension: &str) -> Result<String, Error> {
    if file_extension == "txt" {
        return read_document_txt(file_name);
    }

    if file_extension == "pdf" {
        let content = match read_document_pdf(file_name) {
            Ok(content) if !content.is_empty() => content, // Si la función tiene éxito y el contenido no está vacío, úsalo.
            Ok(_) => {
                println!("Problemas al extraer el contenido. ,intentando alternativa...");

                division_pdf(file_name).unwrap();

                match get_files_from_folder("books-fracts") {
                    Ok(filename_extension_tuples) => {
                        let mut combined_content = String::new();
                        for (file, _) in filename_extension_tuples.iter() {
                            if let Ok(content) = read_tet_document_pdf(file) {
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
                }
            }
            Err(error) => {
                println!(
                    "Problemas al leer el PDF. Error:{:?}, intentando alternativa...",
                    error
                );

                division_pdf(file_name).unwrap();

                match get_files_from_folder("books-fracts") {
                    Ok(filename_extension_tuples) => {
                        let mut combined_content = String::new();
                        for (file, _) in filename_extension_tuples.iter() {
                            if let Ok(content) = read_tet_document_pdf(file) {
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
                }
            }
        };
        Ok(content)
    } else {
        println!("El archivo no tiene extension 'txt' ó 'pdf' ");
        Ok(String::new())
    }
}

pub fn clean_folder(folder_name: &str) {
    let folder_path = format!("./{}/", folder_name);
    fs::remove_dir_all(&folder_path).unwrap();
    fs::create_dir(&folder_path).unwrap();
}
