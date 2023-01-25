
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum Products {
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "balance")]
    Balance,
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "investments")]
    Investments,
    #[serde(rename = "liabilities")]
    Liabilities,
    #[serde(rename = "payment_initiation")]
    PaymentInitiation,
    #[serde(rename = "identity_verification")]
    IdentityVerification,
    #[serde(rename = "transactions")]
    Transactions,
    #[serde(rename = "credit_details")]
    CreditDetails,
    #[serde(rename = "income")]
    Income,
    #[serde(rename = "income_verification")]
    IncomeVerification,
    #[serde(rename = "deposit_switch")]
    DepositSwitch,
    #[serde(rename = "standing_orders")]
    StandingOrders,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "employment")]
    Employment,
    #[serde(rename = "recurring_transactions")]
    RecurringTransactions,
}