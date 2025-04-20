use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub description: String,
}

pub struct Index {
    pub products: Vec<Product>,
    pub inverted_index: HashMap<String, Vec<usize>>,
}

impl Index {
    pub fn new(products: Vec<Product>) -> Self {
        let mut inverted_index: HashMap<String, Vec<usize>> = HashMap::new();

        for product in &products {
            let combined = format!("{} {} {}", product.name, product.brand, product.category);

            // ✅ CORREÇÃO ABAIXO
            let lower = combined.to_lowercase();
            let tokens = lower
                .split_whitespace()
                .map(|s| s.to_string());

            for token in tokens {
                inverted_index
                    .entry(token)
                    .or_insert_with(Vec::new)
                    .push(product.id);
            }
        }

        Index {
            products,
            inverted_index,
        }
    }
}
