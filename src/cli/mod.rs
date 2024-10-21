pub mod commands;

use commands::{create_nft, delete_nft, read_nft, update_nft};
use std::io::{self, Write};

pub fn run_cli() {
    loop {
        println!("\nGerenciador de NFTs");
        println!("1. Criar NFT");
        println!("2. Ler NFTs");
        println!("3. Atualizar NFT");
        println!("4. Deletar NFT");
        println!("5. Sair");
        print!("Selecione uma opção: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => create_nft(),
            "2" => read_nft(),
            "3" => update_nft(),
            "4" => delete_nft(),
            "5" => {
                println!("Encerrando o programa.");
                break;
            }
            _ => println!("Opção inválida. Tente novamente."),
        }
    }
}
