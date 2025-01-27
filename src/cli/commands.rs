use crate::models::nft::NFT;
use crate::models::category::Category;
use crate::storage::file_storage::{FileStorage, StorageError};
use chrono::NaiveDate;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

pub fn collect_nft_data(
    token_id: String,
    owner_id: u64,
    creation_date: NaiveDate,
    category: Category,
) -> Result<NFT, String> {
    let nft = NFT::new(token_id, owner_id, creation_date, category);
    nft.validate()?;
    Ok(nft)
}

pub fn process_create_nft(nft: &NFT, storage: &mut FileStorage) -> Result<(), StorageError> {
    storage.save(nft)
}

pub fn create_nft(reader: &mut impl BufRead, db_path: &str) {
    println!("\nCriando um novo NFT...");

    // Token ID
    let token_id = loop {
        let input = get_input("Digite o Token ID (não vazio): ", reader);
        if !input.trim().is_empty() {
            break input;
        }
        println!("Token ID não pode ser vazio.");
    };
    println!("Token ID recebido: {}", token_id);

    // Owner ID
    let owner_id = loop {
        let input = get_input("Digite o Owner ID (número inteiro maior que 0): ", reader);
        match input.parse::<u64>() {
            Ok(id) if id > 0 => {
                println!("Owner ID recebido: {}", id);
                break id;
            }
            _ => println!("Owner ID inválido. Por favor, insira um número inteiro maior que 0."),
        }
    };

    // Creation Date
    let creation_date = loop {
        let input = get_input("Digite a Data de Criação (AAAA-MM-DD): ", reader);
        match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            Ok(date) => {
                let current_date = chrono::Local::now().date_naive();
                if date <= current_date {
                    println!("Data de Criação recebida: {}", date);
                    break date;
                } else {
                    println!("A data não pode ser no futuro.");
                }
            }
            Err(_) => println!("Data inválida. Formato esperado: AAAA-MM-DD."),
        }
    };

    // Category
    println!("\nCategorias disponíveis:");
    println!("- Art");
    println!("- Music");
    println!("- Virtual Real Estate");
    println!("- Collectible");
    println!("- Game Item");
    println!("- Outra (digite uma descrição personalizada)");

    let category = loop {
        let input = get_input("Digite a Categoria do NFT: ", reader);
        match Category::from_str(&input) {
            Ok(category) => {
                println!("Categoria recebida: {}", category);
                break category;
            }
            Err(e) => println!("{}", e),
        }
    };

    match collect_nft_data(token_id, owner_id, creation_date, category) {
        Ok(nft) => {
            let mut storage = FileStorage::new(db_path);
            if let Err(e) = process_create_nft(&nft, &mut storage) {
                println!("Erro ao salvar NFT: {}", e);
            } else {
                println!("NFT salvo com sucesso!");
            }
        }
        Err(e) => {
            println!("Erro ao coletar dados do NFT: {}", e);
        }
    }
}

pub fn read_nft(db_path: &str) -> Result<Vec<NFT>, StorageError> {
    println!("\nListando NFTs...");

    let mut storage = FileStorage::new(db_path);
    match storage.load_all() {
        Ok(nfts) => {
            if nfts.is_empty() {
                println!("Nenhum NFT encontrado.");
            } else {
                for nft in nfts.iter() {
                    println!("------------------------------");
                    println!("Token ID: {}", nft.token_id);
                    println!("Owner ID: {}", nft.owner_id);
                    println!("Data de Criação: {}", nft.creation_date);
                    println!("Categoria: {}", nft.category);
                }
                println!("------------------------------");
            }
            Ok(nfts)
        }
        Err(e) => {
            println!("Erro ao carregar NFTs: {}", e);
            Err(e)
        }
    }
}

pub fn process_update_nft(
    token_id: &str,
    new_owner_id: u64,
    storage: &mut FileStorage,
) -> Result<(), String> {
    let mut nfts = storage.load_all().map_err(|e| e.to_string())?;

    if let Some(nft) = nfts.iter_mut().find(|n| n.token_id == token_id) {
        nft.owner_id = new_owner_id;
        
        // Validação dos dados
        nft.validate()?;
        
        storage.save_all(&nfts).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("NFT com Token ID '{}' não encontrado.", token_id))
    }
}

pub fn update_nft(reader: &mut impl BufRead, db_path: &str) {
    println!("\nAtualizando um NFT...");

    let token_id = get_input("Digite o Token ID do NFT que deseja atualizar: ", reader);
    let new_owner_id = loop {
        let input = get_input("Digite o novo Owner ID (número inteiro maior que 0): ", reader);
        match input.parse::<u64>() {
            Ok(id) if id > 0 => {
                println!("Novo Owner ID recebido: {}", id);
                break id;
            }
            _ => println!("Owner ID inválido. Por favor, insira um número inteiro maior que 0."),
        }
    };

    let mut storage = FileStorage::new(db_path);
    match process_update_nft(&token_id, new_owner_id, &mut storage) {
        Ok(_) => println!("NFT atualizado com sucesso!"),
        Err(e) => println!("Erro ao atualizar NFT: {}", e),
    }
}

pub fn process_delete_nft(token_id: &str, storage: &mut FileStorage) -> Result<(), String> {
    let mut nfts = storage.load_all().map_err(|e| e.to_string())?;
    let original_len = nfts.len();
    nfts.retain(|nft| nft.token_id != token_id);

    if nfts.len() < original_len {
        storage.save_all(&nfts).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("NFT com Token ID '{}' não encontrado.", token_id))
    }
}

pub fn delete_nft(reader: &mut impl BufRead, db_path: &str) {
    println!("\nDeletando um NFT...");

    let token_id = get_input("Digite o Token ID do NFT que deseja deletar: ", reader);
    println!("Token ID a ser deletado: {}", token_id);

    let mut storage = FileStorage::new(db_path);
    match process_delete_nft(&token_id, &mut storage) {
        Ok(_) => println!("NFT deletado com sucesso!"),
        Err(e) => println!("Erro ao deletar NFT: {}", e),
    }
}

fn get_input(prompt: &str, reader: &mut impl BufRead) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    input.trim().to_string()
}