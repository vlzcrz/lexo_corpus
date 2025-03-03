use std::{fs, io};

use owo_colors::OwoColorize;

use crate::modules::{
    debug::debug_menu::debug_menu,
    lexo_corpus::{option_one, option_two},
};

pub fn main_menu() {
    let mut user_input = String::new();
    let folder_fracts_exists = fs::exists("./books-fracts").unwrap();
    if !folder_fracts_exists {
        fs::create_dir("./books-fracts").unwrap();
    }
    let folder_data_exists = fs::exists("./books-data").unwrap();
    if !folder_data_exists {
        fs::create_dir("./books-data").unwrap();
    }
    let folder_plot_exists = fs::exists("./books-plot").unwrap();
    if !folder_plot_exists {
        fs::create_dir("./books-plot").unwrap();
    }
    let folder_log_exists = fs::exists("./logs").unwrap();
    if !folder_log_exists {
        fs::create_dir("./logs").unwrap();
    }

    while user_input.trim() != "0" {
        println!(
            "
[LEXO CORPUS PR-CLI] Menú principal
1.- Analizar documento de manera individual.
2.- Analizar lote de documentos etiquetados (csv). 
3.- [DEBUG] Debug Menú fn.
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

        if user_input.trim() == "3" {
            debug_menu();
        }
    }

    println!("Ha salido de LEXO CORPUS PR-CLI.")
}
