use std::{
    cmp::min,
    collections::HashMap,
    fmt::Write,
    fs, io, thread,
    time::{Duration, Instant},
};

use cli_table::{format::Justify, Cell, CellStruct, Style, Table};
use crossterm::style::Stylize;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::modules::{
    cli_handlers::clear_screen,
    exception_handlers::AnalysisError,
    file_handlers::{document_extract_content, extract_csv_labeled_data, get_files_from_folder},
    lexical_analisis::{
        analyzer_content_dataset_opt, copy_interword_to_main, copy_words_to_main,
        create_inter_words, create_inter_words_differ, input_inter_words,
    },
    log_handlers::{create_log_instance, write_log_result},
    plot_handlers::{
        lineplot_alpha_year, means_hashmap_to_vectors, plot_heaps_law, plot_heat_map, plot_zipf_law,
    },
};

use super::{
    file_handlers::{create_csv_inter_words, create_csv_ordered},
    lexical_analisis::{analyzer_content, initializer_word_hashmap_handler},
    linear_regression::linear_regression_x1,
    zipfs_handlers::{apply_to_log10, get_zipf_law_results},
};

pub fn option_one() -> Result<(), AnalysisError> {
    let mut file_log = create_log_instance().map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al crear el log del proceso 'Option One': {}",
            e
        ))
    })?;
    // Variable de salida para cuando se ingresa un pdf correcto
    let mut did_read = false;
    // Listado de palabras permitidas para filtrar dentro del corpus
    let mut ascii_interest: Vec<u8> = (97..121).collect();
    ascii_interest.push(39);
    let mut ascii_interest_numbers: Vec<u8> = (48..57).collect();
    ascii_interest.append(&mut ascii_interest_numbers);
    // Nombre del archivo ingresado por el usuario
    let mut file_path_input = String::new();
    // HashMap para guardar las palabras encontradas dentro del texto junto con su cantidad de repeticiones
    let mut words: HashMap<String, u32> = HashMap::new();
    // HashMap para guardar las palabras inter-word de interes el documento
    let (mut inter_words_hashmaps, mut last_positions, inter_words_strings) = create_inter_words()
        .map_err(|e| {
            write_log_result(
                format!("Error al crear los interwords de interes. Error: {}", e),
                &mut file_log,
            )
            .unwrap();
            AnalysisError::ProcessingError(format!(
                "[Error] Error al crear los interwords de interes: {}",
                e
            ))
        })?;

    // LOG
    write_log_result(
        format!(
            "\n[Completado] Interwords creados sin problemas, interwords: {:?}",
            inter_words_strings
        ),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;

    let default_folder_data = "books-data";
    let default_folder_plot = "books-plot";
    let mut content = String::new();
    let mut file_name = String::new();
    let mut file_extension = String::new();

    while !did_read {
        //clear_screen();
        println!("Ingresa el nombre del archivo con su extension .txt ó .pdf (Presione '0' para cancelar)");
        io::stdin()
            .read_line(&mut file_path_input)
            .map_err(|e| AnalysisError::IoError(e))?;

        if file_path_input.trim() == "0" {
            return Ok(());
        }

        let (name_f, extension_f) = file_path_input.split_once(".").ok_or_else(|| {
            // LOG
            let _ = write_log_result(
                format!(
                    "\n[Error] Error al leer el nombre del archivo y su extension. Valor ingresado: {}",
                    file_path_input
                ),
                &mut file_log,
            )
            .map_err(|e| {
                AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
            });
            AnalysisError::ParseError("Nombre de archivo ó extensión no identificable".to_string())
        })?;

        file_name = name_f.trim().to_string();
        file_extension = extension_f.trim().to_string();

        match document_extract_content(&file_name, &file_extension) {
            Ok(extracted_content) => {
                did_read = true;
                content = extracted_content
                    .to_lowercase()
                    .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], "")
            }
            Err(e) => {
                eprintln!(
                    "Error al leer el archivo: {}. Por favor, intente nuevamente.",
                    e
                );
                write_log_result(
                    format!(
                        "\n[Error] Error al extraer el archivo: {}.{} , error: {}.",
                        file_name, file_extension, e,
                    ),
                    &mut file_log,
                )
                .map_err(|e| {
                    AnalysisError::FileSystemOperationError(format!(
                        "Error al escribir logs': {}",
                        e
                    ))
                })?;
                continue;
            }
        };
    }

    // LOG
    write_log_result(
        format!(
            "\n[Completado] Contenido leido sin problemas: {}.{}",
            file_name, file_extension
        ),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;

    let started = Instant::now();
    //clear_screen();

    println!(
        "# Inicio del proceso de extracción y analisis del documento: {}.{} ...",
        file_name, file_extension
    );

    let (n_words_total_vec, n_words_unique_vec) = analyzer_content(
        content,
        &mut words,
        &ascii_interest,
        &mut inter_words_hashmaps,
        &mut last_positions,
        &inter_words_strings,
    )
    .map_err(|e| {
        // LOG
        let _ = write_log_result(
            format!(
                "\n[Error] Error al analizar el contenido: {}.{}, error: {:?}",
                file_name, file_extension, e
            ),
            &mut file_log,
        )
        .map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
        });
        AnalysisError::ProcessingError(format!("Error de analisis del contenido {}", e))
    })?;

    write_log_result(
        format!("\n[Completado] Contenido analizado correctamente."),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;

    let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
    if keys.is_empty() && values.is_empty() {
        return Err(AnalysisError::EmptyResultError);
    }
    println!("# Finalizado.");
    println!("# Inicio de procesamiento del contenido...");

    get_zipf_law_results(&mut keys, &mut values);
    create_csv_ordered(&keys, &values, &file_name, &default_folder_data);
    let (vec_distance, vec_frequency) = create_csv_inter_words(
        &file_name,
        &inter_words_hashmaps,
        &inter_words_strings,
        &default_folder_data,
    )
    .map_err(|e| {
        let _ = write_log_result(
            format!(
                "[Error] Error en la elaboración de csv interwords {} ...",
                e,
            ),
            &mut file_log,
        )
        .map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
        });
        AnalysisError::ProcessingError(format!(
            "Error en la generación de los csv inter words {}",
            e
        ))
    })?;

    plot_heat_map(
        "Frequency distribution of inter word's distance",
        "Distance",
        "Inter Word",
        &vec_distance,
        &vec_frequency,
        &inter_words_strings,
        &default_folder_plot,
        &file_name,
        &file_extension,
    );

    plot_heaps_law(
        &n_words_total_vec,
        &n_words_unique_vec,
        &default_folder_plot,
        &file_name,
    );

    write_log_result(
        format!("\n[Completado] Graficos heatmap y heap's law completado."),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;

    let (log_ranking, log_values) = apply_to_log10(values).map_err(|e| {
        AnalysisError::ParseError(format!("Error en el cálculo logarítmico en base 10 {}", e))
    })?;
    let zipfs_parameters = linear_regression_x1(&log_ranking, &log_values).map_err(|e| {
        let _ = write_log_result(
            format!("\n[Error] Error en cálcular la regresión lineal. {}", e),
            &mut file_log,
        )
        .map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
        });
        AnalysisError::ProcessingError(format!("Error en la regresión lineal {}", e))
    })?;

    plot_zipf_law(
        &log_ranking,
        &log_values,
        &zipfs_parameters,
        &default_folder_plot,
        &file_name,
    );

    println!("# Finalizado.");
    write_log_result(
        format!(
            "\n[Completado] Archivo procesado: {}.{} ha finalizado correctamente.",
            file_name, file_extension,
        ),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;
    println!("Ejecutado en {:.3?}", started.elapsed());
    Ok(())
}

