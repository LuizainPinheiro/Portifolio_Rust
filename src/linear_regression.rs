// linear_regression.rs

use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub fn ler_csv(caminho_arquivo: &str) -> Result<(Vec<f64>, Vec<f64>), String> {
    let arquivo = match File::open(caminho_arquivo) {
        Ok(f) => f,
        Err(e) => return Err(format!("Erro ao abrir o arquivo '{}': {}", caminho_arquivo, e)),
    };

    let leitor = BufReader::new(arquivo);
    let mut tempo = Vec::new();
    let mut valores = Vec::new();
    let mut primeira_linha = true; // Para ignorar o cabeçalho

    for linha_result in leitor.lines() {
        match linha_result {
            Ok(linha) => {
                if primeira_linha {
                    primeira_linha = false;
                    continue; // Ignora a linha de cabeçalho
                }

                let partes: Vec<&str> = linha.split(',').collect();
                if partes.len() != 2 {
                    return Err(format!("Formato de linha inválido em '{}': Esperava 2 colunas, encontrou {}", caminho_arquivo, partes.len()));
                }

                match partes[0].trim().parse::<f64>() {
                    Ok(t) => tempo.push(t),
                    Err(_) => return Err(format!("Erro ao converter o valor de tempo '{}' para número em '{}'", partes[0].trim(), caminho_arquivo)),
                }

                match partes[1].trim().parse::<f64>() {
                    Ok(v) => valores.push(v),
                    Err(_) => return Err(format!("Erro ao converter o valor da série '{}' para número em '{}'", partes[1].trim(), caminho_arquivo)),
                }
            }
            Err(e) => return Err(format!("Erro ao ler uma linha do arquivo '{}': {}", caminho_arquivo, e)),
        }
    }

    if tempo.is_empty() {
        return Err(format!("Nenhum dado válido encontrado no arquivo '{}'", caminho_arquivo));
    }

    Ok((tempo, valores))
}

pub fn calcular_media(data: &[f64]) -> Result<f64, String> {
    if data.is_empty() {
        return Err("Não é possível calcular a média de um array vazio".to_string());
    }
    let sum: f64 = data.iter().sum();
    Ok(sum / (data.len() as f64))
}

pub fn regressao_linear(x: &[f64], y: &[f64]) -> Result<(f64, f64), String> {
    let n = x.len();
    if n != y.len() {
        return Err("Os arrays 'x' e 'y' devem ter o mesmo comprimento".to_string());
    }
    if n < 2 {
        return Err("São necessários pelo menos dois pontos de dados para realizar a regressão linear".to_string());
    }

    match calcular_media(x) {
        Ok(media_x) => {
            match calcular_media(y) {
                Ok(media_y) => {
                    let mut numerador = 0.0;
                    let mut denominador = 0.0;

                    for i in 0..n {
                        numerador += (x[i] - media_x) * (y[i] - media_y);
                        denominador += (x[i] - media_x).powi(2);
                    }

                    if denominador.abs() < f64::EPSILON {
                        return Err("Não é possível calcular a regressão linear (variância de x é zero)".to_string());
                    }

                    let b1 = numerador / denominador; // Inclinação
                    let b0 = media_y - b1 * media_x;   // Intercepto

                    Ok((b0, b1))
                }
                Err(erro) => Err(erro),
            }
        }
        Err(erro) => Err(erro),
    }
}

pub fn calcular_r_quadrado(y_real: &[f64], y_previsto: &[f64]) -> Result<f64, String> {
    let n = y_real.len();
    if n != y_previsto.len() {
        return Err("Os arrays 'y_real' e 'y_previsto' devem ter o mesmo comprimento".to_string());
    }
    if n < 2 {
        return Err("São necessários pelo menos dois pontos de dados para calcular o R²".to_string());
    }

    match calcular_media(y_real) {
        Ok(media_y) => {
            let mut sq_total = 0.0;
            let mut sq_erro = 0.0;

            for i in 0..n {
                sq_total += (y_real[i] - media_y).powi(2);
                sq_erro += (y_real[i] - y_previsto[i]).powi(2);
            }

            if sq_total.abs() < f64::EPSILON {
                if sq_erro.abs() < f64::EPSILON {
                    return Ok(1.0); 
                } else {
                    return Err("Não é possível calcular o R² (variância de y_real é zero e erro não é zero)".to_string());
                }
            }

            Ok(1.0 - (sq_erro / sq_total))
        }
        Err(erro) => Err(erro),
    }
}

