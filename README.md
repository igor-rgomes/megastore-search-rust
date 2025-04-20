# 🔎 Sistema de Busca Otimizado - MegaStore 🦀

Projeto acadêmico desenvolvido para a disciplina **Data Structures Strategy and Implementation**, com foco em estruturas de dados eficientes para sistemas de **recomendação e busca** em catálogos extensos.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![license](https://img.shields.io/badge/license-MIT-brightgreen)
![status](https://img.shields.io/badge/status-finalizado-blue)

---

## 📦 Sobre o Projeto

Este sistema foi desenvolvido para simular um mecanismo de busca **escalável e performático** para o catálogo de produtos da MegaStore, utilizando:

- **Índice invertido com `HashMap`**
- **Busca por relevância textual**
- **Cache de resultados para otimização (opcional)**
- **Implementação segura e eficiente com Rust**

---

## 🧠 Tecnologias Utilizadas

- Linguagem: [Rust](https://www.rust-lang.org/)
- Gerenciador de pacotes: `cargo`
- Bibliotecas padrão: `std::collections::HashMap`

---

## 🗂 Estrutura de Pastas

```txt
src/
├── lib.rs               # Ponto de entrada do sistema de busca
└── modules/
    ├── index.rs         # Criação do índice invertido
    ├── search.rs        # Algoritmo de busca baseado em tokens
    └── cache.rs         # Sistema opcional de cache para buscas

tests/
└── search_tests.rs      # Testes unitários para busca e indexação

Cargo.toml               # Configuração do projeto Rust
