use std::{fs, io};

use crate::modules::debug::debug_text_extracts::{
    read_document_pdf_debug, read_document_txt_debug, read_pdf_tesseract_debug, read_pdf_tet,
};

pub fn debug_menu() {
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
[LEXO CORPUS DEBUG] Menú principal
1.- Mostrar extracción de texto archivos txt (read_file [OS bytes])
2.- Mostrar extracción de texto archivos pdf (pdf extract [Rust Crate])
3.- Mostrar extracción de texto archivos pdf (TET lib [Python Lib])
4.- Mostrar extracción de texto archivos pdf (Tesseract-ocr (v5) [Program executable])
0.- Salir.
        "
        );
        user_input.clear();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == "1" {
            let _ = read_document_txt_debug().unwrap();
            println!(
                "[DEBUG FUNCTION read_document_txt: \nExtract: {}",
                "Completed!"
            );
        }

        if user_input.trim() == "2" {
            let _ = read_document_pdf_debug().unwrap();
            println!(
                "[DEBUG FUNCTION read_document_pdf: \nExtract: {}",
                "Completed!"
            );
        }

        if user_input.trim() == "3" {
            let _ = read_pdf_tet().unwrap();
            println!(
                "[DEBUG FUNCTION read_tet_document_pdf: \nExtract: {}",
                "Completed!"
            );
        }

        if user_input.trim() == "4" {
            let _ = read_pdf_tesseract_debug().unwrap();
            println!(
                "[DEBUG FUNCTION read_pdf_tesseract: \nExtract: {}",
                "Completed!"
            );
        }
    }

    println!("Ha salido de LEXO CORPUS DEBUG.")
}
