// src/models/nft.rs

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone, PartialEq)]
pub struct NFT {
    #[validate(length(min = 1))]
    pub token_id: String,

    pub owner_id: u64,

    pub creation_date: NaiveDate,

    #[validate(length(min = 1))]
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

    pub fn validate_nft(&self) -> Result<(), validator::ValidationErrors> {
        self.validate()
    }
}
