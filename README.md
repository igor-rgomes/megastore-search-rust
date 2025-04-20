# 🔍 Sistema de Busca Otimizado - MegaStore 🦀

![Rust Version](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat&logo=rust)
![License: MIT](https://img.shields.io/badge/license-MIT-green)
![Status](https://img.shields.io/badge/status-finalizado-blue)

> Projeto acadêmico para disciplina **Data Structures Strategy and Implementation**, com foco em estruturas de dados eficientes para sistemas de recomendação e busca em catálogos extensos.

---

## 📦 Sobre o Projeto

Este sistema foi desenvolvido para simular um mecanismo de busca escalável para a MegaStore, utilizando:

- Índice invertido com `HashMap`
- Busca por relevância textual
- Cache de resultados para otimização
- Implementado com foco em desempenho e modularidade usando Rust 🦀

---

## 📂 Estrutura

```txt
├── src/
│   ├── lib.rs
│   └── modules/
│       ├── index.rs     # Define Product e Index
│       ├── search.rs    # Algoritmo de busca por token
│       └── cache.rs     # Cache opcional de resultados
├── tests/
│   └── search_tests.rs  # Testes unitários
└── Cargo.toml
