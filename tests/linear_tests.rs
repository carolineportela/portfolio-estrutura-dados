use timewise_regression::linear_regression;

#[test]
fn perfect_line_fit() {
    let x_samples = [1.0, 2.0, 3.0];
    let y_samples = [2.0, 4.0, 6.0]; 

    let (intercept, slope) = linear_regression(&x_samples, &y_samples)
        .expect("linear_regression falhou em dados válidos");

    assert!((intercept - 0.0).abs() < 1e-8, "Intercepto inesperado");
    assert!((slope - 2.0).abs() < 1e-8, "Slope inesperado");
}

#[test]
fn error_on_empty_input() {
    let err = linear_regression(&[], &[]).unwrap_err();
    assert!(
        err.contains("não podem estar vazios"),
        "Esperava erro por input vazio, mas obteve: {}",
        err
    );
}

#[test]
fn error_on_zero_variance() {
    let x = [1.0, 1.0, 1.0];
    let y = [2.0, 4.0, 6.0];
    let err = linear_regression(&x, &y).unwrap_err();
    assert!(
        err.contains("Variância zero"),
        "Esperava erro por variância zero, mas obteve: {}",
        err
    );
}
