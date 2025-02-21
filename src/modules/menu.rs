use std::io;

use crate::modules::{
    file_handlers::clean_folder,
    lexo_corpus::{option_one, option_three, option_two},
};

pub fn main_menu() {
    let mut user_input = String::new();

    while user_input.trim() != "0" {
        println!(
            "
[LEXO CORPUS PR-CLI] Menu principal
1.- Analizar documento de manera individual.
2.- Analizar lote de documentos de manera individual. 
3.- Analizar lote de documentos de manera conjunta. 
4.- Limpiar carpeta.
0.- Salir.
        "
        );
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[DEBUG] Error imprevisto: CLI MENU ERROR menu.rs");

        if user_input.trim() == "1" {
            option_one();
        }

        if user_input.trim() == "2" {
            option_two();
        }

        if user_input.trim() == "3" {
            option_three();
        }

        if user_input.trim() == "4" {
            clean_folder("books-fracts");
        }
    }

    println!("Ha salido de LEXO CORPUS PR-CLI.")
}
