
use serde::{Serialize, Deserialize};
use super::{
    Credit1099Filer, Credit1099Payer, Credit1099Recipient, CreditDocumentMetadata,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub april_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub august_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_not_present_transaction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop_insurance_proceeds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub december_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<CreditDocumentMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excess_golden_parachute_payments: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub february_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federal_income_tax_withheld: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filer: Option<Credit1099Filer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fishing_boat_proceeds: Option<f64>,
    #[serde(rename = "form_1099_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form1099_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_proceeds_paid_to_an_attorney: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub january_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub july_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub june_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub march_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub may_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_and_healthcare_payments: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonemployee_compensation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub november_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_payment_transactions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub october_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_income: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<Credit1099Payer>,
    #[serde(
        rename = "payer_made_direct_sales_of_5000_or_more_of_consumer_products_to_buyer"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_made_direct_sales_of5000_or_more_of_consumer_products_to_buyer: Option<
        String,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_state_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_state_number_lower: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_state_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_state_income_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pse_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pse_telephone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Credit1099Recipient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rents: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalties: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_state_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_state_income_tax: Option<f64>,
    #[serde(rename = "section_409a_deferrals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section409_a_deferrals: Option<f64>,
    #[serde(rename = "section_409a_income")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section409_a_income: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub september_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_income: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_income_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_tax_withheld: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_tax_withheld_lower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitute_payments_in_lieu_of_dividends_or_interest: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions_reported: Option<String>,
}
impl std::fmt::Display for Credit1099 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}