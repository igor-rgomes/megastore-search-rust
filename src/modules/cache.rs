use std::collections::HashMap;

pub struct Cache {
    store: HashMap<String, Vec<usize>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, query: &str) -> Option<&Vec<usize>> {
        self.store.get(query)
    }

    pub fn insert(&mut self, query: String, result: Vec<usize>) {
        self.store.insert(query, result);
    }

    pub fn contains(&self, query: &str) -> bool {
        self.store.contains_key(query)
    }
}