pub fn calcular_mse(y_real: &[f64], y_previsto: &[f64]) -> Result<f64, String> {
    let n = y_real.len();
    if n != y_previsto.len() {
        return Err("Os arrays 'y_real' e 'y_previsto' devem ter o mesmo comprimento".to_string());
    }
    if n == 0 {
        return Err("Os arrays 'y_real' e 'y_previsto' não podem estar vazios".to_string());
    }

    let mut soma_erros_quadrados = 0.0;
    for i in 0..n {
        soma_erros_quadrados += (y_real[i] - y_previsto[i]).powi(2);
    }

    Ok(soma_erros_quadrados / (n as f64))
}

pub fn prever(x: f64, b0: f64, b1: f64) -> f64 {
    b0 + b1 * x
}

#[cfg(test)]
mod tests_regressao {
    use super::*;

    #[test]
    fn test_regressao_tamanhos_diferentes() {
        let x = [1.0, 2.0, 3.0];
        let y = [4.0, 5.0];
        let resultado = regressao_linear(&x, &y);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Os arrays 'x' e 'y' devem ter o mesmo comprimento");
    }

    #[test]
    fn test_regressao_poucos_pontos() {
        let x = [1.0];
        let y = [2.0];
        let resultado = regressao_linear(&x, &y);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "São necessários pelo menos dois pontos de dados para realizar a regressão linear");
    }

    #[test]
    fn test_regressao_variancia_zero() {
        let x = [2.0, 2.0, 2.0];
        let y = [1.0, 2.0, 3.0];
        let resultado = regressao_linear(&x, &y);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Não é possível calcular a regressão linear (variância de x é zero)");
    }

    #[test]
    fn test_regressao_simples() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 5.0, 4.0, 5.0];
        let resultado = regressao_linear(&x, &y);
        assert!(resultado.is_ok());
        let (b0, b1) = resultado.unwrap();
        assert!((b0 - 2.2).abs() < 0.001);
        assert!((b1 - 0.6).abs() < 0.001);
    }
}

#[cfg(test)]
mod tests_media {
    use super::*;

    #[test]
    fn test_media_vazio() {
        let data = [];
        let resultado = calcular_media(&data);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Não é possível calcular a média de um array vazio");
    }

    #[test]
    fn test_media_simples() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let resultado = calcular_media(&data);
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), 3.0);
    }
}

#[cfg(test)]
mod tests_metricas {
    use super::*;

    #[test]
    fn test_r_quadrado_tamanhos_diferentes() {
        let y_real = [1.0, 2.0, 3.0];
        let y_previsto = [1.1, 1.9];
        let resultado = calcular_r_quadrado(&y_real, &y_previsto);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Os arrays 'y_real' e 'y_previsto' devem ter o mesmo comprimento");
    }

    #[test]
    fn test_r_quadrado_poucos_pontos() {
        let y_real = [1.0];
        let y_previsto = [1.1];
        let resultado = calcular_r_quadrado(&y_real, &y_previsto);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "São necessários pelo menos dois pontos de dados para calcular o R²");
    }

    #[test]
    fn test_r_quadrado_variancia_zero() {
        let y_real = [2.0, 2.0, 2.0];
        let y_previsto = [2.0, 2.0, 2.0];
        let resultado = calcular_r_quadrado(&y_real, &y_previsto);
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap(), 1.0);
    }

    #[test]
    fn test_r_quadrado_simples() {
        let y_real = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y_previsto = [1.1, 1.9, 3.2, 3.8, 5.1];
        let resultado = calcular_r_quadrado(&y_real, &y_previsto);
        assert!(resultado.is_ok());
        assert!((resultado.unwrap() - 0.98).abs() < 0.01);
    }

    #[test]
    fn test_mse_tamanhos_diferentes() {
        let y_real = [1.0, 2.0, 3.0];
        let y_previsto = [1.1, 1.9];
        let resultado = calcular_mse(&y_real, &y_previsto);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Os arrays 'y_real' e 'y_previsto' devem ter o mesmo comprimento");
    }

    #[test]
    fn test_mse_vazio() {
        let y_real = [];
        let y_previsto = [];
        let resultado = calcular_mse(&y_real, &y_previsto);
        assert!(resultado.is_err());
        assert_eq!(resultado.unwrap_err(), "Os arrays 'y_real' e 'y_previsto' não podem estar vazios");
    }

    #[test]
    fn test_mse_simples() {
        let y_real = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y_previsto = [1.1, 1.9, 3.2, 3.8, 5.1];
        let resultado = calcular_mse(&y_real, &y_previsto);
        assert!(resultado.is_ok());
        assert!((resultado.unwrap() - 0.022).abs() < 0.002); 
    }
}

#[cfg(test)]
mod tests_previsao {
    use super::*;

    #[test]
    fn test_previsao() {
        let intercepto = 2.0;
        let inclinacao = 0.5;
        let x_futuro = 10.0;
        let previsao = prever(x_futuro, intercepto, inclinacao);
        assert_eq!(previsao, 7.0);
    }
}