mod linear;
mod predict;

pub use linear::{linear_regression, mean_squared_error, r2_score};
pub use predict::predict;
