use std::{
    collections::HashMap,
    io::{self, Error},
};

// Funcion para verificar que es una letra que pertenece a nuestro rango ASCII de interes (32-126)
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

pub fn create_inter_words() -> Result<(Vec<HashMap<u64, u32>>, Vec<u64>, Vec<String>), Error> {
    let inter_words_strings: Vec<String> = input_inter_words()?;
    let mut last_positions: Vec<u64> = Vec::new();
    let mut inter_words_hashmaps: Vec<HashMap<u64, u32>> = Vec::new();
    let mut count_strings = 0;
    while count_strings < inter_words_strings.len() {
        let inter_words: HashMap<u64, u32> = HashMap::new();
        last_positions.push(0);
        inter_words_hashmaps.push(inter_words);
        count_strings += 1
    }

    Ok((inter_words_hashmaps, last_positions, inter_words_strings))
}

pub fn analyzer_content(
    content: String,
    words: &mut HashMap<String, u32>,
    ascii_interest: &Vec<u8>,
    inter_words_hashmaps: &mut Vec<HashMap<u64, u32>>,
    last_positions: &mut Vec<u64>,
    inter_words_strings: &Vec<String>,
) {
    for (index, word) in content.split_whitespace().enumerate() {
        if is_ascii_valid(word, ascii_interest).unwrap() {
            let count = words.entry(word.to_string()).or_insert(0);
            *count += 1;

            for (index_input_strings, inter_word_string) in inter_words_strings.iter().enumerate() {
                if word == inter_word_string {
                    if inter_words_hashmaps[index_input_strings].is_empty() {
                        inter_words_hashmaps[index_input_strings]
                            .entry(0)
                            .or_insert(0);

                        last_positions[index_input_strings] = index as u64;
                        continue;
                    }
                    let token_distance = index as u64 - 1 - last_positions[index_input_strings];
                    let count_distance = inter_words_hashmaps[index_input_strings]
                        .entry(token_distance)
                        .or_insert(0);
                    *count_distance += 1;

                    last_positions[index_input_strings] = index as u64;
                }
            }
        }
    }

    for hashmap in inter_words_hashmaps.iter_mut() {
        hashmap.remove(&0);
    }
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
