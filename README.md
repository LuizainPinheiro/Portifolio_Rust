# Portifolio_Rust
Portfólio em Rust: Implementação de Regressão Linear Pura para Séries Temporais

Este projeto implementa uma função de regressão linear simples para analisar tendências lineares em dados de séries temporais utilizando a linguagem de programação Rust.

## Visão Geral

O objetivo principal deste projeto é fornecer uma biblioteca Rust que permita:

* Ler dados de séries temporais a partir de um arquivo CSV (`dados.csv`).
* Calcular os coeficientes de uma reta de regressão linear (intercepto e inclinação) para modelar a relação entre o tempo e os valores da série.
* Avaliar a qualidade do modelo de regressão utilizando as métricas de Coeficiente de Determinação (R²) e Erro Quadrático Médio (MSE).
* Realizar previsões de valores futuros com base no modelo de regressão ajustado.

Este projeto foi desenvolvido como um trabalho individual para a disciplina [Nome da Disciplina] sob as seguintes diretrizes:

* Utilização do GitHub para versionamento e entrega.
* Implementação das funcionalidades em Rust, sem o uso de crates externas para os cálculos principais.

## Funcionalidades Implementadas

### `linear_regression.rs`

Este arquivo contém a implementação das seguintes funções:

* `ler_csv(caminho_arquivo: &str) -> Result<(Vec<f64>, Vec<f64
