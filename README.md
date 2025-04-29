# Análise de Regressão Linear Simples em Séries Temporais com Rust

Este projeto implementa um algoritmo de regressão linear simples para analisar dados de séries temporais utilizando a linguagem de programação Rust. Ele permite ler dados de um arquivo CSV, calcular os coeficientes da reta de regressão, avaliar a qualidade do modelo através de métricas estatísticas (R² e MSE) e realizar previsões futuras.

## Estrutura do Projeto

O projeto é composto pelos seguintes arquivos principais:

-   `src/main.rs`: Ponto de entrada e orquestrador do programa. Responsável por ler os dados, chamar as funções de análise e exibir os resultados.
-   `src/linear_regression.rs`: Contém a implementação da lógica e dos algoritmos para realizar a análise de regressão linear, incluindo funções para leitura de dados, cálculo dos coeficientes, avaliação do modelo e previsão.
-   `dados.csv`: Arquivo de exemplo contendo os dados da série temporal no formato CSV (tempo, valor).

## Funcionalidades Implementadas

O módulo `linear_regression.rs` oferece as seguintes funcionalidades:

### `ler_csv(caminho_arquivo: &str) -> Result<(Vec<f64>, Vec<f64>), String>`

-   **Propósito:** Lê dados de séries temporais de um arquivo CSV.
-   **Parâmetros:**
    -   `caminho_arquivo`: Uma string slice (`&str`) representando o caminho para o arquivo CSV.
-   **Retorno:**
    -   `Result<(Vec<f64>, Vec<f64>), String>`:
        -   `Ok((Vec<f64>, Vec<f64>))`: Uma tupla contendo dois vetores de números de ponto flutuante de 64 bits. O primeiro vetor representa os pontos no tempo, e o segundo vetor representa os valores da série temporal correspondentes.
        -   `Err(String)`: Uma string descrevendo o erro ocorrido durante a leitura do arquivo (por exemplo, falha ao abrir o arquivo, formato de linha inválido, erro de conversão de tipo).
-   **Comportamento:** A função abre o arquivo CSV, lê linha por linha (ignorando a primeira linha como cabeçalho), separa os valores por vírgula e tenta converter cada parte para um número `f64`. Retorna um erro se o formato da linha for inválido ou se a conversão falhar.

### `calcular_media(data: &[f64]) -> Result<f64, String>`

-   **Propósito:** Calcula a média de um slice de números de ponto flutuante de 64 bits.
-   **Parâmetros:**
    -   `data`: Um slice (`&[f64]`) contendo os valores para calcular a média.
-   **Retorno:**
    -   `Result<f64, String>`:
        -   `Ok(f64)`: A média dos valores no slice.
        -   `Err(String)`: Uma string informando que não é possível calcular a média de um array vazio.

### `regressao_linear(x: &[f64], y: &[f64]) -> Result<(f64, f64), String>`

-   **Propósito:** Calcula os coeficientes (intercepto e inclinação) de uma regressão linear simples.
-   **Parâmetros:**
    -   `x`: Um slice (`&[f64]`) representando os valores da variável independente (tempo).
    -   `y`: Um slice (`&[f64]`) representando os valores da variável dependente (série temporal).
-   **Retorno:**
    -   `Result<(f64, f64), String>`:
        -   `Ok((f64, f64))`: Uma tupla contendo o intercepto (b0) e a inclinação (b1) da reta de regressão.
        -   `Err(String)`: Uma string descrevendo o erro ocorrido durante o cálculo (por exemplo, arrays com comprimentos diferentes, número insuficiente de pontos, variância de x igual a zero).
-   **Comportamento:** Implementa as fórmulas estatísticas para calcular o intercepto e a inclinação da reta de melhor ajuste aos dados fornecidos.

### `calcular_r_quadrado(y_real: &[f64], y_previsto: &[f64]) -> Result<f64, String>`

-   **Propósito:** Calcula o Coeficiente de Determinação (R²) para avaliar a qualidade do ajuste do modelo de regressão.
-   **Parâmetros:**
    -   `y_real`: Um slice (`&[f64]`) contendo os valores reais da variável dependente.
    -   `y_previsto`: Um slice (`&[f64]`) contendo os valores previstos pelo modelo de regressão.
-   **Retorno:**
    -   `Result<f64, String>`:
        -   `Ok(f64)`: O valor do R².
        -   `Err(String)`: Uma string descrevendo o erro ocorrido durante o cálculo (por exemplo, arrays com comprimentos diferentes, número insuficiente de pontos, variância de y_real igual a zero com erro não zero).

### `calcular_mse(y_real: &[f64], y_previsto: &[f64]) -> Result<f64, String>`

-   **Propósito:** Calcula o Erro Quadrático Médio (MSE) para quantificar a magnitude do erro entre os valores previstos e reais.
-   **Parâmetros:**
    -   `y_real`: Um slice (`&[f64]`) contendo os valores reais da variável dependente.
    -   `y_previsto`: Um slice (`&[f64]`) contendo os valores previstos pelo modelo de regressão.
-   **Retorno:**
    -   `Result<f64, String>`:
        -   `Ok(f64)`: O valor do MSE.
        -   `Err(String)`: Uma string descrevendo o erro ocorrido durante o cálculo (por exemplo, arrays com comprimentos diferentes, arrays vazios).

### `prever(x: f64, b0: f64, b1: f64) -> f64`

-   **Propósito:** Realiza uma previsão utilizando o modelo de regressão linear ajustado.
-   **Parâmetros:**
    -   `x`: O valor da variável independente (tempo) para o qual se deseja fazer a previsão.
    -   `b0`: O intercepto da reta de regressão.
    -   `b1`: A inclinação da reta de regressão.
-   **Retorno:**
    -   `f64`: O valor previsto da variável dependente.

## Como Executar

1.  **Certifique-se de ter o Rust instalado em seu sistema.** Você pode verificar a instalação executando `rustc --version` e `cargo --version` no seu terminal. Caso não tenha, siga as instruções em [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
2.  **Clone este repositório (se aplicável) ou navegue até o diretório do projeto.**
3.  **Execute o programa utilizando o comando:**

    ```bash
    cargo run
    ```

    Este comando irá compilar e executar o arquivo `src/main.rs`. Certifique-se de que o arquivo `dados.csv` esteja presente no diretório raiz do projeto ou ajuste o caminho do arquivo no `src/main.rs` conforme necessário.

## Testes Unitários

O projeto inclui testes unitários para garantir a confiabilidade de cada função no módulo `linear_regression.rs`. Para executar os testes, utilize o comando:

```bash
cargo test
