use std::io;

use crate::modules::{file_handlers::clean_folder, lexo_corpus::option_one};

pub fn main_menu() {
    let mut user_input = String::new();

    while user_input.trim() != "0" {
        println!(
            "
            
            [LEXO CORPUS PR-CLI] Menu principal
            1.- Analizar documento de manera individual
            2.- Analizar lote de documentos de manera automatizada 
            3.- Limpiar carpeta
            0.- Salir
        "
        );
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[DEBUG] Error imprevisto: CLI MENU ERROR menu.rs");

        println!("{}", user_input);

        if user_input.trim() == "1" {
            option_one();
        }

        if user_input.trim() == "2" {
            println!("Función no implementada aún")
        }

        if user_input.trim() == "3" {
            clean_folder("books-fracts");
        }
    }

    println!("Ha salido de LEXO CORPUS PR-CLI.")
}
