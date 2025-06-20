pub fn predict(intercept: f64, slope: f64, future_x: &[f64]) -> Vec<f64> {
    future_x.iter().map(|&x_val| intercept + slope * x_val).collect()
}
