// src/models/nft.rs

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct NFT {
    #[validate(length(min = 1, message = "Token ID não pode ser vazio"))]
    pub token_id: String,

    pub owner_id: u64,

    pub creation_date: NaiveDate,

    #[validate(length(min = 1, message = "Categoria não pode ser vazia"))]
    pub category: String,
}


impl NFT {
    pub fn new(
        token_id: String,
        owner_id: u64,
        creation_date: NaiveDate,
        category: String,
    ) -> Self {
        NFT {
            token_id,
            owner_id,
            creation_date,
            category,
        }
    }

    pub fn validate_nft(&self) -> Result<(), String> {
        self.validate()
            .map_err(|e| format!("Erro de validação: {:?}", e))
    }
}
