// src/lib.rs

pub struct LinearRegression {
    slope: f64,
    intercept: f64,
}

impl LinearRegression {
    // Ajusta o modelo de regressão linear aos dados
    pub fn fit(x: &[f64], y: &[f64]) -> Result<LinearRegression, &'static str> {
        if x.len() == 0 || y.len() == 0 || x.len() != y.len() {
            return Err("Os arrays de entrada devem ter o mesmo tamanho e não podem estar vazios.");
        }

        let n = x.len() as f64;
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
        let sum_x_squared: f64 = x.iter().map(|xi| xi * xi).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x_squared - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        Ok(LinearRegression { slope, intercept })
    }

    // Realiza previsões com base nos coeficientes ajustados
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    // Calcula o coeficiente de determinação (R²)
    pub fn r_squared(&self, x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
        if x.len() == 0 || y.len() == 0 || x.len() != y.len() {
            return Err("Os arrays de entrada devem ter o mesmo tamanho e não podem estar vazios.");
        }

        let mean_y: f64 = y.iter().sum::<f64>() / y.len() as f64;
        let ss_total: f64 = y.iter().map(|yi| (yi - mean_y).powi(2)).sum();
        let ss_residual: f64 = y.iter().enumerate().map(|(i, yi)| {
            let predicted = self.predict(x[i]);
            (yi - predicted).powi(2)
        }).sum();

        Ok(1.0 - (ss_residual / ss_total))
    }

    // Calcula o erro quadrático médio (MSE)
    pub fn mean_squared_error(&self, x: &[f64], y: &[f64]) -> Result<f64, &'static str> {
        if x.len() == 0 || y.len() == 0 || x.len() != y.len() {
            return Err("Os arrays de entrada devem ter o mesmo tamanho e não podem estar vazios.");
        }

        let mse: f64 = x.iter().enumerate().map(|(i, xi)| {
            let predicted = self.predict(*xi);
            (y[i] - predicted).powi(2)
        }).sum::<f64>() / x.len() as f64;

        Ok(mse)
    }
}