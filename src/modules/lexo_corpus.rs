use std::{
    cmp::min,
    collections::HashMap,
    fmt::Write,
    fs, io, thread,
    time::{Duration, Instant},
};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::modules::{
    cli_handlers::clear_screen,
    file_handlers::{document_extract_content, extract_csv_labeled_data, get_files_from_folder},
    lexical_analisis::{
        analyzer_content_dataset_opt, analyzer_content_opt3, copy_interword_to_main,
        copy_words_to_main, create_inter_words, create_inter_words_differ, input_inter_words,
    },
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

pub fn option_one() {
    // Directorio donde se encuentran los pdf
    //let base_path = String::from("books-pdf/");
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
    let (mut inter_words_hashmaps, mut last_positions, inter_words_strings) =
        create_inter_words().unwrap();

    let default_folder_data = "books-data";
    let default_folder_plot = "books-plot";
    let mut content = String::new();
    let mut file_name = String::new();
    let mut file_extension = String::new();

    while !did_read {
        clear_screen();
        println!("Ingresa el nombre del archivo con su extension .txt ó .pdf (Presione '0' para cancelar)");
        io::stdin().read_line(&mut file_path_input).unwrap();

        if file_path_input.trim() == "0" {
            return;
        }

        let (name_f, extension_f) = file_path_input.split_once(".").unwrap();
        file_name = name_f.trim().to_string();
        file_extension = extension_f.trim().to_string();
        content = match document_extract_content(&file_name, &file_extension) {
            Ok(content) => {
                did_read = true;
                content
                    .to_lowercase()
                    .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], "")
            }
            Err(_) => {
                file_path_input = String::new();
                String::new()
            }
        };
    }
    let started = Instant::now();
    clear_screen();
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
    .unwrap();

    let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
    if keys.is_empty() && values.is_empty() {
        return;
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
    .unwrap();
    //let log_n_words_total = vec_apply_to_log10(&n_words_total_vec).unwrap();
    //let log_n_words_unique = vec_apply_to_log10(&n_words_unique_vec).unwrap();

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

    let (log_ranking, log_values) = apply_to_log10(values).unwrap();
    let zipfs_parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();

    plot_zipf_law(
        &log_ranking,
        &log_values,
        &zipfs_parameters,
        &default_folder_plot,
        &file_name,
    );

    println!("# Finalizado.");
    println!("Ejecutado en {:.3?}", started.elapsed());
}

