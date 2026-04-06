use std::hash::{Hash, Hasher};

/// Representa um produto do catálogo.
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub brand: String,
    pub category: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u64, name: &str, description: &str, brand: &str, category: &str, price: f64) -> Self {
        Product {
            id,
            name: name.to_string(),
            description: description.to_string(),
            brand: brand.to_string(),
            category: category.to_string(),
            price,
        }
    }
}

// Implementação de Hash para permitir uso em HashMap (se necessário)
impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}