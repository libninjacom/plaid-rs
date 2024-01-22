use serde::{Serialize, Deserialize};
/**The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:

`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
`is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account

For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalEvaluateCoreAttributes {
    ///The number of times the account's addresses on file have changed over the past 28 days
    #[serde(rename = "address_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_change_count28_d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 90 days
    #[serde(rename = "address_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_change_count90_d: Option<i64>,
    ///Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<f64>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_last_updated: Option<chrono::DateTime<chrono::Utc>>,
    ///The total number of credit (inflow) transactions over the past 10 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_10d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count10_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 30 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count30_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 60 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count60_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 90 days from the account that will be debited
    #[serde(rename = "credit_transactions_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_transactions_count90_d: Option<i64>,
    ///Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<f64>,
    ///The number of days since the first time the Item was connected to an application via Plaid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_since_first_plaid_connection: Option<i64>,
    ///The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance
    #[serde(rename = "days_with_negative_balance_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_with_negative_balance_count90_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 10 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_10d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count10_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 30 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count30_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 60 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count60_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 90 days from the account that will be debited
    #[serde(rename = "debit_transactions_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debit_transactions_count90_d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 28 days
    #[serde(rename = "email_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_change_count28_d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 90 days
    #[serde(rename = "email_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_change_count90_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count30_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_3d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count3_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_plaid_non_oauth_authentication_attempts_count7_d: Option<i64>,
    ///Indicates if the receiver bank account is closed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_closed: Option<bool>,
    ///Indicates if the receiver bank account is either frozen or restricted
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_frozen_or_restricted: Option<bool>,
    ///Indicates if the ACH transaction funding account is a savings/money market account
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_savings_or_money_market_account: Option<bool>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count30_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count60_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count7_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited.
    #[serde(rename = "nsf_overdraft_transactions_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count90_d: Option<i64>,
    ///The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance30_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_31d_to_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance31_d_to60_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance60_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_61d_to_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance61_d_to90_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(rename = "p10_eod_balance_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p10_eod_balance90_d: Option<f64>,
    ///The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p50_credit_transactions_amount_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_credit_transactions_amount28_d: Option<f64>,
    ///The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p50_debit_transactions_amount_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_debit_transactions_amount28_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance30_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_31d_to_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance31_d_to60_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance60_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_61d_to_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance61_d_to90_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(rename = "p50_eod_balance_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p50_eod_balance90_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance30_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_31d_to_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance31_d_to60_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance60_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_61d_to_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance61_d_to90_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    #[serde(rename = "p90_eod_balance_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p90_eod_balance90_d: Option<f64>,
    ///The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p95_credit_transactions_amount_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p95_credit_transactions_amount28_d: Option<f64>,
    ///The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    #[serde(rename = "p95_debit_transactions_amount_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub p95_debit_transactions_amount28_d: Option<f64>,
    ///The number of times the account's phone numbers on file have changed over the past 28 days
    #[serde(rename = "phone_change_count_28d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_change_count28_d: Option<i64>,
    ///The number of times the account's phone numbers on file have changed over the past 90 days
    #[serde(rename = "phone_change_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_change_count90_d: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 30 days
    #[serde(rename = "plaid_connections_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_connections_count30_d: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 7 days
    #[serde(rename = "plaid_connections_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_connections_count7_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count30_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_3d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count3_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plaid_non_oauth_authentication_attempts_count7_d: Option<i64>,
    ///The total credit (inflow) transaction amount over the past 10 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_10d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount10_d: Option<f64>,
    ///The total credit (inflow) transaction amount over the past 30 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount30_d: Option<f64>,
    ///The total credit (inflow) transaction amount over the past 60 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount60_d: Option<f64>,
    ///The total credit (inflow) transaction amount over the past 90 days from the account that will be debited
    #[serde(rename = "total_credit_transactions_amount_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_credit_transactions_amount90_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 10 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_10d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount10_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 30 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount30_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 60 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount60_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 90 days from the account that will be debited
    #[serde(rename = "total_debit_transactions_amount_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_debit_transactions_amount90_d: Option<f64>,
    ///The total number of times the Item has been connected to applications via Plaid
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_plaid_connections_count: Option<i64>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the transactions for the given account have been updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transactions_last_updated: Option<chrono::DateTime<chrono::Utc>>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count30_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count60_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_7d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count7_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited.
    #[serde(rename = "unauthorized_transactions_count_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_transactions_count90_d: Option<i64>,
}
impl std::fmt::Display for SignalEvaluateCoreAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}