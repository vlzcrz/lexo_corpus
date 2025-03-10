use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, File},
    io::{stdout, Error, Read},
};

use owo_colors::OwoColorize;
use pyo3::{ffi::c_str, prelude::*};

use crate::modules::tesseract_handlers::read_pdf_tesseract;

use super::exception_handlers::AnalysisError;

// una función que permita leer el documento pdf
pub fn read_document_pdf(file_name: &str) -> Result<String, AnalysisError> {
    let file_path = format!("./books-pdf/{}.pdf", file_name);
    let bytes = std::fs::read(file_path).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Lectura fallida: Documento pdf requiere permisos adicionales. [{}]",
            e
        ))
    })?;
    let content = pdf_extract::extract_text_from_mem(&bytes).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al extraer el contenido del pdf, pdf no compatible. [{}]",
            e
        ))
    })?;
    Ok(content)
}

// una función que permita leer el documento txt
pub fn read_document_txt(file_name: &str) -> Result<String, AnalysisError> {
    let file_path = format!("books-txt/{}.txt", file_name);
    let mut f = File::open(file_path).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Lectura fallida: Documento txt requiere permisos adicionales. {}",
            e
        ))
    })?;
    let mut content = String::new();
    f.read_to_string(&mut content).map_err(|e| {
        AnalysisError::ParseError(format!(
            "Error al extraer el contenido del txt, txt no compatible. {}",
            e
        ))
    })?;
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

type RowExtractSingular = (String, i32);

pub fn extract_csv_labeled_data(
    file_name: &str,
    file_extension: &str,
) -> Result<Vec<(String, i32)>, Error> {
    let file_path = format!("./labeled-data-singular/{}.{}", file_name, file_extension);
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut csv_content: Vec<(String, i32)> = Vec::new();
    for result in rdr.deserialize() {
        let record: RowExtractSingular = result.unwrap();
        csv_content.push(record);
    }
    Ok(csv_content)
}

type RowExtractMultiple = (String, i32, String);

pub fn extract_csv_labeled_data_multiple(
    file_name: &str,
    file_extension: &str,
) -> Result<Vec<(String, i32, String)>, Error> {
    let file_path = format!("./labeled-data-multiple/{}.{}", file_name, file_extension);
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut csv_content: Vec<(String, i32, String)> = Vec::new();
    for result in rdr.deserialize() {
        let record: RowExtractMultiple = result.unwrap();
        csv_content.push(record);
    }
    Ok(csv_content)
}

pub fn division_pdf(file_name: &str) -> Result<(), AnalysisError> {
    let file_path = format!("books-pdf/{}.pdf", file_name);
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/file_handler.py"
    )));

    Python::with_gil(|py| {
        let module = PyModule::from_code(py, code, c_str!("file_handler"), c_str!("file_handler"))
            .map_err(|e| {
                AnalysisError::ProcessingError(format!("Error al cargar el módulo Python. {}", e))
            })?;

        let function = module.getattr("split_pdf").map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al obtener la función split_pdf. {}", e))
        })?;

        function.call1((file_path,)).map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al dividir el PDF en Python. {}", e))
        })?;

        println!("División PDF exitosa");
        Ok(())
    })
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

    // Ordenamiento numérico
    files_and_extensions_tuple.sort_by_key(|(name, _)| {
        name.chars()
            .filter(|c| c.is_numeric()) // Extrae solo los números
            .collect::<String>() // Convierte a String
            .parse::<u32>() // Convierte a número
            .unwrap_or(0) // Si falla, usa 0
    });

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

pub fn page_snapshots_by_pdf_pages(file_name: &str) -> Result<(), AnalysisError> {
    let file_path = format!("books-pdf/{}.pdf", file_name);
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/file_handler.py"
    )));

    Python::with_gil(|py| {
        let module = PyModule::from_code(py, code, c_str!("file_handler"), c_str!("file_handler"))
            .map_err(|e| {
                AnalysisError::ProcessingError(format!("Error al cargar el módulo Python. {}", e))
            })?;

        let function = module.getattr("page_snapshots_by_pdf_pages").map_err(|e| {
            AnalysisError::ProcessingError(format!(
                "Error al obtener la función page_snapshots_by_pdf_pages. {}",
                e
            ))
        })?;

        function.call1((file_path,)).unwrap();

        println!("Snapshots PDF exitoso");
        Ok(())
    })
}

