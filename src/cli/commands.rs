use crate::models::nft::{NFT, NFTCategory};
use crate::storage::file_storage::FileStorage;
use std::io::{self, Write};

pub fn create_nft() {
    println!("\nCriando um novo NFT...");

    let token_id = uuid::Uuid::new_v4().to_string();

    let owner_id = loop {
        print!("Digite o Owner ID (não vazio): ");
        io::stdout().flush().unwrap();
        let mut owner_id_input = String::new();
        io::stdin().read_line(&mut owner_id_input).unwrap();
        let owner_id = owner_id_input.trim();
        if !owner_id.is_empty() {
            break owner_id.to_string();
        } else {
            println!("Owner ID não pode ser vazio.");
        }
    };

    let creation_date = chrono::Local::now().date_naive();

    let category = loop {
        println!("Selecione a categoria do NFT:");
        println!("1. Arte");
        println!("2. Colecionável");
        println!("3. Item de Jogo");
        println!("4. Outro");
        print!("Opção: ");
        io::stdout().flush().unwrap();
        let mut category_choice = String::new();
        io::stdin().read_line(&mut category_choice).unwrap();

        match category_choice.trim() {
            "1" => break NFTCategory::Art,
            "2" => break NFTCategory::Collectible,
            "3" => break NFTCategory::GameItem,
            "4" => break NFTCategory::Other,
            _ => println!("Categoria inválida. Tente novamente."),
        }
    };

    let nft = NFT::new(token_id, owner_id, creation_date, category);

    // Validação dos dados
    if let Err(e) = nft.validate_nft() {
        println!("Erro de validação: {:?}", e);
        return;
    }

    let mut storage = FileStorage::new("nfts.db");
    match storage.save(&nft) {
        Ok(_) => println!("NFT salvo com sucesso!"),
        Err(e) => println!("Erro ao salvar NFT: {}", e),
    }
}

pub fn read_nft() {
    println!("\nLendo NFTs...");

    let mut storage = FileStorage::new("nfts.db");
    match storage.load_all() {
        Ok(nfts) => {
            if nfts.is_empty() {
                println!("Nenhum NFT encontrado.");
            } else {
                for nft in nfts {
                    println!("{:?}", nft);
                }
            }
        }
        Err(e) => println!("Erro ao carregar NFTs: {}", e),
    }
}

pub fn update_nft() {
    println!("\nAtualizando um NFT...");

    print!("Digite o Token ID do NFT que deseja atualizar: ");
    io::stdout().flush().unwrap();
    let mut token_id = String::new();
    io::stdin().read_line(&mut token_id).unwrap();
    let token_id = token_id.trim().to_string();

    let mut storage = FileStorage::new("nfts.db");
    match storage.load_all() {
        Ok(mut nfts) => {
            if let Some(pos) = nfts.iter().position(|nft| nft.token_id == token_id) {
                // Coleta e validação do novo owner_id
                let new_owner_id = loop {
                    print!("Digite o novo Owner ID (não vazio): ");
                    io::stdout().flush().unwrap();
                    let mut owner_id_input = String::new();
                    io::stdin().read_line(&mut owner_id_input).unwrap();
                    let owner_id = owner_id_input.trim();
                    if !owner_id.is_empty() {
                        break owner_id.to_string();
                    } else {
                        println!("Owner ID não pode ser vazio.");
                    }
                };

                // Atualiza o owner_id
                nfts[pos].owner_id = new_owner_id;

                // Salva as alterações
                match storage.save_all(&nfts) {
                    Ok(_) => println!("NFT atualizado com sucesso!"),
                    Err(e) => println!("Erro ao atualizar NFT: {}", e),
                }
            } else {
                println!("NFT com Token ID '{}' não encontrado.", token_id);
            }
        }
        Err(e) => println!("Erro ao carregar NFTs: {}", e),
    }
}

pub fn delete_nft() {
    println!("\nDeletando um NFT...");

    print!("Digite o Token ID do NFT que deseja deletar: ");
    io::stdout().flush().unwrap();
    let mut token_id = String::new();
    io::stdin().read_line(&mut token_id).unwrap();
    let token_id = token_id.trim().to_string();

    let mut storage = FileStorage::new("nfts.db");
    match storage.load_all() {
        Ok(mut nfts) => {
            let original_len = nfts.len();
            nfts.retain(|nft| nft.token_id != token_id);

            if nfts.len() < original_len {
                match storage.save_all(&nfts) {
                    Ok(_) => println!("NFT deletado com sucesso!"),
                    Err(e) => println!("Erro ao deletar NFT: {}", e),
                }
            } else {
                println!("NFT com Token ID '{}' não encontrado.", token_id);
            }
        }
        Err(e) => println!("Erro ao carregar NFTs: {}", e),
    }
}
