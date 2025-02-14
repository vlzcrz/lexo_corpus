pub mod modules;
use std::{collections::HashMap, io};

use modules::{
    file_handlers::{
        create_csv_ordered, division_pdf, get_files_from_folder, read_document_pdf,
        read_tet_document_pdf,
    },
    lexical_analisis::{analyzer_content, create_inter_words, initializer_word_hashmap_handler},
    linear_regression::linear_regression_x1,
    menu::main_menu,
    plot_handlers::{scatter_plot, to_tuples},
    zipfs_handlers::{apply_to_log10, get_zipf_law_results},
};

fn main() {
    main_menu();
    // Directorio donde se encuentran los pdf
    let base_path = String::from("books-pdf/");
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

    let file_names_extension_vector = get_files_from_folder("books-pdf");
    println!("{:?}", file_names_extension_vector);

    println!(
        "Ingresa el nombre del archivo con su extension .txt ó .pdf (Presione '0' para cancelar)"
    );
    io::stdin()
        .read_line(&mut file_path_input)
        .expect("[DEBUG] Error imprevisto: no deberia ejecutarse jamas ln:17 - main.rs");

    let file_path = base_path.clone() + file_path_input.trim();
    let file_path = file_path.as_str();
    let mut content = match read_document_pdf(&file_path) {
        Ok(content) => {
            did_read = true;
            content
                .to_lowercase()
                .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], "")
        }
        Err(_) => {
            if file_path_input.trim() == "0" {
                return;
                did_read = true;
            }
            println!("error");
            file_path_input = String::new();
            String::new()
        }
    };
    while !did_read {
        println!("Ingresa el nombre del archivo con su extension .txt ó .pdf (Presione '0' para cancelar)");
        io::stdin()
            .read_line(&mut file_path_input)
            .expect("[DEBUG] Error imprevisto: no deberia ejecutarse jamas ln:36 - main.rs");

        if file_path_input.trim() == "0" {
            did_read = true;
        }

        let file_path: String = base_path.clone() + file_path_input.trim();
        let file_path = file_path.as_str();
        content = match read_document_pdf(&file_path) {
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
    file_path_input = format!("{}", file_path_input.trim());
    let (file_name, file_extension) = file_path_input.split_once(".").unwrap();
    // TODO: personalizar el error para evitar el uso de unwrap() en caso de fallo.
    let (mut inter_words_hashmaps, mut last_positions, inter_words_strings) =
        create_inter_words().unwrap();
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
    create_csv_ordered(&keys, &values, file_name);
    let (log_ranking, log_values) = apply_to_log10(values).unwrap();
    let parameters = linear_regression_x1(&log_values, &log_ranking).unwrap();
    let tuple_to_plot = to_tuples(log_ranking, log_values).unwrap();
    scatter_plot(tuple_to_plot, file_name, &parameters).unwrap();

    // [DEBUG]
    //println!("{:?}", keys);
    //println!("{:?}", words);
    //println!("{:?}", inter_words_strings);
    //println!("{:?}", inter_words_hashmaps);
    println!("{:?}", parameters);
}
