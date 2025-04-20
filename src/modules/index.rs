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
    pub inverted_index: HashMap<String, Vec<usize>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            inverted_index: HashMap::new(),
        }
    }

    pub fn index_products(&mut self, products: &[Product]) {
        for product in products {
            let combined = format!("{} {} {}", product.name, product.brand, product.category);
            let tokens = combined
                .to_lowercase()
                .split_whitespace()
                .map(|s| s.to_string());

            for token in tokens {
                self.inverted_index
                    .entry(token)
                    .or_default()
                    .push(product.id);
            }
        }
    }
}