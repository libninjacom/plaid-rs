
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateRequired {
    pub client_name: String,
    pub country_codes: Vec<String>,
    pub language: String,
    pub user: LinkTokenCreateRequestUser,
}
impl std::fmt::Display for LinkTokenCreateRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}