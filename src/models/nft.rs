use serde::{Serialize, Deserialize};
use validator::Validate;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NFTCategory {
    Art,
    Collectible,
    GameItem,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct NFT {
    #[validate(length(min = 1))]
    pub token_id: String,

    #[validate(length(min = 1))]
    pub owner_id: String,

    pub creation_date: NaiveDate,

    pub category: NFTCategory,
}

impl NFT {
    pub fn new(
        token_id: String,
        owner_id: String,
        creation_date: NaiveDate,
        category: NFTCategory,
    ) -> Self {
        NFT {
            token_id,
            owner_id,
            creation_date,
            category,
        }
    }

    pub fn validate_nft(&self) -> Result<(), validator::ValidationErrors> {
        <NFT as Validate>::validate(self)
    }
}