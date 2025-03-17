use pyo3::{
    ffi::c_str,
    types::{PyAnyMethods, PyModule},
    Python,
};

use super::{
    exception_handlers::AnalysisError,
    file_handlers::{clean_folder, page_snapshots_all_pdf_pages},
};

pub fn procesar_imagen_ocr_debug(file_page: u16) -> Result<String, AnalysisError> {
    let image_path = format!("./books-snaps/snapshot350dpi_page_{}.png", file_page);
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/rapid_ocr_debug.py"
    )));

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            code,
            c_str!("rapid_ocr_debug"),
            c_str!("rapid_ocr_debug"),
        )
        .map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al cargar el módulo Python. {}", e))
        })?;

        let function = module.getattr("procesar_imagen_ocr_debug").map_err(|e| {
            AnalysisError::ProcessingError(format!(
                "Error al obtener la función procesar_imagen_ocr_debug. {}",
                e
            ))
        })?;

        let result = function.call1((image_path,)).unwrap();
        let content = result.extract().unwrap();
        clean_folder("books-snaps");
        Ok(content)
    })
}

pub fn read_document_pdf_rapid_ocr(file_name: &str) -> Result<String, AnalysisError> {
    clean_folder("books-snaps");
    page_snapshots_all_pdf_pages(file_name).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al rasterizar las paginas del pdf. Error: {}",
            e
        ))
    })?;
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/rapid_ocr_handler.py"
    )));

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            code,
            c_str!("rapid_ocr_handler"),
            c_str!("rapid_ocr_handler"),
        )
        .map_err(|e| {
            AnalysisError::ProcessingError(format!("Error al cargar el módulo Python. {}", e))
        })?;

        let function = module.getattr("procesar_imagen_ocr").map_err(|e| {
            AnalysisError::ProcessingError(format!(
                "Error al obtener la función procesar_imagen_ocr. {}",
                e
            ))
        })?;

        let result = function.call1(()).unwrap();
        let content = result.extract().unwrap();
        clean_folder("books-snaps");
        Ok(content)
    })
}
