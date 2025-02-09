use std::io::{self, Error};

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
