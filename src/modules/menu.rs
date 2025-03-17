use std::io;

use owo_colors::OwoColorize;
use symspell::{AsciiStringStrategy, SymSpell};

use crate::modules::alt_ocr_menu::alt_ocr_menu;
use crate::modules::cli_handlers::clear_screen;
use crate::modules::lexo_corpus::OcrOptions::{self, RapidOCR};

use crate::modules::{
    debug::debug_menu::debug_menu,
    lexo_corpus::{option_one, option_three, option_two},
};

pub fn main_menu(symspell: &SymSpell<AsciiStringStrategy>) {
    let mut actual_ocr: OcrOptions = RapidOCR;
    let mut user_input = String::new();

    while user_input.trim() != "0" {
        clear_screen();
        println!(
            "
[LEXO CORPUS PR-CLI] Menú principal (Alternativa actualmente en uso: {})
1.- Analizar documento de manera individual.
2.- Analizar lote de documentos etiquetados para un presidente (csv). 
3.- Analizar lote de documentos etiquetados para varios presidentes (csv). 
4.- [DEBUG] Debug Menú fn.
5.- [ALT] Cambiar alternativas OCRs
0.- Salir.
        ",
            actual_ocr.to_string().on_white().bold()
        );
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[DEBUG] Error imprevisto: CLI MENU ERROR menu.rs");

        match user_input.trim() {
            "1" => match option_one(&actual_ocr, symspell) {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            },
            "2" => match option_two(&actual_ocr, symspell) {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            },
            "3" => match option_three(&actual_ocr, symspell) {
                Ok(_) => {
                    println!("\n{}", " Ejecución finalizada ".on_green().bold())
                }
                Err(e) => {
                    eprintln!("\n{} -> {}", " Error en la ejecución ".on_red().bold(), e);
                }
            },
            "4" => debug_menu(symspell),
            "5" => actual_ocr = alt_ocr_menu(actual_ocr),
            "0" => break,
            _ => println!(
                "{}",
                "Opción no válida. Intente nuevamente.".on_red().bold()
            ),
        }
    }

    println!("Ha salido de LEXO CORPUS PR-CLI.")
}
