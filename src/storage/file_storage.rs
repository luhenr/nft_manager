use crate::models::nft::NFT;
use serde_cbor::{from_reader, to_writer};
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Serde(serde_cbor::Error),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "Erro de IO: {}", e),
            StorageError::Serde(e) => write!(f, "Erro de Serialização: {}", e),
        }
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StorageError::Io(e) => Some(e),
            StorageError::Serde(e) => Some(e),
        }
    }
}

pub struct FileStorage {
    pub file_path: String,
}

impl FileStorage {
    pub fn new(file_path: &str) -> Self {
        FileStorage {
            file_path: file_path.to_string(),
        }
    }

    pub fn save(&mut self, nft: &NFT) -> Result<(), StorageError> {
        let mut nfts = self.load_all()?;
        nfts.push(nft.clone());
        self.save_all(&nfts)
    }

    pub fn load_all(&mut self) -> Result<Vec<NFT>, StorageError> {
        match OpenOptions::new().read(true).open(&self.file_path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let nfts = from_reader(reader).map_err(StorageError::Serde)?;
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

    pub fn save_all(&mut self, nfts: &[NFT]) -> Result<(), StorageError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .map_err(StorageError::Io)?;

        let writer = BufWriter::new(file);
        to_writer(writer, &nfts).map_err(StorageError::Serde)?;
        Ok(())
    }
}
