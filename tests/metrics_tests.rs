use timewise_regression::{mean_squared_error, r2_score};

#[test]
fn mse_and_r2_are_correct() {
    let actual_values = [1.0, 2.0, 3.0, 4.0];
    let predicted_values = [0.9, 2.1, 2.9, 4.1];

    let mse_value = mean_squared_error(&actual_values, &predicted_values);
    let r2_value = r2_score(&actual_values, &predicted_values);

    assert!(mse_value < 0.02, "MSE está alto: {}", mse_value);
    assert!(r2_value > 0.95, "R² está baixo: {}", r2_value);
}
