// src/cli/mod.rs

pub mod commands;
use commands::{create_nft, delete_nft, read_nft, update_nft};
use std::env;
use std::io::{self, BufRead, BufReader};
use std::io::Write;

pub fn run_cli() {
    // Obtém o caminho do banco de dados da variável de ambiente ou usa o padrão
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "nfts.db".to_string());
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());

    loop {
        println!("\n=== Gerenciador de NFTs ===");
        println!("1. Criar NFT");
        println!("2. Listar NFTs");
        println!("3. Atualizar NFT");
        println!("4. Deletar NFT");
        println!("5. Sair");

        let choice = get_input("Selecione uma opção: ", &mut reader);

        match choice.trim() {
            "1" => create_nft(&mut reader, &db_path),
            "2" => read_nft(&db_path),
            "3" => update_nft(&mut reader, &db_path),
            "4" => delete_nft(&mut reader, &db_path),
            "5" => {
                println!("Saindo...");
                break;
            }
            _ => println!("Opção inválida. Por favor, tente novamente."),
        }
    }
}

fn get_input(prompt: &str, reader: &mut impl BufRead) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}
