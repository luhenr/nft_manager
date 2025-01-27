use chrono::NaiveDate;
use nft_manager::cli::commands::{
    collect_nft_data, process_create_nft, process_delete_nft, process_update_nft,
};
use nft_manager::models::category::Category;
use nft_manager::models::nft::NFT;
use nft_manager::storage::file_storage::FileStorage;
use tempfile::tempdir;

#[test]
fn test_nft_creation() {
    let nft = NFT::new(
        "token123".to_string(),
        456u64,
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida"),
        Category::Other("Arte".to_string()), // Ajustado para usar Category
    );

    assert_eq!(nft.token_id, "token123");
    assert_eq!(nft.owner_id, 456u64);
    assert_eq!(
        nft.creation_date,
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida")
    );
    assert_eq!(nft.category, Category::Other("Arte".to_string())); // Ajustado para usar Category
}

#[test]
fn test_nft_validation() {
    let nft = NFT::new(
        "".to_string(), // token_id vazio
        0,              // owner_id inválido (zero não é permitido)
        NaiveDate::from_ymd_opt(2023, 10, 21).expect("Data inválida"),
        Category::Other("".to_string()), // Ajustado para usar Category
    );

    assert!(nft.validate().is_err()); // Ajustado para usar validate
}

#[test]
fn test_collect_nft_data_valid() {
    let nft = collect_nft_data(
        "token_test".to_string(),
        123u64,
        NaiveDate::from_ymd_opt(2023, 11, 5).expect("Data inválida"),
        Category::Other("Arte".to_string()), // Ajustado para usar Category
    );

    assert!(nft.is_ok());
}

#[test]
fn test_collect_nft_data_invalid() {
    let nft = collect_nft_data(
        "".to_string(), // token_id vazio
        0u64,           // owner_id inválido
        NaiveDate::from_ymd_opt(2023, 11, 5).expect("Data inválida"),
        Category::Other("".to_string()), // Ajustado para usar Category
    );

    assert!(nft.is_err());
}

#[test]
fn test_process_create_nft() {
    let nft = NFT::new(
        "token_process_test".to_string(),
        789u64,
        NaiveDate::from_ymd_opt(2023, 11, 5).expect("Data inválida"),
        Category::Other("Colecionável".to_string()), // Ajustado para usar Category
    );

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    let result = process_create_nft(&nft, &mut storage);
    assert!(result.is_ok());

    let nfts = storage.load_all().unwrap();
    assert_eq!(nfts.len(), 1);
    assert_eq!(nfts[0], nft);
}

#[test]
fn test_process_update_nft() {
    let nft = NFT::new(
        "token_update_test".to_string(),
        123u64,
        NaiveDate::from_ymd_opt(2023, 11, 5).expect("Data inválida"),
        Category::Other("Arte".to_string()), // Ajustado para usar Category
    );

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);
    storage.save(&nft).unwrap();

    let result = process_update_nft("token_update_test", 456u64, &mut storage);
    assert!(result.is_ok());

    let nfts = storage.load_all().unwrap();
    assert_eq!(nfts[0].owner_id, 456u64);
}

#[test]
fn test_process_delete_nft() {
    let nft = NFT::new(
        "token_delete_test".to_string(),
        123u64,
        NaiveDate::from_ymd_opt(2023, 11, 5).expect("Data inválida"),
        Category::Other("Arte".to_string()), // Ajustado para usar Category
    );

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);
    storage.save(&nft).unwrap();

    let result = process_delete_nft("token_delete_test", &mut storage);
    assert!(result.is_ok());

    let nfts = storage.load_all().unwrap();
    assert!(nfts.is_empty());
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

    let result = process_update_nft("nonexistent_token", 456u64, &mut storage);
    assert!(result.is_err());
}

#[test]
fn test_delete_nonexistent_nft() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("nfts_test.db");
    let file_path_str = file_path.to_str().unwrap();

    let mut storage = FileStorage::new(file_path_str);

    let result = process_delete_nft("nonexistent_token", &mut storage);
    assert!(result.is_err());
}
