use std::collections::HashMap;

pub struct Graph {
    adjacencia: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacencia: HashMap::new(),
        }
    }

    pub fn adicionar_recomendacao(&mut self, produto: &str, recomendado: &str) {
        self.adjacencia
            .entry(produto.to_string())
            .or_insert(Vec::new())
            .push(recomendado.to_string());
    }

    pub fn recomendar(&self, produto: &str) -> Option<&Vec<String>> {
        self.adjacencia.get(produto)
    }
}
