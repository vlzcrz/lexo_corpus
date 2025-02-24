use std::{fmt::Display, io::Error};

#[derive(Debug)]
pub enum AnalysisError {
    IoError(Error),
    ParseError(String),
    ProcessingError(String),
    LectureCsvDatasetError(String),
    FileSystemOperationError(String),
    EmptyResultError,
}

impl Display for AnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalysisError::IoError(e) => write!(f, "Error de input del usuario: {}", e),
            AnalysisError::ParseError(msg) => write!(f, "Error de análisis: {}", msg),
            AnalysisError::ProcessingError(msg) => write!(f, "Error de procesamiento: {}", msg),
            AnalysisError::LectureCsvDatasetError(msg) => {
                write!(f, "Error de lectura csv: {}", msg)
            }
            AnalysisError::FileSystemOperationError(msg) => {
                write!(f, "Error con el sistema de archivos: {}", msg)
            }
            AnalysisError::EmptyResultError => {
                write!(f, "No se encontraron resultados para procesar")
            }
        }
    }
}

impl std::error::Error for AnalysisError {}
