use std::collections::HashMap;

pub fn search(query: &str, index: &HashMap<String, Vec<usize>>) -> Vec<usize> {
    let lower = query.to_lowercase();
    let tokens = lower.split_whitespace();

    let mut scores: HashMap<usize, usize> = HashMap::new();

    for token in tokens {
        if let Some(ids) = index.get(token) {
            for id in ids {
                *scores.entry(*id).or_insert(0) += 1;
            }
        }
    }

    let mut results: Vec<(usize, usize)> = scores.into_iter().collect();
    results.sort_by(|a, b| b.1.cmp(&a.1));

    results.into_iter().map(|(id, _)| id).collect()
}
