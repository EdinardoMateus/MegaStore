use megastore::{Product, SearchEngine};

fn main() {
    // Exemplo de uso do sistema de busca e recomendação
    let mut engine = SearchEngine::new();

    // Inserindo alguns produtos de exemplo
    let products = vec![
        Product::new(1, "Smartphone Edge 60", "Alta resolução, 5G", "Motorola", "Eletrônicos", 3799.99),
        Product::new(2, "Smartphone Moto G86", "Bateria de longa duração", "Motorola", "Eletrônicos", 1899.99),
        Product::new(3, "Notebook Lenovo", "16GB RAM, 512GB SSD", "CompuTech", "Computadores", 3299.99),
        Product::new(4, "Fone Bluetooth", "Cancelamento de ruído", "JBL", "Acessórios", 199.99),
        Product::new(5, "Smartphone Moto G35", "Câmera quádrupla", "Motorola", "Eletrônicos", 1399.99),
        Product::new(6, "Capa protetora", "Para smartphone Edge 60", "Motorola", "Acessórios", 19.99),
    ];
    engine.add_products(products);

    // Buscas
    println!("=== Busca por nome 'Smartphone Edge 60' ===");
    for p in engine.search_by_name("Smartphone Edge 60") {
        println!("{:?}", p);
    }

    println!("\n=== Busca por marca 'Motorola' ===");
    for p in engine.search_by_brand("Motorola") {
        println!("{} - {}", p.name, p.price);
    }

    println!("\n=== Busca combinada: marca 'Motorola' e categoria 'Eletrônicos' ===");
    for p in engine.search_combined(None, Some("Motorola"), Some("Eletrônicos")) {
        println!("{}", p.name);
    }

    println!("\n=== Recomendações para produto ID 1 (Smartphone Edge 60) ===");
    for p in engine.recommend_for_product(1, 3) {
        println!("Recomendado: {} (categoria {})", p.name, p.category);
    }

    println!("\n=== Recomendações a partir da busca por 'Smartphone' ===");
    for p in engine.recommend_by_search_term("Smartphone", 3) {
        println!("Sugestão: {}", p.name);
    }
}