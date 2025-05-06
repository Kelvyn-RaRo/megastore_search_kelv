use std::collections::HashMap;

pub struct SearchEngine {
    index: HashMap<String, Vec<String>>,
}

impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            index: HashMap::new(),
        }
    }

    pub fn indexar(&mut self, termo: &str, produto: &str) {
        self.index
            .entry(termo.to_lowercase())
            .or_insert(Vec::new())
            .push(produto.to_string());
    }

    pub fn buscar(&self, termo: &str) -> Vec<String> {
        self.index
            .get(&termo.to_lowercase())
            .cloned()
            .unwrap_or_else(Vec::new)
    }
}
