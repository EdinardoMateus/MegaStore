use megastore_search::{Product, SearchEngine};

#[test]
fn test_insert_and_search_by_id() {
    let mut engine = SearchEngine::new();
    let p = Product::new(42, "Teste", "Desc", "Marca", "Cat", 10.0);
    engine.add_product(p);
    let found = engine.search_by_id(42);
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "Teste");
}

#[test]
fn test_search_by_name_normalization() {
    let mut engine = SearchEngine::new();
    let p = Product::new(1, "Café Especial!", "Grãos", "CaféCo", "Alimentos", 25.0);
    engine.add_product(p);
    let results = engine.search_by_name("café especial");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, 1);
}

#[test]
fn test_combined_search() {
    let mut engine = SearchEngine::new();
    engine.add_products(vec![
        Product::new(1, "TV 4K", "Tela grande", "Samsung", "Eletrônicos", 2000.0),
        Product::new(2, "TV LED", "Economia", "LG", "Eletrônicos", 1200.0),
        Product::new(3, "Smartphone", "5G", "Samsung", "Eletrônicos", 800.0),
    ]);
    let results = engine.search_combined(None, Some("Samsung"), Some("Eletrônicos"));
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|p| p.id == 1));
    assert!(results.iter().any(|p| p.id == 3));
}

#[test]
fn test_recommendation_graph() {
    let mut engine = SearchEngine::new();
    engine.add_products(vec![
        Product::new(10, "Prod A", "", "MarcaX", "Cat1", 100.0),
        Product::new(11, "Prod B", "", "MarcaX", "Cat1", 150.0),
        Product::new(12, "Prod C", "", "MarcaY", "Cat2", 200.0),
    ]);
    let recs = engine.recommend_for_product(10, 2);
    // Prod A e Prod B compartilham marca e categoria -> devem ser vizinhos
    assert_eq!(recs.len(), 1);
    assert_eq!(recs[0].id, 11);
}