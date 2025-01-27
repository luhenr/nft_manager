use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::models::category::Category;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NFT {
    pub token_id: String,      // Campo string
    pub owner_id: u64,         // Campo numérico
    pub creation_date: NaiveDate,  // Campo data
    pub category: Category     // Campo enum
}

impl NFT {
    pub fn new(
        token_id: String,
        owner_id: u64,
        creation_date: NaiveDate,
        category: Category,
    ) -> Self {
        NFT {
            token_id,
            owner_id,
            creation_date,
            category,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        // Validação do token_id
        if self.token_id.trim().is_empty() {
            return Err("Token ID não pode ser vazio".to_string());
        }

        // Validação do owner_id
        if self.owner_id == 0 {
            return Err("Owner ID deve ser maior que zero".to_string());
        }

        // Validação da data
        let current_date = chrono::Local::now().date_naive();
        if self.creation_date > current_date {
            return Err("Data de criação não pode ser no futuro".to_string());
        }

        Ok(())
    }
}