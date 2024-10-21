use nft_manager::models::nft::{NFT, NFTCategory};
use nft_manager::storage::file_storage::FileStorage;
use chrono::NaiveDate;
use tempfile::tempdir;

#[test]
fn test_nft_creation() {
    let nft = NFT::new(
        "token123".to_string(),
        "owner456".to_string(),
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::Art,
    );

    assert_eq!(nft.token_id, "token123");
    assert_eq!(nft.owner_id, "owner456");
    assert_eq!(nft.creation_date, NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
);
    match nft.category {
        NFTCategory::Art => {}
        _ => panic!("Categoria não é Art"),
    }
}

#[test]
fn test_nft_validation() {
    let nft = NFT::new(
        "".to_string(), // token_id vazio
        "".to_string(), // owner_id vazio
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::Art,
    );

    assert!(nft.validate_nft().is_err());
}

#[test]
fn test_storage_save_and_load() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    let nft = NFT::new(
        "token123".to_string(),
        "owner456".to_string(),
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::Art,
    );

    storage.save(&nft).unwrap();
    let nfts = storage.load_all().unwrap();

    assert_eq!(nfts.len(), 1);
    assert_eq!(nfts[0].token_id, "token123");
}

#[test]
fn test_storage_update() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    let mut nft = NFT::new(
        "token123".to_string(),
        "owner456".to_string(),
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::Art,
    );

    storage.save(&nft).unwrap();

    // Atualiza o owner_id
    nft.owner_id = "new_owner".to_string();

    // Salva todas as NFTs (neste caso, apenas uma)
    storage.save_all(&[nft.clone()]).unwrap();

    let nfts = storage.load_all().unwrap();
    assert_eq!(nfts.len(), 1);
    assert_eq!(nfts[0].owner_id, "new_owner");
}

#[test]
fn test_storage_delete() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    let nft1 = NFT::new(
        "token123".to_string(),
        "owner456".to_string(),
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::Art,
    );

    let nft2 = NFT::new(
        "token789".to_string(),
        "owner999".to_string(),
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida"),
        NFTCategory::Collectible,
    );

    storage.save(&nft1).unwrap();
    storage.save(&nft2).unwrap();

    let mut nfts = storage.load_all().unwrap();
    assert_eq!(nfts.len(), 2);

    // Deleta nft1
    nfts.retain(|nft| nft.token_id != "token123");
    storage.save_all(&nfts).unwrap();

    let nfts = storage.load_all().unwrap();
    assert_eq!(nfts.len(), 1);
    assert_eq!(nfts[0].token_id, "token789");
}

#[test]
fn test_create_nft_with_invalid_data() {
    let nft = NFT::new(
        "".to_string(), // token_id inválido
        "".to_string(), // owner_id inválido
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::Other,
    );

    let result = nft.validate_nft();
    assert!(result.is_err());
}

#[test]
fn test_create_nft_with_valid_data() {
    let nft = NFT::new(
        "valid_token_id".to_string(),
        "valid_owner_id".to_string(),
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
,
        NFTCategory::GameItem,
    );

    let result = nft.validate_nft();
    assert!(result.is_ok());
}

#[test]
fn test_read_empty_storage() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("empty_nfts.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    let nfts = storage.load_all().unwrap();
    assert_eq!(nfts.len(), 0);
}

#[test]
fn test_update_nonexistent_nft() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    // Tenta carregar NFTs (nenhuma salva)
    let nfts = storage.load_all().unwrap();

    // Tenta atualizar uma NFT que não existe
    let pos = nfts.iter().position(|nft| nft.token_id == "nonexistent_token");
    assert!(pos.is_none());
}

#[test]
fn test_delete_nonexistent_nft() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    // Tenta carregar NFTs (nenhuma salva)
    let mut nfts = storage.load_all().unwrap();

    // Tenta deletar uma NFT que não existe
    let original_len = nfts.len();
    nfts.retain(|nft| nft.token_id != "nonexistent_token");
    assert_eq!(nfts.len(), original_len);
}
