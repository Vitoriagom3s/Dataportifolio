 #  Implementação de Regressão Linear Pura para Séries Temporais
![image](https://github.com/user-attachments/assets/547b32ed-b3e3-463b-8308-dad9dd3ea855)

## Descrição
Este projeto implementa um módulo de regressão linear pura em Rust, que pode ser utilizado para análise de séries temporais. O módulo inclui funções para ajustar um modelo de regressão linear, realizar previsões e calcular métricas de avaliação como R² e erro quadrático médio (MSE).

## Funcionalidades
- Ajuste de um modelo de regressão linear a um conjunto de dados.
- Previsões de valores futuros com base no modelo ajustado.
- Cálculo do coeficiente de determinação (R²) para avaliar a qualidade do ajuste.
- Cálculo do erro quadrático médio (MSE) para medir a precisão das previsões.

## Instalação

Para usar este projeto, você precisa ter o Rust instalado em sua máquina. Você pode instalar o Rust seguindo as instruções em [rust-lang.org](https://www.rust-lang.org/tools/install).


## Aqui está um exemplo de como usar o módulo de regressão linear:

use seu_modulo:LinearRegression;

fn main() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 3.0, 5.0];

    let model = LinearRegression::fit(&x, &y).unwrap();
    let prediction = model.predict(4.0);
    println!("Previsão para x=4.0: {}", prediction);

    let r_squared = model.r_squared(&x, &y).unwrap();
    println!("Coeficiente de determinação (R²): {}", r_squared);

    let mse = model.mean_squared_error(&x, &y).unwrap();
    println!("Erro quadrático médio (MSE): {}", mse);
}

## Para executar os testes, use o seguinte comando:
cargo test

Os testes incluem verificações para o ajuste do modelo, previsões, cálculo de R² e MSE.


## Explicação dos Testes
test_fit: Verifica se a função fit calcula corretamente a inclinação (slope) e o intercepto (intercept) do modelo de regressão linear.

test_predict: Verifica se a função predict retorna o valor esperado para um dado de entrada, utilizando os coeficientes ajustados.

test_r_squared: Verifica se a função r_squared calcula corretamente o coeficiente de determinação (R²) para um conjunto de dados.

test_mean_squared_error: Verifica se a função mean_squared_error calcula corretamente o erro quadrático médio (MSE) para um conjunto de dados.
