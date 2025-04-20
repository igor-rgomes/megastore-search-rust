use megastore_search_rust::modules::{index::*, search::*};

#[test]
fn test_busca_por_termo_simples() {
    let produtos = vec![
        Product {
            id: 1,
            name: "Notebook Dell".to_string(),
            brand: "Dell".to_string(),
            category: "Informática".to_string(),
            description: "Notebook potente com SSD".to_string(),
        },
        Product {
            id: 2,
            name: "Camiseta Azul".to_string(),
            brand: "Hering".to_string(),
            category: "Vestuário".to_string(),
            description: "Camiseta básica de algodão".to_string(),
        },
    ];

    let index = Index::new(produtos.clone());
    let resultados = search("notebook", &index.inverted_index);

    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0], 1);
}

#[test]
fn test_busca_sem_resultado() {
    let produtos = vec![
        Product {
            id: 1,
            name: "Notebook Dell".to_string(),
            brand: "Dell".to_string(),
            category: "Informática".to_string(),
            description: "Notebook potente com SSD".to_string(),
        },
    ];

    let index = Index::new(produtos);
    let resultados = search("geladeira", &index.inverted_index);

    assert!(resultados.is_empty());
}

#[test]
fn test_busca_com_multiplos_resultados() {
    let produtos = vec![
        Product {
            id: 1,
            name: "Notebook Dell".to_string(),
            brand: "Dell".to_string(),
            category: "Informática".to_string(),
            description: "Notebook potente com SSD".to_string(),
        },
        Product {
            id: 2,
            name: "Notebook Lenovo".to_string(),
            brand: "Lenovo".to_string(),
            category: "Informática".to_string(),
            description: "Leve e rápido".to_string(),
        },
    ];

    let index = Index::new(produtos);
    let resultados = search("notebook", &index.inverted_index);

    assert_eq!(resultados.len(), 2);
    assert!(resultados.contains(&1));
    assert!(resultados.contains(&2));
}