pub fn option_two() -> Result<(), AnalysisError> {
    let mut input = String::new();
    let mut valid_input = false;
    let mut file_name_dataset = String::new();
    let mut file_extension_dataset = String::new();

    let mut processed_file_status_table: Vec<Vec<CellStruct>> = Vec::new();
    let mut file_log = create_log_instance().map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al crear el archivo log. {}", e))
    })?;

    while !valid_input {
        clear_screen();
        println!("Seleccione un data label (csv) para iniciar el lote de procesamiento de textos. (Presione '0' para cancelar)");
        let labeled_data_files = get_files_from_folder("labeled-data").map_err(|e| {
            AnalysisError::LectureCsvDatasetError(format!(
                "Error en la lectura del dataset (csv) {}",
                e
            ))
        })?;
        let max = labeled_data_files.len() as u16;
        for (index, (file_name, file_extension)) in labeled_data_files.iter().enumerate() {
            println!("{}.- {}.{}", index + 1, file_name, file_extension);
        }
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| AnalysisError::IoError(e))?;

        if input.trim() == "0" {
            return Ok(());
        }

        let parsed_input: u16 = match input.trim().parse() {
            Ok(parsed_int) => parsed_int,
            Err(error) => {
                println!("Solo se permite ingresar caracteres numericos y no alfanumericos u otro tipo. {:?}", error);
                thread::sleep(Duration::from_millis(12));
                input = String::new();
                continue;
            }
        };

        if parsed_input > max {
            input = String::new();
            continue;
        }
        let (file_name_ds, file_extension_ds) = &labeled_data_files[(parsed_input - 1) as usize];
        file_name_dataset = file_name_ds.to_string();
        file_extension_dataset = file_extension_ds.to_string();
        if !file_name_dataset.is_empty() {
            valid_input = true;
        }
    }

    let csv_content = extract_csv_labeled_data(&file_name_dataset, &file_extension_dataset)
        .map_err(|e| {
            AnalysisError::ProcessingError(format!("Error en el contenido del dataset (csv) {}", e))
        })?;

    let mut year_alphas_hashmaps: HashMap<i32, Vec<f64>> = HashMap::new();
    let inter_words_strings = input_inter_words().map_err(|e| AnalysisError::IoError(e))?;
    // LOG
    write_log_result(
        format!(
            "\n[Completado] Inter words instanciados correctamente. {:?}",
            inter_words_strings
        ),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;
    // ENDLOG
    let mut ascii_interest: Vec<u8> = (97..121).collect();
    ascii_interest.push(39);
    let mut ascii_interest_numbers: Vec<u8> = (48..57).collect();
    ascii_interest.append(&mut ascii_interest_numbers);

    let mut loading_value = 0;
    let total_load_size = csv_content.len() as u64;

    let started = Instant::now();
    let folder_warehouse = format!("./{}", file_name_dataset);
    let folder_warehouse_data = format!("./{}/data", &file_name_dataset);
    let folder_warehouse_plot = format!("./{}/plot", &file_name_dataset);
    let folder_warehouse_zipf_plot = format!("{}/zipfs", &folder_warehouse_plot);
    let folder_warehouse_heaps_plot = format!("{}/heaps", &folder_warehouse_plot);
    let folder_warehouse_heatmap_plot = format!("{}/heatmaps", &folder_warehouse_plot);

    let folder_warehouse_exist = fs::exists(&folder_warehouse).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al verificar la carpeta raiz, la ruta no existe ó permisos insuficientes {}",
            e
        ))
    })?;
    let folder_warehouse_data_exist = fs::exists(&folder_warehouse_data).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al verificar la carpeta raiz, la ruta no existe ó permisos insuficientes {}",
            e
        ))
    })?;
    let folder_warehouse_plot_exist = fs::exists(&folder_warehouse_plot).map_err(|e| {
        AnalysisError::FileSystemOperationError(format!(
            "Error al verificar la carpeta raiz, la ruta no existe ó permisos insuficientes {}",
            e
        ))
    })?;

    let folder_warehouse_zipfs_plot_exist =
        fs::exists(&folder_warehouse_zipf_plot).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!(
                "Error al verificar la carpeta zipf dentro de plot, la ruta no existe ó permisos insuficientes {}",
                e
            ))
        })?;

    let folder_warehouse_heaps_plot_exist =
        fs::exists(&folder_warehouse_heaps_plot).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!(
                "Error al verificar la carpeta heaps dentro de plot, la ruta no existe ó permisos insuficientes {}",
                e
            ))
        })?;

    let folder_warehouse_heatmaps_plot_exist =
        fs::exists(&folder_warehouse_heatmap_plot).map_err(|e| {
            AnalysisError::FileSystemOperationError(format!(
                "Error al verificar la carpeta heatmaps dentro de plot, la ruta no existe ó permisos insuficientes {}",
                e
            ))
        })?;

    if !folder_warehouse_exist {
        fs::create_dir(&folder_warehouse).unwrap();
    }

    if !folder_warehouse_data_exist {
        fs::create_dir(&folder_warehouse_data).unwrap();
    }

    if !folder_warehouse_plot_exist {
        fs::create_dir(&folder_warehouse_plot).unwrap();
    }

    if !folder_warehouse_zipfs_plot_exist {
        fs::create_dir(&folder_warehouse_zipf_plot).unwrap();
    }

    if !folder_warehouse_heaps_plot_exist {
        fs::create_dir(&folder_warehouse_heaps_plot).unwrap();
    }

    if !folder_warehouse_heatmaps_plot_exist {
        fs::create_dir(&folder_warehouse_heatmap_plot).unwrap();
    }

    let pb = ProgressBar::new(total_load_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.blue} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );
    let mut general_words_unique_hasmamp: HashMap<String, u32> = HashMap::new();
    let mut general_words_hashmaps: HashMap<String, u32> = HashMap::new();
    let (mut general_inter_words_hashmaps, _) = create_inter_words_differ(&inter_words_strings)
        .map_err(|e| {
            AnalysisError::ProcessingError(format!(
                "Error al inicializar el hashmap general interwords de interes (automatizada): {}",
                e
            ))
        })?;
    let mut general_n_words_vec: Vec<u32> = Vec::new();
    let mut general_n_words_unique_vec: Vec<u32> = Vec::new();
    let mut total_words: u32 = 0;
    let mut total_unique_words: u32 = 0;

    clear_screen();
    println!(
        "Iniciando procesamiento para el dataset: {}.{}",
        file_name_dataset, file_extension_dataset
    );
    // LOG
    write_log_result(
        format!(
            "\n[Completado] Se ha leido el dataset correctamente. {}.{}",
            file_name_dataset, file_extension_dataset
        ),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;
    // ENDLOG

    for (file, year) in csv_content.iter() {
        let (file_name, file_extension) = match file.split_once(".") {
            Some((file_name_extract, file_extension_extract)) => {
                // LOG
                write_log_result(
                    format!(
                        "\n[Completado] Se ha leido la fila sin problemas. [Fila: file:{}, year:{}]. ",
                        file, year
                    ),
                    &mut file_log,
                )
                .map_err(|e| {
                    AnalysisError::FileSystemOperationError(format!(
                        "Error al escribir logs': {}",
                        e
                    ))
                })?;
                // ENDLOG
                (file_name_extract, file_extension_extract)
            }
            None => {
                let mut processed_file_status: Vec<CellStruct> = Vec::new();
                processed_file_status.push(file.clone().cell());
                processed_file_status.push(" Error ".on_red().cell().justify(Justify::Right));
                processed_file_status_table.push(processed_file_status);

                // LOG
                write_log_result(
                    format!(
                        "\n[Error] Error en fila del dataset, nombre de archivo ó extensión no identificable [Fila: file:{}, year:{}]. ",
                        file, year
                    ),
                    &mut file_log,
                )
                .map_err(|e| {
                    AnalysisError::FileSystemOperationError(format!(
                        "Error al escribir logs': {}",
                        e
                    ))
                })?;
                // ENDLOG
                continue;
            }
        };

        let mut words: HashMap<String, u32> = HashMap::new();
        let (mut inter_words_hashmaps, mut last_positions) =
            create_inter_words_differ(&inter_words_strings).map_err(|e| {

                let mut processed_file_status: Vec<CellStruct> = Vec::new();
                processed_file_status.push(file.clone().cell());
                processed_file_status.push(" Error ".on_red().cell().justify(Justify::Right));
                processed_file_status_table.push(processed_file_status);
                AnalysisError::ProcessingError(format!(
                    "Error al inicializar el hashmap específica interwords de interes (automatizada): {}",
                    e
                ))
            })?;
        let content = match document_extract_content(&file_name, &file_extension) {
            Ok(content) => {
                // LOG
                write_log_result(
                    format!(
                        "\n[Completado] Se ha extraido el contenido sin problemas. {}.{} ",
                        file_name, file_extension
                    ),
                    &mut file_log,
                )
                .map_err(|e| {
                    AnalysisError::FileSystemOperationError(format!(
                        "Error al escribir logs': {}",
                        e
                    ))
                })?;
                // ENDLOG
                content
                    .to_lowercase()
                    .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], "")
            }
            Err(_) => {
                // LOG
                write_log_result(
                    format!(
                        "\n[Error] Error en la extracción del contenido. {}.{}",
                        file_name, file_extension
                    ),
                    &mut file_log,
                )
                .map_err(|e| {
                    AnalysisError::FileSystemOperationError(format!(
                        "Error al escribir logs': {}",
                        e
                    ))
                })?;
                // ENDLOG
                let mut processed_file_status: Vec<CellStruct> = Vec::new();
                processed_file_status.push(file.clone().cell());
                processed_file_status.push(" Error ".on_red().cell().justify(Justify::Right));
                processed_file_status_table.push(processed_file_status);
                String::new()
            }
        };

        let (n_words_total_vec, n_words_unique_vec) = analyzer_content_dataset_opt(
            content,
            &mut words,
            &mut general_words_unique_hasmamp,
            &ascii_interest,
            &mut inter_words_hashmaps,
            &mut last_positions,
            &inter_words_strings,
            &mut total_words,
            &mut total_unique_words,
            &mut general_n_words_vec,
            &mut general_n_words_unique_vec,
        )
        .map_err(|e| {
            // LOG
            let _ = write_log_result(
                format!(
                    "\n[Error] Ha ocurrido un problema al analizar el contenido del documento. {}.{}", file_name, file_extension
                ),
                &mut file_log,
            )
            .map_err(|e| {
                AnalysisError::FileSystemOperationError(format!(
                    "Error al escribir logs': {}",
                    e
                ))
            });
            // ENDLOG
            let mut processed_file_status: Vec<CellStruct> = Vec::new();
            processed_file_status.push(file.clone().cell());
            processed_file_status.push(" Error ".on_red().cell().justify(Justify::Right));
            processed_file_status_table.push(processed_file_status);
            AnalysisError::ProcessingError(format!("Error con el analisis del documento {}", e))
        })?;

        // LOG
        write_log_result(
            format!("\n[Completado] Se ha analizado el contenido completo del documento."),
            &mut file_log,
        )
        .map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
        })?;
        // ENDLOG

        let (mut keys, mut values) = initializer_word_hashmap_handler(&words).map_err(|e| {
            let mut processed_file_status: Vec<CellStruct> = Vec::new();
            processed_file_status.push(file.clone().cell());
            processed_file_status.push(" Incomplete ".on_yellow().cell().justify(Justify::Right));
            processed_file_status_table.push(processed_file_status);
            AnalysisError::ProcessingError(format!(
                "Error al inicializar los valores zipf para el documento: {}, {}",
                file_name, e
            ))
        })?;
        if keys.is_empty() && values.is_empty() {
            continue;
        }

        get_zipf_law_results(&mut keys, &mut values);
        create_csv_ordered(&keys, &values, &file_name, &folder_warehouse_data);
        let (vec_distance, vec_frequency) = create_csv_inter_words(
            &file_name,
            &inter_words_hashmaps,
            &inter_words_strings,
            &folder_warehouse_data,
        )
        .map_err(|e| {
            let mut processed_file_status: Vec<CellStruct> = Vec::new();
            processed_file_status.push(file.clone().cell());
            processed_file_status.push(" Incomplete ".on_yellow().cell().justify(Justify::Right));
            processed_file_status_table.push(processed_file_status);
            AnalysisError::ProcessingError(format!(
                "Error en la generación de los csv inter words para el documento: {}, {}",
                file_name, e
            ))
        })?;

        copy_words_to_main(&mut general_words_hashmaps, &words);
        copy_interword_to_main(&mut general_inter_words_hashmaps, &inter_words_hashmaps);

        let (log_ranking, log_values) = apply_to_log10(values).map_err(|e| {
            AnalysisError::ParseError(format!(
                "Error en el cálculo logarítmico en base 10 para el documento: {}, {}",
                file_name, e
            ))
        })?;
        let parameters = linear_regression_x1(&log_ranking, &log_values).map_err(|e| {
            AnalysisError::ProcessingError(format!(
                "Error en el cálculo de la regresión lineal para el documento: {}, {}",
                file_name, e
            ))
        })?;

        plot_heat_map(
            "Frequency distribution of inter word's distance",
            "Distance",
            "Inter Word",
            &vec_distance,
            &vec_frequency,
            &inter_words_strings,
            &folder_warehouse_heatmap_plot,
            &file_name,
            &file_extension,
        );

        plot_zipf_law(
            &log_ranking,
            &log_values,
            &parameters,
            &folder_warehouse_zipf_plot,
            &file_name,
        );

        plot_heaps_law(
            &n_words_total_vec,
            &n_words_unique_vec,
            &folder_warehouse_heaps_plot,
            &file_name,
        );

        // LOG
        write_log_result(
            format!(
                "\n[Completado] Todos los graficos han sido generados para el documento: {}.{}",
                file_name, file_extension
            ),
            &mut file_log,
        )
        .map_err(|e| {
            AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
        })?;
        // ENDLOG

        let mut processed_file_status: Vec<CellStruct> = Vec::new();
        processed_file_status.push(file.clone().cell());
        processed_file_status.push(" Completed ".on_green().cell().justify(Justify::Right));
        processed_file_status_table.push(processed_file_status);

        let alphas = year_alphas_hashmaps
            .entry(*year)
            .or_insert(vec![parameters[1].abs()]);
        alphas.push(parameters[1].abs());

        let new = min(loading_value + 1, total_load_size);
        loading_value = new;
        pb.set_position(new);
    }
    pb.finish_with_message("Carga completada.");
    println!("# Inicio de elaboración de graficos...");
    let (x_values, y_values) = means_hashmap_to_vectors(year_alphas_hashmaps).map_err(|e| {
        AnalysisError::ParseError(format!(
            "Error al incializar los valores para el grafico year-alpha {}",
            e
        ))
    })?;

    lineplot_alpha_year(
        "Alpha variation",
        "Year",
        "Alpha",
        &x_values,
        &y_values,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    let (mut keys, mut values) = initializer_word_hashmap_handler(&general_words_hashmaps).unwrap();
    if keys.is_empty() && values.is_empty() {
        return Err(AnalysisError::EmptyResultError);
    }

    get_zipf_law_results(&mut keys, &mut values);
    create_csv_ordered(&keys, &values, &file_name_dataset, &folder_warehouse_data);
    let (vec_distance, vec_frequency) = create_csv_inter_words(
        &file_name_dataset,
        &general_inter_words_hashmaps,
        &inter_words_strings,
        &folder_warehouse_data,
    )
    .map_err(|e| {
        AnalysisError::ProcessingError(format!(
            "Error en la generación de los csv inter words (general) {}",
            e
        ))
    })?;

    let (log_ranking, log_values) = apply_to_log10(values).map_err(|e| {
        AnalysisError::ParseError(format!(
            "Error en el cálculo logarítmico en base 10 (general): {}",
            e
        ))
    })?;
    let parameters = linear_regression_x1(&log_ranking, &log_values).map_err(|e| {
        AnalysisError::ProcessingError(format!(
            "Error en el cálculo de la regresión lineal (general) {}",
            e
        ))
    })?;

    plot_heat_map(
        "Frequency distribution of inter word's distance",
        "Distance",
        "Inter Word",
        &vec_distance,
        &vec_frequency,
        &inter_words_strings,
        &folder_warehouse_plot,
        &file_name_dataset,
        &file_extension_dataset,
    );

    plot_zipf_law(
        &log_ranking,
        &log_values,
        &parameters,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    // Filtro para visualización mas apropiada de la scatterplot
    //let n_words_total = general_n_words_vec[general_n_words_vec.len() - 1];

    let mut doc_heap_x_values: Vec<u32> = Vec::new();
    let mut doc_heap_y_values: Vec<u32> = Vec::new();
    let mut thresholds: Vec<u32> = Vec::new();
    for i in 0..=40 {
        let threshold = (i * (total_words)) / 40;
        thresholds.push(threshold);
    }
    let mut threshold_index = 0;
    for (index, n_value) in general_n_words_vec.iter().enumerate() {
        if n_value >= &thresholds[threshold_index] {
            doc_heap_x_values.push(*n_value);
            doc_heap_y_values.push(general_n_words_unique_vec[index]);
            threshold_index += 1;
        }
    }

    plot_heaps_law(
        &doc_heap_x_values,
        &doc_heap_y_values,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    println!("# Finalizado...");
    println!("# Reporte:");
    let table = processed_file_status_table
        .table()
        .title(vec![
            "Archivo".cell().bold(true),
            "Estado".cell().bold(true),
        ])
        .bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);

    // LOG
    write_log_result(
        format!(
            "\n[Completado] Se han generado graficos extras del dataset como conjunto.\n[Completado] El procesamiento del dataset ha finalizado: {}.{}",
            file_name_dataset, file_extension_dataset
        ),
        &mut file_log,
    )
    .map_err(|e| {
        AnalysisError::FileSystemOperationError(format!("Error al escribir logs': {}", e))
    })?;
    // ENDLOG
    println!("Ejecutado en {:.3?}", started.elapsed());

    Ok(())
}
