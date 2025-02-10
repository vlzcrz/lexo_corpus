use std::io::Error;

use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, Circle, EmptyElement, IntoDrawingArea},
    series::{LineSeries, PointSeries},
    style::{BLUE, RED, WHITE},
};

pub fn scatter_plot(
    tuple: Vec<(f64, f64)>,
    file_name: &str,
    lr_parameters: &Vec<f64>,
) -> Result<(), Error> {
    let image_extension: &str = "png";
    let plot_path = format!("books-plot/{}.{}", file_name, image_extension);
    let plot_title = format!("Zipf's Law (Log) - File: {}", file_name);

    let (x_limit_last, _) = &tuple[tuple.len() - 1];
    let (x_last, y_limit_first) = &tuple[0];
    // Linear Regression alpha y beta
    let beta = &lr_parameters[0];
    let alpha = &lr_parameters[1];
    let xFirst = (*y_limit_first - beta) / alpha;
    let xLast = (0f64 - beta) / alpha;
    // Linear Regression points
    let polyfit_point_one = (xFirst, *y_limit_first);
    let polyfit_point_two = (xLast, 0f64);
    println!("{:?} | {:?}", polyfit_point_one, polyfit_point_two);

    let root = BitMapBackend::new(&plot_path, (960, 720)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(5, 5, 5, 5);

    println!("{:?}", tuple);
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 28))
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(0f64..*x_limit_last + 0.05, 0f64..*y_limit_first)
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
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
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
