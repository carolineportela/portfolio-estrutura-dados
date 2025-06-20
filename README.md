# portfolio-estrutura-dados


Portfólio em Rust: Implementação de Regressão Linear Pura para Séries Temporais

---

O **TimeWise Regression** é uma biblioteca em Rust para análise de séries temporais via regressão linear simples “pura” (sem crates estatísticos externos). Ela permite:

- Ajustar uma reta (mínimos quadrados) aos dados históricos de uma variável no tempo.  
- Calcular métricas de qualidade do ajuste (MSE e R²).  
- Prever valores futuros a partir dos coeficientes da reta ajustada.

Destina-se a analistas de dados e cientistas que precisam de uma solução leve e eficiente para previsões básicas.

---

## Tecnologias

- **Rust (2021/2024 edition)**  
- Crates utilizadas:  
  - `anyhow` (tratamento de erros)  
  - `clap` (opcional, se usar CLI)  
- Sem dependências externas para cálculo estatístico: toda a lógica é “pura” Rust.  

---

## Como clonar e compilar

```bash
# Clone o repositório e entre na pasta
git clone https://github.com/carolineportela/portfolio-estrutura-dados.git
cd portfolio-estrutura-dados

# Compile o projeto
cargo build

# Execute todos os testes
cargo test

# Rode o exemplo/demo
cargo run
