use pyo3::{
    ffi::c_str,
    types::{PyAnyMethods, PyModule},
    Python,
};

use super::{
    exception_handlers::AnalysisError,
    file_handlers::{clean_folder, page_snapshots_all_pdf_pages},
};

pub fn read_document_pdf_paddle(file_name: &str) -> Result<String, AnalysisError> {
    clean_folder("books-snaps");
    page_snapshots_all_pdf_pages(file_name).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al rasterizar las paginas del pdf. Error: {}",
            e
        ))
    })?;
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/paddle_ocr_handler.py"
    )));

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            code,
            c_str!("paddle_ocr_handler"),
            c_str!("paddle_ocr_handler"),
        )
        .map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al cargar el módulo Python. {}", e))
        })?;

        let function = module.getattr("get_text").map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al obtener la función get_text. {}", e))
        })?;

        let result = function.call1(()).unwrap();
        let content = result.extract().unwrap();
        clean_folder("books-snaps");
        Ok(content)
    })
}

pub fn get_text_on_image(file_page: u16) -> Result<String, AnalysisError> {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/paddle_ocr_handler.py"
    )));

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            code,
            c_str!("paddle_ocr_handler"),
            c_str!("paddle_ocr_handler"),
        )
        .map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al cargar el módulo Python. {}", e))
        })?;

        let function = module.getattr("get_text_on_image").map_err(|e| {
            AnalysisError::ProcessingError(format!(
                "Error al obtener la función get_text_on_image. {}",
                e
            ))
        })?;

        let result = function.call1((file_page,)).unwrap();
        let content = result.extract().unwrap();

        Ok(content)
    })
}
