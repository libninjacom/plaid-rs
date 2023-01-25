
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum PhoneType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "fax")]
    Fax,
}