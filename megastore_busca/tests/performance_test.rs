use megastore::{Product, SearchEngine};

#[test]
#[ignore] // Ignorado por padrão porque gera muitos dados; execute com `cargo test -- --ignored`
fn test_large_catalog_performance() {
    let mut engine = SearchEngine::new();
    let num_products = 10_000;
    println!("Inserindo {} produtos...", num_products);
    for i in 0..num_products {
        let category = if i % 10 == 0 { "CatA" } else { "CatB" };
        let p = Product::new(i, &format!("Produto {}", i), "", "MarcaTeste", category, i as f64);
        engine.add_product(p);
    }
    // Teste de busca por nome
    let start = std::time::Instant::now();
    let results = engine.search_by_name("Produto 5000");
    let duration = start.elapsed();
    println!("Busca por nome exato: {:?} para {} resultados", duration, results.len());
    assert!(duration.as_millis() < 100); // Deve ser rápido mesmo com 100k itens

    // Teste de recomendação
    let start = std::time::Instant::now();
    let recs = engine.recommend_for_product(5000, 5);
    let duration = start.elapsed();
    println!("Recomendação: {:?} para {} resultados", duration, recs.len());
    assert!(duration.as_millis() < 200);
}