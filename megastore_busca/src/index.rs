use std::collections::HashMap;
use crate::product::Product;

/// Índices baseados em tabelas hash para acesso rápido por diferentes chaves.
pub struct ProductIndex {
    by_id: HashMap<u64, Product>,
    by_name: HashMap<String, Vec<u64>>,      // nome normalizado -> lista de IDs
    by_brand: HashMap<String, Vec<u64>>,
    by_category: HashMap<String, Vec<u64>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        ProductIndex {
            by_id: HashMap::new(),
            by_name: HashMap::new(),
            by_brand: HashMap::new(),
            by_category: HashMap::new(),
        }
    }

    /// Adiciona um produto a todos os índices.
    pub fn insert(&mut self, product: Product) {
        let id = product.id;
        // Normalização: minúsculas e remove pontuação simples
        let name_key = Self::normalize(&product.name);
        let brand_key = Self::normalize(&product.brand);
        let category_key = Self::normalize(&product.category);

        self.by_id.insert(id, product);
        self.by_name.entry(name_key).or_insert_with(Vec::new).push(id);
        self.by_brand.entry(brand_key).or_insert_with(Vec::new).push(id);
        self.by_category.entry(category_key).or_insert_with(Vec::new).push(id);
    }

    /// Busca por ID – O(1) esperado.
    pub fn get_by_id(&self, id: u64) -> Option<&Product> {
        self.by_id.get(&id)
    }

    /// Busca por nome (prefixo ou exato? Aqui exato após normalização).
    pub fn get_by_name(&self, name: &str) -> Vec<&Product> {
        let key = Self::normalize(name);
        self.by_name
            .get(&key)
            .map(|ids| ids.iter().filter_map(|id| self.by_id.get(id)).collect())
            .unwrap_or_default()
    }

    /// Busca por marca.
    pub fn get_by_brand(&self, brand: &str) -> Vec<&Product> {
        let key = Self::normalize(brand);
        self.by_brand
            .get(&key)
            .map(|ids| ids.iter().filter_map(|id| self.by_id.get(id)).collect())
            .unwrap_or_default()
    }

    /// Busca por categoria.
    pub fn get_by_category(&self, category: &str) -> Vec<&Product> {
        let key = Self::normalize(category);
        self.by_category
            .get(&key)
            .map(|ids| ids.iter().filter_map(|id| self.by_id.get(id)).collect())
            .unwrap_or_default()
    }

    /// Retorna todos os produtos (para iteração).
    pub fn all_products(&self) -> Vec<&Product> {
        self.by_id.values().collect()
    }

    /// Normalização simples: minúsculas e remove caracteres não alfanuméricos.
    fn normalize(s: &str) -> String {
        s.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == ' ')
            .collect()
    }
}

impl Default for ProductIndex {
    fn default() -> Self {
        Self::new()
    }
}