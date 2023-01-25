
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum Form1099Type {
    #[serde(rename = "FORM_1099_TYPE_UNKNOWN")]
    Form1099TypeUnknown,
    #[serde(rename = "FORM_1099_TYPE_MISC")]
    Form1099TypeMisc,
    #[serde(rename = "FORM_1099_TYPE_K")]
    Form1099TypeK,
}