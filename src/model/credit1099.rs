use serde::{Serialize, Deserialize};
use super::{
    Credit1099Filer, Credit1099Payer, Credit1099Recipient, CreditDocumentMetadata,
};
///An object representing an end user's 1099 tax form
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Credit1099 {
    ///Amount reported for April.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub april_amount: Option<f64>,
    ///Amount reported for August.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub august_amount: Option<f64>,
    ///Amount in card not present transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_not_present_transaction: Option<f64>,
    ///Amount of crop insurance proceeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crop_insurance_proceeds: Option<f64>,
    ///Amount reported for December.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub december_amount: Option<f64>,
    ///An identifier of the document referenced by the document metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///Object representing metadata pertaining to the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<CreditDocumentMetadata>,
    ///Amount of golden parachute payments made by payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excess_golden_parachute_payments: Option<f64>,
    ///Amount reported for February.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub february_amount: Option<f64>,
    ///Amount of federal income tax withheld from payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub federal_income_tax_withheld: Option<f64>,
    ///An object representing a filer used by 1099-K tax documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filer: Option<Credit1099Filer>,
    ///Amount of fishing boat proceeds from payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fishing_boat_proceeds: Option<f64>,
    ///Form 1099 Type
    #[serde(rename = "form_1099_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form1099_type: Option<String>,
    ///Gross amount reported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    ///Amount of gross proceeds paid to an attorney by payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_proceeds_paid_to_an_attorney: Option<f64>,
    ///Amount reported for January.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub january_amount: Option<f64>,
    ///Amount reported for July.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub july_amount: Option<f64>,
    ///Amount reported for June.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub june_amount: Option<f64>,
    ///Amount reported for March.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub march_amount: Option<f64>,
    ///Amount reported for May.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub may_amount: Option<f64>,
    ///Amount of medical and healthcare payments from payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medical_and_healthcare_payments: Option<f64>,
    ///Merchant category of filer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    ///Amount of nonemployee compensation from payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonemployee_compensation: Option<f64>,
    ///Amount reported for November.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub november_amount: Option<f64>,
    ///Number of payment transactions made.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_payment_transactions: Option<String>,
    ///Amount reported for October.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub october_amount: Option<f64>,
    ///Amount in other income by payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other_income: Option<f64>,
    ///An object representing a payer used by 1099-MISC tax documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer: Option<Credit1099Payer>,
    ///Whether or not payer made direct sales over $5000 of consumer products.
    #[serde(
        rename = "payer_made_direct_sales_of_5000_or_more_of_consumer_products_to_buyer"
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer_made_direct_sales_of5000_or_more_of_consumer_products_to_buyer: Option<
        String,
    >,
    ///Primary state ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer_state_number: Option<String>,
    ///Secondary state ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payer_state_number_lower: Option<String>,
    ///Primary state of business.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_state: Option<String>,
    ///Primary state ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_state_id: Option<String>,
    ///State income tax reported for primary state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_state_income_tax: Option<f64>,
    ///Name of the PSE (Payment Settlement Entity).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pse_name: Option<String>,
    ///Formatted (XXX) XXX-XXXX. Phone number of the PSE (Payment Settlement Entity).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pse_telephone_number: Option<String>,
    ///An object representing a recipient used in both 1099-K and 1099-MISC tax documents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<Credit1099Recipient>,
    ///Amount in rent by payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rents: Option<f64>,
    ///Amount in royalties by payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub royalties: Option<f64>,
    ///Secondary state of business.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_state: Option<String>,
    ///Secondary state ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_state_id: Option<String>,
    ///State income tax reported for secondary state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_state_income_tax: Option<f64>,
    ///Amount of 409A deferrals earned by payer.
    #[serde(rename = "section_409a_deferrals")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section409_a_deferrals: Option<f64>,
    ///Amount of 409A income earned by payer.
    #[serde(rename = "section_409a_income")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section409_a_income: Option<f64>,
    ///Amount reported for September.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub september_amount: Option<f64>,
    ///State income reported for primary state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_income: Option<f64>,
    ///State income reported for secondary state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_income_lower: Option<f64>,
    ///Amount of state tax withheld of payer for primary state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_tax_withheld: Option<f64>,
    ///Amount of state tax withheld of payer for secondary state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_tax_withheld_lower: Option<f64>,
    ///Amount of substitute payments made by payer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub substitute_payments_in_lieu_of_dividends_or_interest: Option<f64>,
    ///Tax year of the tax form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<String>,
    ///One of the values will be provided Payment card Third party network
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions_reported: Option<String>,
}
impl std::fmt::Display for Credit1099 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}