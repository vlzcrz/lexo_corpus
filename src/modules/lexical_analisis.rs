use std::{
    collections::HashMap,
    fmt::Write,
    io::{self},
};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use owo_colors::OwoColorize;
use symspell::{AsciiStringStrategy, SymSpell};

use crate::modules::cli_handlers::clear_screen;

use super::exception_handlers::AnalysisError;

/// Verifica que la palabra a analizar sea valida dado un vector de valores ascii permitidos
/// ## Params
/// ```
/// - word: &str
/// - ascii_interest: &Vec<u8>
/// ```
/// word: Palabra en formato string literal a analizar
/// ascii_interest: vector que almacena valores de 8 bit permitiendo valores ascii del 0 al 255
/// ## Returns
/// ```
/// - Ok(bool)
/// ```
/// Si la variable 'word' es admitida retorna true, caso contrario false
///
pub fn is_ascii_valid(word: &str, ascii_interest: &Vec<u8>) -> Result<bool, AnalysisError> {
    let bytes_word = word.as_bytes();
    for byte in bytes_word {
        if !ascii_interest.contains(byte) {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Captura los interwords de interes del usuario por consola uno por uno hasta recibir '0'
///
/// ## Returns
///
/// ```
/// - Ok(Vec<String>)
/// ```
/// Si el usuario ingresó palabras o devuelve el vector vacio.
///
/// ## Errors
///```
/// - AnalysisError::IoError
/// ```
/// Si hay un problema con la entrada de datos por consola.
pub fn input_inter_words() -> Result<Vec<String>, AnalysisError> {
    let mut inter_words: Vec<String> = Vec::new();
    let mut exit_while = false;
    while !exit_while {
        let mut inter_word = String::new();
        clear_screen();
        println!("Ingrese el inter word de interes para analizar en su archivo pdf o txt, ó ingrese '0' para ya no continuar, actualmente tiene seleccionado: {:?}", inter_words);
        io::stdin()
            .read_line(&mut inter_word)
            .map_err(|e| AnalysisError::IoError(e))?;
        let inter_word = inter_word.trim().to_string().to_lowercase();
        if inter_word == "0" {
            exit_while = true;
        } else {
            inter_words.push(inter_word);
        }
    }
    Ok(inter_words)
}

/// Crea y inicializa un vector con los interwords del usuario capturados por consola, un vector de hashmaps (key: distancia, value: frecuencia) para cada interword identificado,
/// un vector para la trazabilidad de posiciones de cada interword (manteniendo correlatividad).
///
/// ## Returns
/// ```
/// - Ok(Vec<HashMap<u32, u32>>, Vec<u32>, Vec<String>)
/// ```
/// Si hay almenos una interword, se elaboran los hashmaps y vectores de trazabilidad de posición (del interword en cuestión) devolviendo una tupla de 3 variables no vacias.
/// de otra forma, devuelve una tupla de 3 variables vacias.
/// ## Errors
/// ```
/// - AnalysisError::IoError
/// ```
/// Si hay un problema con la entrada de datos por consola.
pub fn create_inter_words() -> Result<(Vec<HashMap<u32, u32>>, Vec<u32>, Vec<String>), AnalysisError>
{
    let inter_words_strings: Vec<String> = input_inter_words()?;
    let mut last_positions: Vec<u32> = Vec::new();
    let mut inter_words_hashmaps: Vec<HashMap<u32, u32>> = Vec::new();
    let mut count_strings = 0;
    while count_strings < inter_words_strings.len() {
        let inter_words: HashMap<u32, u32> = HashMap::new();
        last_positions.push(0);
        inter_words_hashmaps.push(inter_words);
        count_strings += 1
    }

    Ok((inter_words_hashmaps, last_positions, inter_words_strings))
}

/// A partir de un vector con los interwords inicializado. Crea y inicializa un vector de hashmaps (key: distancia, value: frecuencia) para cada interword identificado,
/// un vector para la trazabilidad de posiciones de cada interword (manteniendo correlatividad).
///
/// ## Params
/// ```
/// - inter_words_strings: &Vec<String>
/// ```
/// Vector referencial (Borrowed) contenedora de interwords
/// ## Returns
/// ```
/// - Ok(Vec<HashMap<u32, u32>>, Vec<u32>, Vec<String>)
/// ```
/// Si hay almenos una interword, se elaboran los hashmaps y vectores de trazabilidad de posición (del interword en cuestión) devolviendo una tupla de 3 variables no vacias.
/// de otra forma, devuelve una tupla de 3 variables vacias.
/// ## Errors
/// ```
///
/// ```
/// Si hay un problema con la entrada de datos por consola.
pub fn create_inter_words_differ(
    inter_words_strings: &Vec<String>,
) -> Result<(Vec<HashMap<u32, u32>>, Vec<u32>), AnalysisError> {
    let mut last_positions: Vec<u32> = Vec::new();
    let mut inter_words_hashmaps: Vec<HashMap<u32, u32>> = Vec::new();
    let mut count_strings = 0;
    while count_strings < inter_words_strings.len() {
        let inter_words: HashMap<u32, u32> = HashMap::new();
        last_positions.push(0);
        inter_words_hashmaps.push(inter_words);
        count_strings += 1
    }

    Ok((inter_words_hashmaps, last_positions))
}

pub fn symspell_processing(
    content: String,
    symspell: &SymSpell<AsciiStringStrategy>,
) -> Result<String, AnalysisError> {
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut processed_content = String::new();
    let last_word_idx = words.len() - 1;
    // batch dinamico
    let mut batch_size = 16;
    let mut batch_idx = 0;
    println!("Procesamiento sintactico (Symspell)");

    let pb = ProgressBar::new(last_word_idx as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{wide_bar:.white/white}] {percent} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("|-"),
    );
    while batch_idx < last_word_idx {
        let mut sentence_from_ocr = String::new();
        let mut sentence_dummy: String = String::new();
        for word_idx in batch_idx..(batch_idx + batch_size) {
            sentence_dummy.push_str(words[word_idx]);
        }

        if sentence_dummy.len() > 200 {
            batch_size = 6;
        }

        let mut sentence: String = "".to_string();
        let mut sentences: Vec<String> = Vec::new();
        let mut sentence_restructured = String::new();
        for word_idx in batch_idx..(batch_idx + batch_size) {
            sentence_from_ocr.push_str(words[word_idx]);
            sentence_from_ocr.push_str(" ");

            if !words[word_idx].parse::<i32>().is_err() {
                if sentence != "" {
                    sentences.push(sentence.to_string());
                }
                sentences.push(words[word_idx].to_string());
                sentence = "".to_string();
                continue;
            }
            sentence.push_str(words[word_idx]);
            sentence.push_str(" ");

            if word_idx == batch_idx + batch_size - 1 {
                sentences.push(sentence.to_string());
            }
        }

        //println!("{:?}", sentences);

        for s in sentences.into_iter() {
            if !s.parse::<i32>().is_err() {
                sentence_restructured.push_str(&s);
                sentence_restructured.push_str(" ");
            } else if s.split_whitespace().count() == 1 {
                let sym_word = symspell_similarity(&s, symspell)?;
                sentence_restructured.push_str(&sym_word);
                sentence_restructured.push_str(" ");
            } else if s.len() > 10 {
                let seg_word = symspell_segmentation(&s, symspell)?;
                sentence_restructured.push_str(&seg_word);
                sentence_restructured.push_str(" ");
            } else {
                let sym_sentence = symspell_compound(&s, symspell)?;
                sentence_restructured.push_str(&sym_sentence);
                sentence_restructured.push_str(" ");
            }
        }
        //println!(" {} ", sentence_from_ocr.on_red());
        //println!(" {} ", sentence_restructured.on_green());
        processed_content.push_str(&sentence_restructured);
        processed_content.push_str(" ");

        batch_idx += batch_size;
        batch_size = 16;

        pb.set_position(batch_idx as u64);
        // Calculamos de cuanto deberia ser el ultimo batch
        if (batch_size + batch_idx) > last_word_idx {
            let overflow = batch_idx + batch_size - last_word_idx;
            batch_size -= overflow
        }
    }

    Ok(processed_content)
}

pub fn symspell_processing_debug(
    content: String,
    symspell: &SymSpell<AsciiStringStrategy>,
) -> Result<String, AnalysisError> {
    let words: Vec<&str> = content.split_whitespace().collect();
    let mut processed_content = String::new();
    let last_word_idx = words.len() - 1;
    // batch dinamico
    let mut batch_size = 16;
    let mut batch_idx = 0;
    println!("Procesamiento sintactico (Symspell)");

    while batch_idx < last_word_idx {
        let mut sentence_from_ocr = String::new();
        let mut sentence_dummy: String = String::new();
        for word_idx in batch_idx..(batch_idx + batch_size) {
            sentence_dummy.push_str(words[word_idx]);
        }

        if sentence_dummy.len() > 200 {
            batch_size = 6;
        }

        let mut sentence: String = "".to_string();
        let mut sentences: Vec<String> = Vec::new();
        let mut sentence_restructured = String::new();
        for word_idx in batch_idx..(batch_idx + batch_size) {
            sentence_from_ocr.push_str(words[word_idx]);
            sentence_from_ocr.push_str(" ");

            if !words[word_idx].parse::<i32>().is_err() {
                if sentence != "" {
                    sentences.push(sentence.to_string());
                }
                sentences.push(words[word_idx].to_string());
                sentence = "".to_string();
                continue;
            }
            sentence.push_str(words[word_idx]);
            sentence.push_str(" ");

            if word_idx == batch_idx + batch_size - 1 {
                sentences.push(sentence.to_string());
            }
        }

        println!("{:?}", sentences);

        for s in sentences.into_iter() {
            if !s.parse::<i32>().is_err() {
                sentence_restructured.push_str(&s);
                sentence_restructured.push_str(" ");
            } else if s.split_whitespace().count() == 1 {
                let sym_word = symspell_similarity(&s, symspell)?;
                sentence_restructured.push_str(&sym_word);
                sentence_restructured.push_str(" ");
            } else if s.len() > 10 {
                let seg_word = symspell_segmentation(&s, symspell)?;
                sentence_restructured.push_str(&seg_word);
                sentence_restructured.push_str(" ");
            } else {
                let sym_sentence = symspell_compound(&s, symspell)?;
                sentence_restructured.push_str(&sym_sentence);
                sentence_restructured.push_str(" ");
            }
        }
        println!(" {} ", sentence_from_ocr.on_red());
        println!(" {} ", sentence_restructured.on_green());
        processed_content.push_str(&sentence_restructured);
        processed_content.push_str(" ");

        batch_idx += batch_size;
        batch_size = 16;

        // Calculamos de cuanto deberia ser el ultimo batch
        if (batch_size + batch_idx) > last_word_idx {
            let overflow = batch_idx + batch_size - last_word_idx;
            batch_size -= overflow
        }
    }

    Ok(processed_content)
}

fn symspell_compound(
    sentence: &str,
    symspell: &SymSpell<AsciiStringStrategy>,
) -> Result<String, AnalysisError> {
    let mut sentence_restructured = String::new();
    let sentence_restructured_vec = symspell.lookup_compound(&sentence, 2);
    for word_restructured in sentence_restructured_vec {
        //println!("{:?}", word_restructured);
        let term = word_restructured.term;
        sentence_restructured.push_str(&term);
    }
    Ok(sentence_restructured)
}

fn symspell_similarity(
    word: &str,
    symspell: &SymSpell<AsciiStringStrategy>,
) -> Result<String, AnalysisError> {
    let mut word_restructured = String::new();
    let suggestion_vec = symspell.lookup(word, symspell::Verbosity::Top, 2);
    for word_suggestion in suggestion_vec {
        //println!("{:?}", word_suggestion);
        let term = word_suggestion.term;
        word_restructured.push_str(&term);
    }
    Ok(word_restructured)
}

fn symspell_segmentation(
    word: &str,
    symspell: &SymSpell<AsciiStringStrategy>,
) -> Result<String, AnalysisError> {
    let word_restructured: String;
    let suggestion_word = symspell.word_segmentation(word, 2);
    word_restructured = suggestion_word.segmented_string;
    Ok(word_restructured)
}

/// Analiza el contenido de un texto extraido, iterando entre palabras mediante espacios en blancos y saltos de lineas, verificando que la palabra
/// se encuentre permitida a analizarse, guardando la palabra y sus cantidad de repetición sea almacenada.
///
/// ## Params
/// ```
/// - content: String,
/// - words: &mut HashMap<String, u32>,
/// - ascii_interest: &Vec<u8>,
/// - inter_words_hashmaps: &mut Vec<HashMap<u32, u32>>,
/// - last_positions: &mut Vec<u32>,
/// - inter_words_strings: &Vec<String>,
/// ```
/// content: Texto plano extraido de un documento
/// words: Hashmap de palabras (key: palabra, value: frecuencia) para almacenar palabras admitidas para analizar
/// ascii_interest; Vector de 8 bit que almacena valores ascii admitidos para analizar
/// inter_words_hashmaps: Vector de Hashmaps de interwords, cada elemento del vector tiene un hashmap asociado a la ubicación del elemento en inter_words_strings
/// last_positions: Vector auxiliar para mantener trazabilidad del posicionamiento de las palabras interword identificadas
/// inter_words_strings: Vector de palabras interwords de interes para analizar.
/// ## Returns
/// ```
/// - Ok((Vec<u32>, Vec<u32>))
/// ```
/// Retorna una tupla de vectores que contienen la cantidad total de palabras encontradas en el contenido y la cantidad total de palabras unicas encontradas en el texto,
/// realizando multiples registros a medida que el documento se analiza completamente (snapshots dividido en batch de 10).
pub fn analyzer_content(
    content: String,
    words: &mut HashMap<String, u32>,
    ascii_interest: &Vec<u8>,
    inter_words_hashmaps: &mut Vec<HashMap<u32, u32>>,
    last_positions: &mut Vec<u32>,
    inter_words_strings: &Vec<String>,
) -> Result<(Vec<u32>, Vec<u32>), AnalysisError> {
    let len_words = content.split_whitespace().count() as i32 - 1;
    let mut batches: Vec<i32> = Vec::new();
    let mut batch_index_iter = 0;
    for i in 1..=10 {
        batches.push((len_words * i) / 10);
    }
    batches.insert(0, 1);

    let mut n_words_total = 0;
    let mut n_words_unique = 0;

    let mut n_words_total_vec: Vec<u32> = Vec::new();
    let mut n_words_unique_vec: Vec<u32> = Vec::new();

    for (index_word, word) in content.split_whitespace().enumerate() {
        if is_ascii_valid(word, ascii_interest)? {
            let count = words.entry(word.to_string()).or_insert(0);
            if *count == 0 {
                n_words_unique += 1;
            }
            *count += 1;

            n_words_total += 1;

            for (index_input_strings, inter_word_string) in inter_words_strings.iter().enumerate() {
                if word == inter_word_string {
                    if inter_words_hashmaps[index_input_strings].is_empty() {
                        inter_words_hashmaps[index_input_strings]
                            .entry(0)
                            .or_insert(0);

                        last_positions[index_input_strings] = index_word as u32;
                        continue;
                    }
                    let token_distance =
                        index_word as u32 - 1 - last_positions[index_input_strings];
                    let count_distance = inter_words_hashmaps[index_input_strings]
                        .entry(token_distance)
                        .or_insert(0);
                    *count_distance += 1;

                    last_positions[index_input_strings] = index_word as u32;
                }
            }
        }

        if index_word as i32 >= batches[batch_index_iter] {
            n_words_total_vec.push(n_words_total);
            n_words_unique_vec.push(n_words_unique);
            batch_index_iter += 1;
        }
    }

    for hashmap in inter_words_hashmaps.iter_mut() {
        hashmap.remove(&0);
    }

    Ok((n_words_total_vec, n_words_unique_vec))
}

pub fn analyzer_content_dataset_opt(
    content: String,
    words: &mut HashMap<String, u32>,
    words_unique_hashmap: &mut HashMap<String, u32>,
    ascii_interest: &Vec<u8>,
    inter_words_hashmaps: &mut Vec<HashMap<u32, u32>>,
    last_positions: &mut Vec<u32>,
    inter_words_strings: &Vec<String>,
    n_words_total: &mut u32,
    n_words_unique: &mut u32,
    n_words_total_vec: &mut Vec<u32>,
    n_words_unique_vec: &mut Vec<u32>,
) -> Result<(Vec<u32>, Vec<u32>), AnalysisError> {
    let len_words = content.split_whitespace().count() as i32 - 1;
    let mut batches: Vec<i32> = Vec::new();
    let mut batch_index_iter = 0;
    for i in 1..=10 {
        batches.push((len_words * i) / 10);
    }
    //batches.insert(0, 1);

    let mut n_words_total_by_doc: u32 = 0;
    let mut n_words_unique_by_doc: u32 = 0;
    let mut n_words_total_vec_by_doc: Vec<u32> = Vec::new();
    let mut n_words_unique_vec_by_doc: Vec<u32> = Vec::new();

    for (index_word, word) in content.split_whitespace().enumerate() {
        if is_ascii_valid(word, ascii_interest).unwrap() {
            let count = words.entry(word.to_string()).or_insert(0);
            if *count == 0 {
                n_words_unique_by_doc += 1;
            }
            *count += 1;

            let count_unique = words_unique_hashmap.entry(word.to_string()).or_insert(0);
            if *count_unique == 0 {
                *n_words_unique += 1;
            }
            *count_unique += 1;

            *n_words_total += 1;
            n_words_total_by_doc += 1;

            for (index_input_strings, inter_word_string) in inter_words_strings.iter().enumerate() {
                if word == inter_word_string {
                    if inter_words_hashmaps[index_input_strings].is_empty() {
                        inter_words_hashmaps[index_input_strings]
                            .entry(0)
                            .or_insert(0);

                        last_positions[index_input_strings] = index_word as u32;
                        continue;
                    }
                    let token_distance =
                        index_word as u32 - 1 - last_positions[index_input_strings];
                    let count_distance = inter_words_hashmaps[index_input_strings]
                        .entry(token_distance)
                        .or_insert(0);
                    *count_distance += 1;

                    last_positions[index_input_strings] = index_word as u32;
                }
            }
        }

        if index_word as i32 >= batches[batch_index_iter] {
            n_words_total_vec.push(*n_words_total);
            n_words_unique_vec.push(*n_words_unique);
            n_words_total_vec_by_doc.push(n_words_total_by_doc);
            n_words_unique_vec_by_doc.push(n_words_unique_by_doc);
            batch_index_iter += 1;
        }
    }

    for hashmap in inter_words_hashmaps.iter_mut() {
        hashmap.remove(&0);
    }

    Ok((n_words_total_vec_by_doc, n_words_unique_vec_by_doc))
}

pub fn initializer_word_hashmap_handler(
    words: &HashMap<String, u32>,
) -> Result<(Vec<String>, Vec<u32>), AnalysisError> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<u32> = Vec::new();
    for (key, value) in words {
        keys.push(key.to_string());
        values.push(*value);
    }
    if keys.is_empty() && values.is_empty() {
        return Err(AnalysisError::EmptyResultError);
    }
    Ok((keys, values))
}

pub fn copy_interword_to_main(
    main_interword_hashmaps: &mut Vec<HashMap<u32, u32>>,
    sub_interword_hashmaps: &Vec<HashMap<u32, u32>>,
) {
    for (index_sub_hashmap, sub_hashmap) in sub_interword_hashmaps.iter().enumerate() {
        for (distance, frequency) in sub_hashmap.iter() {
            let distance_pointer = main_interword_hashmaps[index_sub_hashmap]
                .entry(*distance)
                .or_insert(0);
            *distance_pointer += frequency;
        }
    }
}

pub fn copy_words_to_main(
    main_words_hashmap: &mut HashMap<String, u32>,
    sub_words_hashmap: &HashMap<String, u32>,
) {
    for (word, frequency) in sub_words_hashmap.iter() {
        let count = main_words_hashmap.entry(word.to_string()).or_insert(0);
        *count += frequency;
    }
}

pub fn analyzer_content_opt3(
    content: String,
    words: &mut HashMap<String, u32>,
    words_per_doc: &mut HashMap<String, u32>,
    ascii_interest: &Vec<u8>,
    inter_words_hashmaps: &mut Vec<HashMap<u32, u32>>,
    last_positions: &mut Vec<u32>,
    inter_words_strings: &Vec<String>,
    n_words_total: &mut u32,
    n_words_unique: &mut u32,
    n_words_total_vec: &mut Vec<u32>,
    n_words_unique_vec: &mut Vec<u32>,
) {
    let len_words = content.split_whitespace().count() as i32 - 1;
    let mut batches: Vec<i32> = Vec::new();
    let mut batch_index_iter = 0;
    for i in 1..=10 {
        batches.push((len_words * i) / 10);
    }
    // batches.insert(0, 1); este dbe iniciarse desde afuera
    let mut n_words_total_by_doc: u32 = 0;
    let mut n_words_unique_by_doc: u32 = 0;
    let mut n_words_total_vec_by_doc: Vec<u32> = Vec::new();
    let mut n_words_unique_vec_by_doc: Vec<u32> = Vec::new();

    for (index_word, word) in content.split_whitespace().enumerate() {
        if is_ascii_valid(word, ascii_interest).unwrap() {
            let count_word_per_doc = words_per_doc.entry(word.to_string()).or_insert(0);
            if *count_word_per_doc == 0 {
                n_words_unique_by_doc += 1;
            }
            *count_word_per_doc += 1;

            let count = words.entry(word.to_string()).or_insert(0);
            if *count == 0 {
                *n_words_unique += 1;
            }
            *count += 1;

            *n_words_total += 1;
            n_words_total_by_doc += 1;

            for (index_input_strings, inter_word_string) in inter_words_strings.iter().enumerate() {
                if word == inter_word_string {
                    if inter_words_hashmaps[index_input_strings].is_empty() {
                        inter_words_hashmaps[index_input_strings]
                            .entry(0)
                            .or_insert(0);

                        last_positions[index_input_strings] = index_word as u32;
                        continue;
                    }
                    let token_distance =
                        index_word as u32 - 1 - last_positions[index_input_strings];
                    let count_distance = inter_words_hashmaps[index_input_strings]
                        .entry(token_distance)
                        .or_insert(0);
                    *count_distance += 1;

                    last_positions[index_input_strings] = index_word as u32;
                }
            }
        }

        if index_word as i32 >= batches[batch_index_iter] {
            n_words_total_vec.push(*n_words_total);
            n_words_unique_vec.push(*n_words_unique);
            n_words_total_vec_by_doc.push(n_words_total_by_doc);
            n_words_unique_vec_by_doc.push(n_words_unique_by_doc);
            batch_index_iter += 1;
        }
    }

    for position in last_positions.iter_mut() {
        *position = 0;
    }

    for hashmap in inter_words_hashmaps.iter_mut() {
        hashmap.remove(&0);
    }
}
