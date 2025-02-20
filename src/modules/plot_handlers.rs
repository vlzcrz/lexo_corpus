use std::{collections::HashMap, io::Error};

use pyo3::{
    ffi::c_str,
    types::{PyAnyMethods, PyModule},
    Python,
};

pub fn means_hashmap_to_vectors(
    hashmap: HashMap<i32, Vec<f64>>,
) -> Result<(Vec<i32>, Vec<f64>), Error> {
    let mut tmp_tuple: Vec<(i32, f64)> = hashmap
        .into_iter()
        .map(|(year, alphas)| {
            let mean = alphas.iter().sum::<f64>() / alphas.len() as f64;
            (year, mean)
        })
        .collect();

    tmp_tuple.sort_by(|a, b| a.0.cmp(&b.0));
    let (x_values, y_values) = tmp_tuple.into_iter().unzip();
    Ok((x_values, y_values))
}

pub fn plot_heaps_law(
    x_values: &Vec<u32>,
    y_values: &Vec<u32>,
    folder_name: &str,
    file_name: &str,
) -> Result<bool, Error> {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    let call_result = Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_heaps_law").unwrap();

        // Nombre del archivo PDF de entrada
        //let input_pdf = "books-pdf/tallerads.pdf";

        // Llamar a la función split_pdf en Python

        let file_name_formatted = format!("heaps-law ({})", file_name);
        let args = (x_values, y_values, folder_name, file_name_formatted);
        let result = function.call1(args);

        match result {
            Ok(_) => {
                println!("Plot exitoso");
                return true;
            }
            Err(err) => {
                println!("Error al plotear: {:?}", err);
                return false;
            }
        }
    });

    Ok(call_result)
}

pub fn plot_zipf_law(
    x_values: &Vec<f64>,
    y_values: &Vec<f64>,
    lr_parameters: &Vec<f64>,
    folder_name: &str,
    file_name: &str,
) -> Result<bool, Error> {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    let call_result = Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_log10_zipf_law").unwrap();

        // Nombre del archivo PDF de entrada
        //let input_pdf = "books-pdf/tallerads.pdf";

        let file_name_formatted = format!("zipf-law ({})", file_name);
        // Llamar a la función split_pdf en Python
        let args = (
            x_values,
            y_values,
            lr_parameters[1],
            lr_parameters[0],
            folder_name,
            file_name_formatted,
        );
        let result = function.call1(args);

        match result {
            Ok(_) => {
                println!("Plot exitoso");
                return true;
            }
            Err(err) => {
                println!("Error al plotear: {:?}", err);
                return false;
            }
        }
    });

    Ok(call_result)
}

pub fn lineplot_alpha_year(
    title: &str,
    x_label: &str,
    y_label: &str,
    x_values: &Vec<i32>,
    y_values: &Vec<f64>,
    folder_name: &str,
    file_name: &str,
) -> Result<bool, Error> {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    let call_result = Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_csv_dataset").unwrap();

        // Nombre del archivo PDF de entrada
        //let input_pdf = "books-pdf/tallerads.pdf";

        // Llamar a la función split_pdf en Python

        let file_name_formatted = format!("{}", file_name);
        let args = (
            title,
            x_label,
            y_label,
            x_values,
            y_values,
            folder_name,
            file_name_formatted,
        );
        let result = function.call1(args);

        match result {
            Ok(_) => {
                println!("Plot exitoso");
                return true;
            }
            Err(err) => {
                println!("Error al plotear: {:?}", err);
                return false;
            }
        }
    });

    Ok(call_result)
}
