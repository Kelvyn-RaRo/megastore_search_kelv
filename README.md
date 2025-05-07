# megastore_search_kelv
Pesquisa de produtos em rust com dados estruturados em grafos

Este projeto é uma aplicação simples e eficiente desenvolvida em Rust, que permite a busca de produtos por palavras-chave e recomenda itens relacionados com base em grafos. Ideal para catálogos de e-commerce e sistemas de recomendação simples.

---

##  Tecnologias Utilizadas

-  Linguagem: **Rust**
-  Estruturas: **HashMap**, **Grafos**
-  Editor: **Visual Studio Code**
-  Gerenciador de pacotes: **Cargo**

---

##  Como Executar o Projeto

Clone o repositório e execute:

```PROMPT
cargo run
```

---

##  Como Rodar o Teste

```PROMPT
cargo test
```

---

##  Exemplo de Uso

```text
 Resultado da busca por 'tv':
- Smart TV Samsung 50"

 Recomendados para 'Smart TV Samsung 50"':
- Suporte de Parede
- Soundbar JBL
```

---

##  Estrutura do Projeto

```text
📁 src/
├── main.rs         # Arquivo principal
├── search.rs       # Módulo de busca com HashMap
└── graph.rs        # Módulo de recomendações com grafos
```

---

##  Estruturas de Dados

- **HashMap**: Para indexar produtos por palavras-chave.
- **Grafo (HashMap com vetores)**: Para armazenar produtos recomendados por item.

---

## Desempenho

- Busca por palavra-chave: **Tempo constante (O(1))**
- Recomendação por grafo: **Busca leve e direta (O(1) a O(n))**
- Ideal para sistemas pequenos a médios de catálogo e recomendação.

---

##  Licença

Distribuído sob a licença MIT. Veja `LICENSE` para mais informações.

---

## 👤 Autor

Kelvyn Mendes  
Projeto desenvolvido para fins educacionais e de portfólio.
