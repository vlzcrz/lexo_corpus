use std::io::Error;

use super::merge_sort_utils::merge_sort;

// Una función que permita obtener la tabla (o vectores) 'ranking' y 'frecuencia' según el documento leido
pub fn get_zipf_law_results(keys_vector: &mut Vec<String>, values_vector: &mut Vec<u32>) {
    merge_sort(
        values_vector,
        keys_vector,
        0,
        (values_vector.len() - 1) as u32,
    );
    keys_vector.reverse();
    values_vector.reverse();
}

pub fn apply_to_log10(values_vector: Vec<u32>) -> Result<(Vec<f64>, Vec<f64>), Error> {
    let capacity = values_vector.len() as u32;
    let ranking: Vec<u32> = (1..=capacity).collect();

    // Aplicamos log base 10 para graficar asimilando una recta con pendiente negativa
    let log_values: Vec<f64> = values_vector
        .iter()
        .map(|&val| (val as f64).log10())
        .collect();
    let log_ranking: Vec<f64> = ranking.iter().map(|&val| (val as f64).log10()).collect();

    Ok((log_ranking, log_values))
}
