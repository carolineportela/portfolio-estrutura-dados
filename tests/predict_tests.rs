use timewise_regression::{linear_regression, predict};

#[test]
fn predict_future_values_correctly() {
    let train_x = [1.0, 2.0, 3.0];
    let train_y = [2.0, 4.0, 6.0];

    let (intercept, slope) = linear_regression(&train_x, &train_y)
        .expect("Failed to compute regression coefficients");

    let future_x = [10.0, 20.0];
    let forecasted_y = predict(intercept, slope, &future_x);

    assert_eq!(
        forecasted_y,
        vec![20.0, 40.0],
        "Previsões incorretas: {:?}",
        forecasted_y
    );
}
