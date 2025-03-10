use std::io;

use owo_colors::OwoColorize;

use crate::modules::{
    debug::debug_menu::debug_menu,
    file_handlers::initialize_main_folders,
    lexo_corpus::{option_one, option_three, option_two},
};

use super::exception_handlers::AnalysisError;

pub fn tesseract_menu() -> Result<i32, AnalysisError> {
    let mut user_input = String::new();
    initialize_main_folders();

    while user_input.trim() != "5" {
        println!(
            "
[LEXO CORPUS PR-CLI] Menú principal (Alternativa actualmente en uso: Tesseract OCR v5)
1.- Analizar documento de manera individual.
2.- Analizar lote de documentos etiquetados para un presidente (csv). 
3.- Analizar lote de documentos etiquetados para varios presidentes (csv). 
4.- [DEBUG] Debug Menú fn.
5.- [ALT] Cambiar alternativa por TET lib
0.- Salir.
        "
        );
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[DEBUG] Error imprevisto: CLI MENU ERROR menu.rs");

        if user_input.trim() == "1" {
            match option_one(true) {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            }
        }

        if user_input.trim() == "2" {
            match option_two(true) {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            }
        }

        if user_input.trim() == "3" {
            match option_three(true) {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            }
        }

        if user_input.trim() == "4" {
            debug_menu();
        }

        if user_input.trim() == "0" {
            return Ok(0);
        }
    }

    Ok(9)
}
