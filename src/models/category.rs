use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Art,
    Music,
    VirtualRealEstate,
    Collectible,
    GameItem,
    Other(String),
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Art => write!(f, "Art"),
            Category::Music => write!(f, "Music"),
            Category::VirtualRealEstate => write!(f, "Virtual Real Estate"),
            Category::Collectible => write!(f, "Collectible"),
            Category::GameItem => write!(f, "Game Item"),
            Category::Other(description) => write!(f, "Other: {}", description),
        }
    }
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "art" => Ok(Category::Art),
            "music" => Ok(Category::Music),
            "virtual real estate" => Ok(Category::VirtualRealEstate),
            "collectible" => Ok(Category::Collectible),
            "game item" => Ok(Category::GameItem),
            _ if !s.trim().is_empty() => Ok(Category::Other(s.to_string())),
            _ => Err("Categoria inválida".to_string()),
        }
    }
}