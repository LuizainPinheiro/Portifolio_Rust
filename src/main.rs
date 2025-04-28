// main.rs

mod linear_regression;

fn main() {
    println!("Iniciando análise de séries temporais com regressão linear...");

    let caminho_arquivo = "dados.csv"; 

    match linear_regression::ler_csv(caminho_arquivo) {
        Ok((tempo, valores)) => {
            println!("Dados lidos do arquivo '{}':", caminho_arquivo);
            println!("Tempo (x): {:?}", tempo);
            println!("Valores (y): {:?}", valores);

            // Calcular os coeficientes da regressão linear
            match linear_regression::regressao_linear(&tempo, &valores) {
                Ok((intercepto, inclinacao)) => {
                    println!("\nModelo de regressão linear:");
                    println!("  Intercepto (b0): {}", intercepto);
                    println!("  Inclinação (b1): {}", inclinacao);

                    // Calcular os valores previstos
                    let valores_previstos: Vec<f64> = tempo.iter().map(|&t| linear_regression::prever(t, intercepto, inclinacao)).collect();
                    println!("Valores Previstos (ŷ): {:?}", valores_previstos);

                    // Calcular R²
                    match linear_regression::calcular_r_quadrado(&valores, &valores_previstos) {
                        Ok(r_quadrado) => println!("Coeficiente de Determinação (R²): {:.4}", r_quadrado),
                        Err(erro) => println!("Erro ao calcular R²: {}", erro),
                    }

                    // Calcular MSE
                    match linear_regression::calcular_mse(&valores, &valores_previstos) {
                        Ok(mse) => println!("Erro Quadrático Médio (MSE): {:.4}", mse),
                        Err(erro) => println!("Erro ao calcular MSE: {}", erro),
                    }

                    // Fazer uma previsão para um novo valor de tempo
                    let tempo_futuro = tempo.last().map(|t| t + 1.0).unwrap_or(1.0); // Próximo período
                    let previsao_futura = linear_regression::prever(tempo_futuro, intercepto, inclinacao);
                    println!("\nPrevisão para tempo = {:.1}: {:.2}", tempo_futuro, previsao_futura);
                }
                Err(erro) => {
                    println!("Erro ao calcular a regressão linear: {}", erro);
                }
            }
        }
        Err(erro) => {
            println!("Erro ao ler os dados do arquivo: {}", erro);
        }
    }
}