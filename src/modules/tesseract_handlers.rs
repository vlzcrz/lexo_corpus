use std::collections::HashMap;

use crossterm::style::Stylize;
use rusty_tesseract::{Args, Image};

use super::{
    exception_handlers::AnalysisError,
    file_handlers::{clean_folder, get_files_from_folder, page_snapshots_all_pdf_pages},
};

pub fn read_document_pdf_tesseract() -> Result<String, AnalysisError> {
    let my_args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from([(
            "tessedict_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzáéíóúABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
        dpi: Some(350),
        psm: Some(6),
        oem: Some(3),
    };

    println!("##{}##", " Tesseract details info ".on_white().bold());
    //tesseract version
    let tesseract_version = rusty_tesseract::get_tesseract_version().unwrap();
    println!("The tesseract version is: {:?}", tesseract_version);

    //available languages
    let tesseract_langs = rusty_tesseract::get_tesseract_langs().unwrap();
    println!("The available languages are: {:?}", tesseract_langs);

    //available config parameters
    let parameters = rusty_tesseract::get_tesseract_config_parameters().unwrap();
    println!(
        "Example config parameter: {}",
        parameters.config_parameters.first().unwrap()
    );

    let mut combined_content = String::new();
    let images = get_files_from_folder("books-snaps").unwrap();
    for (image_name, image_extension) in images.iter() {
        let image_path = format!("./books-snaps/{}.{}", image_name, image_extension);
        let image = Image::from_path(image_path).unwrap();
        let content = rusty_tesseract::image_to_string(&image, &my_args).unwrap();
        combined_content.push_str(&content);
        combined_content.push(' ');
    }

    Ok(combined_content)
}

pub fn read_pdf_tesseract(file_name: &str) -> Result<String, AnalysisError> {
    clean_folder("books-snaps");
    page_snapshots_all_pdf_pages(file_name).unwrap();

    let content = read_document_pdf_tesseract()?;

    clean_folder("books-snaps");
    Ok(content)
}
