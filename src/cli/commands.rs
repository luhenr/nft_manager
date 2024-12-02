use crate::models::nft::NFT;
use crate::storage::file_storage::{FileStorage, StorageError};
use chrono::NaiveDate;
use std::io::{self, BufRead, Write};

pub fn collect_nft_data(
    token_id: String,
    owner_id: u64,
    creation_date: NaiveDate,
    category: String,
) -> Result<NFT, String> {
    let nft = NFT::new(token_id, owner_id, creation_date, category);

    // Validação dos dados
    nft.validate_nft()?;
    Ok(nft)
}

pub fn process_create_nft(nft: &NFT, storage: &mut FileStorage) -> Result<(), StorageError> {
    storage.save(nft)
}

pub fn create_nft(reader: &mut impl BufRead, db_path: &str) {
    println!("\nCriando um novo NFT...");

    // Coleta de dados do usuário
    let token_id = get_input("Digite o Token ID (não vazio): ", reader);

    if token_id.is_empty() {
        println!("Token ID não pode ser vazio.");
        return;
    }
    println!("Token ID recebido: {}", token_id);
    let owner_id = loop {
        let input = get_input("Digite o Owner ID (número inteiro): ", reader);
        match input.parse::<u64>() {
            Ok(id) => {
                println!("Owner ID recebido: {}", id);
                break id;
            }
            Err(_) => println!("Owner ID inválido. Por favor, insira um número inteiro."),
        }
    };
    let creation_date = loop {
        let input = get_input("Digite a Data de Criação (AAAA-MM-DD): ", reader);
        match NaiveDate::parse_from_str(&input, "%Y-%m-%d") {
            Ok(date) => {
                println!("Data de Criação recebida: {}", date);
                break date;
            }
            Err(_) => println!("Data inválida. Formato esperado: AAAA-MM-DD."),
        }
    };
    let category = get_input("Digite a Categoria do NFT (não vazia): ", reader);
    println!("Categoria recebida: {}", category);

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
                for nft in &nfts {
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

    let position = nfts.iter().position(|nft| nft.token_id == token_id);
    if let Some(pos) = position {
        nfts[pos].owner_id = new_owner_id;

        // Validação dos dados
        if let Err(e) = nfts[pos].validate_nft() {
            return Err(format!("Erro de validação: {:?}", e));
        }

        storage.save_all(&nfts).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("NFT com Token ID '{}' não encontrado.", token_id))
    }
}

pub fn update_nft(reader: &mut impl BufRead, db_path: &str) {
    println!("\nAtualizando um NFT...");

    let token_id = get_input("Digite o Token ID do NFT que deseja atualizar: ", reader);
    println!("Token ID a ser atualizado: {}", token_id);
    let new_owner_id = loop {
        let input = get_input("Digite o novo Owner ID (número inteiro): ", reader);
        match input.parse::<u64>() {
            Ok(id) => {
                println!("Novo Owner ID recebido: {}", id);
                break id;
            }
            Err(_) => println!("Owner ID inválido. Por favor, insira um número inteiro."),
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
