
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum DocType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "DOCUMENT_TYPE_PAYSTUB")]
    DocumentTypePaystub,
    #[serde(rename = "DOCUMENT_TYPE_BANK_STATEMENT")]
    DocumentTypeBankStatement,
    #[serde(rename = "DOCUMENT_TYPE_US_TAX_W2")]
    DocumentTypeUsTaxW2,
    #[serde(rename = "DOCUMENT_TYPE_US_MILITARY_ERAS")]
    DocumentTypeUsMilitaryEras,
    #[serde(rename = "DOCUMENT_TYPE_US_MILITARY_LES")]
    DocumentTypeUsMilitaryLes,
    #[serde(rename = "DOCUMENT_TYPE_US_MILITARY_CLES")]
    DocumentTypeUsMilitaryCles,
    #[serde(rename = "DOCUMENT_TYPE_GIG")]
    DocumentTypeGig,
    #[serde(rename = "DOCUMENT_TYPE_NONE")]
    DocumentTypeNone,
    #[serde(rename = "DOCUMENT_TYPE_US_TAX_1099_MISC")]
    DocumentTypeUsTax1099Misc,
    #[serde(rename = "DOCUMENT_TYPE_US_TAX_1099_K")]
    DocumentTypeUsTax1099K,
    #[serde(rename = "DOCUMENT_TYPE_PLAID_GENERATED_PAYSTUB_PDF")]
    DocumentTypePlaidGeneratedPaystubPdf,
}