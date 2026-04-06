//! Biblioteca para sistema de busca e recomendação de produtos da MegaStore.
//! Utiliza tabelas hash para indexação rápida e grafos para recomendações.

pub mod product;
pub mod index;
pub mod graph;
pub mod search;

pub use product::Product;
pub use index::ProductIndex;
pub use graph::RecommendationGraph;
pub use search::SearchEngine;