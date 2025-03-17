use std::{fs, io};

use symspell::{AsciiStringStrategy, SymSpell};

use crate::modules::debug::debug_text_extracts::{
    read_document_pdf_debug, read_document_txt_debug, read_pdf_rapid_ocr_debug,
    read_pdf_tesseract_debug, read_pdf_tet,
};

pub fn debug_menu(symspell: &SymSpell<AsciiStringStrategy>) {
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
2.- Mostrar extracción de texto archivo pdf (pdf extract [Rust Crate])
3.- Mostrar extracción de texto archivo pdf (TET lib [Python Lib])
4.- Mostrar extracción de texto archivo pdf (Tesseract-ocr (v5) [Executable])
5.- Mostrar extracción de texto archivo pdf (Rapid OCR (v2.0.2))
0.- Salir.
        "
        );
        user_input.clear();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == "1" {
            let _ = read_document_txt_debug().unwrap();
            println!(
                "[DEBUG FUNCTION read_document_txt: \nEstado: {}",
                "Completado!"
            );
        }

        if user_input.trim() == "2" {
            let _ = read_document_pdf_debug().unwrap();
            println!(
                "[DEBUG FUNCTION read_document_pdf: \nEstado: {}",
                "Completado!"
            );
        }

        if user_input.trim() == "3" {
            let _ = read_pdf_tet().unwrap();
            println!(
                "[DEBUG FUNCTION read_tet_document_pdf: \nEstado: {}",
                "Completado!"
            );
        }

        if user_input.trim() == "4" {
            let _ = read_pdf_tesseract_debug().unwrap();
            println!(
                "[DEBUG FUNCTION read_pdf_tesseract: \nEstado: {}",
                "Completado!"
            );
        }

        if user_input.trim() == "5" {
            let _ = read_pdf_rapid_ocr_debug(symspell).unwrap();
            println!(
                "[DEBUG FUNCTION read_pdf_RapidOCR: \nExtract: {}",
                "Completed!"
            );
        }
    }

    println!("Ha salido de LEXO CORPUS DEBUG.")
}
