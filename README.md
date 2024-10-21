# NFT Manager
## Descrição do Projeto
O NFT Manager é uma aplicação de linha de comando (CLI) escrita em Rust que permite realizar operações CRUD (Criar, Ler, Atualizar, Deletar) em Tokens Não Fungíveis (NFTs). O objetivo do projeto é fornecer uma ferramenta simples e eficiente para gerenciar NFTs, aplicando as melhores práticas de desenvolvimento em Rust e conceitos importantes de programação.

## Funcionalidades Implementadas
* **Criar NFT**: Permite criar um novo NFT, solicitando informações ao usuário e validando os dados antes de salvar.
* **Listar NFTs**: Exibe todos os NFTs armazenados no sistema.
* **Atualizar NFT**: Permite atualizar o proprietário (owner_id) de um NFT existente.
* **Deletar NFT**: Remove um NFT do sistema com base no seu token_id.

## Estrutura do Projeto

```bash
nft_manager/
├── Cargo.toml
├── .gitignore
├── README.md
├── src
│   ├── lib.rs
│   ├── main.rs
│   ├── cli
│   │   ├── mod.rs
│   │   └── commands.rs
│   ├── models
│   │   ├── mod.rs
│   │   └── nft.rs
│   └── storage
│       ├── mod.rs
│       └── file_storage.rs
└── tests
    └── nft_tests.rs
```

`main.rs`: Ponto de entrada da aplicação CLI.

`lib.rs`: Define os módulos da biblioteca para serem usados nos testes e em main.rs.

`cli/`: Contém a lógica da interface de linha de comando.

`commands.rs`: Implementa os comandos e interações com o usuário.

`models/`: Define as estruturas de dados.

`nft.rs`: Define a struct NFT e o enum NFTCategory.

`storage/`: Gerencia a persistência dos dados.

`file_storage.rs`: Implementa o armazenamento em arquivo usando serialização binária.

`tests/`: Contém testes automatizados para as funcionalidades.

## Detalhes Técnicos
### Struct `NFT`
Representa a entidade principal com os seguintes campos:
* `token_id: String`: Identificador único do NFT, gerado automaticamente usando UUID.
* `owner_id: String`: Identificador do proprietário atual do NFT.
* `creation_date: NaiveDate`: Data de criação do NFT.
* `category: NFTCategory`: Categoria do NFT.

### Enum `NFTCategory`
Define as categorias possíveis para um NFT:
* `Art`
* `Collectible`
* `GameItem`
* `Other`

### Validação de Dados
Utiliza o crate `validator` para garantir a integridade dos dados:
* Campos `token_id` e `owner_id` devem ser strings não vazias.
* As datas são validadas para garantir que sejam válidas.

### Persistência
* Os NFTs são armazenados em um arquivo chamado `nfts.db` na raiz do projeto.
* Utiliza o crate `bincode` para serialização e desserialização binária dos dados.

### Testes Automatizados
* Testes para todas as funcionalidades, garantindo o correto funcionamento do sistema.
* Utiliza o crate `tempfile` para criar arquivos temporários durante os testes.

## Como Executar o Projeto
### Pré-requisitos
* Rust instalado (compilador e ferramentas). Você pode instalá-lo a partir de [rust-lang.org](https://www.rust-lang.org/).

### Clonar o Repositório
```bash
git clone https://github.com/luhenr/nft_manager.git
cd nft_manager
```

### Compilar o Projeto
```bash
cargo build
```

### Executar a Aplicação
```bash
cargo run
```

### Executar os Testes
```bash
cargo test
```

## Utilização
Ao executar o comando `cargo run`, o aplicativo apresentará um menu com as seguintes opções:

1. **Criar NFT**: Permite criar um novo NFT.
2. **Listar NFTs**: Exibe todos os NFTs armazenados.
3. **Atualizar NFT**: Atualiza o proprietário de um NFT existente.
4. **Deletar NFT**: Remove um NFT do sistema.
5. **Sair**: Encerra a aplicação.



