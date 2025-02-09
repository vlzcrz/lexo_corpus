use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

pub fn linear_regression_x1(x_data: &[f64], y_data: &[f64]) -> Result<Vec<f64>, linregress::Error> {
    let data = vec![("Y", y_data.to_vec()), ("X", x_data.to_vec())];
    let data = RegressionDataBuilder::new().build_from(data)?;
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula("Y ~ X")
        .fit()?;
    let params = model.parameters().to_vec();
    Ok(params)
}
