use std::{collections::HashMap, io::Error};

use pyo3::{
    ffi::c_str,
    types::{PyAnyMethods, PyModule},
    Python,
};

use super::exception_handlers::AnalysisError;

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

pub fn president_hashmap_to_vector(
    hashmap: HashMap<i32, String>,
) -> Result<(Vec<i32>, Vec<String>), Error> {
    let mut tmp_tuple: Vec<(i32, String)> = hashmap
        .into_iter()
        .map(|(year, president_name)| (year, president_name))
        .collect();

    tmp_tuple.sort_by(|a, b| a.0.cmp(&b.0));
    let (x_values, y_values) = tmp_tuple.into_iter().unzip();
    Ok((x_values, y_values))
}

pub fn get_president_ordered_by_year(
    csv_content: &Vec<(String, i32, String)>,
) -> Result<Vec<String>, AnalysisError> {
    let mut president_values: Vec<String> = Vec::new();
    let mut president_counts_by_year: HashMap<i32, HashMap<String, i32>> = HashMap::new();
    for (_, year, president) in csv_content.iter() {
        let hashmap_president_count = president_counts_by_year
            .entry(*year)
            .or_insert_with(HashMap::new);
        *hashmap_president_count
            .entry(president.clone())
            .or_insert(0) += 1;
    }

    // Recolectar años y ordenarlos de mayor a menor
    let mut years: Vec<i32> = president_counts_by_year.keys().cloned().collect();
    years.sort_by(|a, b| b.cmp(a)); // Ordenar de mayor a menor

    // Para cada año, encontrar el presidente que más se repite
    for year in years {
        if let Some(president_counts) = president_counts_by_year.get(&year) {
            // Si no hay presidentes para este año, continuamos al siguiente
            if president_counts.is_empty() {
                continue;
            }

            // Encontrar el presidente con mayor conteo
            let mut max_count = 0;
            let mut most_frequent_president = String::new();

            for (president, count) in president_counts {
                if *count > max_count {
                    max_count = *count;
                    most_frequent_president = president.clone();
                }
            }

            // Agregar el presidente más frecuente al vector de resultados
            president_values.push(most_frequent_president);
        }
    }

    Ok(president_values)
}

pub fn plot_heaps_law(
    x_values: &Vec<u32>,
    y_values: &Vec<u32>,
    folder_name: &str,
    file_name: &str,
) {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_heaps_law").unwrap();

        let file_name_formatted = format!("heaps-law ({})", file_name);
        let args = (x_values, y_values, folder_name, file_name_formatted);
        let _ = function.call1(args);
    });
}

pub fn plot_zipf_law(
    x_values: &Vec<f64>,
    y_values: &Vec<f64>,
    lr_parameters: &Vec<f64>,
    folder_name: &str,
    file_name: &str,
) {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_log10_zipf_law").unwrap();

        let file_name_formatted = format!("zipf-law ({})", file_name);
        let args = (
            x_values,
            y_values,
            lr_parameters[1],
            lr_parameters[0],
            folder_name,
            file_name_formatted,
        );
        let _ = function.call1(args);
    });
}

pub fn lineplot_alpha_year(
    title: &str,
    x_label: &str,
    y_label: &str,
    x_values: &Vec<i32>,
    y_values: &Vec<f64>,
    folder_name: &str,
    file_name: &str,
) {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_csv_dataset").unwrap();
        let file_name_formatted = format!("year-alpha {}", file_name);
        let title_formatted = format!("{} ({} dataset)", title, file_name);
        let args = (
            title_formatted,
            x_label,
            y_label,
            x_values,
            y_values,
            folder_name,
            file_name_formatted,
        );
        let _ = function.call1(args);
    });
}

pub fn lineplot_alpha_year_president(
    title: &str,
    x_label: &str,
    y_label: &str,
    president_legend: &Vec<String>,
    x_values: &Vec<i32>,
    y_values: &Vec<f64>,
    folder_name: &str,
    file_name: &str,
) {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("lineplot_csv_dataset_multiple").unwrap();
        let file_name_formatted = format!("year-alpha-president {}", file_name);
        let title_formatted = format!("{} ({} dataset-multiple)", title, file_name);
        let args = (
            title_formatted,
            x_label,
            y_label,
            president_legend,
            x_values,
            y_values,
            folder_name,
            file_name_formatted,
        );
        let _ = function.call1(args);
    });
}

pub fn plot_heat_map(
    title: &str,
    x_label: &str,
    y_label: &str,
    x_values: &Vec<Vec<u32>>,
    y_values: &Vec<Vec<u32>>,
    word_values: &Vec<String>,
    folder_name: &str,
    file_name: &str,
    file_extension: &str,
) {
    let code = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/utils/plot_handler.py"
    )));

    Python::with_gil(|py| {
        let module =
            PyModule::from_code(py, code, c_str!("plot_handler"), c_str!("plot_handler")).unwrap();
        let function = module.getattr("heat_map").unwrap();

        let file_name_formatted = format!("interwords-heatmap: {}", file_name);
        let title_formatted = format!("{} - file: {}.{}", title, file_name, file_extension);
        let args = (
            title_formatted,
            x_label,
            y_label,
            word_values,
            x_values,
            y_values,
            folder_name,
            file_name_formatted,
        );
        let _ = function.call1(args);
    });
}
