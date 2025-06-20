use timewise_regression::{linear_regression, mean_squared_error, predict, r2_score};

fn main() {

    let training_x = [1.0, 2.0, 3.0, 4.0];
    let training_y = [2.1, 3.9, 6.0, 7.8];

    let (intercept, slope) = linear_regression(&training_x, &training_y)
        .expect("Falha ao ajustar regressão: verifique os dados de entrada");

    println!("Modelo ajustado: y = {:+.4} + {:+.4} x", intercept, slope);

    let future_x = [5.0, 6.0];
    let predictions = predict(intercept, slope, &future_x);
    println!("Previsões para {:?}: {:?}", future_x, predictions);
    
    let fitted_y: Vec<f64> = training_x.iter().map(|&x| intercept + slope * x).collect();
    let mse_value = mean_squared_error(&training_y, &fitted_y);
    let r2_value = r2_score(&training_y, &fitted_y);
    println!("MSE no treinamento = {:.4}", mse_value);
    println!("R² no treinamento = {:.4}", r2_value);
}
