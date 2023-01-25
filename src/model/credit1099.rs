
use serde::{Serialize, Deserialize};
use super::{
    Credit1099Filer, Credit1099Payer, Credit1099Recipient, CreditDocumentMetadata,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099 {
    pub april_amount: Option<f64>,
    pub august_amount: Option<f64>,
    pub card_not_present_transaction: Option<f64>,
    pub crop_insurance_proceeds: Option<f64>,
    pub december_amount: Option<f64>,
    pub document_id: Option<String>,
    pub document_metadata: Option<CreditDocumentMetadata>,
    pub excess_golden_parachute_payments: Option<f64>,
    pub february_amount: Option<f64>,
    pub federal_income_tax_withheld: Option<f64>,
    pub filer: Option<Credit1099Filer>,
    pub fishing_boat_proceeds: Option<f64>,
    #[serde(rename = "form_1099_type")]
    pub form1099_type: Option<String>,
    pub gross_amount: Option<f64>,
    pub gross_proceeds_paid_to_an_attorney: Option<f64>,
    pub january_amount: Option<f64>,
    pub july_amount: Option<f64>,
    pub june_amount: Option<f64>,
    pub march_amount: Option<f64>,
    pub may_amount: Option<f64>,
    pub medical_and_healthcare_payments: Option<f64>,
    pub merchant_category_code: Option<String>,
    pub nonemployee_compensation: Option<f64>,
    pub november_amount: Option<f64>,
    pub number_of_payment_transactions: Option<String>,
    pub october_amount: Option<f64>,
    pub other_income: Option<f64>,
    pub payer: Option<Credit1099Payer>,
    #[serde(
        rename = "payer_made_direct_sales_of_5000_or_more_of_consumer_products_to_buyer"
    )]
    pub payer_made_direct_sales_of5000_or_more_of_consumer_products_to_buyer: Option<
        String,
    >,
    pub payer_state_number: Option<String>,
    pub payer_state_number_lower: Option<String>,
    pub primary_state: Option<String>,
    pub primary_state_id: Option<String>,
    pub primary_state_income_tax: Option<f64>,
    pub pse_name: Option<String>,
    pub pse_telephone_number: Option<String>,
    pub recipient: Option<Credit1099Recipient>,
    pub rents: Option<f64>,
    pub royalties: Option<f64>,
    pub secondary_state: Option<String>,
    pub secondary_state_id: Option<String>,
    pub secondary_state_income_tax: Option<f64>,
    #[serde(rename = "section_409a_deferrals")]
    pub section409_a_deferrals: Option<f64>,
    #[serde(rename = "section_409a_income")]
    pub section409_a_income: Option<f64>,
    pub september_amount: Option<f64>,
    pub state_income: Option<f64>,
    pub state_income_lower: Option<f64>,
    pub state_tax_withheld: Option<f64>,
    pub state_tax_withheld_lower: Option<f64>,
    pub substitute_payments_in_lieu_of_dividends_or_interest: Option<f64>,
    pub tax_year: Option<String>,
    pub transactions_reported: Option<String>,
}
impl std::fmt::Display for Credit1099 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}