pub fn document_extract_content(
    file_name: &str,
    file_extension: &str,
) -> Result<String, AnalysisError> {
    if file_extension == "txt" {
        return read_document_txt(file_name);
    }

    if file_extension == "pdf" {
        let content = match read_document_pdf(file_name) {
            Ok(content) if !content.is_empty() => content,
            // Se tienen que realizar ambos casos, ya que la lectura del pdf puede realizarse y no extraerse ningun contenido o bien falla al abrir el pdf
            Ok(_) => {
                println!(
                    "\n{} -> Problemas al extraer el contenido, intentando alternativa TET ...",
                    " Atención ".on_yellow().bold()
                );

                division_pdf(file_name)?;

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
                    "\n{} -> Problemas al extraer el contenido, intentando alternativa TET ... [{}]", " Atención ".on_yellow().bold(),
                    error
                );

                division_pdf(file_name)?;

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
        println!("Archivo no admitido. debe tener extension 'txt' ó 'pdf' ");
        Ok(String::new())
    }
}

pub fn document_extract_content_tesseract_opt(
    file_name: &str,
    file_extension: &str,
) -> Result<String, AnalysisError> {
    if file_extension == "txt" {
        return read_document_txt(file_name);
    }

    if file_extension == "pdf" {
        let content = match read_document_pdf(file_name) {
            Ok(content) if !content.is_empty() => content,
            // Se tienen que realizar ambos casos, ya que la lectura del pdf puede realizarse y no extraerse ningun contenido o bien falla al abrir el pdf
            Ok(_) => {
                println!(
                    "\n{} -> Problemas al extraer el contenido, intentando alternativa Tesseract-ocr ...",
                    " Atención ".on_yellow().bold()
                );

                read_pdf_tesseract(file_name).unwrap()
            }
            Err(error) => {
                println!(
                    "\n{} -> Problemas al extraer el contenido, intentando alternativa Tesseract-ocr ... [{}]", " Atención ".on_yellow().bold(),
                    error
                );

                read_pdf_tesseract(file_name).unwrap()
            }
        };
        Ok(content)
    } else {
        println!("Archivo no admitido. debe tener extension 'txt' ó 'pdf' ");
        Ok(String::new())
    }
}

pub fn clean_folder(folder_name: &str) {
    let folder_path = format!("./{}/", folder_name);
    fs::remove_dir_all(&folder_path).unwrap();
    fs::create_dir(&folder_path).unwrap();
}

pub fn file_exists(file_name: &str, file_extension: &str) -> Result<bool, AnalysisError> {
    if file_extension == "txt" {
        let file_path = format!("./books-txt/{}.{}", file_name, file_extension);
        let file_exists = fs::exists(file_path).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("El archivo txt no existe. [{}]", e))
        })?;

        if !file_exists {
            println!(
                "{} -> El archivo txt no existe.",
                "Error de ejecución".on_red()
            );
        } else {
            println!("{}", " Archivo encontrado. ".on_green());
        }

        return Ok(file_exists);
    }

    if file_extension == "pdf" {
        let file_path = format!("./books-pdf/{}.{}", file_name, file_extension);
        let file_exists = fs::exists(file_path).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("El archivo pdf no existe. [{}]", e))
        })?;

        if !file_exists {
            println!(
                "{} -> El archivo pdf no existe.",
                "Error de ejecución".on_red()
            );
        } else {
            println!("{}", " Archivo encontrado ".on_green());
        }

        return Ok(file_exists);
    }

    println!(
        "{} -> El archivo debe tener extensión txt o pdf.",
        "Error de ejecución".on_red()
    );
    Ok(false)
}

pub fn file_exists_silenced(file_name: &str, file_extension: &str) -> Result<bool, AnalysisError> {
    if file_extension == "txt" {
        let file_path = format!("./books-txt/{}.{}", file_name, file_extension);
        let file_exists = fs::exists(file_path).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("El archivo txt no existe. [{}]", e))
        })?;

        return Ok(file_exists);
    }

    if file_extension == "pdf" {
        let file_path = format!("./books-pdf/{}.{}", file_name, file_extension);
        let file_exists = fs::exists(file_path).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("El archivo pdf no existe. [{}]", e))
        })?;

        return Ok(file_exists);
    }

    Ok(false)
}

