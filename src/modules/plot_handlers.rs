use std::{collections::HashMap, io::Error};

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, EmptyElement, IntoDrawingArea},
    series::{LineSeries, PointSeries},
    style::{BLUE, RED, WHITE},
};
use pyo3::{
    ffi::c_str,
    types::{PyAnyMethods, PyModule},
    Python,
};

pub fn scatter_plot(
    tuple: Vec<(f64, f64)>,
    file_name: &str,
    lr_parameters: &Vec<f64>,
    folder_name: &str,
) -> Result<(), Error> {
    let image_extension: &str = "png";
    let plot_path = format!(
        "{}/{}-zipf-plot.{}",
        folder_name, file_name, image_extension
    );
    let plot_title = format!("Zipf's Law (Log) - File: {}", file_name);

    let (_, y_limit_first) = &tuple[0];
    // Linear Regression alpha y beta
    let beta = &lr_parameters[0];
    let alpha = &lr_parameters[1];
    let x_first = (*y_limit_first - beta) / alpha;
    let x_last = (0f64 - beta) / alpha;
    // Linear Regression points
    let polyfit_point_one = (x_first, *y_limit_first);
    let polyfit_point_two = (x_last, 0f64);

    let root = BitMapBackend::new(&plot_path, (960, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(5, 5, 5, 5);

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 28))
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(x_first..x_last, 0f64..*y_limit_first)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_labels(5)
        .y_labels(5)
        .x_desc("Log(ranking)")
        .y_desc("Log(frequency)")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(tuple.iter().map(|&p| p), &RED))
        .unwrap()
        .label("label 1");

    chart
        .draw_series(LineSeries::new(
            vec![polyfit_point_one, polyfit_point_two],
            &BLUE,
        ))
        .unwrap();

    chart
        .draw_series(PointSeries::of_element(
            tuple.iter().map(|&p| p),
            3,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        ))
        .unwrap();

    root.present().unwrap();
    Ok(())
}

pub fn to_tuples(x_values: Vec<f64>, y_values: Vec<f64>) -> Result<Vec<(f64, f64)>, Error> {
    let mut tuple: Vec<(f64, f64)> = Vec::new();
    for (index, x_value) in x_values.iter().enumerate() {
        tuple.push((*x_value, y_values[index]));
    }
    Ok(tuple)
}

pub fn to_tuples_x_int(x_values: Vec<i32>, y_values: Vec<f64>) -> Result<Vec<(i32, f64)>, Error> {
    let mut tuple: Vec<(i32, f64)> = Vec::new();
    for (index, x_value) in x_values.iter().enumerate() {
        tuple.push((*x_value, y_values[index]));
    }
    Ok(tuple)
}

pub fn to_tuples_generic<X, Y>(x_values: Vec<X>, y_values: Vec<Y>) -> Result<Vec<(X, Y)>, Error> {
    if x_values.len() != y_values.len() {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Los valores de x_values y y_values deben ser del mismo largo",
        ));
    }
    let result = x_values.into_iter().zip(y_values.into_iter()).collect();

    Ok(result)
}

pub fn hashmap_means(hashmap: HashMap<i32, Vec<f64>>) -> Result<(Vec<i32>, Vec<f64>), Error> {
    let mut x_values: Vec<i32> = Vec::new();
    let mut y_values: Vec<f64> = Vec::new();
    for (year, alphas) in hashmap.iter() {
        let alphas_length = alphas.len() as f64;
        let mut alpha_sum: f64 = 0.0;
        for alpha in alphas.iter() {
            alpha_sum += alpha
        }
        x_values.push(*year);
        y_values.push(alpha_sum / alphas_length);
    }
    Ok((x_values, y_values))
}

pub fn scatter_plot_alpha(
    tuple: Vec<(i32, f64)>,
    file_name: &str,
    folder_name: &str,
) -> Result<(), Error> {
    let image_extension: &str = "png";
    let plot_path = format!("{}/{}.{}", folder_name, file_name, image_extension);
    let plot_title = format!("Alpha by year - Dataset: {}", file_name);

    // search max x, y values
    let mut max_y: f64 = 0.0;
    let mut max_x: i32 = 0;
    let mut min_y: f64 = 9999.0;
    let mut min_x: i32 = 9999;
    for (x_value, y_value) in tuple.iter() {
        if x_value > &max_x {
            max_x = *x_value
        }

        if x_value < &min_x {
            min_x = *x_value
        }

        if y_value > &max_y {
            max_y = *y_value
        }

        if y_value < &min_y {
            min_y = *y_value
        }
    }

    let root = BitMapBackend::new(&plot_path, (960, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(15, 15, 15, 15);

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 28))
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(min_x..max_x, 0f64..max_y + 0.05)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(6)
        .y_labels(6)
        .x_desc("Years")
        .y_desc("Alpha")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(tuple.iter().map(|&p| p), &BLUE))
        .unwrap()
        .label("label 1");

    chart
        .draw_series(PointSeries::of_element(
            tuple.iter().map(|&p| p),
            3,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        ))
        .unwrap();

    root.present().unwrap();
    Ok(())
}

pub fn scatter_plot_heaps_law(
    mut tuple: Vec<(u32, u32)>,
    file_name: &str,
    folder_name: &str,
) -> Result<(), Error> {
    let image_extension: &str = "png";
    let plot_path = format!(
        "{}/{}-heaps-plot.{}",
        folder_name, file_name, image_extension
    );
    let plot_title = format!("Heap's Law - Dataset: {}", file_name);
    if !tuple.contains(&(0, 0)) {
        tuple.insert(0, (0, 0));
    }

    // search max x, y values
    let mut max_y: u32 = 0;
    let mut max_x: u32 = 0;
    for (x_value, y_value) in tuple.iter() {
        if x_value > &max_x {
            max_x = *x_value
        }

        if y_value > &max_y {
            max_y = *y_value
        }
    }

    let root = BitMapBackend::new(&plot_path, (960, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(15, 15, 15, 15);

    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 28))
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0u32..max_x, 0u32..max_y + 2)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(6)
        .y_labels(6)
        .x_desc("Total document's words")
        .y_desc("Total unique words found")
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(tuple.iter().map(|&p| p), &BLUE))
        .unwrap()
        .label("label 1");

    chart
        .draw_series(PointSeries::of_element(
            tuple.iter().map(|&p| p),
            3,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        ))
        .unwrap();

    root.present().unwrap();
    Ok(())
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
        let args = (x_values, y_values, folder_name, file_name);
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
