use std::io;

use owo_colors::OwoColorize;

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
2.- Analizar lote de documentos etiquetados (csv). 
3.- Limpiar carpeta books-fracts.
0.- Salir.
        "
        );
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[DEBUG] Error imprevisto: CLI MENU ERROR menu.rs");

        if user_input.trim() == "1" {
            match option_one() {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            }
        }

        if user_input.trim() == "2" {
            match option_two() {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            }
        }

        if user_input.trim() == "4" {
            //option_three();
        }

        if user_input.trim() == "3" {
            clean_folder("books-fracts");
        }
    }

    println!("Ha salido de LEXO CORPUS PR-CLI.")
}
