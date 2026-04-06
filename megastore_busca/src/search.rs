use crate::index::ProductIndex;
use crate::graph::RecommendationGraph;
use crate::product::Product;

/// Motor de busca principal que integra índices e recomendações.
pub struct SearchEngine {
    index: ProductIndex,
    graph: RecommendationGraph,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            index: ProductIndex::new(),
            graph: RecommendationGraph::new(),
        }
    }

    /// Adiciona um produto ao catálogo e atualiza o grafo de recomendações.
    /// Para eficiência, reconstruímos o grafo somente quando necessário.
    /// Aqui optamos por reconstruir após cada inserção (simplificação).
    pub fn add_product(&mut self, product: Product) {
        self.index.insert(product);
        self.rebuild_recommendation_graph();
    }

    /// Adiciona vários produtos de uma vez.
    pub fn add_products(&mut self, products: Vec<Product>) {
        for product in products {
            self.index.insert(product);
        }
        self.rebuild_recommendation_graph();
    }

    /// Reconstrói o grafo baseado nos produtos atuais.
    fn rebuild_recommendation_graph(&mut self) {
        let all_prods = self.index.all_products();
        self.graph.build_from_products(&all_prods);
    }

    /// Busca exata por ID.
    pub fn search_by_id(&self, id: u64) -> Option<&Product> {
        self.index.get_by_id(id)
    }

    /// Busca por nome (normalizado).
    pub fn search_by_name(&self, name: &str) -> Vec<&Product> {
        self.index.get_by_name(name)
    }

    /// Busca por marca.
    pub fn search_by_brand(&self, brand: &str) -> Vec<&Product> {
        self.index.get_by_brand(brand)
    }

    /// Busca por categoria.
    pub fn search_by_category(&self, category: &str) -> Vec<&Product> {
        self.index.get_by_category(category)
    }

    /// Busca combinada: produtos que atendem a todos os critérios fornecidos (interseção).
    /// Recebe opções para cada campo.
    pub fn search_combined(
        &self,
        name: Option<&str>,
        brand: Option<&str>,
        category: Option<&str>,
    ) -> Vec<&Product> {
        let mut results: Option<HashSet<u64>> = None;

        if let Some(n) = name {
            let ids: HashSet<u64> = self.index.get_by_name(n).iter().map(|p| p.id).collect();
            results = Some(match results {
                Some(set) => set.intersection(&ids).copied().collect(),
                None => ids,
            });
        }
        if let Some(b) = brand {
            let ids: HashSet<u64> = self.index.get_by_brand(b).iter().map(|p| p.id).collect();
            results = Some(match results {
                Some(set) => set.intersection(&ids).copied().collect(),
                None => ids,
            });
        }
        if let Some(c) = category {
            let ids: HashSet<u64> = self.index.get_by_category(c).iter().map(|p| p.id).collect();
            results = Some(match results {
                Some(set) => set.intersection(&ids).copied().collect(),
                None => ids,
            });
        }

        match results {
            Some(ids) => ids
                .iter()
                .filter_map(|id| self.index.get_by_id(*id))
                .collect(),
            None => vec![],
        }
    }

    /// Recomenda produtos baseados em um produto de referência (por ID).
    pub fn recommend_for_product(&self, product_id: u64, top_n: usize) -> Vec<&Product> {
        let neighbor_ids = self.graph.recommend(product_id, top_n);
        neighbor_ids
            .iter()
            .filter_map(|id| self.index.get_by_id(*id))
            .collect()
    }

    /// Recomenda produtos baseados em um termo de busca: primeiro encontra produtos
    /// pelo nome, depois recomenda a partir do primeiro resultado.
    pub fn recommend_by_search_term(&self, term: &str, top_n: usize) -> Vec<&Product> {
        let matches = self.search_by_name(term);
        if let Some(first) = matches.first() {
            self.recommend_for_product(first.id, top_n)
        } else {
            vec![]
        }
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// Usamos HashSet para operações de interseção na busca combinada
use std::collections::HashSet;