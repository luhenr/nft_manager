// src/main.rs

mod cli;
mod models;
mod storage;

use std::env;

/// Função principal do programa.
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let arg = &args[1];
        if arg == "--help" || arg == "-h" {
            // Exibe a mensagem de ajuda
            println!("Gerenciador de NFTs");
            println!("Uso:");
            println!("  nft_manager [--help]");
            println!("\nOpções:");
            println!("  --help, -h     Exibe esta mensagem de ajuda");
            // Sai do programa
            return;
        } else {
            println!("Opção de linha de comando desconhecida: {}", arg);
            println!("Use '--help' para ver as opções disponíveis.");
            return;
        }
    }

    cli::run_cli();
}
