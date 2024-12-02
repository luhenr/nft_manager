// src/storage/file_storage.rs

use crate::models::nft::NFT;
use bincode::{deserialize_from, serialize_into};
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};

/// Enumeração dos possíveis erros de armazenamento.
#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Bincode(bincode::Error),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "Erro de IO: {}", e),
            StorageError::Bincode(e) => write!(f, "Erro de Bincode: {}", e),
        }
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StorageError::Io(e) => Some(e),
            StorageError::Bincode(e) => Some(e),
        }
    }
}

/// Estrutura para manipular o armazenamento de NFTs em arquivo.
pub struct FileStorage {
    pub file_path: String,
}

impl FileStorage {
    /// Cria uma nova instância de `FileStorage`.
    pub fn new(file_path: &str) -> Self {
        FileStorage {
            file_path: file_path.to_string(),
        }
    }

    /// Salva um único NFT no armazenamento.
    pub fn save(&mut self, nft: &NFT) -> Result<(), StorageError> {
        let mut nfts = self.load_all().unwrap_or_else(|_| Vec::new());
        nfts.push(nft.clone());
        self.save_all(&nfts)
    }

    /// Carrega todos os NFTs do armazenamento.
    pub fn load_all(&mut self) -> Result<Vec<NFT>, StorageError> {
        match OpenOptions::new().read(true).open(&self.file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let nfts: Vec<NFT> = deserialize_from(reader).map_err(StorageError::Bincode)?;
                Ok(nfts)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    Ok(vec![])
                } else {
                    Err(StorageError::Io(e))
                }
            }
        }
    }

    /// Salva todos os NFTs fornecidos no armazenamento, sobrescrevendo o conteúdo anterior.
    pub fn save_all(&mut self, nfts: &[NFT]) -> Result<(), StorageError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(StorageError::Io)?;

        let writer = BufWriter::new(file);
        serialize_into(writer, &nfts).map_err(StorageError::Bincode)?;
        Ok(())
    }
}
