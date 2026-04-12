# MegaStore

# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição
Este projeto implementa um motor de busca e recomendação para o catálogo da MegaStore, utilizando **tabelas hash** para indexação rápida (O(1) esperado) e **grafos** para sugerir produtos relacionados. A solução é escrita em Rust, garantindo segurança, desempenho e escalabilidade.

## Tecnologias utilizadas
- **Rust 2021** (linguagem principal)
- Biblioteca padrão (`std::collections::HashMap`, `HashSet`)
- Testes: `cargo test`
- Geração de dados sintéticos: `rand` (apenas em testes)

## Como Executar 
- cargo run

## Como testar 
- cargo test     ##testes rápidos
- cargo test -- --ignored    ##testes de performance (opcional)

## Exemplo de uso
use megastore_busca::{Product, SearchEngine};

let mut engine = SearchEngine::new();
engine.add_product(Product::new(1, "Smartphone Edge 60", "5G", "Motorola", "Eletrônicos", 3799.99));
let results = engine.search_by_name("smartphone Edge 60");

## Arquitetura
- `Product`: dados do produto.
- `ProductIndex`: índices hash por ID, nome, marca, categoria.
- `RecommendationGraph`: grafo de adjacência para recomendações.
- `SearchEngine`: fachada principal.

## Desempenho
Busca O(1) médio. Testes com 100k produtos mostram latência < 100ms.