pub fn option_two() {
    let mut input = String::new();
    let mut valid_input = false;
    let mut file_name_dataset = String::new();
    let mut file_extension_dataset = String::new();
    while !valid_input {
        clear_screen();
        println!("Seleccione un data label (csv) para iniciar el lote de procesamiento de textos. (Presione '0' para cancelar)");
        let labeled_data_files = get_files_from_folder("labeled-data").unwrap();
        let max = labeled_data_files.len() as u16;
        for (index, (file_name, file_extension)) in labeled_data_files.iter().enumerate() {
            println!("{}.- {}.{}", index + 1, file_name, file_extension);
        }
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "0" {
            return;
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

    let csv_content =
        extract_csv_labeled_data(&file_name_dataset, &file_extension_dataset).unwrap();

    let mut year_alphas_hashmaps: HashMap<i32, Vec<f64>> = HashMap::new();
    let inter_words_strings = input_inter_words().unwrap();
    let mut ascii_interest: Vec<u8> = (97..121).collect();
    ascii_interest.push(39);
    let mut ascii_interest_numbers: Vec<u8> = (48..57).collect();
    ascii_interest.append(&mut ascii_interest_numbers);

    let mut loading_value = 0;
    let total_load_size = csv_content.len() as u64;
    clear_screen();
    println!(
        "Iniciando procesamiento para el dataset: {}.{}",
        file_name_dataset, file_extension_dataset
    );

    let started = Instant::now();
    let folder_warehouse = format!("./{}", file_name_dataset);
    let folder_warehouse_data = format!("./{}/data", &file_name_dataset);

    let folder_warehouse_plot = format!("./{}/plot", &file_name_dataset);
    let folder_warehouse_exist = fs::exists(&folder_warehouse).unwrap();
    let folder_warehouse_data_exist = fs::exists(&folder_warehouse_data).unwrap();
    let folder_warehouse_plot_exist = fs::exists(&folder_warehouse_plot).unwrap();

    if !folder_warehouse_exist {
        fs::create_dir(&folder_warehouse).unwrap();
    }

    if !folder_warehouse_data_exist {
        fs::create_dir(&folder_warehouse_data).unwrap();
    }

    if !folder_warehouse_plot_exist {
        fs::create_dir(&folder_warehouse_plot).unwrap();
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
    let (mut general_inter_words_hashmaps, _) =
        create_inter_words_differ(&inter_words_strings).unwrap();
    let mut general_n_words_vec: Vec<u32> = Vec::new();
    let mut general_n_words_unique_vec: Vec<u32> = Vec::new();
    let mut total_words: u32 = 0;
    let mut total_unique_words: u32 = 0;

    for (file, year) in csv_content.iter() {
        let (file_name, file_extension) = file.split_once(".").unwrap();
        let mut words: HashMap<String, u32> = HashMap::new();
        let (mut inter_words_hashmaps, mut last_positions) =
            create_inter_words_differ(&inter_words_strings).unwrap();
        let content = match document_extract_content(&file_name, &file_extension) {
            Ok(content) => content
                .to_lowercase()
                .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], ""),
            Err(_) => String::new(),
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
        .unwrap();

        let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
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
        .unwrap();

        copy_words_to_main(&mut general_words_hashmaps, &words);
        copy_interword_to_main(&mut general_inter_words_hashmaps, &inter_words_hashmaps);

        let (log_ranking, log_values) = apply_to_log10(values).unwrap();
        let parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();

        plot_heat_map(
            "Frequency distribution of inter word's distance",
            "Distance",
            "Inter Word",
            &vec_distance,
            &vec_frequency,
            &inter_words_strings,
            &folder_warehouse_plot,
            &file_name,
            &file_extension,
        );

        plot_zipf_law(
            &log_ranking,
            &log_values,
            &parameters,
            &folder_warehouse_plot,
            &file_name,
        );

        plot_heaps_law(
            &n_words_total_vec,
            &n_words_unique_vec,
            &folder_warehouse_plot,
            &file_name,
        );

        let alphas = year_alphas_hashmaps
            .entry(*year)
            .or_insert(vec![parameters[1].abs()]);
        alphas.push(parameters[1].abs());

        let new = min(loading_value + 1, total_load_size);
        loading_value = new;
        pb.set_position(new);
    }
    pb.finish_with_message("Carga completada.");
    println!("# Inicio de elaboración de Grafico alpha...");
    let (x_values, y_values) = means_hashmap_to_vectors(year_alphas_hashmaps).unwrap();

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
        return;
    }

    get_zipf_law_results(&mut keys, &mut values);
    create_csv_ordered(&keys, &values, &file_name_dataset, &folder_warehouse_data);
    let (vec_distance, vec_frequency) = create_csv_inter_words(
        &file_name_dataset,
        &general_inter_words_hashmaps,
        &inter_words_strings,
        &folder_warehouse_data,
    )
    .unwrap();

    let (log_ranking, log_values) = apply_to_log10(values).unwrap();
    let parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();

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

    println!("{:?}", general_n_words_vec);
    println!("{:?}", general_n_words_unique_vec);

    // Filtro para visualización mas apropiada de la scatterplot
    //let n_words_total = general_n_words_vec[general_n_words_vec.len() - 1];

    let mut doc_heap_x_values: Vec<u32> = Vec::new();
    let mut doc_heap_y_values: Vec<u32> = Vec::new();
    let mut tresholds: Vec<u32> = Vec::new();
    for i in 0..=40 {
        let treshold = (i * (total_words)) / 40;
        tresholds.push(treshold);
    }
    let mut treshold_index = 0;
    for (index, n_value) in general_n_words_vec.iter().enumerate() {
        if n_value >= &tresholds[treshold_index] {
            doc_heap_x_values.push(*n_value);
            doc_heap_y_values.push(general_n_words_unique_vec[index]);
            treshold_index += 1;
        }
    }

    plot_heaps_law(
        &doc_heap_x_values,
        &doc_heap_y_values,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    println!("# Finalizado...");
    println!("Ejecutado en {:.3?}", started.elapsed());
}

pub fn option_three() {
    let mut input = String::new();
    let mut valid_input = false;
    let mut file_name_dataset = String::new();
    let mut file_extension_dataset = String::new();

    // BLOQUE 1 - INPUT DEL DATASET
    while !valid_input {
        clear_screen();
        println!("Seleccione un data label (csv) para iniciar el lote de procesamiento de textos. (Presione '0' para cancelar)");
        let labeled_data_files = get_files_from_folder("labeled-data").unwrap();
        let max = labeled_data_files.len() as u16;
        for (index, (file_name, file_extension)) in labeled_data_files.iter().enumerate() {
            println!("{}.- {}.{}", index + 1, file_name, file_extension);
        }
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "0" {
            return;
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

    // FIN BLOQUE 1 -------------------------------
    // BLOQUE 2 LECTURA CSV Y INICIALIZACIÓN DE VARIABLES
    let csv_content =
        extract_csv_labeled_data(&file_name_dataset, &file_extension_dataset).unwrap();

    let mut words: HashMap<String, u32> = HashMap::new();
    let mut year_alphas_hashmaps: HashMap<i32, Vec<f64>> = HashMap::new();
    let inter_words_strings = input_inter_words().unwrap();
    let (mut inter_words_hashmaps, mut last_positions) =
        create_inter_words_differ(&inter_words_strings).unwrap();

    let mut n_words_total: u32 = 0;
    let mut n_words_unique: u32 = 0;
    let mut n_words_total_vec: Vec<u32> = Vec::new();
    let mut n_words_unique_vec: Vec<u32> = Vec::new();

    let mut ascii_interest: Vec<u8> = (97..121).collect();
    ascii_interest.push(39);
    let mut ascii_interest_numbers: Vec<u8> = (48..57).collect();
    ascii_interest.append(&mut ascii_interest_numbers);

    let mut loading_value = 0;
    let total_load_size = csv_content.len() as u64;
    clear_screen();

    // FIN BLOQUE 2 ------------------------------
    // INICIO BLOQUE 3 PROCESAMIENTO
    println!(
        "Iniciando procesamiento para el dataset: {}.{}",
        file_name_dataset, file_extension_dataset
    );

    let started = Instant::now();
    let folder_warehouse = format!("./{}-compendium", file_name_dataset);
    let folder_warehouse_data = format!("./{}/data", &folder_warehouse);
    let folder_warehouse_plot = format!("./{}/plot", &folder_warehouse);
    let folder_warehouse_exist = fs::exists(&folder_warehouse).unwrap();
    let folder_warehouse_data_exist = fs::exists(&folder_warehouse_data).unwrap();
    let folder_warehouse_plot_exist = fs::exists(&folder_warehouse_plot).unwrap();
    if !folder_warehouse_exist {
        fs::create_dir(&folder_warehouse).unwrap();
    }

    if !folder_warehouse_data_exist {
        fs::create_dir(&folder_warehouse_data).unwrap();
    }

    if !folder_warehouse_plot_exist {
        fs::create_dir(&folder_warehouse_plot).unwrap();
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
    // BLOQUE 3-1

    for (file, year) in csv_content.iter() {
        let (file_name, file_extension) = file.split_once(".").unwrap();
        let mut words_per_doc: HashMap<String, u32> = HashMap::new();
        let content = match document_extract_content(&file_name, &file_extension) {
            Ok(content) => content
                .to_lowercase()
                .replace(&[',', '.', '(', ')', '[', ']', '~', '`'][..], ""),
            Err(_) => String::new(),
        };
        analyzer_content_opt3(
            content,
            &mut words,
            &mut words_per_doc,
            &ascii_interest,
            &mut inter_words_hashmaps,
            &mut last_positions,
            &inter_words_strings,
            &mut n_words_total,
            &mut n_words_unique,
            &mut n_words_total_vec,
            &mut n_words_unique_vec,
        );

        let (mut keys, mut values) = initializer_word_hashmap_handler(&words_per_doc).unwrap();
        if keys.is_empty() && values.is_empty() {
            return;
        }
        get_zipf_law_results(&mut keys, &mut values);

        let (log_ranking, log_values) = apply_to_log10(values).unwrap();

        let parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();
        let alphas = year_alphas_hashmaps
            .entry(*year)
            .or_insert(vec![parameters[1].abs()]);
        alphas.push(parameters[1].abs());

        let new = min(loading_value + 1, total_load_size);
        loading_value = new;
        pb.set_position(new);
    }
    pb.finish_with_message("Carga completada.");
    println!("# Inicio de elaboración de Grafico...");

    let (mut keys, mut values) = initializer_word_hashmap_handler(&words).unwrap();
    if keys.is_empty() && values.is_empty() {
        return;
    }

    get_zipf_law_results(&mut keys, &mut values);
    create_csv_ordered(&keys, &values, &file_name_dataset, &folder_warehouse_data);

    let (log_ranking, log_values) = apply_to_log10(values).unwrap();
    let parameters = linear_regression_x1(&log_ranking, &log_values).unwrap();

    plot_zipf_law(
        &log_ranking,
        &log_values,
        &parameters,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    let (vec_distance, vec_frequency) = create_csv_inter_words(
        &file_name_dataset,
        &inter_words_hashmaps,
        &inter_words_strings,
        &folder_warehouse_data,
    )
    .unwrap();

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

    let (x_values, y_values) = means_hashmap_to_vectors(year_alphas_hashmaps).unwrap();

    lineplot_alpha_year(
        "Alpha variation",
        "Year",
        "Alpha",
        &x_values,
        &y_values,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    // Filtro para visualización mas apropiada de la scatterplot
    let mut doc_heap_x_values: Vec<u32> = Vec::new();
    let mut doc_heap_y_values: Vec<u32> = Vec::new();
    let mut tresholds: Vec<u32> = Vec::new();
    for i in 0..=40 {
        let treshold = (i * (n_words_total)) / 40;
        tresholds.push(treshold);
    }
    let mut treshold_index = 0;
    for (index, n_value) in n_words_total_vec.iter().enumerate() {
        if n_value >= &tresholds[treshold_index] {
            doc_heap_x_values.push(*n_value);
            doc_heap_y_values.push(n_words_unique_vec[index]);
            treshold_index += 1;
        }
    }

    println!("{:?}", n_words_total_vec);
    println!("{:?}", n_words_unique_vec);

    plot_heaps_law(
        &doc_heap_x_values,
        &doc_heap_y_values,
        &folder_warehouse_plot,
        &file_name_dataset,
    );

    println!("# Finalizado...");
    println!("Ejecutado en {:.3?}", started.elapsed());
}
