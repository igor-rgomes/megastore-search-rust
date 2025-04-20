use std::collections::HashMap;

pub fn search(query: &str, index: &HashMap<String, Vec<usize>>) -> Vec<usize> {
    let mut result_counter: HashMap<usize, usize> = HashMap::new();
    let lower = query.to_lowercase();
    let tokens = lower.split_whitespace();

    for token in tokens {
        if let Some(ids) = index.get(token) {
            for id in ids {
                *result_counter.entry(*id).or_insert(0) += 1;
            }
        }
    }

    let mut results: Vec<(usize, usize)> = result_counter.into_iter().collect();
    results.sort_by(|a, b| b.1.cmp(&a.1));
    results.into_iter().map(|(id, _)| id).collect()
}
