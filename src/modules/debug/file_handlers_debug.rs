use std::{
    fs::File,
    io::{BufWriter, Write},
};

use owo_colors::OwoColorize;

use crate::modules::exception_handlers::AnalysisError;

pub fn create_and_write(content: &str, output_path: &str) -> Result<(), AnalysisError> {
    let file = File::create(output_path).map_err(|_| {
        AnalysisError::FileSystemOperationError(format!(
            "{} Error al crear el archivo txt",
            "Error"
        ))
    })?;

    let mut writer = BufWriter::new(file);

    let _ = writer.write_all(content.as_bytes());
    let _ = writer.flush();
    println!(
        "{} Se ha creado el txt con el contenido anidado.",
        "Completado".on_green()
    );

    Ok(())
}
