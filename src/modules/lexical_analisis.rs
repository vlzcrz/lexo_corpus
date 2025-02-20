use std::{
    collections::HashMap,
    io::{self, Error},
};

use crate::modules::cli_handlers::clear_screen;

// Funcion para verificar que es una letra que pertenece a nuestro rango ASCII de interes
pub fn is_ascii_valid(word: &str, ascii_interest: &Vec<u8>) -> Result<bool, Error> {
    let bytes_word = word.as_bytes();
    for byte in bytes_word {
        if !ascii_interest.contains(byte) {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn input_inter_words() -> Result<Vec<String>, Error> {
    let mut inter_words: Vec<String> = Vec::new();
    let mut exit_while = false;
    while !exit_while {
        let mut inter_word = String::new();
        clear_screen();
        println!("Ingrese el inter word de interes para analizar en su archivo pdf o txt, ó ingrese '0' para ya no continuar, actualmente tiene seleccionado: {:?}", inter_words);
        io::stdin().read_line(&mut inter_word)?;
        let inter_word = inter_word.trim().to_string().to_lowercase();
        if inter_word == "0" {
            exit_while = true;
        } else {
            inter_words.push(inter_word);
        }
    }
    Ok(inter_words)
}

pub fn create_inter_words() -> Result<(Vec<HashMap<u32, u32>>, Vec<u32>, Vec<String>), Error> {
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

pub fn create_inter_words_differ(
    inter_words_strings: &Vec<String>,
) -> Result<(Vec<HashMap<u32, u32>>, Vec<u32>), Error> {
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

pub fn analyzer_content(
    content: String,
    words: &mut HashMap<String, u32>,
    ascii_interest: &Vec<u8>,
    inter_words_hashmaps: &mut Vec<HashMap<u32, u32>>,
    last_positions: &mut Vec<u32>,
    inter_words_strings: &Vec<String>,
) -> Result<(Vec<u32>, Vec<u32>), Error> {
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
        if is_ascii_valid(word, ascii_interest).unwrap() {
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

pub fn initializer_word_hashmap_handler(
    words: &HashMap<String, u32>,
) -> Result<(Vec<String>, Vec<u32>), Error> {
    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<u32> = Vec::new();
    for (key, value) in words {
        keys.push(key.to_string());
        values.push(*value);
    }

    Ok((keys, values))
}
