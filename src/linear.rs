pub fn linear_regression(
    x_values: &[f64],
    y_values: &[f64],
) -> Result<(f64, f64), String> {
    let sample_size = x_values.len();
    if sample_size == 0 || sample_size != y_values.len() {
        return Err(
            "x_values e y_values devem ter o mesmo tamanho e não podem estar vazios"
                .to_string(),
        );
    }
    let sample_size_f64 = sample_size as f64;

    let total_x: f64 = x_values.iter().sum();
    let total_y: f64 = y_values.iter().sum();

    let mean_x = total_x / sample_size_f64;
    let mean_y = total_y / sample_size_f64;

    let cross_deviation: f64 = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(&x_i, &y_i)| (x_i - mean_x) * (y_i - mean_y))
        .sum();

    let squared_deviation_x: f64 = x_values
        .iter()
        .map(|&x_i| (x_i - mean_x).powi(2))
        .sum();

    if squared_deviation_x.abs() < std::f64::EPSILON {
        return Err("Variância zero em x: não é possível ajustar reta".to_string());
    }

    let slope = cross_deviation / squared_deviation_x;
    let intercept = mean_y - slope * mean_x;

    Ok((intercept, slope))
}

pub fn mean_squared_error(actual_values: &[f64], predicted_values: &[f64]) -> f64 {
    let sample_size = actual_values.len();
    assert!(
        sample_size > 0 && sample_size == predicted_values.len(),
        "Vetores devem ter o mesmo tamanho e não estar vazios"
    );
    let sample_size_f64 = sample_size as f64;

    let total_squared_error: f64 = actual_values
        .iter()
        .zip(predicted_values.iter())
        .map(|(&actual, &predicted)| (actual - predicted).powi(2))
        .sum();

    total_squared_error / sample_size_f64
}

pub fn r2_score(actual_values: &[f64], predicted_values: &[f64]) -> f64 {
    let sample_size = actual_values.len();
    assert!(
        sample_size > 0 && sample_size == predicted_values.len(),
        "Vetores devem ter o mesmo tamanho e não estar vazios"
    );
    let sample_size_f64 = sample_size as f64;

    let mean_actual: f64 = actual_values.iter().sum::<f64>() / sample_size_f64;

    let total_sum_of_squares: f64 = actual_values
        .iter()
        .map(|&actual| (actual - mean_actual).powi(2))
        .sum();

    let residual_sum_of_squares: f64 = actual_values
        .iter()
        .zip(predicted_values.iter())
        .map(|(&actual, &predicted)| (actual - predicted).powi(2))
        .sum();

    1.0 - (residual_sum_of_squares / total_sum_of_squares)
}
