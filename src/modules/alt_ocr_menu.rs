use owo_colors::OwoColorize;

use crate::modules::lexo_corpus::OcrOptions::{RapidOCR, TETlib, Tesseract};
use std::io;

use super::lexo_corpus::OcrOptions;

pub fn alt_ocr_menu(actual_ocr: OcrOptions) -> OcrOptions {
    let mut user_input = String::new();

    while user_input.trim() != "0" {
        println!(
            "
[LEXO CORPUS PR-CLI] Menú OCR (Alternativa actualmente en uso: {})
1.- Cambiar alternativa por TET Lib
2.- Cambiar alternativa por Tesseract OCR (v5) 
3.- Cambiar alternativa por Rapid OCR (v2.0.2)
0.- Salir.
        ",
            actual_ocr.to_string().on_white().bold()
        );
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("[DEBUG] Error imprevisto: CLI MENU ERROR menu.rs");

        match user_input.trim() {
            "1" => return TETlib,
            "2" => return Tesseract,
            "3" => return RapidOCR,
            "0" => return actual_ocr,
            _ => println!(
                "{}",
                "Opción no válida. Intente nuevamente.".on_red().bold()
            ),
        }
    }

    actual_ocr
}
