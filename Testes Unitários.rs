// src/lib.rs (continuação)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 3.0, 5.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        assert!((model.slope - 1.5).abs() < 1e-5);
        assert!((model.intercept - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_predict() {
        let model = LinearRegression { slope: 1.5, intercept: 0.5 };
        let prediction = model.predict(4.0);
        assert!((prediction - 6.5).abs() < 1e-5);
    }

    #[test]
    fn test_r_squared() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 3.0, 5.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        let r2 = model.r_squared(&x, &y).unwrap();
        assert!((r2 - 0.8666666666666667).abs() < 1e-5);
    }

    #[test]
    fn test_mean_squared_error() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![2.0, 3.0, 5.0];
        let model = LinearRegression::fit(&x, &y).unwrap();
        let mse = model.mean_squared_error(&x, &y).unwrap();
        assert!((mse - 0.3333333333333333).abs() < 1e-5);
    }
}