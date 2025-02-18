use std::{
    collections::HashMap,
    fs::{self, File},
    io, result,
};

use crate::modules::{
    file_handlers::{document_extract_content, extract_csv_labeled_data, get_files_from_folder},
    lexical_analisis::{create_inter_words, create_inter_words_differ, input_inter_words},
    plot_handlers::{hashmap_means, scatter_plot_alpha, to_tuples_x_int},
};

use super::{
    file_handlers::{create_csv_inter_words, create_csv_ordered},
    lexical_analisis::{analyzer_content, initializer_word_hashmap_handler},
    linear_regression::linear_regression_x1,
    plot_handlers::{scatter_plot, to_tuples},
    zipfs_handlers::{apply_to_log10, get_zipf_law_results},
};

pub fn option_one() {
    // Directorio donde se encuentran los pdf
    //let base_path = String::from("books-pdf/");
    // Variable de salida para cuando se ingresa un pdf correcto
    let mut did_read = false;
    // Listado de palabras permitidas para filtrar dentro del corpus
    let mut ascii_interest: Vec<u8> = (97..121).collect();
    ascii_interest.push(39);
    let mut ascii_interest_numbers: Vec<u8> = (48..57).collect();
    ascii_interest.append(&mut ascii_interest_numbers);
    // Nombre del archivo ingresado por el usuario
    let mut file_path_input = String::new();
    // HashMap para guardar las palabras encontradas dentro del texto junto con su cantidad de repeticiones
    let mut words: HashMap<String, u32> = HashMap::new();
    // HashMap para guardar las palabras inter-word de interes el documento
    let (mut inter_words_hashmaps, mut last_positions, inter_words_strings) =
        create_inter_words().unwrap();

    let mut content = String::new();
    let mut file_name = String::new();
    let mut file_extension = String::new();

    while !did_read {
        println!("Ingresa el nombre del archivo con su extension .txt ó .pdf (Presione '0' para cancelar)");
        io::stdin()
            .read_line(&mut file_path_input)
            .expect("[DEBUG] Error imprevisto: no deberia ejecutarse jamas ln:36 - main.rs");

        if file_path_input.trim() == "0" {
            did_read = true;
            return;
        }

        let (name_f, extension_f) = file_path_input.split_once(".").unwrap();
        file_name = name_f.trim().to_string();
        file_extension = extension_f.trim().to_string();
        content = match document_extract_content(&file_name, &file_extension) {
            Ok(content) => {
                did_read = true;
                content
                    .to_lowercase()
                    .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], "")
            }
            Err(_) => {
                file_path_input = String::new();
                String::new()
            }
        };
    }

    analyzer_content(
        content,
        &mut words,
        &ascii_interest,
        &mut inter_words_hashmaps,
        &mut last_positions,
        &inter_words_strings,
    );
    let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
    if keys.is_empty() && values.is_empty() {
        return;
    }

    get_zipf_law_results(&mut keys, &mut values);
    create_csv_ordered(&keys, &values, &file_name);
    create_csv_inter_words(&file_name, &inter_words_hashmaps, &inter_words_strings);
    let (log_ranking, log_values) = apply_to_log10(values).unwrap();
    let parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();
    println!("{:?}", parameters);
    let tuples_to_plot = to_tuples(log_ranking, log_values).unwrap();
    scatter_plot(tuples_to_plot, &file_name, &parameters).unwrap();
}

pub fn option_two() {
    let mut input = String::new();
    let mut valid_input = false;
    while !valid_input {
        println!("Seleccione un data label (csv) para iniciar el lote de procesamiento de textos. (Presione '0' para cancelar)");
        let labeled_data_files = get_files_from_folder("labeled-data").unwrap();
        let max = labeled_data_files.len() as u16;
        for (index, (file_name, file_extension)) in labeled_data_files.iter().enumerate() {
            println!("{}.- {}.{}", index + 1, file_name, file_extension);
        }
        io::stdin()
            .read_line(&mut input)
            .expect("[DEBUG] Error imprevisto: no deberia ejecutarse jamas ln:36 - main.rs");

        if input.trim() == "0" {
            return;
        }

        let parsed_input: u16 = match input.trim().parse() {
            Ok(parsed_int) => parsed_int,
            Err(error) => {
                println!("Solo se permite ingresar caracteres numericos y no alfanumericos u otro tipo. {:?}", error);
                input = String::new();
                continue;
            }
        };

        if parsed_input > max {
            input = String::new();
            continue;
        }
        let (file_name_dataset, file_extension_dataset) =
            &labeled_data_files[(parsed_input - 1) as usize];

        if !file_name_dataset.is_empty() {
            valid_input = true;
        }
        println!(
            "Archivo seleccionado: {}.{}",
            file_name_dataset, file_extension_dataset
        );

        let csv_content =
            extract_csv_labeled_data(&file_name_dataset, &file_extension_dataset).unwrap();
        let mut year_alphas_hashmaps: HashMap<i32, Vec<f64>> = HashMap::new();
        let inter_words_strings = input_inter_words().unwrap();
        let mut ascii_interest: Vec<u8> = (97..121).collect();
        ascii_interest.push(39);
        let mut ascii_interest_numbers: Vec<u8> = (48..57).collect();
        ascii_interest.append(&mut ascii_interest_numbers);
        for (file, year) in csv_content.iter() {
            let (file_name, file_extension) = file.split_once(".").unwrap();
            let mut words: HashMap<String, u32> = HashMap::new();
            let (mut inter_words_hashmaps, mut last_positions) =
                create_inter_words_differ(&inter_words_strings).unwrap();
            let content = match document_extract_content(&file_name, &file_extension) {
                Ok(content) => content
                    .to_lowercase()
                    .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], ""),
                Err(_) => String::new(),
            };
            analyzer_content(
                content,
                &mut words,
                &ascii_interest,
                &mut inter_words_hashmaps,
                &mut last_positions,
                &inter_words_strings,
            );
            let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
            if keys.is_empty() && values.is_empty() {
                continue;
            }

            get_zipf_law_results(&mut keys, &mut values);
            create_csv_ordered(&keys, &values, &file_name);
            create_csv_inter_words(&file_name, &inter_words_hashmaps, &inter_words_strings);
            let (log_ranking, log_values) = apply_to_log10(values).unwrap();
            let parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();
            let tuples_to_plot = to_tuples(log_ranking, log_values).unwrap();
            scatter_plot(tuples_to_plot, &file_name, &parameters).unwrap();

            let alphas = year_alphas_hashmaps
                .entry(*year)
                .or_insert(vec![parameters[1].abs()]);
            alphas.push(parameters[1].abs());
        }

        let (x_values, y_values) = hashmap_means(year_alphas_hashmaps).unwrap();
        let mut tuples_to_plot = to_tuples_x_int(x_values, y_values).unwrap();
        tuples_to_plot.sort_by_key(|k| k.0);
        scatter_plot_alpha(tuples_to_plot, file_name_dataset).unwrap();
    }
}
