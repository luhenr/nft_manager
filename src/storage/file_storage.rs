use crate::models::nft::NFT;
use bincode::{deserialize_from, serialize_into};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error, ErrorKind, Result};

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: &str) -> Self {
        FileStorage {
            file_path: file_path.to_string(),
        }
    }

    pub fn save(&mut self, nft: &NFT) -> Result<()> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path)?;
        let mut writer = BufWriter::new(file);
        serialize_into(&mut writer, &nft).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Erro ao serializar NFT: {}", e),
            )
        })?;
        Ok(())
    }

    pub fn save_all(&mut self, nfts: &[NFT]) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.file_path)?;
        let mut writer = BufWriter::new(file);
        for nft in nfts {
            serialize_into(&mut writer, &nft).map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("Erro ao serializar NFT: {}", e),
                )
            })?;
        }
        Ok(())
    }

    pub fn load_all(&mut self) -> Result<Vec<NFT>> {
        let file = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return Ok(Vec::new());
                } else {
                    return Err(e);
                }
            }
        };

        let mut reader = BufReader::new(file);
        let mut nfts = Vec::new();

        loop {
            match deserialize_from(&mut reader) {
                Ok(nft) => nfts.push(nft),
                Err(e) => {
                    if let bincode::ErrorKind::Io(ref err) = *e {
                        if err.kind() == ErrorKind::UnexpectedEof {
                            break;
                        }
                    }
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Erro ao desserializar NFT: {}", e),
                    ));
                }
            }
        }

        Ok(nfts)
    }
}
