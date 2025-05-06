mod search;
mod graph;

use search::SearchEngine;
use graph::Graph;

fn main() {
    let mut buscador = SearchEngine::new();
    buscador.indexar("tv", "Smart TV Samsung 50\"");
    buscador.indexar("geladeira", "Geladeira Brastemp Frost Free");
    buscador.indexar("notebook", "Notebook Dell i7 16GB");

    println!("🔍 Resultado da busca por 'tv':");
    for produto in buscador.buscar("tv") {
        println!("- {}", produto);
    }

    let mut grafo = Graph::new();
    grafo.adicionar_recomendacao("Smart TV Samsung 50\"", "Suporte de Parede");
    grafo.adicionar_recomendacao("Smart TV Samsung 50\"", "Soundbar JBL");

    println!("\n📦 Recomendados para 'Smart TV Samsung 50\"':");
    if let Some(recomendados) = grafo.recomendar("Smart TV Samsung 50\"") {
        for produto in recomendados {
            println!("- {}", produto);
        }
    }
}
