use std::{fs::File, io::Write};

use jiff::{Unit, Zoned};

use super::exception_handlers::AnalysisError;

/// Crea un archivo txt para registrar logs ubicado en la carpeta /logs.
///
/// ## Returns
/// ```
/// - Ok(File)
/// ```
/// Retorna el archivo creado.
/// ## Errors
/// ```
/// - jiff::Error
/// ```
/// Si hay un problema con obtener el datetime actual en el sistema.
pub fn create_log_instance() -> Result<File, jiff::Error> {
    let now = Zoned::now().datetime().round(Unit::Second)?;
    let mut file = File::create(format!("./logs/{}.txt", now)).unwrap();
    write_log_result(format!("Initialized log process at: {}", now), &mut file).unwrap();
    Ok(file)
}

/// Registra un evento sobre algun proceso en un archivo txt dado.
///
/// ## Params
/// ```
/// - msg: String
/// - file: &mut File
/// ```
/// msg: Mensaje descriptivo del evento que se quiere registrar
/// file: Archivo txt referencial para escribir algun evento
/// ## Returns
/// ```
/// - Ok()
/// ```
/// Vacio
/// ## Errors
/// ```
/// - AnalysisError::FileSystemOperationError
/// ```
/// Si hay un problema con manipular el archivo txt para registrar el log
pub fn write_log_result(msg: String, file: &mut File) -> Result<(), AnalysisError> {
    file.write(msg.as_bytes()).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("[Error] Error al registrar Log. {}", e))
    })?;
    Ok(())
}
