use nft_manager::storage::file_storage::{FileStorage, StorageError};

#[test]
fn test_save_all_io_error() {
    // Simular um erro de I/O usando um caminho inválido
    let mut mock_file_storage = FileStorage::new("/invalid_path/nfts.db");

    // Tentar salvar NFTs, o que deve resultar em um erro de I/O
    let result = mock_file_storage.save_all(&[]);
    assert!(matches!(result, Err(StorageError::Io(_))));
}
