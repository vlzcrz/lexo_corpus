use std::{collections::HashMap, io};

use lexo_corpus::modules::{
    file_handlers::{create_csv_ordered, read_document_pdf},
    lexical_analisis::{analyzer_content, initializer_word_hashmap_handler},
    linear_regression::linear_regression_x1,
    plot_handlers::{scatter_plot, to_tuples},
    zipfs_handlers::{apply_to_log10, get_zipf_law_results},
};

fn main() {
    // Directorio donde se encuentran los pdf
    let base_path = String::from("books-pdf/");
    // Variable de salida para cuando se ingresa un pdf correcto
    let mut did_read = false;
    // Listado de palabras permitidas para filtrar dentro del corpus
    let mut ascii_interest: Vec<u8> = (97..121).collect();
    ascii_interest.push(39);
    // Nombre del archivo ingresado por el usuario
    let mut file_path_input = String::new();
    // HashMap para guardar las palabras encontradas dentro del texto junto con su cantidad de repeticiones
    let mut words: HashMap<String, u32> = HashMap::new();

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
                did_read = true
            }

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
    let (file_name, _) = file_path_input.split_once(".").unwrap();
    // TODO: personalizar el error para evitar el uso de unwrap() en caso de fallo.

    let (inter_words_hashmaps, inter_words_strings) =
        analyzer_content(content, &mut words, &ascii_interest).unwrap();

    let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
    if keys.is_empty() && values.is_empty() {
        return;
    }

    get_zipf_law_results(&mut keys, &mut values);
    create_csv_ordered(&keys, &values, file_name);
    let (log_ranking, log_values) = apply_to_log10(values).unwrap();

    let parameters = linear_regression_x1(&log_values, &log_ranking).unwrap();

    let tuple_to_plot = to_tuples(log_ranking, log_values).unwrap();

    // [DEBUG]
    //println!("{:?}", keys);
    //println!("{:?}", words);
    //println!("{:?}", inter_words_strings);
    //println!("{:?}", inter_words_hashmaps);
    println!("{:?}", parameters);

    scatter_plot(tuple_to_plot, file_name, &parameters).unwrap();
}
