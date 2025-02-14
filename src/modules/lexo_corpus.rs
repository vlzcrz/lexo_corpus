use std::{collections::HashMap, fs, io};

use crate::modules::{
    file_handlers::{document_extract_content, read_document_pdf},
    lexical_analisis::create_inter_words,
};

use super::lexical_analisis::{analyzer_content, initializer_word_hashmap_handler};

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
    let (mut inter_words_hashmaps, mut last_positions, inter_words_strings) =
        create_inter_words().unwrap();

    let mut content = String::new();

    while !did_read {
        println!("Ingresa el nombre del archivo con su extension .txt ó .pdf (Presione '0' para cancelar)");
        io::stdin()
            .read_line(&mut file_path_input)
            .expect("[DEBUG] Error imprevisto: no deberia ejecutarse jamas ln:36 - main.rs");

        if file_path_input.trim() == "0" {
            did_read = true;
            return;
        }

        file_path_input = format!("books-pdf/{}", file_path_input.trim());
        let (name_f, extension_f) = file_path_input
            .strip_prefix("books-pdf/")
            .unwrap()
            .split_once(".")
            .unwrap();
        println!("name_f: {} extension_f:{}", name_f, extension_f);
        content = match document_extract_content(name_f, extension_f) {
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

    println!("{}", content);

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
}