pub fn initialize_warehouse_folders(
    file_name_dataset: &str,
) -> Result<(String, String, String, String, String), AnalysisError> {
    let folder_warehouse = format!("./{}", file_name_dataset);
    let folder_warehouse_data = format!("{}/data", &folder_warehouse);
    let folder_warehouse_plot = format!("{}/plot", &folder_warehouse);
    let folder_warehouse_zipf_plot = format!("{}/zipfs", &folder_warehouse_plot);
    let folder_warehouse_heaps_plot = format!("{}/heaps", &folder_warehouse_plot);
    let folder_warehouse_heatmap_plot = format!("{}/heatmaps", &folder_warehouse_plot);

    let folder_warehouse_exist = fs::exists(&folder_warehouse).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al verificar la carpeta raiz, la ruta no existe ó permisos insuficientes {}",
            e
        ))
    })?;
    let folder_warehouse_data_exist = fs::exists(&folder_warehouse_data).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al verificar la carpeta raiz, la ruta no existe ó permisos insuficientes {}",
            e
        ))
    })?;
    let folder_warehouse_plot_exist = fs::exists(&folder_warehouse_plot).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al verificar la carpeta raiz, la ruta no existe ó permisos insuficientes {}",
            e
        ))
    })?;

    let folder_warehouse_zipfs_plot_exist =
        fs::exists(&folder_warehouse_zipf_plot).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!(
                "Error al verificar la carpeta zipf dentro de plot, la ruta no existe ó permisos insuficientes {}",
                e
            ))
        })?;

    let folder_warehouse_heaps_plot_exist =
        fs::exists(&folder_warehouse_heaps_plot).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!(
                "Error al verificar la carpeta heaps dentro de plot, la ruta no existe ó permisos insuficientes {}",
                e
            ))
        })?;

    let folder_warehouse_heatmaps_plot_exist =
        fs::exists(&folder_warehouse_heatmap_plot).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!(
                "Error al verificar la carpeta heatmaps dentro de plot, la ruta no existe ó permisos insuficientes {}",
                e
            ))
        })?;

    if !folder_warehouse_exist {
        fs::create_dir(&folder_warehouse).unwrap();
    }

    if !folder_warehouse_data_exist {
        fs::create_dir(&folder_warehouse_data).unwrap();
    }

    if !folder_warehouse_plot_exist {
        fs::create_dir(&folder_warehouse_plot).unwrap();
    }

    if !folder_warehouse_zipfs_plot_exist {
        fs::create_dir(&folder_warehouse_zipf_plot).unwrap();
    }

    if !folder_warehouse_heaps_plot_exist {
        fs::create_dir(&folder_warehouse_heaps_plot).unwrap();
    }

    if !folder_warehouse_heatmaps_plot_exist {
        fs::create_dir(&folder_warehouse_heatmap_plot).unwrap();
    }

    Ok((
        folder_warehouse_data,
        folder_warehouse_plot,
        folder_warehouse_zipf_plot,
        folder_warehouse_heaps_plot,
        folder_warehouse_heatmap_plot,
    ))
}

pub fn initialize_main_folders() {
    let folder_fracts_exists = fs::exists("./books-fracts").unwrap();
    if !folder_fracts_exists {
        fs::create_dir("./books-fracts").unwrap();
    }

    let folder_log_exists = fs::exists("./logs").unwrap();
    if !folder_log_exists {
        fs::create_dir("./logs").unwrap();
    }

    let folder_books_pdf_exists = fs::exists("./books-pdf").unwrap();
    if !folder_books_pdf_exists {
        fs::create_dir("./books-pdf").unwrap();
    }

    let folder_books_txt_exists = fs::exists("./books-txt").unwrap();
    if !folder_books_txt_exists {
        fs::create_dir("./books-txt").unwrap();
    }

    let folder_labeled_datasets_exists = fs::exists("./labeled-data-singular").unwrap();
    if !folder_labeled_datasets_exists {
        fs::create_dir("./labeled-data-singular").unwrap();
    }

    let folder_labeled_datasets_exists = fs::exists("./labeled-data-multiple").unwrap();
    if !folder_labeled_datasets_exists {
        fs::create_dir("./labeled-data-multiple").unwrap();
    }
}
