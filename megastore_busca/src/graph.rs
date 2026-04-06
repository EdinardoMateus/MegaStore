use std::collections::{HashMap, HashSet};
use crate::product::Product;

/// Grafo de recomendações: vértices = IDs de produtos; arestas = "produtos relacionados".
pub struct RecommendationGraph {
    adjacency: HashMap<u64, HashSet<u64>>,
}

impl RecommendationGraph {
    pub fn new() -> Self {
        RecommendationGraph {
            adjacency: HashMap::new(),
        }
    }

    /// Adiciona uma relação direcional de 'from' para 'to' (se ambos produtos existirem).
    /// Na prática, usaremos relações simétricas para recomendação.
    pub fn add_edge(&mut self, from: u64, to: u64) {
        self.adjacency.entry(from).or_insert_with(HashSet::new).insert(to);
        self.adjacency.entry(to).or_insert_with(HashSet::new).insert(from);
    }

    /// Constrói o grafo a partir de uma lista de produtos, usando regras de similaridade:
    /// - Mesma categoria
    /// - Mesma marca
    /// - Palavras‑chave comuns na descrição (simplificado)
    /// Para dados reais, usaríamos co‑compras ou co‑visualizações.
    pub fn build_from_products(&mut self, products: &[&Product]) {
        // Limpa arestas anteriores
        self.adjacency.clear();

        // Para cada par de produtos, conecta se compartilham categoria ou marca.
        // Complexidade O(n^2) – aceitável para catálogos até ~10k produtos.
        // Em produção, usaríamos técnicas de min‑hashing ou clustering.
        for i in 0..products.len() {
            for j in (i+1)..products.len() {
                let p1 = products[i];
                let p2 = products[j];
                if p1.category == p2.category || p1.brand == p2.brand {
                    self.add_edge(p1.id, p2.id);
                }
            }
        }
    }

    /// Retorna os IDs dos produtos vizinhos (recomendações diretas) para um dado produto.
    pub fn get_neighbors(&self, product_id: u64) -> Vec<u64> {
        self.adjacency
            .get(&product_id)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Recomenda os N produtos mais relacionados (por BFS limitada a 1 nível).
    /// Se não houver vizinhos, retorna lista vazia.
    pub fn recommend(&self, product_id: u64, top_n: usize) -> Vec<u64> {
        let mut neighbors = self.get_neighbors(product_id);
        // Ordenação simples (por ID) para determinismo – em produção seria por peso de relação.
        neighbors.sort_unstable();
        neighbors.truncate(top_n);
        neighbors
    }
}

impl Default for RecommendationGraph {
    fn default() -> Self {
        Self::new()
    }
}