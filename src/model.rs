use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/auth/get` results.
    pub options: Option<AuthGetRequestOptions>,
}
impl std::fmt::Display for AuthGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetRequestOptions {
    #[serde(rename = "account_ids")]
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AuthGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetResponse {
    #[serde(rename = "accounts")]
    ///The `accounts` for which numbers are being retrieved.
    pub accounts: Vec<AccountBase>,
    #[serde(rename = "numbers")]
    ///An object containing identifying numbers used for making electronic transfers to and from the `accounts`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by any `accounts` for which data has been requested, the array for that type will be empty.
    pub numbers: AuthGetNumbers,
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetNumbers {
    #[serde(rename = "ach")]
    ///An array of ACH numbers identifying accounts.
    pub ach: Vec<NumbersAch>,
    #[serde(rename = "eft")]
    ///An array of EFT numbers identifying accounts.
    pub eft: Vec<NumbersEft>,
    #[serde(rename = "international")]
    ///An array of IBAN numbers identifying accounts.
    pub international: Vec<NumbersInternational>,
    #[serde(rename = "bacs")]
    ///An array of BACS numbers identifying accounts.
    pub bacs: Vec<NumbersBacs>,
}
impl std::fmt::Display for AuthGetNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequest {
    #[serde(rename = "options")]
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: Option<TransactionsGetRequestOptions>,
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "start_date")]
    ///The earliest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    #[serde(rename = "end_date")]
    ///The latest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
}
impl std::fmt::Display for TransactionsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequestOptions {
    #[serde(rename = "account_ids")]
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "count")]
    ///The number of transactions to fetch.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of transactions to skip. The default value is 0.
    pub offset: Option<i64>,
    #[serde(rename = "include_original_description")]
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    pub include_original_description: Option<bool>,
    #[serde(rename = "include_personal_finance_category_beta")]
    ///Please use [`include_personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-get-request-options-include-personal-finance-category) instead.
    pub include_personal_finance_category_beta: Option<bool>,
    #[serde(rename = "include_personal_finance_category")]
    /**Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-personal-finance-category) object in the response.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.

We’re introducing Category Rules - a new beta endpoint that will enable you to change the `personal_finance_category` for a transaction based on your users’ needs. When rules are set, the selected category will override the Plaid provided category. To learn more, send a note to transactions-feedback@plaid.com.*/
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetResponse {
    #[serde(rename = "accounts")]
    ///An array containing the `accounts` associated with the Item for which transactions are being returned. Each transaction can be mapped to its corresponding account via the `account_id` field.
    pub accounts: Vec<AccountBase>,
    #[serde(rename = "transactions")]
    ///An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the `count` parameter.
    pub transactions: Vec<Transaction>,
    #[serde(rename = "total_transactions")]
    ///The total number of transactions available within the date range specified. If `total_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    pub total_transactions: i64,
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRefreshRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for TransactionsRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRefreshResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: Option<TransactionsRecurringGetRequestOptions>,
    #[serde(rename = "account_ids")]
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
}
impl std::fmt::Display for TransactionsRecurringGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequestOptions {
    #[serde(rename = "include_personal_finance_category")]
    /**Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-personal-finance-category) object for each transaction stream in the response.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.*/
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsRecurringGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetResponse {
    #[serde(rename = "inflow_streams")]
    ///An array of depository transaction streams.
    pub inflow_streams: Vec<TransactionStream>,
    #[serde(rename = "outflow_streams")]
    ///An array of expense transaction streams.
    pub outflow_streams: Vec<TransactionStream>,
    #[serde(rename = "updated_datetime")]
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time transaction streams for the given account were updated on
    pub updated_datetime: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsRecurringGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesCreateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "personal_finance_category")]
    /**Personal finance detailed category.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.
*/
    pub personal_finance_category: String,
    #[serde(rename = "rule_details")]
    ///A representation of transactions rule details.
    pub rule_details: TransactionsRuleDetails,
}
impl std::fmt::Display for TransactionsRulesCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesCreateResponse {
    #[serde(rename = "rule")]
    ///A representation of a transactions category rule.
    pub rule: TransactionsCategoryRule,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsRulesCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesListRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for TransactionsRulesListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesListResponse {
    #[serde(rename = "rules")]
    ///A list of the Item's transaction rules
    pub rules: Vec<TransactionsCategoryRule>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsRulesListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "rule_id")]
    ///A rule's unique identifier
    pub rule_id: String,
}
impl std::fmt::Display for TransactionsRulesRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsRulesRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "cursor")]
    /**The cursor value represents the last update requested. Providing it will cause the response to only return changes after this update.
If omitted, the entire history of updates will be returned, starting with the first-added transactions on the item.
Note: The upper-bound length of this cursor is 256 characters of base64.*/
    pub cursor: Option<String>,
    #[serde(rename = "count")]
    ///The number of transaction updates to fetch.
    pub count: Option<i64>,
    #[serde(rename = "options")]
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: Option<TransactionsSyncRequestOptions>,
}
impl std::fmt::Display for TransactionsSyncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncRequestOptions {
    #[serde(rename = "include_original_description")]
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    pub include_original_description: Option<bool>,
    #[serde(rename = "include_personal_finance_category")]
    /**Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-sync-response-added-personal-finance-category) object in the response.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.

We’re introducing Category Rules - a new beta endpoint that will enable you to change the `personal_finance_category` for a transaction based on your users’ needs. When rules are set, the selected category will override the Plaid provided category. To learn more, send a note to transactions-feedback@plaid.com.*/
    pub include_personal_finance_category: Option<bool>,
}
impl std::fmt::Display for TransactionsSyncRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncResponse {
    #[serde(rename = "added")]
    ///Transactions that have been added to the item since `cursor` ordered by ascending last modified time.
    pub added: Vec<Transaction>,
    #[serde(rename = "modified")]
    ///Transactions that have been modified on the item since `cursor` ordered by ascending last modified time.
    pub modified: Vec<Transaction>,
    #[serde(rename = "removed")]
    ///Transactions that have been removed from the item since `cursor` ordered by ascending last modified time.
    pub removed: Vec<RemovedTransaction>,
    #[serde(rename = "next_cursor")]
    ///Cursor used for fetching any future updates after the latest update provided in this response.
    pub next_cursor: String,
    #[serde(rename = "has_more")]
    ///Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`.
    pub has_more: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransactionsSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetRequest {
    #[serde(rename = "count")]
    ///The total number of Institutions to return.
    pub count: i64,
    #[serde(rename = "offset")]
    ///The number of Institutions to skip.
    pub offset: i64,
    #[serde(rename = "country_codes")]
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard.

In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<String>,
    #[serde(rename = "options")]
    ///An optional object to filter `/institutions/get` results.
    pub options: Option<InstitutionsGetRequestOptions>,
}
impl std::fmt::Display for InstitutionsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetRequestOptions {
    #[serde(rename = "products")]
    ///Filter the Institutions based on which products they support.
    pub products: Option<Vec<String>>,
    #[serde(rename = "routing_numbers")]
    ///Specify an array of routing numbers to filter institutions. The response will only return institutions that match all of the routing numbers in the array. Routing number records used for this matching are not comprehensive; failure to match a given routing number to an institution does not mean that the institution is unsupported by Plaid.
    pub routing_numbers: Option<Vec<String>>,
    #[serde(rename = "oauth")]
    ///Limit results to institutions with or without OAuth login flows.
    pub oauth: Option<bool>,
    #[serde(rename = "include_optional_metadata")]
    /**When `true`, return the institution's homepage URL, logo and primary brand color.

Note that Plaid does not own any of the logos shared by the API, and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    pub include_optional_metadata: Option<bool>,
    #[serde(rename = "include_auth_metadata")]
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    #[serde(rename = "include_payment_initiation_metadata")]
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
}
impl std::fmt::Display for InstitutionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetResponse {
    #[serde(rename = "institutions")]
    ///A list of Plaid institutions
    pub institutions: Vec<Institution>,
    #[serde(rename = "total")]
    ///The total number of institutions available via this endpoint
    pub total: i64,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchRequest {
    #[serde(rename = "query")]
    ///The search query. Institutions with names matching the query are returned
    pub query: String,
    #[serde(rename = "products")]
    ///Filter the Institutions based on whether they support all products listed in `products`. Provide `null` to get institutions regardless of supported products. Note that when `auth` is specified as a product, if you are enabled for Instant Match or Automated Micro-deposits, institutions that support those products will be returned even if `auth` is not present in their product array.
    pub products: Option<Vec<String>>,
    #[serde(rename = "country_codes")]
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<String>,
    #[serde(rename = "options")]
    ///An optional object to filter `/institutions/search` results.
    pub options: Option<InstitutionsSearchRequestOptions>,
}
impl std::fmt::Display for InstitutionsSearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchRequestOptions {
    #[serde(rename = "oauth")]
    ///Limit results to institutions with or without OAuth login flows.
    pub oauth: Option<bool>,
    #[serde(rename = "include_optional_metadata")]
    ///When true, return the institution's homepage URL, logo and primary brand color.
    pub include_optional_metadata: Option<bool>,
    #[serde(rename = "include_auth_metadata")]
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    #[serde(rename = "include_payment_initiation_metadata")]
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
    #[serde(rename = "payment_initiation")]
    ///Additional options that will be used to filter institutions by various Payment Initiation configurations.
    pub payment_initiation: Option<InstitutionsSearchPaymentInitiationOptions>,
}
impl std::fmt::Display for InstitutionsSearchRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    #[serde(rename = "payment_id")]
    ///A unique ID identifying the payment
    pub payment_id: Option<String>,
    #[serde(rename = "consent_id")]
    ///A unique ID identifying the payment consent
    pub consent_id: Option<String>,
}
impl std::fmt::Display for InstitutionsSearchPaymentInitiationOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchResponse {
    #[serde(rename = "institutions")]
    ///An array of institutions matching the search criteria
    pub institutions: Vec<Institution>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequest {
    #[serde(rename = "institution_id")]
    ///The ID of the institution to get details about
    pub institution_id: String,
    #[serde(rename = "country_codes")]
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<String>,
    #[serde(rename = "options")]
    ///Specifies optional parameters for `/institutions/get_by_id`. If provided, must not be `null`.
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
impl std::fmt::Display for InstitutionsGetByIdRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequestOptions {
    #[serde(rename = "include_optional_metadata")]
    /**When `true`, return an institution's logo, brand color, and URL. When available, the bank's logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format. The default value is `false`.

Note that Plaid does not own any of the logos shared by the API and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    pub include_optional_metadata: Option<bool>,
    #[serde(rename = "include_status")]
    ///If `true`, the response will include status information about the institution. Default value is `false`.
    pub include_status: Option<bool>,
    #[serde(rename = "include_auth_metadata")]
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    #[serde(rename = "include_payment_initiation_metadata")]
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
}
impl std::fmt::Display for InstitutionsGetByIdRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdResponse {
    #[serde(rename = "institution")]
    ///Details relating to a specific financial institution
    pub institution: Institution,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InstitutionsGetByIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/accounts/get` results.
    pub options: Option<AccountsGetRequestOptions>,
}
impl std::fmt::Display for AccountsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequestOptions {
    #[serde(rename = "account_ids")]
    ///An array of `account_ids` to retrieve for the Account.
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for AccountsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    #[serde(rename = "accounts")]
    /**An array of financial institution accounts associated with the Item.
If `/accounts/balance/get` was called, each account will include real-time balance information.*/
    pub accounts: Vec<AccountBase>,
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AccountsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesGetRequest {}
impl std::fmt::Display for CategoriesGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesGetResponse {
    #[serde(rename = "categories")]
    ///An array of all of the transaction categories used by Plaid.
    pub categories: Vec<Category>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CategoriesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOverridePassword(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOverrideUsername(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequest {
    #[serde(rename = "institution_id")]
    ///The ID of the institution the Item will be associated with
    pub institution_id: String,
    #[serde(rename = "options")]
    ///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub options: Option<SandboxProcessorTokenCreateRequestOptions>,
}
impl std::fmt::Display for SandboxProcessorTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    #[serde(rename = "override_username")]
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    pub override_username: Option<String>,
    #[serde(rename = "override_password")]
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    pub override_password: Option<String>,
}
impl std::fmt::Display for SandboxProcessorTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateResponse {
    #[serde(rename = "processor_token")]
    ///A processor token that can be used to call the `/processor/` endpoints.
    pub processor_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxProcessorTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequest {
    #[serde(rename = "institution_id")]
    ///The ID of the institution the Item will be associated with
    pub institution_id: String,
    #[serde(rename = "initial_products")]
    ///The products to initially pull for the Item. May be any products that the specified `institution_id`  supports. This array may not be empty.
    pub initial_products: Vec<String>,
    #[serde(rename = "options")]
    ///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub options: Option<SandboxPublicTokenCreateRequestOptions>,
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: Option<String>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptions {
    #[serde(rename = "webhook")]
    ///Specify a webhook to associate with the new Item.
    pub webhook: Option<String>,
    #[serde(rename = "override_username")]
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    pub override_username: Option<String>,
    #[serde(rename = "override_password")]
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    pub override_password: Option<String>,
    #[serde(rename = "transactions")]
    ///An optional set of parameters corresponding to transactions options.
    pub transactions: Option<SandboxPublicTokenCreateRequestOptionsTransactions>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptionsTransactions {
    #[serde(rename = "start_date")]
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: Option<String>,
}
impl std::fmt::Display for SandboxPublicTokenCreateRequestOptionsTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    #[serde(rename = "public_token")]
    ///A public token that can be exchanged for an access token using `/item/public_token/exchange`
    pub public_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxPublicTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "webhook_type")]
    ///The webhook types that can be fired by this test endpoint.
    pub webhook_type: Option<String>,
    #[serde(rename = "webhook_code")]
    ///The webhook codes that can be fired by this test endpoint.
    pub webhook_code: String,
}
impl std::fmt::Display for SandboxItemFireWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WebhookType {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "HOLDINGS")]
    Holdings,
    #[serde(rename = "INVESTMENTS_TRANSACTIONS")]
    InvestmentsTransactions,
    #[serde(rename = "ITEM")]
    Item,
    #[serde(rename = "LIABILITIES")]
    Liabilities,
    #[serde(rename = "TRANSACTIONS")]
    Transactions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookResponse {
    #[serde(rename = "webhook_fired")]
    ///Value is `true`  if the test` webhook_code`  was successfully fired.
    pub webhook_fired: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxItemFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/accounts/balance/get` results.
    pub options: Option<AccountsBalanceGetRequestOptions>,
}
impl std::fmt::Display for AccountsBalanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequestOptions {
    #[serde(rename = "account_ids")]
    /**A list of `account_ids` to retrieve for the Item. The default value is `null`.

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "min_last_updated_datetime")]
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.

This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.*/
    pub min_last_updated_datetime: Option<String>,
}
impl std::fmt::Display for AccountsBalanceGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MinLastUpdatedDatetime(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/identity/get` results.
    pub options: Option<IdentityGetRequestOptions>,
}
impl std::fmt::Display for IdentityGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetRequestOptions {
    #[serde(rename = "account_ids")]
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for IdentityGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetResponse {
    #[serde(rename = "accounts")]
    ///The accounts for which Identity data has been requested
    pub accounts: Vec<AccountIdentity>,
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IdentityGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "user")]
    ///The user's legal name, phone number, email address and address used to perform fuzzy match.
    pub user: Option<IdentityMatchUser>,
    #[serde(rename = "options")]
    ///An optional object to filter /identity/match results
    pub options: Option<IdentityMatchRequestOptions>,
}
impl std::fmt::Display for IdentityMatchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchRequestOptions {
    #[serde(rename = "account_ids")]
    ///An array of `account_ids` to perform fuzzy match
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for IdentityMatchRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchUser {
    #[serde(rename = "legal_name")]
    ///The user's full legal name.
    pub legal_name: Option<String>,
    #[serde(rename = "phone_number")]
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    pub phone_number: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address.
    pub email_address: Option<String>,
    #[serde(rename = "address")]
    ///Data about the components comprising an address.
    pub address: Option<AddressDataNullable>,
}
impl std::fmt::Display for IdentityMatchUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<String>,
}
impl std::fmt::Display for IdentityMatchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorAuthGetRequest {
    #[serde(rename = "processor_token")]
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: String,
}
impl std::fmt::Display for ProcessorAuthGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorAuthGetResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "numbers")]
    ///An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.
    pub numbers: ProcessorNumber,
    #[serde(rename = "account")]
    ///A single account at a financial institution.
    pub account: AccountBase,
}
impl std::fmt::Display for ProcessorAuthGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateRequest {
    #[serde(rename = "idempotency_key")]
    /**A random key provided by the client, per unique bank transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created.*/
    pub idempotency_key: String,
    #[serde(rename = "processor_token")]
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: String,
    #[serde(rename = "amount")]
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount – should be set to "USD".
    pub iso_currency_code: String,
    #[serde(rename = "description")]
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: Option<String>,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    #[serde(rename = "custom_tag")]
    ///An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters.
    pub custom_tag: Option<String>,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<BankTransferMetadata>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified.
    pub origination_account_id: Option<String>,
}
impl std::fmt::Display for ProcessorBankTransferCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateResponse {
    #[serde(rename = "bank_transfer")]
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorBankTransferCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorNumber {
    #[serde(rename = "ach")]
    ///Identifying information for transferring money to or from a US account via ACH or wire transfer.
    pub ach: Option<NumbersAchNullable>,
    #[serde(rename = "eft")]
    ///Identifying information for transferring money to or from a Canadian bank account via EFT.
    pub eft: Option<NumbersEftNullable>,
    #[serde(rename = "international")]
    ///Identifying information for transferring money to or from an international bank account via wire transfer.
    pub international: Option<NumbersInternationalNullable>,
    #[serde(rename = "bacs")]
    ///Identifying information for transferring money to or from a UK bank account via BACS.
    pub bacs: Option<NumbersBacsNullable>,
}
impl std::fmt::Display for ProcessorNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorIdentityGetRequest {
    #[serde(rename = "processor_token")]
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: String,
}
impl std::fmt::Display for ProcessorIdentityGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorIdentityGetResponse {
    #[serde(rename = "account")]
    ///Identity information about an account
    pub account: AccountIdentity,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorIdentityGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequest {
    #[serde(rename = "processor_token")]
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/processor/balance/get` results.
    pub options: Option<ProcessorBalanceGetRequestOptions>,
}
impl std::fmt::Display for ProcessorBalanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequestOptions {
    #[serde(rename = "min_last_updated_datetime")]
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.

This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.*/
    pub min_last_updated_datetime: Option<String>,
}
impl std::fmt::Display for ProcessorBalanceGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetResponse {
    #[serde(rename = "account")]
    ///A single account at a financial institution.
    pub account: AccountBase,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetRequest {
    #[serde(rename = "key_id")]
    ///The key ID ( `kid` ) from the JWT header.
    pub key_id: String,
}
impl std::fmt::Display for WebhookVerificationKeyGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetResponse {
    #[serde(rename = "key")]
    ///A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
    pub key: JwkPublicKey,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WebhookVerificationKeyGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JwkPublicKey {
    #[serde(rename = "alg")]
    ///The alg member identifies the cryptographic algorithm family used with the key.
    pub alg: String,
    #[serde(rename = "crv")]
    ///The crv member identifies the cryptographic curve used with the key.
    pub crv: String,
    #[serde(rename = "kid")]
    ///The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover.
    pub kid: String,
    #[serde(rename = "kty")]
    ///The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC.
    pub kty: String,
    #[serde(rename = "use")]
    ///The use (public key use) parameter identifies the intended use of the public key.
    pub use_: String,
    #[serde(rename = "x")]
    ///The x member contains the x coordinate for the elliptic curve point.
    pub x: String,
    #[serde(rename = "y")]
    ///The y member contains the y coordinate for the elliptic curve point.
    pub y: String,
    #[serde(rename = "created_at")]
    ///The timestamp when the key was created, in Unix time.
    pub created_at: i64,
    #[serde(rename = "expired_at")]
    ///The timestamp when the key expired, in Unix time.
    pub expired_at: Option<i64>,
}
impl std::fmt::Display for JwkPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/liabilities/get` results. If provided, `options` cannot be null.
    pub options: Option<LiabilitiesGetRequestOptions>,
}
impl std::fmt::Display for LiabilitiesGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetRequestOptions {
    #[serde(rename = "account_ids")]
    /**A list of accounts to retrieve for the Item.

An error will be returned if a provided `account_id` is not associated with the Item*/
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for LiabilitiesGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetResponse {
    #[serde(rename = "accounts")]
    ///An array of accounts associated with the Item
    pub accounts: Vec<AccountBase>,
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "liabilities")]
    ///An object containing liability accounts
    pub liabilities: LiabilitiesObject,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LiabilitiesGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateRequest {
    #[serde(rename = "name")]
    ///The name of the recipient. We recommend using strings of length 18 or less and avoid special characters to ensure compatibility with all institutions.
    pub name: String,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the recipient. If BACS data is not provided, an IBAN is required.
    pub iban: Option<String>,
    #[serde(rename = "bacs")]
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacsNullable>,
    #[serde(rename = "address")]
    ///The optional address of the payment recipient.
    pub address: Option<PaymentInitiationAddress>,
}
impl std::fmt::Display for PaymentInitiationRecipientCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateResponse {
    #[serde(rename = "recipient_id")]
    ///A unique ID identifying the recipient
    pub recipient_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipientCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationRefundStatus {
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "EXECUTED")]
    Executed,
    #[serde(rename = "INITIATED")]
    Initiated,
    #[serde(rename = "FAILED")]
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseResponse {
    #[serde(rename = "refund_id")]
    ///A unique ID identifying the refund
    pub refund_id: String,
    #[serde(rename = "status")]
    /**The status of the refund.

`PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.

`INITIATED`: The refund has been successfully initiated.

`EXECUTED`: Indicates that the refund has been successfully executed.

`FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved.*/
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentReverseResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetRequest {
    #[serde(rename = "recipient_id")]
    ///The ID of the recipient
    pub recipient_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipientGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipient {
    #[serde(rename = "recipient_id")]
    ///The ID of the recipient.
    pub recipient_id: String,
    #[serde(rename = "name")]
    ///The name of the recipient.
    pub name: String,
    #[serde(rename = "address")]
    ///The optional address of the payment recipient.
    pub address: Option<PaymentInitiationAddress>,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the recipient.
    pub iban: Option<String>,
    #[serde(rename = "bacs")]
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacsNullable>,
}
impl std::fmt::Display for PaymentInitiationRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListRequest {}
impl std::fmt::Display for PaymentInitiationRecipientListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListResponse {
    #[serde(rename = "recipients")]
    ///An array of payment recipients created for Payment Initiation
    pub recipients: Vec<PaymentInitiationRecipient>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationRecipientListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    #[serde(rename = "recipient_id")]
    ///The ID of the recipient the payment is for.
    pub recipient_id: String,
    #[serde(rename = "reference")]
    ///A reference for the payment. This must be an alphanumeric string with at most 18 characters and must not contain any special characters (since not all institutions support them).
    pub reference: String,
    #[serde(rename = "amount")]
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    #[serde(rename = "schedule")]
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    pub schedule: Option<ExternalPaymentScheduleRequest>,
    #[serde(rename = "options")]
    ///Additional payment options
    pub options: Option<ExternalPaymentOptions>,
}
impl std::fmt::Display for PaymentInitiationPaymentCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    #[serde(rename = "payment_id")]
    ///The ID of the payment to reverse
    pub payment_id: String,
    #[serde(rename = "idempotency_key")]
    /**A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: String,
    #[serde(rename = "reference")]
    ///A reference for the refund. This must be an alphanumeric string with at most 18 characters and must not contain any special characters or spaces.
    pub reference: String,
}
impl std::fmt::Display for PaymentInitiationPaymentReverseRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentCreateStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    PaymentStatusInputNeeded,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateResponse {
    #[serde(rename = "payment_id")]
    ///A unique ID identifying the payment
    pub payment_id: String,
    #[serde(rename = "status")]
    /**For a payment returned by this endpoint, there is only one possible value:

`PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment*/
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemResetLoginRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for SandboxItemResetLoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemResetLoginResponse {
    #[serde(rename = "reset_login")]
    ///`true` if the call succeeded
    pub reset_login: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxItemResetLoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The `account_id` of the account whose verification status is to be modified
    pub account_id: String,
    #[serde(rename = "verification_status")]
    ///The verification status to set the account to.
    pub verification_status: String,
}
impl std::fmt::Display for SandboxItemSetVerificationStatusRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxItemSetVerificationStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequest {
    #[serde(rename = "client_user_id")]
    ///A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    pub client_user_id: String,
}
impl std::fmt::Display for UserCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateResponse {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: String,
    #[serde(rename = "user_id")]
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for UserCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetRequest {
    #[serde(rename = "payment_id")]
    ///The `payment_id` returned from `/payment_initiation/payment/create`.
    pub payment_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentStatus {
    #[serde(rename = "PAYMENT_STATUS_INPUT_NEEDED")]
    PaymentStatusInputNeeded,
    #[serde(rename = "PAYMENT_STATUS_PROCESSING")]
    PaymentStatusProcessing,
    #[serde(rename = "PAYMENT_STATUS_INITIATED")]
    PaymentStatusInitiated,
    #[serde(rename = "PAYMENT_STATUS_COMPLETED")]
    PaymentStatusCompleted,
    #[serde(rename = "PAYMENT_STATUS_INSUFFICIENT_FUNDS")]
    PaymentStatusInsufficientFunds,
    #[serde(rename = "PAYMENT_STATUS_FAILED")]
    PaymentStatusFailed,
    #[serde(rename = "PAYMENT_STATUS_BLOCKED")]
    PaymentStatusBlocked,
    #[serde(rename = "PAYMENT_STATUS_UNKNOWN")]
    PaymentStatusUnknown,
    #[serde(rename = "PAYMENT_STATUS_EXECUTED")]
    PaymentStatusExecuted,
    #[serde(rename = "PAYMENT_STATUS_AUTHORISING")]
    PaymentStatusAuthorising,
    #[serde(rename = "PAYMENT_STATUS_CANCELLED")]
    PaymentStatusCancelled,
    #[serde(rename = "PAYMENT_STATUS_ESTABLISHED")]
    PaymentStatusEstablished,
    #[serde(rename = "PAYMENT_STATUS_REJECTED")]
    PaymentStatusRejected,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPayment {
    #[serde(rename = "payment_id")]
    ///The ID of the payment. Like all Plaid identifiers, the `payment_id` is case sensitive.
    pub payment_id: String,
    #[serde(rename = "amount")]
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    #[serde(rename = "status")]
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub status: String,
    #[serde(rename = "recipient_id")]
    ///The ID of the recipient
    pub recipient_id: String,
    #[serde(rename = "reference")]
    ///A reference for the payment.
    pub reference: String,
    #[serde(rename = "adjusted_reference")]
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    pub adjusted_reference: Option<String>,
    #[serde(rename = "last_status_update")]
    ///The date and time of the last time the `status` was updated, in IS0 8601 format
    pub last_status_update: String,
    #[serde(rename = "schedule")]
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    pub schedule: Option<ExternalPaymentScheduleGet>,
    #[serde(rename = "refund_details")]
    ///Details about external payment refund
    pub refund_details: Option<ExternalPaymentRefundDetails>,
    #[serde(rename = "bacs")]
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<SenderBacsNullable>,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the sender, if specified in the `/payment_initiation/payment/create` call.
    pub iban: Option<String>,
    #[serde(rename = "refund_ids")]
    ///Refund IDs associated with the payment.
    pub refund_ids: Option<Vec<String>>,
    #[serde(rename = "wallet_id")]
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    #[serde(rename = "scheme")]
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub scheme: Option<String>,
    #[serde(rename = "adjusted_scheme")]
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub adjusted_scheme: Option<String>,
    #[serde(rename = "consent_id")]
    ///The payment consent ID that this payment was initiated with. Is present only when payment was initiated using the payment consent.
    pub consent_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateRequest {
    #[serde(rename = "payment_id")]
    ///The `payment_id` returned from `/payment_initiation/payment/create`.
    pub payment_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateResponse {
    #[serde(rename = "payment_token")]
    ///A `payment_token` that can be provided to Link initialization to enter the payment initiation flow
    pub payment_token: String,
    #[serde(rename = "payment_token_expiration_time")]
    ///The date and time at which the token will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `payment_token` expires after 15 minutes.
    pub payment_token_expiration_time: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequest {
    #[serde(rename = "recipient_id")]
    ///The ID of the recipient the payment consent is for. The created consent can be used to transfer funds to this recipient only.
    pub recipient_id: String,
    #[serde(rename = "reference")]
    ///A reference for the payment consent. This must be an alphanumeric string with at most 18 characters and must not contain any special characters.
    pub reference: String,
    #[serde(rename = "scopes")]
    ///An array of payment consent scopes.
    pub scopes: Vec<String>,
    #[serde(rename = "constraints")]
    ///Limitations that will be applied to payments initiated using the payment consent.
    pub constraints: PaymentInitiationConsentConstraints,
    #[serde(rename = "options")]
    ///Additional payment consent options
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
}
impl std::fmt::Display for PaymentInitiationConsentCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateResponse {
    #[serde(rename = "consent_id")]
    ///A unique ID identifying the payment consent.
    pub consent_id: String,
    #[serde(rename = "status")]
    /**The status of the payment consent.

`UNAUTHORISED`: Consent created, but requires user authorisation.

`REJECTED`: Consent authorisation was rejected by the user and/or the bank.

`AUTHORISED`: Consent is active and ready to be used.

`REVOKED`: Consent has been revoked and can no longer be used.

`EXPIRED`: Consent is no longer valid.*/
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationConsentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentGetRequest {
    #[serde(rename = "consent_id")]
    ///The `consent_id` returned from `/payment_initiation/consent/create`.
    pub consent_id: String,
}
impl std::fmt::Display for PaymentInitiationConsentGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentGetResponse {}
impl std::fmt::Display for PaymentInitiationConsentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsent {
    #[serde(rename = "consent_id")]
    ///The consent ID.
    pub consent_id: String,
    #[serde(rename = "status")]
    /**The status of the payment consent.

`UNAUTHORISED`: Consent created, but requires user authorisation.

`REJECTED`: Consent authorisation was rejected by the user and/or the bank.

`AUTHORISED`: Consent is active and ready to be used.

`REVOKED`: Consent has been revoked and can no longer be used.

`EXPIRED`: Consent is no longer valid.*/
    pub status: String,
    #[serde(rename = "created_at")]
    ///Consent creation timestamp, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: String,
    #[serde(rename = "recipient_id")]
    ///The ID of the recipient the payment consent is for.
    pub recipient_id: String,
    #[serde(rename = "reference")]
    ///A reference for the payment consent.
    pub reference: String,
    #[serde(rename = "constraints")]
    ///Limitations that will be applied to payments initiated using the payment consent.
    pub constraints: PaymentInitiationConsentConstraints,
    #[serde(rename = "scopes")]
    ///An array of payment consent scopes.
    pub scopes: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationConsent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationConsentStatus {
    #[serde(rename = "UNAUTHORISED")]
    Unauthorised,
    #[serde(rename = "AUTHORISED")]
    Authorised,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "EXPIRED")]
    Expired,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentRevokeRequest {
    #[serde(rename = "consent_id")]
    ///The consent ID.
    pub consent_id: String,
}
impl std::fmt::Display for PaymentInitiationConsentRevokeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentRevokeResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationConsentRevokeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteRequest {
    #[serde(rename = "consent_id")]
    ///The consent ID.
    pub consent_id: String,
    #[serde(rename = "amount")]
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    #[serde(rename = "idempotency_key")]
    /**A random key provided by the client, per unique consent payment. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a consent payment fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single payment is created. If the request was successfully processed, it will prevent any payment that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: String,
}
impl std::fmt::Display for PaymentInitiationConsentPaymentExecuteRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteResponse {
    #[serde(rename = "payment_id")]
    ///A unique ID identifying the payment
    pub payment_id: String,
    #[serde(rename = "status")]
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationConsentPaymentExecuteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListRequest {
    #[serde(rename = "count")]
    ///The maximum number of payments to return. If `count` is not specified, a maximum of 10 payments will be returned, beginning with the most recent payment before the cursor (if specified).
    pub count: Option<i64>,
    #[serde(rename = "cursor")]
    ///A string in RFC 3339 format (i.e. "2019-12-06T22:35:49Z"). Only payments created before the cursor will be returned.
    pub cursor: Option<String>,
    #[serde(rename = "consent_id")]
    ///The consent ID. If specified, only payments, executed using this consent, will be returned.
    pub consent_id: Option<String>,
}
impl std::fmt::Display for PaymentInitiationPaymentListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListResponse {
    #[serde(rename = "payments")]
    ///An array of payments that have been created, associated with the given `client_id`.
    pub payments: Vec<PaymentInitiationPayment>,
    #[serde(rename = "next_cursor")]
    ///The value that, when used as the optional `cursor` parameter to `/payment_initiation/payment/list`, will return the next unreturned payment as its first payment.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentInitiationPaymentListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateRequest {
    #[serde(rename = "access_tokens")]
    ///An array of access tokens corresponding to the Items that will be included in the report. The `assets` product must have been initialized for the Items during link; the Assets product cannot be added after initialization.
    pub access_tokens: Vec<String>,
    #[serde(rename = "days_requested")]
    /**The maximum integer number of days of history to include in the Asset Report. If using Fannie Mae Day 1 Certainty, `days_requested` must be at least 61 for new originations or at least 31 for refinancings.

An Asset Report requested with "Additional History" (that is, with more than 61 days of transaction history) will incur an Additional History fee.*/
    pub days_requested: i64,
    #[serde(rename = "options")]
    ///An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
    pub options: Option<AssetReportCreateRequestOptions>,
}
impl std::fmt::Display for AssetReportCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateRequestOptions {
    #[serde(rename = "client_report_id")]
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    pub client_report_id: Option<String>,
    #[serde(rename = "webhook")]
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    pub webhook: Option<String>,
    #[serde(rename = "include_fast_report")]
    ///true to return balance and identity earlier as a fast report. Defaults to false if omitted.
    pub include_fast_report: Option<bool>,
    #[serde(rename = "products")]
    ///Additional information that can be included in the asset report. Possible values: `"investments"`
    pub products: Option<Vec<String>>,
    #[serde(rename = "user")]
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: Option<AssetReportUser>,
}
impl std::fmt::Display for AssetReportCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateResponse {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "asset_report_id")]
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshRequest {
    #[serde(rename = "asset_report_token")]
    ///The `asset_report_token` returned by the original call to `/asset_report/create`
    pub asset_report_token: String,
    #[serde(rename = "days_requested")]
    ///The maximum number of days of history to include in the Asset Report. Must be an integer. If not specified, the value from the original call to `/asset_report/create` will be used.
    pub days_requested: Option<i64>,
    #[serde(rename = "options")]
    ///An optional object to filter `/asset_report/refresh` results. If provided, cannot be `null`. If not specified, the `options` from the original call to `/asset_report/create` will be used.
    pub options: Option<AssetReportRefreshRequestOptions>,
}
impl std::fmt::Display for AssetReportRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshRequestOptions {
    #[serde(rename = "client_report_id")]
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    pub client_report_id: Option<String>,
    #[serde(rename = "webhook")]
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    pub webhook: Option<String>,
    #[serde(rename = "user")]
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: Option<AssetReportUser>,
}
impl std::fmt::Display for AssetReportRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshResponse {
    #[serde(rename = "asset_report_id")]
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRefreshRequest {
    #[serde(rename = "asset_relay_token")]
    pub asset_relay_token: String,
    #[serde(rename = "webhook")]
    ///The URL registered to receive webhooks when the Asset Report of a Relay Token has been refreshed.
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportRelayRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRefreshResponse {
    #[serde(rename = "asset_relay_token")]
    pub asset_relay_token: String,
    #[serde(rename = "asset_report_id")]
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportRelayRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRemoveRequest {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
}
impl std::fmt::Display for AssetReportRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRemoveResponse {
    #[serde(rename = "removed")]
    ///`true` if the Asset Report was successfully removed.
    pub removed: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportFilterRequest {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "account_ids_to_exclude")]
    ///The accounts to exclude from the Asset Report, identified by `account_id`.
    pub account_ids_to_exclude: Vec<String>,
}
impl std::fmt::Display for AssetReportFilterRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportFilterResponse {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "asset_report_id")]
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportFilterResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportGetRequest {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "include_insights")]
    ///`true` if you would like to retrieve the Asset Report with Insights, `false` otherwise. This field defaults to `false` if omitted.
    pub include_insights: Option<bool>,
    #[serde(rename = "fast_report")]
    ///`true` to fetch "fast" version of asset report. Defaults to false if omitted.
    pub fast_report: Option<bool>,
}
impl std::fmt::Display for AssetReportGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportGetResponse {
    #[serde(rename = "report")]
    ///An object representing an Asset Report
    pub report: AssetReport,
    #[serde(rename = "warnings")]
    ///If the Asset Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    pub warnings: Vec<Warning>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportPdfGetRequest {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
}
impl std::fmt::Display for AssetReportPdfGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportPdfGetResponse(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateRequest {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "auditor_id")]
    ///The `auditor_id` of the third party with whom you would like to share the Asset Report.
    pub auditor_id: String,
}
impl std::fmt::Display for AssetReportAuditCopyCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateResponse {
    #[serde(rename = "audit_copy_token")]
    ///A token that can be shared with a third party auditor to allow them to obtain access to the Asset Report. This token should be stored securely.
    pub audit_copy_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportAuditCopyCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveRequest {
    #[serde(rename = "audit_copy_token")]
    ///The `audit_copy_token` granting access to the Audit Copy you would like to revoke.
    pub audit_copy_token: String,
}
impl std::fmt::Display for AssetReportAuditCopyRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveResponse {
    #[serde(rename = "removed")]
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportAuditCopyRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayCreateRequest {
    #[serde(rename = "asset_report_token")]
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: String,
    #[serde(rename = "secondary_client_id")]
    ///The `secondary_client_id` is the client id of the third party with whom you would like to share the Asset Report.
    pub secondary_client_id: String,
    #[serde(rename = "webhook")]
    ///URL to which Plaid will send webhooks when the Secondary Client successfully retrieves an Asset Report by calling `asset_report/relay/get`.
    pub webhook: Option<String>,
}
impl std::fmt::Display for AssetReportRelayCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayCreateResponse {
    #[serde(rename = "asset_relay_token")]
    ///A token that can be shared with a third party to allow them to access the Asset Report. This token should be stored securely.
    pub asset_relay_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportRelayCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayGetRequest {
    #[serde(rename = "asset_relay_token")]
    ///The `asset_relay_token` granting access to the Asset Report you would like to get.
    pub asset_relay_token: String,
}
impl std::fmt::Display for AssetReportRelayGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRemoveRequest {
    #[serde(rename = "asset_relay_token")]
    ///The `asset_relay_token` you would like to revoke.
    pub asset_relay_token: String,
}
impl std::fmt::Display for AssetReportRelayRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRemoveResponse {
    #[serde(rename = "removed")]
    ///`true` if the Asset Relay token was successfully removed.
    pub removed: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for AssetReportRelayRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/investments/holdings/get` results. If provided, must not be `null`.
    pub options: Option<InvestmentHoldingsGetRequestOptions>,
}
impl std::fmt::Display for InvestmentsHoldingsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentHoldingsGetRequestOptions {
    #[serde(rename = "account_ids")]
    ///An array of `account_id`s to retrieve for the Item. An error will be returned if a provided `account_id` is not associated with the Item.
    pub account_ids: Option<Vec<String>>,
}
impl std::fmt::Display for InvestmentHoldingsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetResponse {
    #[serde(rename = "accounts")]
    ///The accounts associated with the Item
    pub accounts: Vec<AccountBase>,
    #[serde(rename = "holdings")]
    ///The holdings belonging to investment accounts associated with the Item. Details of the securities in the holdings are provided in the `securities` field.
    pub holdings: Vec<Holding>,
    #[serde(rename = "securities")]
    ///Objects describing the securities held in the accounts associated with the Item.
    pub securities: Vec<Security>,
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InvestmentsHoldingsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "start_date")]
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    #[serde(rename = "end_date")]
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
    #[serde(rename = "options")]
    ///An optional object to filter `/investments/transactions/get` results. If provided, must be non-`null`.
    pub options: Option<InvestmentsTransactionsGetRequestOptions>,
}
impl std::fmt::Display for InvestmentsTransactionsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequestOptions {
    #[serde(rename = "account_ids")]
    ///An array of `account_ids` to retrieve for the Item.
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "count")]
    /**The number of transactions to fetch.
*/
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of transactions to skip when fetching transaction history
    pub offset: Option<i64>,
}
impl std::fmt::Display for InvestmentsTransactionsGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetResponse {
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "accounts")]
    ///The accounts for which transaction history is being fetched.
    pub accounts: Vec<AccountBase>,
    #[serde(rename = "securities")]
    ///All securities for which there is a corresponding transaction being fetched.
    pub securities: Vec<Security>,
    #[serde(rename = "investment_transactions")]
    ///The transactions being fetched
    pub investment_transactions: Vec<InvestmentTransaction>,
    #[serde(rename = "total_investment_transactions")]
    ///The total number of transactions available within the date range specified. If `total_investment_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.'
    pub total_investment_transactions: i64,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for InvestmentsTransactionsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
    #[serde(rename = "processor")]
    ///The processor you are integrating with.
    pub processor: String,
}
impl std::fmt::Display for ProcessorTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorTokenCreateResponse {
    #[serde(rename = "processor_token")]
    ///The `processor_token` that can then be used by the Plaid partner to make API requests
    pub processor_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
}
impl std::fmt::Display for ProcessorStripeBankAccountTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    #[serde(rename = "stripe_bank_account_token")]
    ///A token that can be sent to Stripe for use in making API calls to Plaid
    pub stripe_bank_account_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProcessorStripeBankAccountTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorApexProcessorTokenCreateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
}
impl std::fmt::Display for ProcessorApexProcessorTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequest {
    #[serde(rename = "target_access_token")]
    ///Access token for the target Item, typically provided in the Import Item response.
    pub target_access_token: String,
    #[serde(rename = "target_account_id")]
    ///Plaid Account ID that specifies the target bank account. This account will become the recipient for a user's direct deposit.
    pub target_account_id: String,
    #[serde(rename = "country_code")]
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: Option<String>,
    #[serde(rename = "options")]
    ///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub options: Option<DepositSwitchCreateRequestOptions>,
}
impl std::fmt::Display for DepositSwitchCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequestOptions {
    #[serde(rename = "webhook")]
    /**The URL registered to receive webhooks when the status of a deposit switch request has changed.
*/
    pub webhook: Option<String>,
    #[serde(rename = "transaction_item_access_tokens")]
    ///An array of access tokens corresponding to transaction items to use when attempting to match the user to their Payroll Provider. These tokens must be created by the same client id as the one creating the switch, and have access to the transactions product.
    pub transaction_item_access_tokens: Option<Vec<String>>,
}
impl std::fmt::Display for DepositSwitchCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateResponse {
    #[serde(rename = "deposit_switch_id")]
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateRequest {
    #[serde(rename = "deposit_switch_id")]
    ///The ID of the deposit switch
    pub deposit_switch_id: String,
}
impl std::fmt::Display for DepositSwitchTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateResponse {
    #[serde(rename = "deposit_switch_token")]
    ///Deposit switch token, used to initialize Link for the Deposit Switch product
    pub deposit_switch_token: String,
    #[serde(rename = "deposit_switch_token_expiration_time")]
    ///Expiration time of the token, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub deposit_switch_token_expiration_time: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetRequest {
    #[serde(rename = "link_token")]
    ///A `link_token` from a previous invocation of `/link/token/create`
    pub link_token: String,
}
impl std::fmt::Display for LinkTokenGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    #[serde(rename = "client_name")]
    ///The name of your application, as it should be displayed in Link. Maximum length of 30 characters. If a value longer than 30 characters is provided, Link will display "This Application" instead.
    pub client_name: String,
    #[serde(rename = "language")]
    /**The language that Link should be displayed in.

Supported languages are:
- English (`'en'`)
- French (`'fr'`)
- Spanish (`'es'`)
- Dutch (`'nl'`)
- German(`'de'`)

When using a Link customization, the language configured here must match the setting in the customization, or the customization will not be applied.*/
    pub language: String,
    #[serde(rename = "country_codes")]
    /**Specify an array of Plaid-supported country codes using the ISO-3166-1 alpha-2 country code standard. Institutions from all listed countries will be shown.  Supported country codes are: `US`, `CA`, `DE`, `ES`, `FR`, `GB`, `IE`, `IT`, `NL`. For a complete mapping of supported products by country, see https://plaid.com/global/.

If Link is launched with multiple country codes, only products that you are enabled for in all countries will be used by Link. Note that while all countries are enabled by default in Sandbox and Development, in Production only US and Canada are enabled by default. To gain access to European institutions in the Production environment, [file a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access) via the Plaid dashboard. If you initialize with a European country code, your users will see the European consent panel during the Link flow.

If using a Link customization, make sure the country codes in the customization match those specified in `country_codes`. If both `country_codes` and a Link customization are used, the value in `country_codes` may override the value in the customization.

If using the Auth features Instant Match, Same-day Micro-deposits, or Automated Micro-deposits, `country_codes` must be set to `['US']`.*/
    pub country_codes: Vec<String>,
    #[serde(rename = "user")]
    ///An object specifying information about the end user who will be linking their account.
    pub user: LinkTokenCreateRequestUser,
    #[serde(rename = "products")]
    /**List of Plaid product(s) you wish to use. If launching Link in update mode, should be omitted; required otherwise.

`balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically be initialized when any other product is initialized.

The products specified here will determine which institutions will be available to your users in Link. Only institutions that support *all* requested products can be selected; a if a user attempts to select an institution that does not support a listed product, a "Connectivity not supported" error message will appear in Link. To maximize the number of institutions available, initialize Link with the minimal product set required for your use case. Additional products can be added after Link initialization by calling the relevant endpoints. For details and exceptions, see [Choosing when to initialize products](https://plaid.com/docs/link/best-practices/#choosing-when-to-initialize-products).

Note that, unless you have opted to disable Instant Match support, institutions that support Instant Match will also be shown in Link if `auth` is specified as a product, even though these institutions do not contain `auth` in their product array.

In Production, you will be billed for each product that you specify when initializing Link. Note that a product cannot be removed from an Item once the Item has been initialized with that product. To stop billing on an Item for subscription-based products, such as Liabilities, Investments, and Transactions, remove the Item via `/item/remove`.*/
    pub products: Option<Vec<String>>,
    #[serde(rename = "additional_consented_products")]
    /**(Beta) This field has no effect unless you are participating in the Product Scope Transparency beta program.
List of additional Plaid product(s) you wish to collect consent for. These products will not be billed until you start using them by calling the relevant endpoints.

`balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically have consent collected.

Institutions that do not support these products will still be shown in Link*/
    pub additional_consented_products: Option<Vec<String>>,
    #[serde(rename = "webhook")]
    ///The destination URL to which any webhooks should be sent.
    pub webhook: Option<String>,
    #[serde(rename = "access_token")]
    ///The `access_token` associated with the Item to update, used when updating or modifying an existing `access_token`. Used when launching Link in update mode, when completing the Same-day (manual) Micro-deposit flow, or (optionally) when initializing Link as part of the Payment Initiation (UK and Europe) flow.
    pub access_token: Option<String>,
    #[serde(rename = "link_customization_name")]
    ///The name of the Link customization from the Plaid Dashboard to be applied to Link. If not specified, the `default` customization will be used. When using a Link customization, the language in the customization must match the language selected via the `language` parameter, and the countries in the customization should match the country codes selected via `country_codes`.
    pub link_customization_name: Option<String>,
    #[serde(rename = "redirect_uri")]
    ///A URI indicating the destination where a user should be forwarded after completing the Link flow; used to support OAuth authentication flows when launching Link in the browser or via a webview. The `redirect_uri` should not contain any query parameters. When used in Production or Development, must be an https URI. To specify any subdomain, use `*` as a wildcard character, e.g. `https://*.example.com/oauth.html`. If `android_package_name` is specified, this field should be left blank.  Note that any redirect URI must also be added to the Allowed redirect URIs list in the [developer dashboard](https://dashboard.plaid.com/team/api).
    pub redirect_uri: Option<String>,
    #[serde(rename = "android_package_name")]
    ///The name of your app's Android package. Required if using the `link_token` to initialize Link on Android. When creating a `link_token` for initializing Link on other platforms, this field must be left blank. Any package name specified here must also be added to the Allowed Android package names setting on the [developer dashboard](https://dashboard.plaid.com/team/api).
    pub android_package_name: Option<String>,
    #[serde(rename = "institution_data")]
    ///A map containing data used to highlight institutions in Link.
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    #[serde(rename = "account_filters")]
    /**By default, Link will provide limited account filtering: it will only display Institutions that are compatible with all products supplied in the `products` parameter of `/link/token/create`, and, if `auth` is specified in the `products` array, will also filter out accounts other than `checking` and `savings` accounts on the Account Select pane. You can further limit the accounts shown in Link by using `account_filters` to specify the account subtypes to be shown in Link. Only the specified subtypes will be shown. This filtering applies to both the Account Select view (if enabled) and the Institution Select view. Institutions that do not support the selected subtypes will be omitted from Link. To indicate that all subtypes should be shown, use the value `"all"`. If the `account_filters` filter is used, any account type for which a filter is not specified will be entirely omitted from Link. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).

For institutions using OAuth, the filter will not affect the list of accounts shown by the bank in the OAuth window.
*/
    pub account_filters: Option<LinkTokenAccountFilters>,
    #[serde(rename = "eu_config")]
    ///Configuration parameters for EU flows
    pub eu_config: Option<LinkTokenEuConfig>,
    #[serde(rename = "institution_id")]
    ///Used for certain Europe-only configurations, as well as certain legacy use cases in other regions.
    pub institution_id: Option<String>,
    #[serde(rename = "payment_initiation")]
    ///Specifies options for initializing Link for use with the Payment Initiation (Europe) product. This field is required if `payment_initiation` is included in the `products` array.
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    #[serde(rename = "deposit_switch")]
    ///Specifies options for initializing Link for use with the Deposit Switch (beta) product. This field is required if `deposit_switch` is included in the `products` array.
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    #[serde(rename = "income_verification")]
    ///Specifies options for initializing Link for use with the Income (beta) product. This field is required if `income_verification` is included in the `products` array.
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    #[serde(rename = "auth")]
    ///Specifies options for initializing Link for use with the Auth product. This field can be used to enable or disable extended Auth flows for the resulting Link session. Omitting any field will result in a default that can be configured by your account manager.
    pub auth: Option<LinkTokenCreateRequestAuth>,
    #[serde(rename = "transfer")]
    ///Specifies options for initializing Link for use with the Transfer product.
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    #[serde(rename = "update")]
    ///Specifies options for initializing Link for [update mode](https://plaid.com/docs/link/update-mode).
    pub update: Option<LinkTokenCreateRequestUpdate>,
    #[serde(rename = "identity_verification")]
    ///Specifies option for initializing Link for use with the Identity Verification product.
    pub identity_verification: Option<LinkTokenCreateRequestIdentityVerification>,
    #[serde(rename = "user_token")]
    ///A user token generated using `/user/create`. Any item created during the link session will be associated with the user.
    pub user_token: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenAccountFilters {
    #[serde(rename = "depository")]
    ///A filter to apply to `depository`-type accounts
    pub depository: Option<DepositoryFilter>,
    #[serde(rename = "credit")]
    ///A filter to apply to `credit`-type accounts
    pub credit: Option<CreditFilter>,
    #[serde(rename = "loan")]
    ///A filter to apply to `loan`-type accounts
    pub loan: Option<LoanFilter>,
    #[serde(rename = "investment")]
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: Option<InvestmentFilter>,
}
impl std::fmt::Display for LinkTokenAccountFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenEuConfig {
    #[serde(rename = "headless")]
    ///If `true`, open Link without an initial UI. Defaults to `false`.
    pub headless: Option<bool>,
}
impl std::fmt::Display for LinkTokenEuConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestPaymentInitiation {
    #[serde(rename = "payment_id")]
    ///The `payment_id` provided by the `/payment_initiation/payment/create` endpoint.
    pub payment_id: String,
    #[serde(rename = "consent_id")]
    ///The `consent_id` provided by the `/payment_initiation/consent/create` endpoint.
    pub consent_id: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestPaymentInitiation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestDepositSwitch {
    #[serde(rename = "deposit_switch_id")]
    ///The `deposit_switch_id` provided by the `/deposit_switch/create` endpoint.
    pub deposit_switch_id: String,
}
impl std::fmt::Display for LinkTokenCreateRequestDepositSwitch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestTransfer {
    #[serde(rename = "intent_id")]
    ///The `id` returned by the `/transfer/intent/create` endpoint.
    pub intent_id: Option<String>,
    #[serde(rename = "payment_profile_id")]
    ///The `payment_profile_id` returned by the `/payment_profile/create` endpoint.
    pub payment_profile_id: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUserStatedIncomeSource {
    #[serde(rename = "employer")]
    ///The employer corresponding to an income source specified by the user
    pub employer: Option<String>,
    #[serde(rename = "category")]
    ///The income category for a specified income source
    pub category: Option<String>,
    #[serde(rename = "pay_per_cycle")]
    ///The income amount paid per cycle for a specified income source
    pub pay_per_cycle: Option<f64>,
    #[serde(rename = "pay_annual")]
    ///The income amount paid annually for a specified income source
    pub pay_annual: Option<f64>,
    #[serde(rename = "pay_type")]
    ///The pay type - `GROSS`, `NET`, or `UNKNOWN` for a specified income source
    pub pay_type: Option<String>,
    #[serde(rename = "pay_frequency")]
    ///The pay frequency of a specified income source
    pub pay_frequency: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestUserStatedIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourceCategory {
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "UNEMPLOYMENT")]
    Unemployment,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "GIG_ECONOMY")]
    GigEconomy,
    #[serde(rename = "RENTAL")]
    Rental,
    #[serde(rename = "CHILD_SUPPORT")]
    ChildSupport,
    #[serde(rename = "MILITARY")]
    Military,
    #[serde(rename = "RETIREMENT")]
    Retirement,
    #[serde(rename = "LONG_TERM_DISABILITY")]
    LongTermDisability,
    #[serde(rename = "BANK_INTEREST")]
    BankInterest,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourceFrequency {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourcePayType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "GROSS")]
    Gross,
    #[serde(rename = "NET")]
    Net,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAuth {
    #[serde(rename = "auth_type_select_enabled")]
    ///Specifies whether Auth Type Select is enabled for the Link session, allowing the end user to choose between linking instantly or manually prior to selecting their financial institution. Note that this can only be true if `same_day_microdeposits_enabled` is set to true.
    pub auth_type_select_enabled: Option<bool>,
    #[serde(rename = "automated_microdeposits_enabled")]
    ///Specifies whether the Link session is enabled for the Automated Micro-deposits flow.
    pub automated_microdeposits_enabled: Option<bool>,
    #[serde(rename = "instant_match_enabled")]
    ///Specifies whether the Link session is enabled for the Instant Match flow.
    pub instant_match_enabled: Option<bool>,
    #[serde(rename = "same_day_microdeposits_enabled")]
    ///Specifies whether the Link session is enabled for the Same Day Micro-deposits flow.
    pub same_day_microdeposits_enabled: Option<bool>,
    #[serde(rename = "flow_type")]
    ///This field has been deprecated in favor of `auth_type_select_enabled`.
    pub flow_type: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIdentityVerification {
    #[serde(rename = "template_id")]
    ///ID of the associated Identity Verification template.
    pub template_id: String,
    #[serde(rename = "consent")]
    pub consent: Option<serde_json::Value>,
    #[serde(rename = "gave_consent")]
    /**A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.

If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.*/
    pub gave_consent: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestIdentityVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateInstitutionData {
    #[serde(rename = "routing_number")]
    ///The routing number of the bank to highlight.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateInstitutionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUser {
    #[serde(rename = "client_user_id")]
    ///A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`. It is currently used as a means of searching logs for the given user in the Plaid Dashboard.
    pub client_user_id: String,
    #[serde(rename = "legal_name")]
    ///The user's full legal name. Currently used only to support certain legacy flows.
    pub legal_name: Option<String>,
    #[serde(rename = "name")]
    ///The user's full name. Optional if using the [Identity Verification](https://plaid.com/docs/api/products/identity-verification) product; if not using Identity Verification, this field is not allowed. Users will not be asked for their name when this field is provided.
    pub name: Option<serde_json::Value>,
    #[serde(rename = "phone_number")]
    ///The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. This field is optional, but required to enable the [returning user experience](https://plaid.com/docs/link/returning-user).
    pub phone_number: Option<String>,
    #[serde(rename = "phone_number_verified_time")]
    /**The date and time the phone number was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This field is optional, but required to enable any [returning user experience](https://plaid.com/docs/link/returning-user).

 Only pass a verification time for a phone number that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.

 Example: `2020-01-01T00:00:00Z`
*/
    pub phone_number_verified_time: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address. This field is optional, but required to enable the [pre-authenticated returning user flow](https://plaid.com/docs/link/returning-user/#enabling-the-returning-user-experience).
    pub email_address: Option<String>,
    #[serde(rename = "email_address_verified_time")]
    /**The date and time the email address was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This is an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user).

 Only pass a verification time for an email address that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.

 Example: `2020-01-01T00:00:00Z`*/
    pub email_address_verified_time: Option<String>,
    #[serde(rename = "ssn")]
    ///To be provided in the format "ddd-dd-dddd". Not currently used.
    pub ssn: Option<String>,
    #[serde(rename = "date_of_birth")]
    ///To be provided in the format "yyyy-mm-dd". Not currently used.
    pub date_of_birth: Option<String>,
    #[serde(rename = "address")]
    ///Home address for the user.
    pub address: Option<UserAddress>,
    #[serde(rename = "id_number")]
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub id_number: Option<UserIdNumber>,
}
impl std::fmt::Display for LinkTokenCreateRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUpdate {
    #[serde(rename = "account_selection_enabled")]
    ///If `true`, enables [update mode with Account Select](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).
    pub account_selection_enabled: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAccountSubtypes {
    #[serde(rename = "depository")]
    ///A filter to apply to `depository`-type accounts
    pub depository: Option<LinkTokenCreateDepositoryFilter>,
    #[serde(rename = "credit")]
    ///A filter to apply to `credit`-type accounts
    pub credit: Option<LinkTokenCreateCreditFilter>,
    #[serde(rename = "loan")]
    ///A filter to apply to `loan`-type accounts
    pub loan: Option<LinkTokenCreateLoanFilter>,
    #[serde(rename = "investment")]
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: Option<LinkTokenCreateInvestmentFilter>,
}
impl std::fmt::Display for LinkTokenCreateRequestAccountSubtypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateDepositoryFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: Option<DepositoryAccountSubtypes>,
}
impl std::fmt::Display for LinkTokenCreateDepositoryFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateCreditFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: Option<CreditAccountSubtypes>,
}
impl std::fmt::Display for LinkTokenCreateCreditFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateLoanFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: Option<LoanAccountSubtypes>,
}
impl std::fmt::Display for LinkTokenCreateLoanFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateInvestmentFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: Option<InvestmentAccountSubtypes>,
}
impl std::fmt::Display for LinkTokenCreateInvestmentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetResponse {
    #[serde(rename = "link_token")]
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    #[serde(rename = "created_at")]
    ///The creation timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: Option<String>,
    #[serde(rename = "expiration")]
    ///The expiration timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub expiration: Option<String>,
    #[serde(rename = "metadata")]
    ///An object specifying the arguments originally provided to the `/link/token/create` call.
    pub metadata: LinkTokenGetMetadataResponse,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetMetadataResponse {
    #[serde(rename = "initial_products")]
    ///The `products` specified in the `/link/token/create` call.
    pub initial_products: Vec<String>,
    #[serde(rename = "webhook")]
    ///The `webhook` specified in the `/link/token/create` call.
    pub webhook: Option<String>,
    #[serde(rename = "country_codes")]
    ///The `country_codes` specified in the `/link/token/create` call.
    pub country_codes: Vec<String>,
    #[serde(rename = "language")]
    ///The `language` specified in the `/link/token/create` call.
    pub language: Option<String>,
    #[serde(rename = "institution_data")]
    ///A map containing data used to highlight institutions in Link.
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    #[serde(rename = "account_filters")]
    /**The `account_filters` specified in the original call to `/link/token/create`.
*/
    pub account_filters: Option<AccountFiltersResponse>,
    #[serde(rename = "redirect_uri")]
    ///The `redirect_uri` specified in the `/link/token/create` call.
    pub redirect_uri: Option<String>,
    #[serde(rename = "client_name")]
    ///The `client_name` specified in the `/link/token/create` call.
    pub client_name: Option<String>,
}
impl std::fmt::Display for LinkTokenGetMetadataResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateResponse {
    #[serde(rename = "link_token")]
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    #[serde(rename = "expiration")]
    ///The expiration date for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `link_token` created to generate a `public_token` that will be exchanged for a new `access_token` expires after 4 hours. A `link_token` created for an existing Item (such as when updating an existing `access_token` by launching Link in update mode) expires after 30 minutes.
    pub expiration: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for LinkTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidError(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "error_type")]
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: String,
    #[serde(rename = "error_code")]
    ///The particular error code. Safe for programmatic use.
    pub error_code: String,
    #[serde(rename = "error_message")]
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    #[serde(rename = "display_message")]
    /**A user-friendly representation of the error code. `null` if the error is not related to user action.

This may change over time and is not safe for programmatic use.*/
    pub display_message: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    pub request_id: Option<String>,
    #[serde(rename = "causes")]
    /**In the Assets product, a request can pertain to more than one Item. If an error is returned for such a request, `causes` will return an array of errors containing a breakdown of these errors on the individual Item level, if any can be identified.

`causes` will only be provided for the `error_type` `ASSET_REPORT_ERROR`. `causes` will also not be populated inside an error nested within a `warning` object.*/
    pub causes: Option<Vec<serde_json::Value>>,
    #[serde(rename = "status")]
    ///The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook.
    pub status: Option<f64>,
    #[serde(rename = "documentation_url")]
    ///The URL of a Plaid documentation page with more information about the error
    pub documentation_url: Option<String>,
    #[serde(rename = "suggested_action")]
    ///Suggested steps for resolving the error
    pub suggested_action: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "other")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum OverrideAccountType {
    #[serde(rename = "investment")]
    Investment,
    #[serde(rename = "credit")]
    Credit,
    #[serde(rename = "depository")]
    Depository,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "payroll")]
    Payroll,
    #[serde(rename = "other")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBase {
    #[serde(rename = "account_id")]
    /**Plaid’s unique identifier for the account. This value will not change unless Plaid can't reconcile the account with the data returned by the financial institution. This may occur, for example, when the name of the account changes. If this happens a new `account_id` will be assigned to the account.

The `account_id` can also change if the `access_token` is deleted and the same credentials that were used to generate that `access_token` are used to generate a new `access_token` on a later date. In that case, the new `account_id` will be different from the old `account_id`.

If an account with a specific `account_id` disappears instead of changing, the account is likely closed. Closed accounts are not returned by the Plaid API.

Like all Plaid identifiers, the `account_id` is case sensitive.*/
    pub account_id: String,
    #[serde(rename = "balances")]
    ///A set of fields describing the balance for an account. Balance information may be cached unless the balance object was returned by `/accounts/balance/get`.
    pub balances: AccountBalance,
    #[serde(rename = "mask")]
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    pub mask: Option<String>,
    #[serde(rename = "name")]
    ///The name of the account, either assigned by the user or by the financial institution itself
    pub name: String,
    #[serde(rename = "official_name")]
    ///The official name of the account as given by the financial institution
    pub official_name: Option<String>,
    #[serde(rename = "type")]
    /**`investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    pub type_: String,
    #[serde(rename = "subtype")]
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    pub subtype: Option<String>,
    #[serde(rename = "verification_status")]
    /**The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.

`pending_automatic_verification`: The Item is pending automatic verification

`pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the two amounts.

`automatically_verified`: The Item has successfully been automatically verified	

`manually_verified`: The Item has successfully been manually verified

`verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.

`verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.	
	*/
    pub verification_status: Option<String>,
}
impl std::fmt::Display for AccountBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    #[serde(rename = "available")]
    /**The amount of funds available to be withdrawn from the account, as determined by the financial institution.

For `credit`-type accounts, the `available` balance typically equals the `limit` less the `current` balance, less any pending outflows plus any pending inflows.

For `depository`-type accounts, the `available` balance typically equals the `current` balance less any pending outflows plus any pending inflows. For `depository`-type accounts, the `available` balance does not include the overdraft limit.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the `available` balance is the total cash available to withdraw as presented by the institution.

Note that not all institutions calculate the `available`  balance. In the event that `available` balance is unavailable, Plaid will return an `available` balance value of `null`.

Available balance may be cached and is not guaranteed to be up-to-date in realtime unless the value was returned by `/accounts/balance/get`.

If `current` is `null` this field is guaranteed not to be `null`.*/
    pub available: Option<f64>,
    #[serde(rename = "current")]
    /**The total amount of funds in or owed by the account.

For `credit`-type accounts, a positive balance indicates the amount owed; a negative amount indicates the lender owing the account holder.

For `loan`-type accounts, the current balance is the principal remaining on the loan, except in the case of student loan accounts at Sallie Mae (`ins_116944`). For Sallie Mae student loans, the account's balance includes both principal and any outstanding interest.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the current balance is the total value of assets as presented by the institution.

Note that balance information may be cached unless the value was returned by `/accounts/balance/get`; if the Item is enabled for Transactions, the balance will be at least as recent as the most recent Transaction update. If you require realtime balance information, use the `available` balance as provided by `/accounts/balance/get`.

When returned by `/accounts/balance/get`, this field may be `null`. When this happens, `available` is guaranteed not to be `null`.*/
    pub current: Option<f64>,
    #[serde(rename = "limit")]
    /**For `credit`-type accounts, this represents the credit limit.

For `depository`-type accounts, this represents the pre-arranged overdraft limit, which is common for current (checking) accounts in Europe.

In North America, this field is typically only available for `credit`-type accounts.*/
    pub limit: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the balance. Always null if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the balance. Always null if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `unofficial_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "last_updated_datetime")]
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time that the balance for the given account has been updated

This is currently only provided when the `min_last_updated_datetime` is passed when calling `/accounts/balance/get` for `ins_128026` (Capital One).*/
    pub last_updated_datetime: Option<String>,
}
impl std::fmt::Display for AccountBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountSubtype {
    #[serde(rename = "401a")]
    AccountSubtype401A,
    #[serde(rename = "401k")]
    AccountSubtype401K,
    #[serde(rename = "403B")]
    AccountSubtype403B,
    #[serde(rename = "457b")]
    AccountSubtype457B,
    #[serde(rename = "529")]
    AccountSubtype529,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "crypto exchange")]
    CryptoExchange,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "ebt")]
    Ebt,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "rdsp")]
    Rdsp,
    #[serde(rename = "resp")]
    Resp,
    #[serde(rename = "rlif")]
    Rlif,
    #[serde(rename = "rrif")]
    Rrif,
    #[serde(rename = "pension")]
    Pension,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
    #[serde(rename = "retirement")]
    Retirement,
    #[serde(rename = "roth")]
    Roth,
    #[serde(rename = "roth 401k")]
    Roth401K,
    #[serde(rename = "rrsp")]
    Rrsp,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "thrift savings plan")]
    ThriftSavingsPlan,
    #[serde(rename = "tfsa")]
    Tfsa,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "ugma")]
    Ugma,
    #[serde(rename = "utma")]
    Utma,
    #[serde(rename = "variable annuity")]
    VariableAnnuity,
    #[serde(rename = "credit card")]
    CreditCard,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "money market")]
    MoneyMarket,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "home equity")]
    HomeEquity,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "overdraft")]
    Overdraft,
    #[serde(rename = "line of credit")]
    LineOfCredit,
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "cash management")]
    CashManagement,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "recurring")]
    Recurring,
    #[serde(rename = "rewards")]
    Rewards,
    #[serde(rename = "safe deposit")]
    SafeDeposit,
    #[serde(rename = "sarsep")]
    Sarsep,
    #[serde(rename = "payroll")]
    Payroll,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersAch {
    #[serde(rename = "account_id")]
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    #[serde(rename = "account")]
    /**The ACH account number for the account.

Note that when using OAuth with Chase Bank (`ins_56`), Chase will issue "tokenized" routing and account numbers, which are not the user's actual account and routing numbers. These tokenized numbers should work identically to normal account and routing numbers. The digits returned in the `mask` field will continue to reflect the actual account number, rather than the tokenized account number; for this reason, when displaying account numbers to the user to help them identify their account in your UI, always use the `mask` rather than truncating the `account` number. If a user revokes their permissions to your app, the tokenized numbers will continue to work for ACH deposits, but not withdrawals.*/
    pub account: String,
    #[serde(rename = "routing")]
    ///The ACH routing number for the account. If the institution is `ins_56`, this may be a tokenized routing number. For more information, see the description of the `account` field.
    pub routing: String,
    #[serde(rename = "wire_routing")]
    ///The wire transfer routing number for the account, if available
    pub wire_routing: Option<String>,
}
impl std::fmt::Display for NumbersAch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersAchNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersEft {
    #[serde(rename = "account_id")]
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    #[serde(rename = "account")]
    ///The EFT account number for the account
    pub account: String,
    #[serde(rename = "institution")]
    ///The EFT institution number for the account
    pub institution: String,
    #[serde(rename = "branch")]
    ///The EFT branch number for the account
    pub branch: String,
}
impl std::fmt::Display for NumbersEft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersEftNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternational {
    #[serde(rename = "account_id")]
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the account
    pub iban: String,
    #[serde(rename = "bic")]
    ///The Bank Identifier Code (BIC) for the account
    pub bic: String,
}
impl std::fmt::Display for NumbersInternational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternationalNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersBacs {
    #[serde(rename = "account_id")]
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    #[serde(rename = "account")]
    ///The BACS account number for the account
    pub account: String,
    #[serde(rename = "sort_code")]
    ///The BACS sort code for the account
    pub sort_code: String,
}
impl std::fmt::Display for NumbersBacs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersBacsNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternationalIban {
    #[serde(rename = "iban")]
    ///International Bank Account Number (IBAN).
    pub iban: String,
    #[serde(rename = "bic")]
    ///The Business Identifier Code, also known as SWIFT code, for this bank account.
    pub bic: String,
}
impl std::fmt::Display for NumbersInternationalIban {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersIban(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersIbanNullable(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientBacs {
    #[serde(rename = "account")]
    ///The account number of the account. Maximum of 10 characters.
    pub account: Option<String>,
    #[serde(rename = "sort_code")]
    ///The 6-character sort code of the account.
    pub sort_code: Option<String>,
}
impl std::fmt::Display for RecipientBacs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientBacsNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct SenderBacsNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationOptionalRestrictionBacs(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPullId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovedTransaction {
    #[serde(rename = "transaction_id")]
    ///The ID of the removed transaction.
    pub transaction_id: Option<String>,
}
impl std::fmt::Display for RemovedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRuleDetails {
    #[serde(rename = "field")]
    ///Transaction field for which the rule is defined.
    pub field: String,
    #[serde(rename = "type")]
    /**Transaction rule's match type. For TRANSACTION_ID field, EXACT_MATCH is available.
Matches are case sensitive.
*/
    pub type_: String,
    #[serde(rename = "query")]
    /**For TRANSACTION_ID field, provide transaction_id. For NAME field, provide a string pattern.
*/
    pub query: String,
}
impl std::fmt::Display for TransactionsRuleDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionsRuleField {
    #[serde(rename = "TRANSACTION_ID")]
    TransactionId,
    #[serde(rename = "NAME")]
    Name,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionsRuleType {
    #[serde(rename = "EXACT_MATCH")]
    ExactMatch,
    #[serde(rename = "SUBSTRING_MATCH")]
    SubstringMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsCategoryRule {
    #[serde(rename = "id")]
    ///A unique identifier of the rule created
    pub id: Option<String>,
    #[serde(rename = "item_id")]
    ///A unique identifier of the item the rule was created for
    pub item_id: Option<String>,
    #[serde(rename = "created_at")]
    /**Date and time when a rule was created in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).
*/
    pub created_at: Option<String>,
    #[serde(rename = "personal_finance_category")]
    /**Personal finance category unique identifier.

In the personal finance category taxonomy, this field is represented by the detailed category field.
*/
    pub personal_finance_category: Option<String>,
    #[serde(rename = "rule_details")]
    ///A representation of transactions rule details.
    pub rule_details: Option<TransactionsRuleDetails>,
}
impl std::fmt::Display for TransactionsCategoryRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionBase {
    #[serde(rename = "transaction_type")]
    /**Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.

`digital:` transactions that took place online.

`place:` transactions that were made at a physical location.

`special:` transactions that relate to banks, e.g. fees or deposits.

`unresolved:` transactions that do not fit into the other three types.
*/
    pub transaction_type: Option<String>,
    #[serde(rename = "pending_transaction_id")]
    ///The ID of a posted transaction's associated pending transaction, where applicable.
    pub pending_transaction_id: Option<String>,
    #[serde(rename = "category_id")]
    /**The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub category_id: Option<String>,
    #[serde(rename = "category")]
    /**A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub category: Option<Vec<String>>,
    #[serde(rename = "location")]
    ///A representation of where a transaction took place
    pub location: Option<Location>,
    #[serde(rename = "payment_meta")]
    /**Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub payment_meta: Option<PaymentMeta>,
    #[serde(rename = "account_owner")]
    ///The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    pub account_owner: Option<String>,
    #[serde(rename = "name")]
    /**The merchant name or transaction description.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub name: Option<String>,
    #[serde(rename = "original_description")]
    ///The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`.
    pub original_description: Option<String>,
    #[serde(rename = "account_id")]
    ///The ID of the account in which this transaction occurred.
    pub account_id: String,
    #[serde(rename = "amount")]
    ///The settled value of the transaction, denominated in the transactions's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "date")]
    ///For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    pub date: String,
    #[serde(rename = "pending")]
    ///When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    pub pending: bool,
    #[serde(rename = "transaction_id")]
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: String,
    #[serde(rename = "merchant_name")]
    ///The merchant name, as extracted by Plaid from the `name` field.
    pub merchant_name: Option<String>,
    #[serde(rename = "check_number")]
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
}
impl std::fmt::Display for TransactionBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "address")]
    ///The street address where the transaction occurred.
    pub address: Option<String>,
    #[serde(rename = "city")]
    ///The city where the transaction occurred.
    pub city: Option<String>,
    #[serde(rename = "region")]
    ///The region or state where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `state`.
    pub region: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code where the transaction occurred.
    pub country: Option<String>,
    #[serde(rename = "lat")]
    ///The latitude where the transaction occurred.
    pub lat: Option<f64>,
    #[serde(rename = "lon")]
    ///The longitude where the transaction occurred.
    pub lon: Option<f64>,
    #[serde(rename = "store_number")]
    ///The merchant defined store number where the transaction occurred.
    pub store_number: Option<String>,
}
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStream {
    #[serde(rename = "account_id")]
    ///The ID of the account to which the stream belongs
    pub account_id: String,
    #[serde(rename = "stream_id")]
    ///A unique id for the stream
    pub stream_id: String,
    #[serde(rename = "category_id")]
    ///The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    pub category_id: String,
    #[serde(rename = "category")]
    ///A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    pub category: Vec<String>,
    #[serde(rename = "description")]
    ///A description of the transaction stream.
    pub description: String,
    #[serde(rename = "merchant_name")]
    ///The merchant associated with the transaction stream.
    pub merchant_name: Option<String>,
    #[serde(rename = "first_date")]
    ///The posted date of the earliest transaction in the stream.
    pub first_date: String,
    #[serde(rename = "last_date")]
    ///The posted date of the latest transaction in the stream.
    pub last_date: String,
    #[serde(rename = "frequency")]
    /**Describes the frequency of the transaction stream.

`WEEKLY`: Assigned to a transaction stream that occurs approximately every week.

`BIWEEKLY`: Assigned to a transaction stream that occurs approximately every 2 weeks.

`SEMI_MONTHLY`: Assigned to a transaction stream that occurs approximately twice per month. This frequency is typically seen for inflow transaction streams.

`MONTHLY`: Assigned to a transaction stream that occurs approximately every month.

`UNKNOWN`: Assigned to a transaction stream that does not fit any of the pre-defined frequencies.*/
    pub frequency: String,
    #[serde(rename = "transaction_ids")]
    ///An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    pub transaction_ids: Vec<String>,
    #[serde(rename = "average_amount")]
    ///Object with data pertaining to an amount on the transaction stream.
    pub average_amount: TransactionStreamAmount,
    #[serde(rename = "last_amount")]
    ///Object with data pertaining to an amount on the transaction stream.
    pub last_amount: TransactionStreamAmount,
    #[serde(rename = "is_active")]
    ///Indicates whether the transaction stream is still live.
    pub is_active: bool,
    #[serde(rename = "status")]
    /**The current status of the transaction stream.

`MATURE`: A `MATURE` recurring stream should have at least 3 transactions and happen on a regular cadence.

`EARLY_DETECTION`: When a recurring transaction first appears in the transaction history and before it fulfills the requirement of a mature stream, the status will be `EARLY_DETECTION`.

`TOMBSTONED`: A stream that was previously in the `EARLY_DETECTION` status will move to the `TOMBSTONED` status when no further transactions were found at the next expected date.

`UNKNOWN`: A stream is assigned an `UNKNOWN` status when none of the other statuses are applicable.*/
    pub status: String,
    #[serde(rename = "personal_finance_category")]
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.*/
    pub personal_finance_category: Option<PersonalFinanceCategory>,
}
impl std::fmt::Display for TransactionStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStreamAmount {
    #[serde(rename = "amount")]
    ///Represents the numerical value of an amount.
    pub amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    /**The ISO-4217 currency code of the amount. Always `null` if `unofficial_currency_code` is non-`null`.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    ///The unofficial currency code of the amount. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for TransactionStreamAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecurringTransactionFrequency {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStreamStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "MATURE")]
    Mature,
    #[serde(rename = "EARLY_DETECTION")]
    EarlyDetection,
    #[serde(rename = "TOMBSTONED")]
    Tombstoned,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    #[serde(rename = "institution_id")]
    ///Unique identifier for the institution
    pub institution_id: String,
    #[serde(rename = "name")]
    ///The official name of the institution
    pub name: String,
    #[serde(rename = "products")]
    ///A list of the Plaid products supported by the institution. Note that only institutions that support Instant Auth will return `auth` in the product array; institutions that do not list `auth` may still support other Auth methods such as Instant Match or Automated Micro-deposit Verification. To identify institutions that support those methods, use the `auth_metadata` object. For more details, see [Full Auth coverage](https://plaid.com/docs/auth/coverage/).
    pub products: Vec<String>,
    #[serde(rename = "country_codes")]
    ///A list of the country codes supported by the institution.
    pub country_codes: Vec<String>,
    #[serde(rename = "url")]
    ///The URL for the institution's website
    pub url: Option<String>,
    #[serde(rename = "primary_color")]
    ///Hexadecimal representation of the primary color used by the institution
    pub primary_color: Option<String>,
    #[serde(rename = "logo")]
    ///Base64 encoded representation of the institution's logo
    pub logo: Option<String>,
    #[serde(rename = "routing_numbers")]
    ///A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution.
    pub routing_numbers: Vec<String>,
    #[serde(rename = "oauth")]
    ///Indicates that the institution has an OAuth login flow.
    pub oauth: bool,
    #[serde(rename = "status")]
    /**The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.

Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment.
*/
    pub status: Option<InstitutionStatus>,
    #[serde(rename = "payment_initiation_metadata")]
    ///Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
    pub payment_initiation_metadata: Option<PaymentInitiationMetadata>,
    #[serde(rename = "auth_metadata")]
    ///Metadata that captures information about the Auth features of an institution.
    pub auth_metadata: Option<AuthMetadata>,
}
impl std::fmt::Display for Institution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionStatus {
    #[serde(rename = "item_logins")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub item_logins: Option<ProductStatus>,
    #[serde(rename = "transactions_updates")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub transactions_updates: Option<ProductStatus>,
    #[serde(rename = "auth")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub auth: Option<ProductStatus>,
    #[serde(rename = "identity")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub identity: Option<ProductStatus>,
    #[serde(rename = "investments_updates")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub investments_updates: Option<ProductStatus>,
    #[serde(rename = "liabilities_updates")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub liabilities_updates: Option<ProductStatus>,
    #[serde(rename = "liabilities")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub liabilities: Option<ProductStatus>,
    #[serde(rename = "investments")]
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub investments: Option<ProductStatus>,
    #[serde(rename = "health_incidents")]
    ///Details of recent health incidents associated with the institution.
    pub health_incidents: Option<Vec<HealthIncident>>,
}
impl std::fmt::Display for InstitutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CountryCode {
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "IT")]
    It,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMeta {
    #[serde(rename = "reference_number")]
    ///The transaction reference number supplied by the financial institution.
    pub reference_number: Option<String>,
    #[serde(rename = "ppd_id")]
    ///The ACH PPD ID for the payer.
    pub ppd_id: Option<String>,
    #[serde(rename = "payee")]
    ///For transfers, the party that is receiving the transaction.
    pub payee: Option<String>,
    #[serde(rename = "by_order_of")]
    ///The party initiating a wire transfer. Will be `null` if the transaction is not a wire transfer.
    pub by_order_of: Option<String>,
    #[serde(rename = "payer")]
    ///For transfers, the party that is paying the transaction.
    pub payer: Option<String>,
    #[serde(rename = "payment_method")]
    ///The type of transfer, e.g. 'ACH'
    pub payment_method: Option<String>,
    #[serde(rename = "payment_processor")]
    ///The name of the payment processor
    pub payment_processor: Option<String>,
    #[serde(rename = "reason")]
    ///The payer-supplied description of the transfer.
    pub reason: Option<String>,
}
impl std::fmt::Display for PaymentMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionCode {
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "atm")]
    Atm,
    #[serde(rename = "bank charge")]
    BankCharge,
    #[serde(rename = "bill payment")]
    BillPayment,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "cashback")]
    Cashback,
    #[serde(rename = "cheque")]
    Cheque,
    #[serde(rename = "direct debit")]
    DirectDebit,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "purchase")]
    Purchase,
    #[serde(rename = "standing order")]
    StandingOrder,
    #[serde(rename = "transfer")]
    Transfer,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "category_id")]
    ///An identifying number for the category. `category_id` is a Plaid-specific identifier and does not necessarily correspond to merchant category codes.
    pub category_id: String,
    #[serde(rename = "group")]
    ///`place` for physical transactions or `special` for other transactions such as bank charges.
    pub group: String,
    #[serde(rename = "hierarchy")]
    ///A hierarchical array of the categories to which this `category_id` belongs.
    pub hierarchy: Vec<String>,
}
impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalFinanceCategory {
    #[serde(rename = "primary")]
    ///A high level category that communicates the broad category of the transaction.
    pub primary: String,
    #[serde(rename = "detailed")]
    ///A granular category conveying the transaction's intent. This field can also be used as a unique identifier for the category.
    pub detailed: String,
}
impl std::fmt::Display for PersonalFinanceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenNullable(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferAccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiSecret(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiClientId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningStatusUpdatedWebhook {
    #[serde(rename = "webhook_type")]
    ///`SCREENING`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    #[serde(rename = "screening_id")]
    ///The ID of the associated screening.
    pub screening_id: serde_json::Value,
}
impl std::fmt::Display for ScreeningStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningStatusUpdatedWebhook {
    #[serde(rename = "webhook_type")]
    ///`ENTITY_SCREENING`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    #[serde(rename = "screening_id")]
    ///The ID of the associated screening.
    pub screening_id: serde_json::Value,
}
impl std::fmt::Display for EntityScreeningStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationStepUpdatedWebhook {
    #[serde(rename = "webhook_type")]
    ///`IDENTITY_VERIFCATION`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`STEP_UPDATED`
    pub webhook_code: String,
    #[serde(rename = "identity_verification_id")]
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: serde_json::Value,
}
impl std::fmt::Display for IdentityVerificationStepUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRetriedWebhook {
    #[serde(rename = "webhook_type")]
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`RETRIED`
    pub webhook_code: String,
    #[serde(rename = "identity_verification_id")]
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: serde_json::Value,
}
impl std::fmt::Display for IdentityVerificationRetriedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationStatusUpdatedWebhook {
    #[serde(rename = "webhook_type")]
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    #[serde(rename = "identity_verification_id")]
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: serde_json::Value,
}
impl std::fmt::Display for IdentityVerificationStatusUpdatedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRemovedWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`TRANSACTIONS_REMOVED`
    pub webhook_code: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "removed_transactions")]
    ///An array of `transaction_ids` corresponding to the removed transactions
    pub removed_transactions: Vec<String>,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for TransactionsRemovedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "new_transactions")]
    ///The number of new transactions detected since the last time this webhook was fired.
    pub new_transactions: f64,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item the webhook relates to.
    pub item_id: String,
}
impl std::fmt::Display for DefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncUpdatesAvailableWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`SYNC_UPDATES_AVAILABLE`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "initial_update_complete")]
    ///Indicates if initial pull information is available.
    pub initial_update_complete: bool,
    #[serde(rename = "historical_update_complete")]
    ///Indicates if historical pull information is available.
    pub historical_update_complete: bool,
}
impl std::fmt::Display for SyncUpdatesAvailableWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringTransactionsUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`RECURRING_TRANSACTIONS_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "account_ids")]
    ///A list of `account_ids` for accounts that have new or updated recurring transactions data.
    pub account_ids: Vec<String>,
}
impl std::fmt::Display for RecurringTransactionsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityDefaultUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`IDENTITY`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "account_ids_with_updated_identity")]
    /**An object with keys of `account_id`'s that are mapped to their respective identity attributes that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["PHONES"] }`
*/
    pub account_ids_with_updated_identity: AccountIdsWithUpdatedIdentity,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for IdentityDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountIdsWithUpdatedIdentity {}
impl std::fmt::Display for AccountIdsWithUpdatedIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityUpdateTypes {
    #[serde(rename = "PHONES")]
    Phones,
    #[serde(rename = "ADDRESSES")]
    Addresses,
    #[serde(rename = "EMAILS")]
    Emails,
    #[serde(rename = "NAMES")]
    Names,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`HISTORICAL_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "new_transactions")]
    ///The number of new, unfetched transactions available
    pub new_transactions: f64,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for HistoricalUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`INITIAL_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "error")]
    ///The error code associated with the webhook.
    pub error: Option<String>,
    #[serde(rename = "new_transactions")]
    ///The number of new, unfetched transactions available.
    pub new_transactions: f64,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for InitialUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    #[serde(rename = "data")]
    ///The phone number.
    pub data: String,
    #[serde(rename = "primary")]
    ///When `true`, identifies the phone number as the primary number on an account.
    pub primary: bool,
    #[serde(rename = "type")]
    ///The type of phone number.
    pub type_: String,
}
impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    #[serde(rename = "data")]
    ///The email address.
    pub data: String,
    #[serde(rename = "primary")]
    ///When `true`, identifies the email address as the primary email on an account.
    pub primary: bool,
    #[serde(rename = "type")]
    ///The type of email account as described by the financial institution.
    pub type_: String,
}
impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "data")]
    ///Data about the components comprising an address.
    pub data: AddressData,
    #[serde(rename = "primary")]
    ///When `true`, identifies the address as the primary address on an account.
    pub primary: Option<bool>,
}
impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDataNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressData {
    #[serde(rename = "city")]
    ///The full city name
    pub city: String,
    #[serde(rename = "region")]
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    #[serde(rename = "postal_code")]
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
impl std::fmt::Display for AddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBalance {
    #[serde(rename = "date")]
    ///The date of the calculated historical balance, in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD)
    pub date: String,
    #[serde(rename = "current")]
    /**The total amount of funds in the account, calculated from the `current` balance in the `balance` object by subtracting inflows and adding back outflows according to the posted date of each transaction.

If the account has any pending transactions, historical balance amounts on or after the date of the earliest pending transaction may differ if retrieved in subsequent Asset Reports as a result of those pending transactions posting.*/
    pub current: f64,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the balance. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the balance. Always `null` if `iso_currency_code` is non-`null`.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for HistoricalBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "names")]
    /**A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. If the name of a business is reported, please contact Plaid Support. In the case of a joint account, Plaid will make a best effort to report the names of all account holders.

If an Item contains multiple accounts with different owner names, some institutions will report all names associated with the Item in each account's `names` array.*/
    pub names: Vec<String>,
    #[serde(rename = "phone_numbers")]
    ///A list of phone numbers associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub phone_numbers: Vec<PhoneNumber>,
    #[serde(rename = "emails")]
    ///A list of email addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub emails: Vec<Email>,
    #[serde(rename = "addresses")]
    ///Data about the various addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub addresses: Vec<Address>,
}
impl std::fmt::Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerOverride {
    #[serde(rename = "names")]
    ///A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. Note that the same name data will be used for all accounts associated with an Item.
    pub names: Vec<String>,
    #[serde(rename = "phone_numbers")]
    ///A list of phone numbers associated with the account.
    pub phone_numbers: Vec<PhoneNumber>,
    #[serde(rename = "emails")]
    ///A list of email addresses associated with the account.
    pub emails: Vec<Email>,
    #[serde(rename = "addresses")]
    ///Data about the various addresses associated with the account.
    pub addresses: Vec<Address>,
}
impl std::fmt::Display for OwnerOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesObject {
    #[serde(rename = "credit")]
    ///The credit accounts returned.
    pub credit: Option<Vec<CreditCardLiability>>,
    #[serde(rename = "mortgage")]
    ///The mortgage accounts returned.
    pub mortgage: Option<Vec<MortgageLiability>>,
    #[serde(rename = "student")]
    ///The student loan accounts returned.
    pub student: Option<Vec<StudentLoan>>,
}
impl std::fmt::Display for LiabilitiesObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoan {
    #[serde(rename = "account_id")]
    ///The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    #[serde(rename = "account_number")]
    ///The account number of the loan. For some institutions, this may be a masked version of the number (e.g., the last 4 digits instead of the entire number).
    pub account_number: Option<String>,
    #[serde(rename = "disbursement_dates")]
    ///The dates on which loaned funds were disbursed or will be disbursed. These are often in the past. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub disbursement_dates: Option<Vec<String>>,
    #[serde(rename = "expected_payoff_date")]
    ///The date when the student loan is expected to be paid off. Availability for this field is limited. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub expected_payoff_date: Option<String>,
    #[serde(rename = "guarantor")]
    ///The guarantor of the student loan.
    pub guarantor: Option<String>,
    #[serde(rename = "interest_rate_percentage")]
    ///The interest rate on the loan as a percentage.
    pub interest_rate_percentage: f64,
    #[serde(rename = "is_overdue")]
    ///`true` if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    #[serde(rename = "last_payment_amount")]
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    #[serde(rename = "last_payment_date")]
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_payment_date: Option<String>,
    #[serde(rename = "last_statement_issue_date")]
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_statement_issue_date: Option<String>,
    #[serde(rename = "loan_name")]
    ///The type of loan, e.g., "Consolidation Loans".
    pub loan_name: Option<String>,
    #[serde(rename = "loan_status")]
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    #[serde(rename = "minimum_payment_amount")]
    /**The minimum payment due for the next billing cycle. There are some exceptions:
Some institutions require a minimum payment across all loans associated with an account number. Our API presents that same minimum payment amount on each loan. The institutions that do this are: Great Lakes ( `ins_116861`), Firstmark (`ins_116295`), Commonbond Firstmark Services (`ins_116950`), Nelnet (`ins_116528`), EdFinancial Services (`ins_116304`), Granite State (`ins_116308`), and Oklahoma Student Loan Authority (`ins_116945`).
Firstmark (`ins_116295` ) and Navient (`ins_116248`) will display as $0 if there is an autopay program in effect.*/
    pub minimum_payment_amount: Option<f64>,
    #[serde(rename = "next_payment_due_date")]
    ///The due date for the next payment. The due date is `null` if a payment is not expected. A payment is not expected if `loan_status.type` is `deferment`, `in_school`, `consolidated`, `paid in full`, or `transferred`. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
    #[serde(rename = "origination_date")]
    /**The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
*/
    pub origination_date: Option<String>,
    #[serde(rename = "origination_principal_amount")]
    ///The original principal balance of the loan.
    pub origination_principal_amount: Option<f64>,
    #[serde(rename = "outstanding_interest_amount")]
    ///The total dollar amount of the accrued interest balance. For Sallie Mae ( `ins_116944`), this amount is included in the current balance of the loan, so this field will return as `null`.
    pub outstanding_interest_amount: Option<f64>,
    #[serde(rename = "payment_reference_number")]
    ///The relevant account number that should be used to reference this loan for payments. In the majority of cases, `payment_reference_number` will match a`ccount_number,` but in some institutions, such as Great Lakes (`ins_116861`), it will be different.
    pub payment_reference_number: Option<String>,
    #[serde(rename = "pslf_status")]
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (`ins_116527`).
    pub pslf_status: PslfStatus,
    #[serde(rename = "repayment_plan")]
    ///An object representing the repayment plan for the student loan
    pub repayment_plan: StudentRepaymentPlan,
    #[serde(rename = "sequence_number")]
    ///The sequence number of the student loan. Heartland ECSI (`ins_116948`) does not make this field available.
    pub sequence_number: Option<String>,
    #[serde(rename = "servicer_address")]
    ///The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
    pub servicer_address: ServicerAddressData,
    #[serde(rename = "ytd_interest_paid")]
    ///The year to date (YTD) interest paid. Availability for this field is limited.
    pub ytd_interest_paid: Option<f64>,
    #[serde(rename = "ytd_principal_paid")]
    ///The year to date (YTD) principal paid. Availability for this field is limited.
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for StudentLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardLiability {
    #[serde(rename = "account_id")]
    ///The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    #[serde(rename = "aprs")]
    ///The various interest rates that apply to the account. APR information is not provided by all card issuers; if APR data is not available, this array will be empty.
    pub aprs: Vec<Apr>,
    #[serde(rename = "is_overdue")]
    ///true if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    #[serde(rename = "last_payment_amount")]
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    #[serde(rename = "last_payment_date")]
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Availability for this field is limited.
    pub last_payment_date: Option<String>,
    #[serde(rename = "last_statement_issue_date")]
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_statement_issue_date: Option<String>,
    #[serde(rename = "last_statement_balance")]
    ///The total amount owed as of the last statement issued
    pub last_statement_balance: Option<f64>,
    #[serde(rename = "minimum_payment_amount")]
    ///The minimum payment due for the next billing cycle.
    pub minimum_payment_amount: Option<f64>,
    #[serde(rename = "next_payment_due_date")]
    ///The due date for the next payment. The due date is `null` if a payment is not expected. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
}
impl std::fmt::Display for CreditCardLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageLiability {
    #[serde(rename = "account_id")]
    ///The ID of the account that this liability belongs to.
    pub account_id: String,
    #[serde(rename = "account_number")]
    ///The account number of the loan.
    pub account_number: String,
    #[serde(rename = "current_late_fee")]
    ///The current outstanding amount charged for late payment.
    pub current_late_fee: Option<f64>,
    #[serde(rename = "escrow_balance")]
    ///Total amount held in escrow to pay taxes and insurance on behalf of the borrower.
    pub escrow_balance: Option<f64>,
    #[serde(rename = "has_pmi")]
    ///Indicates whether the borrower has private mortgage insurance in effect.
    pub has_pmi: Option<bool>,
    #[serde(rename = "has_prepayment_penalty")]
    ///Indicates whether the borrower will pay a penalty for early payoff of mortgage.
    pub has_prepayment_penalty: Option<bool>,
    #[serde(rename = "interest_rate")]
    ///Object containing metadata about the interest rate for the mortgage.
    pub interest_rate: MortgageInterestRate,
    #[serde(rename = "last_payment_amount")]
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    #[serde(rename = "last_payment_date")]
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_payment_date: Option<String>,
    #[serde(rename = "loan_type_description")]
    ///Description of the type of loan, for example `conventional`, `fixed`, or `variable`. This field is provided directly from the loan servicer and does not have an enumerated set of possible values.
    pub loan_type_description: Option<String>,
    #[serde(rename = "loan_term")]
    ///Full duration of mortgage as at origination (e.g. `10 year`).
    pub loan_term: Option<String>,
    #[serde(rename = "maturity_date")]
    ///Original date on which mortgage is due in full. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub maturity_date: Option<String>,
    #[serde(rename = "next_monthly_payment")]
    ///The amount of the next payment.
    pub next_monthly_payment: Option<f64>,
    #[serde(rename = "next_payment_due_date")]
    ///The due date for the next payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
    #[serde(rename = "origination_date")]
    ///The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub origination_date: Option<String>,
    #[serde(rename = "origination_principal_amount")]
    ///The original principal balance of the mortgage.
    pub origination_principal_amount: Option<f64>,
    #[serde(rename = "past_due_amount")]
    ///Amount of loan (principal + interest) past due for payment.
    pub past_due_amount: Option<f64>,
    #[serde(rename = "property_address")]
    ///Object containing fields describing property address.
    pub property_address: MortgagePropertyAddress,
    #[serde(rename = "ytd_interest_paid")]
    ///The year to date (YTD) interest paid.
    pub ytd_interest_paid: Option<f64>,
    #[serde(rename = "ytd_principal_paid")]
    ///The YTD principal paid.
    pub ytd_principal_paid: Option<f64>,
}
impl std::fmt::Display for MortgageLiability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageInterestRate {
    #[serde(rename = "percentage")]
    ///Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    pub percentage: Option<f64>,
    #[serde(rename = "type")]
    ///The type of interest charged (fixed or variable).
    pub type_: Option<String>,
}
impl std::fmt::Display for MortgageInterestRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgagePropertyAddress {
    #[serde(rename = "city")]
    ///The city name.
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    #[serde(rename = "postal_code")]
    ///The five or nine digit postal code.
    pub postal_code: Option<String>,
    #[serde(rename = "region")]
    ///The region or state (example "NC").
    pub region: Option<String>,
    #[serde(rename = "street")]
    ///The full street address (example "564 Main Street, Apt 15").
    pub street: Option<String>,
}
impl std::fmt::Display for MortgagePropertyAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoanStatus {
    #[serde(rename = "end_date")]
    /**The date until which the loan will be in its current status. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
*/
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    ///The status type of the student loan
    pub type_: Option<String>,
}
impl std::fmt::Display for StudentLoanStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentRepaymentPlan {
    #[serde(rename = "description")]
    ///The description of the repayment plan as provided by the servicer.
    pub description: Option<String>,
    #[serde(rename = "type")]
    ///The type of the repayment plan.
    pub type_: Option<String>,
}
impl std::fmt::Display for StudentRepaymentPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PslfStatus {
    #[serde(rename = "estimated_eligibility_date")]
    ///The estimated date borrower will have completed 120 qualifying monthly payments. Returned in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub estimated_eligibility_date: Option<String>,
    #[serde(rename = "payments_made")]
    ///The number of qualifying payments that have been made.
    pub payments_made: Option<f64>,
    #[serde(rename = "payments_remaining")]
    ///The number of qualifying payments remaining.
    pub payments_remaining: Option<f64>,
}
impl std::fmt::Display for PslfStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServicerAddressData {
    #[serde(rename = "city")]
    ///The full city name
    pub city: Option<String>,
    #[serde(rename = "region")]
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
impl std::fmt::Display for ServicerAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Apr {
    #[serde(rename = "apr_percentage")]
    /**Annual Percentage Rate applied.
*/
    pub apr_percentage: f64,
    #[serde(rename = "apr_type")]
    ///The type of balance to which the APR applies.
    pub apr_type: String,
    #[serde(rename = "balance_subject_to_apr")]
    ///Amount of money that is subjected to the APR if a balance was carried beyond payment due date. How it is calculated can vary by card issuer. It is often calculated as an average daily balance.
    pub balance_subject_to_apr: Option<f64>,
    #[serde(rename = "interest_charge_amount")]
    ///Amount of money charged due to interest from last statement.
    pub interest_charge_amount: Option<f64>,
}
impl std::fmt::Display for Apr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMetadata {
    #[serde(rename = "supported_methods")]
    ///Metadata specifically related to which auth methods an institution supports.
    pub supported_methods: Option<AuthSupportedMethods>,
}
impl std::fmt::Display for AuthMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSupportedMethods {
    #[serde(rename = "instant_auth")]
    ///Indicates if instant auth is supported.
    pub instant_auth: bool,
    #[serde(rename = "instant_match")]
    ///Indicates if instant match is supported.
    pub instant_match: bool,
    #[serde(rename = "automated_micro_deposits")]
    ///Indicates if automated microdeposits are supported.
    pub automated_micro_deposits: bool,
}
impl std::fmt::Display for AuthSupportedMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationMetadata {
    #[serde(rename = "supports_international_payments")]
    ///Indicates whether the institution supports payments from a different country.
    pub supports_international_payments: bool,
    #[serde(rename = "supports_sepa_instant")]
    ///Indicates whether the institution supports SEPA Instant payments.
    pub supports_sepa_instant: bool,
    #[serde(rename = "maximum_payment_amount")]
    /**A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.

Example: `{"GBP": "10000"}`
*/
    pub maximum_payment_amount: PaymentInitiationMaximumPaymentAmount,
    #[serde(rename = "supports_refund_details")]
    ///Indicates whether the institution supports returning refund details when initiating a payment.
    pub supports_refund_details: bool,
    #[serde(rename = "standing_order_metadata")]
    ///Metadata specifically related to valid Payment Initiation standing order configurations for the institution.
    pub standing_order_metadata: Option<PaymentInitiationStandingOrderMetadata>,
}
impl std::fmt::Display for PaymentInitiationMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationMaximumPaymentAmount {}
impl std::fmt::Display for PaymentInitiationMaximumPaymentAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationStandingOrderMetadata {
    #[serde(rename = "supports_standing_order_end_date")]
    ///Indicates whether the institution supports closed-ended standing orders by providing an end date.
    pub supports_standing_order_end_date: bool,
    #[serde(rename = "supports_standing_order_negative_execution_days")]
    ///This is only applicable to `MONTHLY` standing orders. Indicates whether the institution supports negative integers (-1 to -5) for setting up a `MONTHLY` standing order relative to the end of the month.
    pub supports_standing_order_negative_execution_days: bool,
    #[serde(rename = "valid_standing_order_intervals")]
    ///A list of the valid standing order intervals supported by the institution.
    pub valid_standing_order_intervals: Vec<String>,
}
impl std::fmt::Display for PaymentInitiationStandingOrderMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationAddress {
    #[serde(rename = "street")]
    ///An array of length 1-2 representing the street address where the recipient is located. Maximum of 70 characters.
    pub street: Vec<String>,
    #[serde(rename = "city")]
    ///The city where the recipient is located. Maximum of 35 characters.
    pub city: String,
    #[serde(rename = "postal_code")]
    ///The postal code where the recipient is located. Maximum of 16 characters.
    pub postal_code: String,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code where the recipient is located.
    pub country: String,
}
impl std::fmt::Display for PaymentInitiationAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleBase {
    #[serde(rename = "interval")]
    ///The frequency interval of the payment.
    pub interval: Option<String>,
    #[serde(rename = "interval_execution_day")]
    /**The day of the interval on which to schedule the payment.

If the payment interval is weekly, `interval_execution_day` should be an integer from 1 (Monday) to 7 (Sunday).

If the payment interval is monthly, `interval_execution_day` should be an integer indicating which day of the month to make the payment on. Integers from 1 to 28 can be used to make a payment on that day of the month. Negative integers from -1 to -5 can be used to make a payment relative to the end of the month. To make a payment on the last day of the month, use -1; to make the payment on the second-to-last day, use -2, and so on.*/
    pub interval_execution_day: Option<i64>,
    #[serde(rename = "start_date")]
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will begin on the first `interval_execution_day` on or after the `start_date`.

If the first `interval_execution_day` on or after the start date is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make the first payment on that day, but it is not guaranteed to do so.*/
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will end on the last `interval_execution_day` on or before the `end_date`.
If the only `interval_execution_day` between the start date and the end date (inclusive) is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make a payment on that day, but it is not guaranteed to do so.*/
    pub end_date: Option<String>,
    #[serde(rename = "adjusted_start_date")]
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, this field will be `null`.
    pub adjusted_start_date: Option<String>,
}
impl std::fmt::Display for ExternalPaymentScheduleBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentScheduleInterval {
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "MONTHLY")]
    Monthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentScheme {
    #[serde(rename = "FASTER_PAYMENTS")]
    FasterPayments,
    #[serde(rename = "SEPA_CREDIT_TRANSFER")]
    SepaCreditTransfer,
    #[serde(rename = "SEPA_CREDIT_TRANSFER_INSTANT")]
    SepaCreditTransferInstant,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationConsentScope {
    #[serde(rename = "ME_TO_ME")]
    MeToMe,
    #[serde(rename = "EXTERNAL")]
    External,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentInitiationConsentOptions {
    #[serde(rename = "wallet_id")]
    ///The EMI (E-Money Institution) wallet that this payment consent is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    #[serde(rename = "request_refund_details")]
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    pub request_refund_details: Option<bool>,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the payer's account. If provided, the end user will be able to set up payment consent using only the specified bank account.
    pub iban: Option<String>,
    #[serde(rename = "bacs")]
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    pub bacs: Option<PaymentInitiationOptionalRestrictionBacs>,
}
impl std::fmt::Display for ExternalPaymentInitiationConsentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentConstraints {
    #[serde(rename = "valid_date_time")]
    ///Life span for the payment consent. After the `to` date the payment consent expires and can no longer be used for payment initiation.
    pub valid_date_time: Option<PaymentConsentValidDateTime>,
    #[serde(rename = "max_payment_amount")]
    ///Maximum amount of a single payment initiated using the payment consent.
    pub max_payment_amount: PaymentConsentMaxPaymentAmount,
    #[serde(rename = "periodic_amounts")]
    ///A list of amount limitations per period of time.
    pub periodic_amounts: Vec<PaymentConsentPeriodicAmount>,
}
impl std::fmt::Display for PaymentInitiationConsentConstraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentMaxPaymentAmount(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsentPaymentIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentOptions {
    #[serde(rename = "request_refund_details")]
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    pub request_refund_details: Option<bool>,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the payer's account. If provided, the end user will be able to send payments only from the specified bank account.
    pub iban: Option<String>,
    #[serde(rename = "bacs")]
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    pub bacs: Option<PaymentInitiationOptionalRestrictionBacs>,
    #[serde(rename = "wallet_id")]
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    #[serde(rename = "scheme")]
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub scheme: Option<String>,
}
impl std::fmt::Display for ExternalPaymentOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentRefundDetails {
    #[serde(rename = "name")]
    ///The name of the account holder.
    pub name: String,
    #[serde(rename = "iban")]
    ///The International Bank Account Number (IBAN) for the account.
    pub iban: Option<String>,
    #[serde(rename = "bacs")]
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacsNullable>,
}
impl std::fmt::Display for ExternalPaymentRefundDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleGet(pub serde_json::Value);
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
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStatus {
    #[serde(rename = "status")]
    /**This field is deprecated in favor of the `breakdown` object, which provides more granular institution health data.

`HEALTHY`: the majority of requests are successful
`DEGRADED`: only some requests are successful
`DOWN`: all requests are failing*/
    pub status: String,
    #[serde(rename = "last_status_change")]
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) formatted timestamp of the last status change for the institution.
*/
    pub last_status_change: String,
    #[serde(rename = "breakdown")]
    ///A detailed breakdown of the institution's performance for a request type. The values for `success`, `error_plaid`, and `error_institution` sum to 1.
    pub breakdown: ProductStatusBreakdown,
}
impl std::fmt::Display for ProductStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStatusBreakdown {
    #[serde(rename = "success")]
    ///The percentage of login attempts that are successful, expressed as a decimal.
    pub success: f64,
    #[serde(rename = "error_plaid")]
    /**The percentage of logins that are failing due to an internal Plaid issue, expressed as a decimal.
*/
    pub error_plaid: f64,
    #[serde(rename = "error_institution")]
    ///The percentage of logins that are failing due to an issue in the institution's system, expressed as a decimal.
    pub error_institution: f64,
    #[serde(rename = "refresh_interval")]
    ///The `refresh_interval` may be `DELAYED` or `STOPPED` even when the success rate is high. This value is only returned for Transactions status breakdowns.
    pub refresh_interval: Option<String>,
}
impl std::fmt::Display for ProductStatusBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCustomPassword {
    #[serde(rename = "version")]
    ///The version of the password schema to use, possible values are 1 or 2. The default value is 2. You should only specify 1 if you know it is necessary for your test suite.
    pub version: Option<String>,
    #[serde(rename = "seed")]
    /**A seed, in the form of a string, that will be used to randomly generate account and transaction data, if this data is not specified using the `override_accounts` argument. If no seed is specified, the randomly generated data will be different each time.

Note that transactions data is generated relative to the Item's creation date. Different Items created on different dates with the same seed for transactions data will have different dates for the transactions. The number of days between each transaction and the Item creation will remain constant. For example, an Item created on December 15 might show a transaction on December 14. An Item created on December 20, using the same seed, would show that same transaction occurring on December 19.*/
    pub seed: String,
    #[serde(rename = "override_accounts")]
    ///An array of account overrides to configure the accounts for the Item. By default, if no override is specified, transactions and account data will be randomly generated based on the account type and subtype, and other products will have fixed or empty data.
    pub override_accounts: Vec<OverrideAccounts>,
    #[serde(rename = "mfa")]
    ///Specifies the multi-factor authentication settings to use with this test account
    pub mfa: Mfa,
    #[serde(rename = "recaptcha")]
    ///You may trigger a reCAPTCHA in Plaid Link in the Sandbox environment by using the recaptcha field. Possible values are `good` or `bad`. A value of `good` will result in successful Item creation and `bad` will result in a `RECAPTCHA_BAD` error to simulate a failed reCAPTCHA. Both values require the reCAPTCHA to be manually solved within Plaid Link.
    pub recaptcha: String,
    #[serde(rename = "force_error")]
    /**An error code to force on Item creation. Possible values are:

`"INSTITUTION_NOT_RESPONDING"`
`"INSTITUTION_NO_LONGER_SUPPORTED"`
`"INVALID_CREDENTIALS"`
`"INVALID_MFA"`
`"ITEM_LOCKED"`
`"ITEM_LOGIN_REQUIRED"`
`"ITEM_NOT_SUPPORTED"`
`"INVALID_LINK_TOKEN"`
`"MFA_NOT_SUPPORTED"`
`"NO_ACCOUNTS"`
`"PLAID_ERROR"`
`"USER_SETUP_REQUIRED"`*/
    pub force_error: String,
}
impl std::fmt::Display for UserCustomPassword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Mfa {
    #[serde(rename = "type")]
    /**Possible values are `device`, `selections`, or `questions`.

If value is `device`, the MFA answer is `1234`.

If value is `selections`, the MFA answer is always the first option.

If value is `questions`, the MFA answer is  `answer_<i>_<j>` for the j-th question in the i-th round, starting from 0. For example, the answer to the first question in the second round is `answer_1_0`.*/
    pub type_: String,
    #[serde(rename = "question_rounds")]
    ///Number of rounds of questions. Required if value of `type` is `questions`.
    pub question_rounds: f64,
    #[serde(rename = "questions_per_round")]
    ///Number of questions per round. Required if value of `type` is `questions`. If value of type is `selections`, default value is 2.
    pub questions_per_round: f64,
    #[serde(rename = "selection_rounds")]
    ///Number of rounds of selections, used if `type` is `selections`. Defaults to 1.
    pub selection_rounds: f64,
    #[serde(rename = "selections_per_question")]
    /**Number of available answers per question, used if `type` is `selection`. Defaults to 2.
*/
    pub selections_per_question: f64,
}
impl std::fmt::Display for Mfa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OverrideAccounts {
    #[serde(rename = "type")]
    /**`investment:` Investment account.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`payroll:` Payroll account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    pub type_: String,
    #[serde(rename = "subtype")]
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    pub subtype: Option<String>,
    #[serde(rename = "starting_balance")]
    /**If provided, the account will start with this amount as the current balance.
*/
    pub starting_balance: f64,
    #[serde(rename = "force_available_balance")]
    ///If provided, the account will always have this amount as its  available balance, regardless of current balance or changes in transactions over time.
    pub force_available_balance: f64,
    #[serde(rename = "currency")]
    ///ISO-4217 currency code. If provided, the account will be denominated in the given currency. Transactions will also be in this currency by default.
    pub currency: String,
    #[serde(rename = "meta")]
    ///Allows specifying the metadata of the test account
    pub meta: Meta,
    #[serde(rename = "numbers")]
    ///Account and bank identifier number data used to configure the test account. All values are optional.
    pub numbers: Numbers,
    #[serde(rename = "transactions")]
    ///Specify the list of transactions on the account.
    pub transactions: Vec<TransactionOverride>,
    #[serde(rename = "holdings")]
    ///Specify the holdings on the account.
    pub holdings: Option<HoldingsOverride>,
    #[serde(rename = "investment_transactions")]
    ///Specify the list of investments transactions on the account.
    pub investment_transactions: Option<InvestmentsTransactionsOverride>,
    #[serde(rename = "identity")]
    ///Data about the owner or owners of an account. Any fields not specified will be filled in with default Sandbox information.
    pub identity: OwnerOverride,
    #[serde(rename = "liability")]
    ///Used to configure Sandbox test data for the Liabilities product
    pub liability: LiabilityOverride,
    #[serde(rename = "inflow_model")]
    ///The `inflow_model` allows you to model a test account that receives regular income or make regular payments on a loan. Any transactions generated by the `inflow_model` will appear in addition to randomly generated test data or transactions specified by `override_accounts`.
    pub inflow_model: InflowModel,
    #[serde(rename = "income")]
    ///Specify payroll data on the account.
    pub income: Option<IncomeOverride>,
}
impl std::fmt::Display for OverrideAccounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "name")]
    ///The account's name
    pub name: String,
    #[serde(rename = "official_name")]
    ///The account's official name
    pub official_name: String,
    #[serde(rename = "limit")]
    ///The account's limit
    pub limit: f64,
}
impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Numbers {
    #[serde(rename = "account")]
    ///Will be used for the account number.
    pub account: Option<String>,
    #[serde(rename = "ach_routing")]
    ///Must be a valid ACH routing number.
    pub ach_routing: Option<String>,
    #[serde(rename = "ach_wire_routing")]
    ///Must be a valid wire transfer routing number.
    pub ach_wire_routing: Option<String>,
    #[serde(rename = "eft_institution")]
    ///EFT institution number. Must be specified alongside `eft_branch`.
    pub eft_institution: Option<String>,
    #[serde(rename = "eft_branch")]
    ///EFT branch number. Must be specified alongside `eft_institution`.
    pub eft_branch: Option<String>,
    #[serde(rename = "international_bic")]
    ///Bank identifier code (BIC). Must be specified alongside `international_iban`.
    pub international_bic: Option<String>,
    #[serde(rename = "international_iban")]
    ///International bank account number (IBAN). If no account number is specified via `account`, will also be used as the account number by default. Must be specified alongside `international_bic`.
    pub international_iban: Option<String>,
    #[serde(rename = "bacs_sort_code")]
    ///BACS sort code
    pub bacs_sort_code: Option<String>,
}
impl std::fmt::Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOverride {
    #[serde(rename = "date_transacted")]
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Transactions in Sandbox will move from pending to posted once their transaction date has been reached. If a `date_transacted` is not provided by the institution, a transaction date may be available in the [`authorized_date`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-authorized-date) field.
    pub date_transacted: String,
    #[serde(rename = "date_posted")]
    ///The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Posted dates in the past or present will result in posted transactions; posted dates in the future will result in pending transactions.
    pub date_posted: String,
    #[serde(rename = "amount")]
    ///The transaction amount. Can be negative.
    pub amount: f64,
    #[serde(rename = "description")]
    ///The transaction description.
    pub description: String,
    #[serde(rename = "currency")]
    ///The ISO-4217 format currency code for the transaction.
    pub currency: Option<String>,
}
impl std::fmt::Display for TransactionOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityOverride {
    #[serde(rename = "isin")]
    ///12-character ISIN, a globally unique securities identifier.
    pub isin: Option<String>,
    #[serde(rename = "cusip")]
    ///9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: Option<String>,
    #[serde(rename = "sedol")]
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: Option<String>,
    #[serde(rename = "name")]
    ///A descriptive name for the security, suitable for display.
    pub name: Option<String>,
    #[serde(rename = "ticker_symbol")]
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: Option<String>,
    #[serde(rename = "currency")]
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: Option<String>,
}
impl std::fmt::Display for SecurityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HoldingsOverride {
    #[serde(rename = "institution_price")]
    ///The last price given by the institution for this security
    pub institution_price: f64,
    #[serde(rename = "institution_price_as_of")]
    ///The date at which `institution_price` was current. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub institution_price_as_of: Option<String>,
    #[serde(rename = "cost_basis")]
    ///The average original value of the holding. Multiple cost basis values for the same security purchased at different prices are not supported.
    pub cost_basis: Option<f64>,
    #[serde(rename = "quantity")]
    ///The total quantity of the asset held, as reported by the financial institution.
    pub quantity: f64,
    #[serde(rename = "currency")]
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    #[serde(rename = "security")]
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: SecurityOverride,
}
impl std::fmt::Display for HoldingsOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsOverride {
    #[serde(rename = "date")]
    ///Posting date for the transaction. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub date: String,
    #[serde(rename = "name")]
    ///The institution's description of the transaction.
    pub name: String,
    #[serde(rename = "quantity")]
    ///The number of units of the security involved in this transaction. Must be positive if the type is a buy and negative if the type is a sell.
    pub quantity: f64,
    #[serde(rename = "price")]
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    #[serde(rename = "fees")]
    ///The combined value of all fees applied to this transaction.
    pub fees: Option<f64>,
    #[serde(rename = "type")]
    /**The type of the investment transaction. Possible values are:
`buy`: Buying an investment
`sell`: Selling an investment
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer*/
    pub type_: String,
    #[serde(rename = "currency")]
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    #[serde(rename = "security")]
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: Option<SecurityOverride>,
}
impl std::fmt::Display for InvestmentsTransactionsOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityOverride {
    #[serde(rename = "type")]
    ///The type of the liability object, either `credit` or `student`. Mortgages are not currently supported in the custom Sandbox.
    pub type_: String,
    #[serde(rename = "purchase_apr")]
    ///The purchase APR percentage value. For simplicity, this is the only interest rate used to calculate interest charges. Can only be set if `type` is `credit`.
    pub purchase_apr: f64,
    #[serde(rename = "cash_apr")]
    ///The cash APR percentage value. Can only be set if `type` is `credit`.
    pub cash_apr: f64,
    #[serde(rename = "balance_transfer_apr")]
    ///The balance transfer APR percentage value. Can only be set if `type` is `credit`. Can only be set if `type` is `credit`.
    pub balance_transfer_apr: f64,
    #[serde(rename = "special_apr")]
    ///The special APR percentage value. Can only be set if `type` is `credit`.
    pub special_apr: f64,
    #[serde(rename = "last_payment_amount")]
    ///Override the `last_payment_amount` field. Can only be set if `type` is `credit`.
    pub last_payment_amount: f64,
    #[serde(rename = "minimum_payment_amount")]
    ///Override the `minimum_payment_amount` field. Can only be set if `type` is `credit` or `student`.
    pub minimum_payment_amount: f64,
    #[serde(rename = "is_overdue")]
    ///Override the `is_overdue` field
    pub is_overdue: bool,
    #[serde(rename = "origination_date")]
    ///The date on which the loan was initially lent, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Can only be set if `type` is `student`.
    pub origination_date: String,
    #[serde(rename = "principal")]
    ///The original loan principal. Can only be set if `type` is `student`.
    pub principal: f64,
    #[serde(rename = "nominal_apr")]
    ///The interest rate on the loan as a percentage. Can only be set if `type` is `student`.
    pub nominal_apr: f64,
    #[serde(rename = "interest_capitalization_grace_period_months")]
    ///If set, interest capitalization begins at the given number of months after loan origination. By default interest is never capitalized. Can only be set if `type` is `student`.
    pub interest_capitalization_grace_period_months: f64,
    #[serde(rename = "repayment_model")]
    ///Student loan repayment information used to configure Sandbox test data for the Liabilities product
    pub repayment_model: StudentLoanRepaymentModel,
    #[serde(rename = "expected_payoff_date")]
    ///Override the `expected_payoff_date` field. Can only be set if `type` is `student`.
    pub expected_payoff_date: String,
    #[serde(rename = "guarantor")]
    ///Override the `guarantor` field. Can only be set if `type` is `student`.
    pub guarantor: String,
    #[serde(rename = "is_federal")]
    ///Override the `is_federal` field. Can only be set if `type` is `student`.
    pub is_federal: bool,
    #[serde(rename = "loan_name")]
    ///Override the `loan_name` field. Can only be set if `type` is `student`.
    pub loan_name: String,
    #[serde(rename = "loan_status")]
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    #[serde(rename = "payment_reference_number")]
    ///Override the `payment_reference_number` field. Can only be set if `type` is `student`.
    pub payment_reference_number: String,
    #[serde(rename = "pslf_status")]
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (`ins_116527`).
    pub pslf_status: PslfStatus,
    #[serde(rename = "repayment_plan_description")]
    ///Override the `repayment_plan.description` field. Can only be set if `type` is `student`.
    pub repayment_plan_description: String,
    #[serde(rename = "repayment_plan_type")]
    ///Override the `repayment_plan.type` field. Can only be set if `type` is `student`. Possible values are: `"extended graduated"`, `"extended standard"`, `"graduated"`, `"income-contingent repayment"`, `"income-based repayment"`, `"interest only"`, `"other"`, `"pay as you earn"`, `"revised pay as you earn"`, or `"standard"`.
    pub repayment_plan_type: String,
    #[serde(rename = "sequence_number")]
    ///Override the `sequence_number` field. Can only be set if `type` is `student`.
    pub sequence_number: String,
    #[serde(rename = "servicer_address")]
    ///A physical mailing address.
    pub servicer_address: Address,
}
impl std::fmt::Display for LiabilityOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoanRepaymentModel {
    #[serde(rename = "type")]
    ///The only currently supported value for this field is `standard`.
    pub type_: String,
    #[serde(rename = "non_repayment_months")]
    ///Configures the number of months before repayment starts.
    pub non_repayment_months: f64,
    #[serde(rename = "repayment_months")]
    ///Configures the number of months of repayments before the loan is paid off.
    pub repayment_months: f64,
}
impl std::fmt::Display for StudentLoanRepaymentModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InflowModel {
    #[serde(rename = "type")]
    /**Inflow model. One of the following:

`none`: No income

`monthly-income`: Income occurs once per month `monthly-balance-payment`: Pays off the balance on a liability account at the given statement day of month.

`monthly-interest-only-payment`: Makes an interest-only payment on a liability account at the given statement day of month.

Note that account types supported by Liabilities will accrue interest in the Sandbox. The types impacted are account type `credit` with subtype `credit` or `paypal`, and account type `loan` with subtype `student` or `mortgage`.*/
    pub type_: String,
    #[serde(rename = "income_amount")]
    ///Amount of income per month. This value is required if `type` is `monthly-income`.
    pub income_amount: f64,
    #[serde(rename = "payment_day_of_month")]
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the income transaction will appear. The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub payment_day_of_month: f64,
    #[serde(rename = "transaction_name")]
    ///The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub transaction_name: String,
    #[serde(rename = "statement_day_of_month")]
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the balance is calculated for the next payment. The name of the income transaction. This field is required if `type` is `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub statement_day_of_month: String,
}
impl std::fmt::Display for InflowModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeOverride {
    #[serde(rename = "paystubs")]
    ///A list of paystubs associated with the account.
    pub paystubs: Option<Vec<PaystubOverride>>,
}
impl std::fmt::Display for IncomeOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverride {
    #[serde(rename = "employer")]
    ///The employer on the paystub.
    pub employer: Option<PaystubOverrideEmployer>,
    #[serde(rename = "employee")]
    ///The employee on the paystub.
    pub employee: Option<PaystubOverrideEmployee>,
    #[serde(rename = "income_breakdown")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    #[serde(rename = "pay_period_details")]
    ///Details about the pay period.
    pub pay_period_details: Option<PayPeriodDetails>,
}
impl std::fmt::Display for PaystubOverride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployer {
    #[serde(rename = "name")]
    ///The name of the employer.
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployee {
    #[serde(rename = "name")]
    ///The name of the employee.
    pub name: Option<String>,
    #[serde(rename = "address")]
    ///The address of the employee.
    pub address: Option<PaystubOverrideEmployeeAddress>,
}
impl std::fmt::Display for PaystubOverrideEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployeeAddress {
    #[serde(rename = "city")]
    ///The full city name.
    pub city: Option<String>,
    #[serde(rename = "region")]
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    #[serde(rename = "postal_code")]
    ///5 digit postal code.
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///The country of the address.
    pub country: Option<String>,
}
impl std::fmt::Display for PaystubOverrideEmployeeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticallyVerifiedWebhook {
    #[serde(rename = "webhook_type")]
    ///`AUTH`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`AUTOMATICALLY_VERIFIED`
    pub webhook_code: String,
    #[serde(rename = "account_id")]
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for AutomaticallyVerifiedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtHeader {
    #[serde(rename = "id")]
    pub id: String,
}
impl std::fmt::Display for JwtHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationExpiredWebhook {
    #[serde(rename = "webhook_type")]
    ///`AUTH`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`VERIFICATION_EXPIRED`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "account_id")]
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
}
impl std::fmt::Display for VerificationExpiredWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookUpdateAcknowledgedWebhook {
    #[serde(rename = "webhook_type")]
    ///`ITEM`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`WEBHOOK_UPDATE_ACKNOWLEDGED`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "new_webhook_url")]
    ///The new webhook URL
    pub new_webhook_url: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for WebhookUpdateAcknowledgedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PendingExpirationWebhook {
    #[serde(rename = "webhook_type")]
    ///`ITEM`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`PENDING_EXPIRATION`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "consent_expiration_time")]
    ///The date and time at which the Item's access consent will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub consent_expiration_time: String,
}
impl std::fmt::Display for PendingExpirationWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemErrorWebhook {
    #[serde(rename = "webhook_type")]
    ///`ITEM`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`ERROR`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for ItemErrorWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemProductReadyWebhook {
    #[serde(rename = "webhook_type")]
    ///`INCOME`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`PRODUCT_READY`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for ItemProductReadyWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecaptchaRequiredError {
    #[serde(rename = "error_type")]
    ///RECAPTCHA_ERROR
    pub error_type: String,
    #[serde(rename = "error_code")]
    ///RECAPTCHA_REQUIRED
    pub error_code: String,
    #[serde(rename = "display_message")]
    pub display_message: String,
    #[serde(rename = "http_code")]
    ///400
    pub http_code: String,
    #[serde(rename = "link_user_experience")]
    ///Your user will be prompted to solve a Google reCAPTCHA challenge in the Link Recaptcha pane. If they solve the challenge successfully, the user's request is resubmitted and they are directed to the next Item creation step.
    pub link_user_experience: String,
    #[serde(rename = "common_causes")]
    ///Plaid's fraud system detects abusive traffic and considers a variety of parameters throughout Item creation requests. When a request is considered risky or possibly fraudulent, Link presents a reCAPTCHA for the user to solve.
    pub common_causes: String,
    #[serde(rename = "troubleshooting_steps")]
    /**Link will automatically guide your user through reCAPTCHA verification. As a general rule, we recommend instrumenting basic fraud monitoring to detect and protect your website from spam and abuse.

If your user cannot verify their session, please submit a Support ticket with the following identifiers: `link_session_id` or `request_id`*/
    pub troubleshooting_steps: String,
}
impl std::fmt::Display for RecaptchaRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransfersEventsUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`BANK_TRANSFERS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`BANK_TRANSFERS_EVENTS_UPDATE`
    pub webhook_code: String,
}
impl std::fmt::Display for BankTransfersEventsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventsUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`TRANSFER`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`TRANSFER_EVENTS_UPDATE`
    pub webhook_code: String,
}
impl std::fmt::Display for TransferEventsUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsDefaultUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`INVESTMENTS_TRANSACTIONS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "new_investments_transactions")]
    ///The number of new transactions reported since the last time this webhook was fired.
    pub new_investments_transactions: f64,
    #[serde(rename = "canceled_investments_transactions")]
    ///The number of canceled transactions reported since the last time this webhook was fired.
    pub canceled_investments_transactions: f64,
}
impl std::fmt::Display for InvestmentsDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HoldingsDefaultUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`HOLDINGS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "new_holdings")]
    ///The number of new holdings reported since the last time this webhook was fired.
    pub new_holdings: f64,
    #[serde(rename = "updated_holdings")]
    ///The number of updated holdings reported since the last time this webhook was fired.
    pub updated_holdings: f64,
}
impl std::fmt::Display for HoldingsDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesDefaultUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`LIABILITIES`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "account_ids_with_new_liabilities")]
    ///An array of `account_id`'s for accounts that contain new liabilities.'
    pub account_ids_with_new_liabilities: Vec<String>,
    #[serde(rename = "account_ids_with_updated_liabilities")]
    /**An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["past_amount_due"] }`
*/
    pub account_ids_with_updated_liabilities: LiabilitiesAccountIdsWithUpdatedLiabilities,
}
impl std::fmt::Display for LiabilitiesDefaultUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesAccountIdsWithUpdatedLiabilities {}
impl std::fmt::Display for LiabilitiesAccountIdsWithUpdatedLiabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsProductReadyWebhook {
    #[serde(rename = "webhook_type")]
    ///`ASSETS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`PRODUCT_READY`
    pub webhook_code: String,
    #[serde(rename = "asset_report_id")]
    ///The `asset_report_id` that can be provided to `/asset_report/get` to retrieve the Asset Report.
    pub asset_report_id: String,
}
impl std::fmt::Display for AssetsProductReadyWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsErrorWebhook {
    #[serde(rename = "webhook_type")]
    ///`ASSETS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`ERROR`
    pub webhook_code: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "asset_report_id")]
    ///The ID associated with the Asset Report.
    pub asset_report_id: String,
}
impl std::fmt::Display for AssetsErrorWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsRelayWebhook {
    #[serde(rename = "webhook_type")]
    ///`ASSETS`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`RELAY_EVENT`
    pub webhook_code: String,
    #[serde(rename = "relay_event")]
    ///The webhook code indicating which endpoint was called. It can be one of `GET_CALLED`, `REFRESH_CALLED` or `AUDIT_COPY_CREATE_CALLED`.
    pub relay_event: String,
    #[serde(rename = "secondary_client_id")]
    ///The id of the client with whom the Asset Report is being shared.
    pub secondary_client_id: String,
    #[serde(rename = "asset_relay_token")]
    ///The `asset_relay_token` that was created by calling `/asset_report/relay/create.
    pub asset_relay_token: String,
    #[serde(rename = "asset_report_id")]
    ///The `asset_report_id` that can be provided to `/asset_report/relay/get` to retrieve the Asset Report.
    pub asset_report_id: String,
}
impl std::fmt::Display for AssetsRelayWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RelayEvent {
    #[serde(rename = "GET_CALLED")]
    GetCalled,
    #[serde(rename = "REFRESH_CALLED")]
    RefreshCalled,
    #[serde(rename = "AUDIT_COPY_CREATE_CALLED")]
    AuditCopyCreateCalled,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cause {}
impl std::fmt::Display for Cause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    #[serde(rename = "warning_type")]
    ///The warning type, which will always be `ASSET_REPORT_WARNING`
    pub warning_type: String,
    #[serde(rename = "warning_code")]
    ///The warning code identifies a specific kind of warning. Currently, the only possible warning code is `OWNERS_UNAVAILABLE`, which indicates that account-owner information is not available.
    pub warning_code: String,
    #[serde(rename = "cause")]
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: Cause,
}
impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentAmountCurrency {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentAmount {
    #[serde(rename = "currency")]
    ///The ISO-4217 currency code of the payment. For standing orders and payment consents, `"GBP"` must be used.
    pub currency: String,
    #[serde(rename = "value")]
    ///The amount of the payment. Must contain at most two digits of precision e.g. `1.23`. Minimum accepted value is `1`.
    pub value: f64,
}
impl std::fmt::Display for PaymentAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentValidDateTime {
    #[serde(rename = "from")]
    ///The date and time from which the consent should be active, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub from: Option<String>,
    #[serde(rename = "to")]
    ///The date and time at which the consent expires, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub to: Option<String>,
}
impl std::fmt::Display for PaymentConsentValidDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmount {
    #[serde(rename = "amount")]
    ///Maximum cumulative amount for all payments in the specified interval.
    pub amount: PaymentConsentPeriodicAmountAmount,
    #[serde(rename = "interval")]
    ///Payment consent periodic interval.
    pub interval: String,
    #[serde(rename = "alignment")]
    /**Where the payment consent period should start.

`CALENDAR`: line up with a calendar.

`CONSENT`: on the date of consent creation.*/
    pub alignment: String,
}
impl std::fmt::Display for PaymentConsentPeriodicAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmountAmount(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicInterval {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "MONTH")]
    Month,
    #[serde(rename = "YEAR")]
    Year,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicAlignment {
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "CONSENT")]
    Consent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportUser {
    #[serde(rename = "client_user_id")]
    ///An identifier you determine and submit for the user.
    pub client_user_id: Option<String>,
    #[serde(rename = "first_name")]
    ///The user's first name. Required for the Fannie Mae Day 1 Certainty™ program.
    pub first_name: Option<String>,
    #[serde(rename = "middle_name")]
    ///The user's middle name
    pub middle_name: Option<String>,
    #[serde(rename = "last_name")]
    ///The user's last name.  Required for the Fannie Mae Day 1 Certainty™ program.
    pub last_name: Option<String>,
    #[serde(rename = "ssn")]
    /**The user's Social Security Number. Required for the Fannie Mae Day 1 Certainty™ program.

Format: "ddd-dd-dddd"*/
    pub ssn: Option<String>,
    #[serde(rename = "phone_number")]
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    pub phone_number: Option<String>,
    #[serde(rename = "email")]
    ///The user's email address.
    pub email: Option<String>,
}
impl std::fmt::Display for AssetReportUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshAssetReportToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneCurrencyCodeList {
    #[serde(rename = "iso_currency_code")]
    ///Plaid supports all ISO 4217 currency codes.
    pub iso_currency_code: String,
    #[serde(rename = "unofficial_currency_code")]
    ///List of unofficial currency codes
    pub unofficial_currency_code: String,
}
impl std::fmt::Display for StandaloneCurrencyCodeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnofficialCurrencyCodeList(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneAccountType {
    #[serde(rename = "depository")]
    ///An account type holding cash, in which funds are deposited. Supported products for `depository` accounts are: Auth (`checking` and `savings` types only), Balance, Transactions, Identity, Payment Initiation, and Assets.
    pub depository: String,
    #[serde(rename = "credit")]
    ///A credit card type account. Supported products for `credit` accounts are: Balance, Transactions, Identity, and Liabilities.
    pub credit: String,
    #[serde(rename = "loan")]
    ///A loan type account. Supported products for `loan` accounts are: Balance, Liabilities, and Transactions.
    pub loan: String,
    #[serde(rename = "investment")]
    ///An investment account. Supported products for `investment` accounts are: Balance and Investments. In API versions 2018-05-22 and earlier, this type is called `brokerage`.
    pub investment: String,
    #[serde(rename = "other")]
    ///Other or unknown account type. Supported products for `other` accounts are: Balance, Transactions, Identity, and Assets.
    pub other: String,
}
impl std::fmt::Display for StandaloneAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryAccount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAccount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAccount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentAccountSubtypeStandalone(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReport {
    #[serde(rename = "asset_report_id")]
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: String,
    #[serde(rename = "client_report_id")]
    ///An identifier you determine and submit for the Asset Report.
    pub client_report_id: Option<String>,
    #[serde(rename = "date_generated")]
    ///The date and time when the Asset Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub date_generated: String,
    #[serde(rename = "days_requested")]
    ///The duration of transaction history you requested
    pub days_requested: f64,
    #[serde(rename = "user")]
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: AssetReportUser,
    #[serde(rename = "items")]
    ///Data returned by Plaid about each of the Items included in the Asset Report.
    pub items: Vec<AssetReportItem>,
}
impl std::fmt::Display for AssetReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportItem {
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "institution_name")]
    ///The full financial institution name associated with the Item.
    pub institution_name: String,
    #[serde(rename = "institution_id")]
    ///The id of the financial institution associated with the Item.
    pub institution_id: String,
    #[serde(rename = "date_last_updated")]
    ///The date and time when this Item’s data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub date_last_updated: String,
    #[serde(rename = "accounts")]
    ///Data about each of the accounts open on the Item.
    pub accounts: Vec<AccountAssets>,
}
impl std::fmt::Display for AssetReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStatusUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`PAYMENT_INITIATION`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`PAYMENT_STATUS_UPDATE`
    pub webhook_code: String,
    #[serde(rename = "payment_id")]
    ///The `payment_id` for the payment being updated
    pub payment_id: String,
    #[serde(rename = "new_payment_status")]
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub new_payment_status: String,
    #[serde(rename = "old_payment_status")]
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub old_payment_status: String,
    #[serde(rename = "original_reference")]
    ///The original value of the reference when creating the payment.
    pub original_reference: Option<String>,
    #[serde(rename = "adjusted_reference")]
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    pub adjusted_reference: Option<String>,
    #[serde(rename = "original_start_date")]
    ///The original value of the `start_date` provided during the creation of a standing order. If the payment is not a standing order, this field will be `null`.
    pub original_start_date: Option<String>,
    #[serde(rename = "adjusted_start_date")]
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, or if the payment is not a standing order, this field will be `null`.
    pub adjusted_start_date: Option<String>,
    #[serde(rename = "timestamp")]
    ///The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2017-09-14T14:42:19.350Z"`
    pub timestamp: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for PaymentStatusUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Holding {
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` associated with the holding.
    pub account_id: String,
    #[serde(rename = "security_id")]
    ///The Plaid `security_id` associated with the holding.
    pub security_id: String,
    #[serde(rename = "institution_price")]
    ///The last price given by the institution for this security.
    pub institution_price: f64,
    #[serde(rename = "institution_price_as_of")]
    ///The date at which `institution_price` was current.
    pub institution_price_as_of: Option<String>,
    #[serde(rename = "institution_price_datetime")]
    /**Date and time at which `institution_price` was current, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ).

This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).
*/
    pub institution_price_datetime: Option<String>,
    #[serde(rename = "institution_value")]
    ///The value of the holding, as reported by the institution.
    pub institution_value: f64,
    #[serde(rename = "cost_basis")]
    ///The original total value or the purchase price per share of the holding. This field is an aggregate on a per holding basis and dependent on the information provided by the institution.
    pub cost_basis: Option<f64>,
    #[serde(rename = "quantity")]
    ///The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts.
    pub quantity: f64,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for Holding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    #[serde(rename = "security_id")]
    ///A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive.
    pub security_id: String,
    #[serde(rename = "isin")]
    ///12-character ISIN, a globally unique securities identifier.
    pub isin: Option<String>,
    #[serde(rename = "cusip")]
    ///9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: Option<String>,
    #[serde(rename = "sedol")]
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: Option<String>,
    #[serde(rename = "institution_security_id")]
    ///An identifier given to the security by the institution
    pub institution_security_id: Option<String>,
    #[serde(rename = "institution_id")]
    ///If `institution_security_id` is present, this field indicates the Plaid `institution_id` of the institution to whom the identifier belongs.
    pub institution_id: Option<String>,
    #[serde(rename = "proxy_security_id")]
    ///In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security.
    pub proxy_security_id: Option<String>,
    #[serde(rename = "name")]
    ///A descriptive name for the security, suitable for display.
    pub name: Option<String>,
    #[serde(rename = "ticker_symbol")]
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: Option<String>,
    #[serde(rename = "is_cash_equivalent")]
    ///Indicates that a security is a highly liquid asset and can be treated like cash.
    pub is_cash_equivalent: Option<bool>,
    #[serde(rename = "type")]
    /**The security type of the holding. Valid security types are:

`cash`: Cash, currency, and money market funds

`cryptocurrency`: Digital or virtual currencies

`derivative`: Options, warrants, and other derivative instruments

`equity`: Domestic and foreign equities

`etf`: Multi-asset exchange-traded investment funds

`fixed income`: Bonds and certificates of deposit (CDs)

`loan`: Loans and loan receivables

`mutual fund`: Open- and closed-end vehicles pooling funds of multiple investors

`other`: Unknown or other investment types*/
    pub type_: Option<String>,
    #[serde(rename = "close_price")]
    /**Price of the security at the close of the previous trading session. Null for non-public securities.

If the security is a foreign currency this field will be updated daily and will be priced in USD.

If the security is a cryptocurrency, this field will be updated multiple times a day. As crypto prices can fluctuate quickly and data may become stale sooner than other asset classes, please refer to update_datetime with the time when the price was last updated.
*/
    pub close_price: Option<f64>,
    #[serde(rename = "close_price_as_of")]
    ///Date for which `close_price` is accurate. Always `null` if `close_price` is `null`.
    pub close_price_as_of: Option<String>,
    #[serde(rename = "update_datetime")]
    ///Date and time at which close_price is accurate, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ). Always null if close_price is null.
    pub update_datetime: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the price given. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentTransactionType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(rename = "cash")]
    Cash,
    #[serde(rename = "fee")]
    Fee,
    #[serde(rename = "transfer")]
    Transfer,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentTransactionSubtype {
    #[serde(rename = "account fee")]
    AccountFee,
    #[serde(rename = "adjustment")]
    Adjustment,
    #[serde(rename = "assignment")]
    Assignment,
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "buy to cover")]
    BuyToCover,
    #[serde(rename = "contribution")]
    Contribution,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "distribution")]
    Distribution,
    #[serde(rename = "dividend")]
    Dividend,
    #[serde(rename = "dividend reinvestment")]
    DividendReinvestment,
    #[serde(rename = "exercise")]
    Exercise,
    #[serde(rename = "expire")]
    Expire,
    #[serde(rename = "fund fee")]
    FundFee,
    #[serde(rename = "interest")]
    Interest,
    #[serde(rename = "interest receivable")]
    InterestReceivable,
    #[serde(rename = "interest reinvestment")]
    InterestReinvestment,
    #[serde(rename = "legal fee")]
    LegalFee,
    #[serde(rename = "loan payment")]
    LoanPayment,
    #[serde(rename = "long-term capital gain")]
    LongTermCapitalGain,
    #[serde(rename = "long-term capital gain reinvestment")]
    LongTermCapitalGainReinvestment,
    #[serde(rename = "management fee")]
    ManagementFee,
    #[serde(rename = "margin expense")]
    MarginExpense,
    #[serde(rename = "merger")]
    Merger,
    #[serde(rename = "miscellaneous fee")]
    MiscellaneousFee,
    #[serde(rename = "non-qualified dividend")]
    NonQualifiedDividend,
    #[serde(rename = "non-resident tax")]
    NonResidentTax,
    #[serde(rename = "pending credit")]
    PendingCredit,
    #[serde(rename = "pending debit")]
    PendingDebit,
    #[serde(rename = "qualified dividend")]
    QualifiedDividend,
    #[serde(rename = "rebalance")]
    Rebalance,
    #[serde(rename = "return of principal")]
    ReturnOfPrincipal,
    #[serde(rename = "request")]
    Request,
    #[serde(rename = "sell")]
    Sell,
    #[serde(rename = "sell short")]
    SellShort,
    #[serde(rename = "send")]
    Send,
    #[serde(rename = "short-term capital gain")]
    ShortTermCapitalGain,
    #[serde(rename = "short-term capital gain reinvestment")]
    ShortTermCapitalGainReinvestment,
    #[serde(rename = "spin off")]
    SpinOff,
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "stock distribution")]
    StockDistribution,
    #[serde(rename = "tax")]
    Tax,
    #[serde(rename = "tax withheld")]
    TaxWithheld,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "transfer fee")]
    TransferFee,
    #[serde(rename = "trust fee")]
    TrustFee,
    #[serde(rename = "unqualified gain")]
    UnqualifiedGain,
    #[serde(rename = "withdrawal")]
    Withdrawal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    #[serde(rename = "investment_transaction_id")]
    ///The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive.
    pub investment_transaction_id: String,
    #[serde(rename = "cancel_transaction_id")]
    ///A legacy field formerly used internally by Plaid to identify certain canceled transactions.
    pub cancel_transaction_id: Option<String>,
    #[serde(rename = "account_id")]
    ///The `account_id` of the account against which this transaction posted.
    pub account_id: String,
    #[serde(rename = "security_id")]
    ///The `security_id` to which this transaction is related.
    pub security_id: Option<String>,
    #[serde(rename = "date")]
    ///The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction.
    pub date: String,
    #[serde(rename = "name")]
    ///The institution’s description of the transaction.
    pub name: String,
    #[serde(rename = "quantity")]
    ///The number of units of the security involved in this transaction.
    pub quantity: f64,
    #[serde(rename = "amount")]
    ///The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities.
    pub amount: f64,
    #[serde(rename = "price")]
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    #[serde(rename = "fees")]
    ///The combined value of all fees applied to this transaction
    pub fees: Option<f64>,
    #[serde(rename = "type")]
    /**Value is one of the following:
`buy`: Buying an investment
`sell`: Selling an investment
`cancel`: A cancellation of a pending transaction
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer

For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).*/
    pub type_: String,
    #[serde(rename = "subtype")]
    ///For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
    pub subtype: String,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for InvestmentTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionType {
    #[serde(rename = "buy")]
    ///Buying an investment
    pub buy: String,
    #[serde(rename = "sell")]
    ///Selling an investment
    pub sell: String,
    #[serde(rename = "cancel")]
    ///A cancellation of a pending transaction
    pub cancel: String,
    #[serde(rename = "cash")]
    ///Activity that modifies a cash position
    pub cash: String,
    #[serde(rename = "fee")]
    ///Fees on the account, e.g. commission, bookkeeping, options-related.
    pub fee: String,
    #[serde(rename = "transfer")]
    ///Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer
    pub transfer: String,
}
impl std::fmt::Display for StandaloneInvestmentTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionBuyType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionCashType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionFeeType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionSellType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionTransferType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermissionRevokedWebhook {
    #[serde(rename = "webhook_type")]
    ///`ITEM`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`USER_PERMISSION_REVOKED`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for UserPermissionRevokedWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchGetRequest {
    #[serde(rename = "deposit_switch_id")]
    ///The ID of the deposit switch
    pub deposit_switch_id: String,
}
impl std::fmt::Display for DepositSwitchGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchGetResponse {
    #[serde(rename = "deposit_switch_id")]
    ///The ID of the deposit switch.
    pub deposit_switch_id: String,
    #[serde(rename = "target_account_id")]
    ///The ID of the bank account the direct deposit was switched to.
    pub target_account_id: Option<String>,
    #[serde(rename = "target_item_id")]
    ///The ID of the Item the direct deposit was switched to.
    pub target_item_id: Option<String>,
    #[serde(rename = "state")]
    /**
The state, or status, of the deposit switch.

- `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

- `processing` – The deposit switch request has been submitted and is being processed.

- `completed` – The user's employer has fulfilled the deposit switch request.

- `error` – There was an error processing the deposit switch request.*/
    pub state: String,
    #[serde(rename = "switch_method")]
    /**The method used to make the deposit switch.

- `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.

- `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.

- `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.'*/
    pub switch_method: Option<String>,
    #[serde(rename = "account_has_multiple_allocations")]
    ///When `true`, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always `null` if the deposit switch has not been completed.
    pub account_has_multiple_allocations: Option<bool>,
    #[serde(rename = "is_allocated_remainder")]
    ///When `true`, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When `false`, user’s direct deposit is allocated as a percent or amount. Always `null` if the deposit switch has not been completed.
    pub is_allocated_remainder: Option<bool>,
    #[serde(rename = "percent_allocated")]
    ///The percentage of direct deposit allocated to the target account. Always `null` if the target account is not allocated a percentage or if the deposit switch has not been completed or if `is_allocated_remainder` is true.
    pub percent_allocated: Option<f64>,
    #[serde(rename = "amount_allocated")]
    ///The dollar amount of direct deposit allocated to the target account. Always `null` if the target account is not allocated an amount or if the deposit switch has not been completed.
    pub amount_allocated: Option<f64>,
    #[serde(rename = "employer_name")]
    ///The name of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    pub employer_name: Option<String>,
    #[serde(rename = "employer_id")]
    ///The ID of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    pub employer_id: Option<String>,
    #[serde(rename = "institution_name")]
    ///The name of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    pub institution_name: Option<String>,
    #[serde(rename = "institution_id")]
    ///The ID of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    pub institution_id: Option<String>,
    #[serde(rename = "date_created")]
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was created.
*/
    pub date_created: String,
    #[serde(rename = "date_completed")]
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was completed. Always `null` if the deposit switch has not been completed.
*/
    pub date_completed: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchStateUpdateWebhook {
    #[serde(rename = "webhook_type")]
    ///`"DEPOSIT_SWITCH"`
    pub webhook_type: Option<String>,
    #[serde(rename = "webhook_code")]
    ///`"SWITCH_STATE_UPDATE"`
    pub webhook_code: Option<String>,
    #[serde(rename = "state")]
    /**
The state, or status, of the deposit switch.

`initialized`: The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

`processing`: The deposit switch request has been submitted and is being processed.

`completed`: The user's employer has fulfilled and completed the deposit switch request.

`error`: There was an error processing the deposit switch request.

For more information, see the [Deposit Switch API reference](/docs/deposit-switch/reference#deposit_switchget).*/
    pub state: Option<String>,
    #[serde(rename = "deposit_switch_id")]
    ///The ID of the deposit switch.
    pub deposit_switch_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchStateUpdateWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyGetRequest {
    #[serde(rename = "audit_copy_token")]
    ///The `audit_copy_token` granting access to the Audit Copy you would like to get.
    pub audit_copy_token: String,
}
impl std::fmt::Display for AssetReportAuditCopyGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferGetRequest {
    #[serde(rename = "transfer_id")]
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: String,
}
impl std::fmt::Display for TransferGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferGetRequest {
    #[serde(rename = "bank_transfer_id")]
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: String,
}
impl std::fmt::Display for BankTransferGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferGetResponse {
    #[serde(rename = "transfer")]
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferGetResponse {
    #[serde(rename = "bank_transfer")]
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    #[serde(rename = "id")]
    ///Plaid’s unique identifier for a transfer.
    pub id: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "account_id")]
    ///The account ID that should be credited/debited for this transfer.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "description")]
    ///The description of the transfer.
    pub description: String,
    #[serde(rename = "created")]
    ///The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: String,
    #[serde(rename = "status")]
    ///The status of the transfer.
    pub status: String,
    #[serde(rename = "sweep_status")]
    /**The status of the sweep for the transfer.
`unswept`: The transfer hasn't been swept yet.
`swept`: The transfer was swept to the sweep account.
`return_swept`: The transfer was returned, funds were pulled back or pushed back to the sweep account.
`null`: The transfer will never be swept (e.g. if the transfer is cancelled or returned before being swept)*/
    pub sweep_status: Option<String>,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`. The cutoff for same-day transfers is 7:45 AM Pacific Time and the cutoff for next-day transfers is 5:45 PM Pacific Time. It is recommended to submit a transfer at least 15 minutes before the cutoff time in order to ensure that it will be processed before the cutoff. Any transfer that is indicated as `same-day-ach` and that misses the same-day cutoff, but is submitted in time for the next-day cutoff, will be sent over next-day rails and will not incur same-day charges. Note that both legs of the transfer will be downgraded if applicable.
    pub network: String,
    #[serde(rename = "cancellable")]
    ///When `true`, you can still cancel this transfer.
    pub cancellable: bool,
    #[serde(rename = "failure_reason")]
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    pub failure_reason: Option<TransferFailure>,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    #[serde(rename = "guarantee_decision")]
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: Option<String>,
    #[serde(rename = "guarantee_decision_rationale")]
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransfer {
    #[serde(rename = "id")]
    ///Plaid’s unique identifier for a bank transfer.
    pub id: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "account_id")]
    ///The account ID that should be credited/debited for this bank transfer.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    #[serde(rename = "amount")]
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    #[serde(rename = "description")]
    ///The description of the transfer.
    pub description: String,
    #[serde(rename = "created")]
    ///The datetime when this bank transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: String,
    #[serde(rename = "status")]
    ///The status of the transfer.
    pub status: String,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: String,
    #[serde(rename = "cancellable")]
    ///When `true`, you can still cancel this bank transfer.
    pub cancellable: bool,
    #[serde(rename = "failure_reason")]
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: Option<BankTransferFailure>,
    #[serde(rename = "custom_tag")]
    ///A string containing the custom tag provided by the client in the create request. Will be null if not provided.
    pub custom_tag: Option<String>,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<BankTransferMetadata>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    #[serde(rename = "direction")]
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: Option<String>,
}
impl std::fmt::Display for BankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AchClass {
    #[serde(rename = "ccd")]
    Ccd,
    #[serde(rename = "ppd")]
    Ppd,
    #[serde(rename = "tel")]
    Tel,
    #[serde(rename = "web")]
    Web,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAmount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepAmount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetFailureReason {
    #[serde(rename = "error_type")]
    ///A broad categorization of the error.
    pub error_type: Option<String>,
    #[serde(rename = "error_code")]
    /**A code representing the reason for a failed transfer intent (i.e., an API error or the authorization being declined).

For a full listing of bank transfer errors, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/).*/
    pub error_code: Option<String>,
    #[serde(rename = "error_message")]
    ///A human-readable description of the code associated with a failed transfer intent.
    pub error_message: Option<String>,
}
impl std::fmt::Display for TransferIntentGetFailureReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentCreateMode {
    #[serde(rename = "PAYMENT")]
    Payment,
    #[serde(rename = "DISBURSEMENT")]
    Disbursement,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferAmount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationUserInRequest {
    #[serde(rename = "legal_name")]
    ///The user's legal name.
    pub legal_name: String,
    #[serde(rename = "phone_number")]
    ///The user's phone number. In order to qualify for a guaranteed transfer, at least one of `phone_number` or `email_address` must be provided.
    pub phone_number: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address. In order to qualify for a guaranteed transfer, at least one of `phone_number` or `email_address` must be provided.
    pub email_address: Option<String>,
    #[serde(rename = "address")]
    ///The address associated with the account holder. Providing this data will improve the likelihood that Plaid will be able to guarantee the transfer, if applicable.
    pub address: Option<TransferUserAddressInRequest>,
}
impl std::fmt::Display for TransferAuthorizationUserInRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserInRequest {
    #[serde(rename = "legal_name")]
    ///The user's legal name.
    pub legal_name: String,
    #[serde(rename = "phone_number")]
    ///The user's phone number.
    pub phone_number: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address.
    pub email_address: Option<String>,
    #[serde(rename = "address")]
    ///The address associated with the account holder. Providing this data will improve the likelihood that Plaid will be able to guarantee the transfer, if applicable.
    pub address: Option<TransferUserAddressInRequest>,
}
impl std::fmt::Display for TransferUserInRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserInResponse {
    #[serde(rename = "legal_name")]
    ///The user's legal name.
    pub legal_name: String,
    #[serde(rename = "phone_number")]
    ///The user's phone number.
    pub phone_number: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address.
    pub email_address: Option<String>,
    #[serde(rename = "address")]
    ///The address associated with the account holder.
    pub address: Option<TransferUserAddressInResponse>,
}
impl std::fmt::Display for TransferUserInResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserAddressInRequest {
    #[serde(rename = "street")]
    ///The street number and name (i.e., "100 Market St.").
    pub street: Option<String>,
    #[serde(rename = "city")]
    ///Ex. "San Francisco"
    pub city: Option<String>,
    #[serde(rename = "region")]
    ///The state or province (e.g., "CA").
    pub region: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code (e.g., "94103").
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///A two-letter country code (e.g., "US").
    pub country: Option<String>,
}
impl std::fmt::Display for TransferUserAddressInRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserAddressInResponse {
    #[serde(rename = "street")]
    ///The street number and name (i.e., "100 Market St.").
    pub street: Option<String>,
    #[serde(rename = "city")]
    ///Ex. "San Francisco"
    pub city: Option<String>,
    #[serde(rename = "region")]
    ///The state or province (e.g., "CA").
    pub region: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code (e.g., "94103").
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///A two-letter country code (e.g., "US").
    pub country: Option<String>,
}
impl std::fmt::Display for TransferUserAddressInResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferUser {
    #[serde(rename = "legal_name")]
    ///The account holder’s full legal name. If the transfer `ach_class` is `ccd`, this should be the business name of the account holder.
    pub legal_name: String,
    #[serde(rename = "email_address")]
    ///The account holder’s email.
    pub email_address: Option<String>,
    #[serde(rename = "routing_number")]
    ///The account holder's routing number. This field is only used in response data. Do not provide this field when making requests.
    pub routing_number: Option<String>,
}
impl std::fmt::Display for BankTransferUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationDecisionRationaleCode {
    #[serde(rename = "NSF")]
    Nsf,
    #[serde(rename = "RISK")]
    Risk,
    #[serde(rename = "TRANSFER_LIMIT_REACHED")]
    TransferLimitReached,
    #[serde(rename = "MANUALLY_VERIFIED_ITEM")]
    ManuallyVerifiedItem,
    #[serde(rename = "LOGIN_REQUIRED")]
    LoginRequired,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationDecisionRationale {
    #[serde(rename = "code")]
    /**A code representing the rationale for approving or declining the proposed transfer. Possible values are:

`MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available. Plaid will offer `approved` as a transaction decision.

`LOGIN_REQUIRED` – Unable to collect the account information due to Item staleness. Can be rectified using Link in update mode. Plaid will offer `approved` as a transaction decision.

`ERROR` – Unable to collect the account information due to an error. Plaid will offer `approved` as a transaction decision.

`NSF` – Transaction likely to result in a return due to insufficient funds. Plaid will offer `declined` as a transaction decision.

`RISK` - Transaction is high-risk. Plaid will offer `declined` as a transaction decision.

`TRANSFER_LIMIT_REACHED` - One or several transfer limits are reached, e.g. monthly transfer limit. Plaid will offer `declined` as a transaction decision.*/
    pub code: String,
    #[serde(rename = "description")]
    ///A human-readable description of the code associated with a transfer approval or transfer decline.
    pub description: String,
}
impl std::fmt::Display for TransferAuthorizationDecisionRationale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecision {
    #[serde(rename = "GUARANTEED")]
    Guaranteed,
    #[serde(rename = "NOT_GUARANTEED")]
    NotGuaranteed,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecisionRationaleCode {
    #[serde(rename = "RETURN_BANK")]
    ReturnBank,
    #[serde(rename = "RETURN_CUSTOMER")]
    ReturnCustomer,
    #[serde(rename = "GUARANTEE_LIMIT_REACHED")]
    GuaranteeLimitReached,
    #[serde(rename = "RISK_ESTIMATE_UNAVAILABLE")]
    RiskEstimateUnavailable,
    #[serde(rename = "REQUIRED_PARAM_MISSING")]
    RequiredParamMissing,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationGuaranteeDecisionRationale {
    #[serde(rename = "code")]
    /**A code representing the reason Plaid declined to guarantee this transfer:

`RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.

`RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.

`GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guaranteed ACH has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.

`RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.

`REQUIRED_PARAM_MISSING`: Required fields are missing.*/
    pub code: String,
    #[serde(rename = "description")]
    ///A human-readable description of why the transfer cannot be guaranteed.
    pub description: String,
}
impl std::fmt::Display for TransferAuthorizationGuaranteeDecisionRationale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer.
    pub network: String,
    #[serde(rename = "origination_account_id")]
    ///Plaid's unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: String,
}
impl std::fmt::Display for TransferAuthorizationProposedTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationDevice {
    #[serde(rename = "ip_address")]
    ///The IP address of the device being used to initiate the authorization. Required for guaranteed ACH customers.
    pub ip_address: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user agent of the device being used to initiate the authorization. Required for guaranteed ACH customers.
    pub user_agent: Option<String>,
}
impl std::fmt::Display for TransferAuthorizationDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMetadata {}
impl std::fmt::Display for TransferMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMetadata {}
impl std::fmt::Display for BankTransferMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "returned")]
    Returned,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferSweepStatus {
    #[serde(rename = "unswept")]
    Unswept,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "reverse_swept")]
    ReverseSwept,
    #[serde(rename = "return_swept")]
    ReturnSwept,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "reversed")]
    Reversed,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferNetwork {
    #[serde(rename = "ach")]
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    #[serde(rename = "wire")]
    Wire,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferFailure {
    #[serde(rename = "ach_return_code")]
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `returned`. For a full listing of ACH return codes, see [Transfer errors](https://plaid.com/docs/errors/transfer/#ach-return-codes).
    pub ach_return_code: Option<String>,
    #[serde(rename = "description")]
    ///A human-readable description of the reason for the failure or reversal.
    pub description: Option<String>,
}
impl std::fmt::Display for TransferFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferFailure {
    #[serde(rename = "ach_return_code")]
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/#ach-return-codes).
    pub ach_return_code: Option<String>,
    #[serde(rename = "description")]
    ///A human-readable description of the reason for the failure or reversal.
    pub description: Option<String>,
}
impl std::fmt::Display for BankTransferFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequest {
    #[serde(rename = "access_token")]
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: Option<String>,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: Option<String>,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`. The cutoff for same-day transfers is 7:45 AM Pacific Time and the cutoff for next-day transfers is 5:45 PM Pacific Time. It is recommended to submit a transfer at least 15 minutes before the cutoff time in order to ensure that it will be processed before the cutoff. Any transfer that is indicated as `same-day-ach` and that misses the same-day cutoff, but is submitted in time for the next-day cutoff, will be sent over next-day rails and will not incur same-day charges. Note that both legs of the transfer will be downgraded if applicable.
    pub network: String,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferAuthorizationUserInRequest,
    #[serde(rename = "device")]
    ///Information about the device being used to initiate the authorization.
    pub device: Option<TransferAuthorizationDevice>,
    #[serde(rename = "origination_account_id")]
    ///Plaid's unique identifier for the origination account for this authorization. If not specified, the default account will be used.
    pub origination_account_id: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: Option<String>,
    #[serde(rename = "user_present")]
    ///Required for guaranteed ACH customers. If the end user is initiating the specific transfer themselves via an interactive UI, this should be `true`; for automatic recurring payments where the end user is not actually initiating each individual transfer, it should be `false`.
    pub user_present: Option<bool>,
    #[serde(rename = "payment_profile_id")]
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: Option<String>,
}
impl std::fmt::Display for TransferAuthorizationCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateRequest {
    #[serde(rename = "idempotency_key")]
    /**Deprecated. `authorization_id` is now used as idempotency instead.

A random key provided by the client, per unique transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single transfer is created.*/
    pub idempotency_key: Option<String>,
    #[serde(rename = "access_token")]
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: Option<String>,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: Option<String>,
    #[serde(rename = "authorization_id")]
    ///Plaid’s unique identifier for a transfer authorization. This parameter also serves the purpose of acting as an idempotency identifier.
    pub authorization_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`. The cutoff for same-day transfers is 7:45 AM Pacific Time and the cutoff for next-day transfers is 5:45 PM Pacific Time. It is recommended to submit a transfer at least 15 minutes before the cutoff time in order to ensure that it will be processed before the cutoff. Any transfer that is indicated as `same-day-ach` and that misses the same-day cutoff, but is submitted in time for the next-day cutoff, will be sent over next-day rails and will not incur same-day charges. Note that both legs of the transfer will be downgraded if applicable.
    pub network: String,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "description")]
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    pub origination_account_id: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: Option<String>,
    #[serde(rename = "payment_profile_id")]
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: Option<String>,
}
impl std::fmt::Display for TransferCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCreateRequest {
    #[serde(rename = "idempotency_key")]
    /**A random key provided by the client, per unique bank transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created.*/
    pub idempotency_key: String,
    #[serde(rename = "access_token")]
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: String,
    #[serde(rename = "network")]
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: String,
    #[serde(rename = "amount")]
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount – should be set to "USD".
    pub iso_currency_code: String,
    #[serde(rename = "description")]
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: Option<String>,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    #[serde(rename = "custom_tag")]
    ///An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters.
    pub custom_tag: Option<String>,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<BankTransferMetadata>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    pub origination_account_id: Option<String>,
}
impl std::fmt::Display for BankTransferCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateResponse {
    #[serde(rename = "authorization")]
    ///Contains the authorization decision for a proposed transfer
    pub authorization: TransferAuthorization,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferAuthorizationCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationDecision {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "declined")]
    Declined,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorization {
    #[serde(rename = "id")]
    ///Plaid’s unique identifier for a transfer authorization.
    pub id: String,
    #[serde(rename = "created")]
    ///The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`.
    pub created: String,
    #[serde(rename = "decision")]
    /**
A decision regarding the proposed transfer.

`approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub decision: String,
    #[serde(rename = "decision_rationale")]
    ///The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    #[serde(rename = "guarantee_decision")]
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: Option<String>,
    #[serde(rename = "guarantee_decision_rationale")]
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    #[serde(rename = "proposed_transfer")]
    ///Details regarding the proposed transfer.
    pub proposed_transfer: TransferAuthorizationProposedTransfer,
}
impl std::fmt::Display for TransferAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateResponse {
    #[serde(rename = "transfer")]
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCreateResponse {
    #[serde(rename = "bank_transfer")]
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferListRequest {
    #[serde(rename = "start_date")]
    ///The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    #[serde(rename = "count")]
    ///The maximum number of transfers to return.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of transfers to skip before returning results.
    pub offset: Option<i64>,
    #[serde(rename = "origination_account_id")]
    ///Filter transfers to only those originated through the specified origination account.
    pub origination_account_id: Option<String>,
}
impl std::fmt::Display for TransferListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferListRequest {
    #[serde(rename = "start_date")]
    ///The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    #[serde(rename = "count")]
    ///The maximum number of bank transfers to return.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of bank transfers to skip before returning results.
    pub offset: Option<i64>,
    #[serde(rename = "origination_account_id")]
    ///Filter bank transfers to only those originated through the specified origination account.
    pub origination_account_id: Option<String>,
    #[serde(rename = "direction")]
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: Option<String>,
}
impl std::fmt::Display for BankTransferListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferListResponse {
    #[serde(rename = "transfers")]
    pub transfers: Vec<Transfer>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferListResponse {
    #[serde(rename = "bank_transfers")]
    pub bank_transfers: Vec<BankTransfer>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferDirection {
    #[serde(rename = "outbound")]
    Outbound,
    #[serde(rename = "inbound")]
    Inbound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCancelRequest {
    #[serde(rename = "transfer_id")]
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: String,
}
impl std::fmt::Display for TransferCancelRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCancelRequest {
    #[serde(rename = "bank_transfer_id")]
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: String,
}
impl std::fmt::Display for BankTransferCancelRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCancelResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferCancelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCancelResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferCancelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferEventListTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventListRequest {
    #[serde(rename = "start_date")]
    ///The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    #[serde(rename = "transfer_id")]
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: Option<String>,
    #[serde(rename = "account_id")]
    ///The account ID to get events for all transactions to/from an account.
    pub account_id: Option<String>,
    #[serde(rename = "transfer_type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
    pub transfer_type: Option<String>,
    #[serde(rename = "event_types")]
    ///Filter events by event type.
    pub event_types: Option<Vec<String>>,
    #[serde(rename = "sweep_id")]
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: Option<String>,
    #[serde(rename = "count")]
    ///The maximum number of transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The offset into the list of transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned.
    pub offset: Option<i64>,
    #[serde(rename = "origination_account_id")]
    ///The origination account ID to get events for transfers from a specific origination account.
    pub origination_account_id: Option<String>,
}
impl std::fmt::Display for TransferEventListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferEventListBankTransferType {
    #[serde(rename = "debit")]
    Debit,
    #[serde(rename = "credit")]
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferEventListDirection {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventListRequest {
    #[serde(rename = "start_date")]
    ///The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    #[serde(rename = "bank_transfer_id")]
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: Option<String>,
    #[serde(rename = "account_id")]
    ///The account ID to get events for all transactions to/from an account.
    pub account_id: Option<String>,
    #[serde(rename = "bank_transfer_type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
    pub bank_transfer_type: Option<String>,
    #[serde(rename = "event_types")]
    ///Filter events by event type.
    pub event_types: Option<Vec<String>>,
    #[serde(rename = "count")]
    ///The maximum number of bank transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The offset into the list of bank transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned.
    pub offset: Option<i64>,
    #[serde(rename = "origination_account_id")]
    ///The origination account ID to get events for transfers from a specific origination account.
    pub origination_account_id: Option<String>,
    #[serde(rename = "direction")]
    /**Indicates the direction of the transfer: `outbound`: for API-initiated transfers
`inbound`: for payments received by the FBO account.*/
    pub direction: Option<String>,
}
impl std::fmt::Display for BankTransferEventListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "returned")]
    Returned,
    #[serde(rename = "swept")]
    Swept,
    #[serde(rename = "reverse_swept")]
    ReverseSwept,
    #[serde(rename = "return_swept")]
    ReturnSwept,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferEventType {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "posted")]
    Posted,
    #[serde(rename = "reversed")]
    Reversed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEvent {
    #[serde(rename = "event_id")]
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    #[serde(rename = "timestamp")]
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: String,
    #[serde(rename = "event_type")]
    /**The type of event that this transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`returned`: A posted transfer was returned.

`swept`: The transfer was swept to / from the sweep account.

`return_swept`: Due to the transfer being returned, funds were pulled from or pushed back to the sweep account.*/
    pub event_type: String,
    #[serde(rename = "account_id")]
    ///The account ID associated with the transfer.
    pub account_id: String,
    #[serde(rename = "transfer_id")]
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: String,
    #[serde(rename = "origination_account_id")]
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    #[serde(rename = "transfer_type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub transfer_type: String,
    #[serde(rename = "transfer_amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub transfer_amount: String,
    #[serde(rename = "failure_reason")]
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    pub failure_reason: Option<TransferFailure>,
    #[serde(rename = "sweep_id")]
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: Option<String>,
    #[serde(rename = "sweep_amount")]
    ///A signed amount of how much was `swept` or `return_swept` (decimal string with two digits of precision e.g. "-5.50").
    pub sweep_amount: Option<String>,
}
impl std::fmt::Display for TransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEvent {
    #[serde(rename = "event_id")]
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    #[serde(rename = "timestamp")]
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: String,
    #[serde(rename = "event_type")]
    /**The type of event that this bank transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`reversed`: A posted transfer was reversed.*/
    pub event_type: String,
    #[serde(rename = "account_id")]
    ///The account ID associated with the bank transfer.
    pub account_id: String,
    #[serde(rename = "bank_transfer_id")]
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: String,
    #[serde(rename = "origination_account_id")]
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    #[serde(rename = "bank_transfer_type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub bank_transfer_type: String,
    #[serde(rename = "bank_transfer_amount")]
    ///The bank transfer amount.
    pub bank_transfer_amount: String,
    #[serde(rename = "bank_transfer_iso_currency_code")]
    ///The currency of the bank transfer amount.
    pub bank_transfer_iso_currency_code: String,
    #[serde(rename = "failure_reason")]
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: Option<BankTransferFailure>,
    #[serde(rename = "direction")]
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: Option<String>,
}
impl std::fmt::Display for BankTransferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventListResponse {
    #[serde(rename = "transfer_events")]
    pub transfer_events: Vec<TransferEvent>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferEventListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventListResponse {
    #[serde(rename = "bank_transfer_events")]
    pub bank_transfer_events: Vec<BankTransferEvent>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferEventListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventSyncRequest {
    #[serde(rename = "after_id")]
    ///The latest (largest) `event_id` fetched via the sync endpoint, or 0 initially.
    pub after_id: i64,
    #[serde(rename = "count")]
    ///The maximum number of bank transfer events to return.
    pub count: Option<i64>,
}
impl std::fmt::Display for BankTransferEventSyncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventSyncRequest {
    #[serde(rename = "after_id")]
    ///The latest (largest) `event_id` fetched via the sync endpoint, or 0 initially.
    pub after_id: i64,
    #[serde(rename = "count")]
    ///The maximum number of transfer events to return.
    pub count: Option<i64>,
}
impl std::fmt::Display for TransferEventSyncRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventSyncResponse {
    #[serde(rename = "bank_transfer_events")]
    pub bank_transfer_events: Vec<BankTransferEvent>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferEventSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventSyncResponse {
    #[serde(rename = "transfer_events")]
    pub transfer_events: Vec<TransferEvent>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferEventSyncResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepGetRequest {
    #[serde(rename = "sweep_id")]
    ///Identifier of the sweep.
    pub sweep_id: String,
}
impl std::fmt::Display for BankTransferSweepGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepGetRequest {
    #[serde(rename = "sweep_id")]
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: String,
}
impl std::fmt::Display for TransferSweepGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepGetResponse {
    #[serde(rename = "sweep")]
    ///BankTransferSweep describes a sweep transfer.
    pub sweep: BankTransferSweep,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferSweepGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepGetResponse {
    #[serde(rename = "sweep")]
    /**Describes a sweep of funds to / from the sweep account.

A sweep is associated with many sweep events (events of type `swept` or `return_swept`) which can be retrieved by invoking the `/transfer/event/list` endpoint with the corresponding `sweep_id`.

`swept` events occur when the transfer amount is credited or debited from your sweep account, depending on the `type` of the transfer. `return_swept` events occur when a transfer is returned and Plaid undoes the credit or debit.

The total sum of the `swept` and `return_swept` events is equal to the `amount` of the sweep Plaid creates and matches the amount of the entry on your sweep account ledger.*/
    pub sweep: TransferSweep,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferSweepGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepListRequest {
    #[serde(rename = "origination_account_id")]
    ///If multiple origination accounts are available, `origination_account_id` must be used to specify the account that the sweeps belong to.
    pub origination_account_id: Option<String>,
    #[serde(rename = "start_time")]
    ///The start datetime of sweeps to return (RFC 3339 format).
    pub start_time: Option<String>,
    #[serde(rename = "end_time")]
    ///The end datetime of sweeps to return (RFC 3339 format).
    pub end_time: Option<String>,
    #[serde(rename = "count")]
    ///The maximum number of sweeps to return.
    pub count: Option<i64>,
}
impl std::fmt::Display for BankTransferSweepListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepListRequest {
    #[serde(rename = "start_date")]
    ///The start datetime of sweeps to return (RFC 3339 format).
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The end datetime of sweeps to return (RFC 3339 format).
    pub end_date: Option<String>,
    #[serde(rename = "count")]
    ///The maximum number of sweeps to return.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of sweeps to skip before returning results.
    pub offset: Option<i64>,
}
impl std::fmt::Display for TransferSweepListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepListResponse {
    #[serde(rename = "sweeps")]
    pub sweeps: Vec<TransferSweep>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferSweepListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepListResponse {
    #[serde(rename = "sweeps")]
    pub sweeps: Vec<BankTransferSweep>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferSweepListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweep {
    #[serde(rename = "id")]
    ///Identifier of the sweep.
    pub id: String,
    #[serde(rename = "created_at")]
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created_at: String,
    #[serde(rename = "amount")]
    ///The amount of the sweep.
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
impl std::fmt::Display for BankTransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweep {
    #[serde(rename = "id")]
    ///Identifier of the sweep.
    pub id: String,
    #[serde(rename = "created")]
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created: String,
    #[serde(rename = "amount")]
    /**Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. "-10.00")

If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced.*/
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
impl std::fmt::Display for TransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulatedTransferSweep {}
impl std::fmt::Display for SimulatedTransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalanceGetRequest {
    #[serde(rename = "origination_account_id")]
    ///If multiple origination accounts are available, `origination_account_id` must be used to specify the account for which balance will be returned.
    pub origination_account_id: Option<String>,
}
impl std::fmt::Display for BankTransferBalanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalanceGetResponse {
    #[serde(rename = "balance")]
    ///Information about the balance of a bank transfer
    pub balance: BankTransferBalance,
    #[serde(rename = "origination_account_id")]
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferBalanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalance {
    #[serde(rename = "available")]
    ///The total available balance - the sum of all successful debit transfer amounts minus all credit transfer amounts.
    pub available: String,
    #[serde(rename = "transactable")]
    ///The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.
    pub transactable: String,
}
impl std::fmt::Display for BankTransferBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountRequest {
    #[serde(rename = "account_number")]
    ///The user's account number.
    pub account_number: String,
    #[serde(rename = "routing_number")]
    ///The user's routing number.
    pub routing_number: String,
    #[serde(rename = "wire_routing_number")]
    ///The user's wire transfer routing number. This is the ABA number; for some institutions, this may differ from the ACH number used in `routing_number`.
    pub wire_routing_number: Option<String>,
    #[serde(rename = "account_type")]
    ///The type of the bank account (`checking` or `savings`).
    pub account_type: String,
}
impl std::fmt::Display for BankTransferMigrateAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountResponse {
    #[serde(rename = "access_token")]
    ///The Plaid `access_token` for the newly created Item.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BankTransferMigrateAccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMigrateAccountRequest {
    #[serde(rename = "account_number")]
    ///The user's account number.
    pub account_number: String,
    #[serde(rename = "routing_number")]
    ///The user's routing number.
    pub routing_number: String,
    #[serde(rename = "wire_routing_number")]
    ///The user's wire transfer routing number. This is the ABA number; for some institutions, this may differ from the ACH number used in `routing_number`.
    pub wire_routing_number: Option<String>,
    #[serde(rename = "account_type")]
    ///The type of the bank account (`checking` or `savings`).
    pub account_type: String,
}
impl std::fmt::Display for TransferMigrateAccountRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMigrateAccountResponse {
    #[serde(rename = "access_token")]
    ///The Plaid `access_token` for the newly created Item.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferMigrateAccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentListRequest {
    #[serde(rename = "start_date")]
    ///The start datetime of repayments to return (RFC 3339 format).
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The end datetime of repayments to return (RFC 3339 format).
    pub end_date: Option<String>,
    #[serde(rename = "count")]
    ///The maximum number of repayments to return.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of repayments to skip before returning results.
    pub offset: Option<i64>,
}
impl std::fmt::Display for TransferRepaymentListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentListResponse {
    #[serde(rename = "repayments")]
    pub repayments: Vec<TransferRepayment>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRepaymentListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepayment {
    #[serde(rename = "repayment_id")]
    ///Identifier of the repayment.
    pub repayment_id: String,
    #[serde(rename = "created")]
    ///The datetime when the repayment occurred, in RFC 3339 format.
    pub created: String,
    #[serde(rename = "amount")]
    ///Decimal amount of the repayment as it appears on your account ledger.
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
}
impl std::fmt::Display for TransferRepayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListRequest {
    #[serde(rename = "repayment_id")]
    ///Identifier of the repayment to query.
    pub repayment_id: String,
    #[serde(rename = "count")]
    ///The maximum number of repayments to return.
    pub count: Option<i64>,
    #[serde(rename = "offset")]
    ///The number of repayments to skip before returning results.
    pub offset: Option<i64>,
}
impl std::fmt::Display for TransferRepaymentReturnListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListResponse {
    #[serde(rename = "repayment_returns")]
    pub repayment_returns: Vec<TransferRepaymentReturn>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferRepaymentReturnListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturn {
    #[serde(rename = "transfer_id")]
    ///The unique identifier of the guaranteed transfer that was returned.
    pub transfer_id: String,
    #[serde(rename = "event_id")]
    ///The unique identifier of the corresponding `returned` transfer event.
    pub event_id: i64,
    #[serde(rename = "amount")]
    ///The value of the returned transfer.
    pub amount: String,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
}
impl std::fmt::Display for TransferRepaymentReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: Option<String>,
    #[serde(rename = "mode")]
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: String,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "description")]
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: Option<String>,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: Option<String>,
    #[serde(rename = "require_guarantee")]
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guaranteed ACH customers only).
    pub require_guarantee: Option<bool>,
}
impl std::fmt::Display for TransferIntentCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "SUCCEEDED")]
    Succeeded,
    #[serde(rename = "FAILED")]
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    #[serde(rename = "id")]
    ///Plaid's unique identifier for the transfer intent object.
    pub id: String,
    #[serde(rename = "created")]
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: String,
    #[serde(rename = "status")]
    /**The status of the transfer intent.

- `PENDING` – The transfer intent is pending.
- `SUCCEEDED` – The transfer intent was successfully created.
- `FAILED` – The transfer intent was unable to be created.*/
    pub status: String,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    pub account_id: Option<String>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: String,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "mode")]
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    #[serde(rename = "description")]
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    #[serde(rename = "require_guarantee")]
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guaranteed ACH customers only).
    pub require_guarantee: Option<bool>,
}
impl std::fmt::Display for TransferIntentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateResponse {
    #[serde(rename = "transfer_intent")]
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentCreate,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferIntentCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetRequest {
    #[serde(rename = "transfer_intent_id")]
    ///Plaid's unique identifier for a transfer intent object.
    pub transfer_intent_id: String,
}
impl std::fmt::Display for TransferIntentGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentAuthorizationDecision {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "DECLINED")]
    Declined,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGet {
    #[serde(rename = "id")]
    ///Plaid's unique identifier for a transfer intent object.
    pub id: String,
    #[serde(rename = "created")]
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: String,
    #[serde(rename = "status")]
    /**The status of the transfer intent.

- `PENDING` – The transfer intent is pending.
- `SUCCEEDED` – The transfer intent was successfully created.
- `FAILED` – The transfer intent was unable to be created.*/
    pub status: String,
    #[serde(rename = "transfer_id")]
    ///Plaid's unique identifier for the transfer created through the UI. Returned only if the transfer was successfully created. Null value otherwise.
    pub transfer_id: Option<String>,
    #[serde(rename = "failure_reason")]
    ///The reason for a failed transfer intent. Returned only if the transfer intent status is `failed`. Null otherwise.
    pub failure_reason: Option<TransferIntentGetFailureReason>,
    #[serde(rename = "authorization_decision")]
    /**
A decision regarding the proposed transfer.

`APPROVED` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`DECLINED` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub authorization_decision: Option<String>,
    #[serde(rename = "authorization_decision_rationale")]
    ///The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    pub authorization_decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    #[serde(rename = "account_id")]
    ///The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    pub account_id: Option<String>,
    #[serde(rename = "origination_account_id")]
    ///Plaid’s unique identifier for the origination account used for the transfer.
    pub origination_account_id: String,
    #[serde(rename = "amount")]
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: String,
    #[serde(rename = "mode")]
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: String,
    #[serde(rename = "ach_class")]
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: String,
    #[serde(rename = "user")]
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    #[serde(rename = "description")]
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    #[serde(rename = "metadata")]
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    #[serde(rename = "iso_currency_code")]
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    #[serde(rename = "require_guarantee")]
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guaranteed ACH customers only).
    pub require_guarantee: Option<bool>,
    #[serde(rename = "guarantee_decision")]
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: Option<String>,
    #[serde(rename = "guarantee_decision_rationale")]
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
}
impl std::fmt::Display for TransferIntentGet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetResponse {
    #[serde(rename = "transfer_intent")]
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentGet,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for TransferIntentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateRequest {
    #[serde(rename = "bank_transfer_id")]
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: String,
    #[serde(rename = "event_type")]
    /**The asynchronous event to be simulated. May be: `posted`, `failed`, or `reversed`.

An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:

`pending` --> `failed`

`pending` --> `posted`

`posted` --> `reversed`
*/
    pub event_type: String,
    #[serde(rename = "failure_reason")]
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: Option<BankTransferFailure>,
}
impl std::fmt::Display for SandboxBankTransferSimulateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSimulateRequest {
    #[serde(rename = "transfer_id")]
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: String,
    #[serde(rename = "event_type")]
    /**The asynchronous event to be simulated. May be: `posted`, `failed`, or `returned`.

An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:

`pending` --> `failed`

`pending` --> `posted`

`posted` --> `returned`
*/
    pub event_type: String,
    #[serde(rename = "failure_reason")]
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    pub failure_reason: Option<TransferFailure>,
}
impl std::fmt::Display for SandboxTransferSimulateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateRequest {}
impl std::fmt::Display for SandboxTransferSweepSimulateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxBankTransferSimulateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSimulateResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxTransferSimulateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateResponse {
    #[serde(rename = "sweep")]
    /**A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint.
Can be null if there are no transfers to include in a sweep.*/
    pub sweep: Option<SimulatedTransferSweep>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxTransferSweepSimulateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateRequest {}
impl std::fmt::Display for SandboxTransferRepaymentSimulateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxTransferRepaymentSimulateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFiltersResponse {
    #[serde(rename = "depository")]
    ///A filter to apply to `depository`-type accounts
    pub depository: Option<DepositoryFilter>,
    #[serde(rename = "credit")]
    ///A filter to apply to `credit`-type accounts
    pub credit: Option<CreditFilter>,
    #[serde(rename = "loan")]
    ///A filter to apply to `loan`-type accounts
    pub loan: Option<LoanFilter>,
    #[serde(rename = "investment")]
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: Option<InvestmentFilter>,
}
impl std::fmt::Display for AccountFiltersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchAccountFilter {
    #[serde(rename = "loan")]
    pub loan: Option<Vec<String>>,
    #[serde(rename = "depository")]
    pub depository: Option<Vec<String>>,
    #[serde(rename = "credit")]
    pub credit: Option<Vec<String>>,
    #[serde(rename = "investment")]
    pub investment: Option<Vec<String>>,
}
impl std::fmt::Display for InstitutionsSearchAccountFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountIdentity(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAssets(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: DepositoryAccountSubtypes,
}
impl std::fmt::Display for DepositoryFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: CreditAccountSubtypes,
}
impl std::fmt::Display for CreditFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: LoanAccountSubtypes,
}
impl std::fmt::Display for LoanFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentFilter {
    #[serde(rename = "account_subtypes")]
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: InvestmentAccountSubtypes,
}
impl std::fmt::Display for InvestmentFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryAccountSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAccountSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAccountSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentAccountSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub enum DepositoryAccountSubtype {
    #[serde(rename = "checking")]
    Checking,
    #[serde(rename = "savings")]
    Savings,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "cd")]
    Cd,
    #[serde(rename = "money market")]
    MoneyMarket,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "prepaid")]
    Prepaid,
    #[serde(rename = "cash management")]
    CashManagement,
    #[serde(rename = "ebt")]
    Ebt,
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditAccountSubtype {
    #[serde(rename = "credit card")]
    CreditCard,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LoanAccountSubtype {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "construction")]
    Construction,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "home equity")]
    HomeEquity,
    #[serde(rename = "loan")]
    Loan,
    #[serde(rename = "mortgage")]
    Mortgage,
    #[serde(rename = "line of credit")]
    LineOfCredit,
    #[serde(rename = "student")]
    Student,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentAccountSubtype {
    #[serde(rename = "529")]
    InvestmentAccountSubtype529,
    #[serde(rename = "401a")]
    InvestmentAccountSubtype401A,
    #[serde(rename = "401k")]
    InvestmentAccountSubtype401K,
    #[serde(rename = "403B")]
    InvestmentAccountSubtype403B,
    #[serde(rename = "457b")]
    InvestmentAccountSubtype457B,
    #[serde(rename = "brokerage")]
    Brokerage,
    #[serde(rename = "cash isa")]
    CashIsa,
    #[serde(rename = "crypto exchange")]
    CryptoExchange,
    #[serde(rename = "education savings account")]
    EducationSavingsAccount,
    #[serde(rename = "fixed annuity")]
    FixedAnnuity,
    #[serde(rename = "gic")]
    Gic,
    #[serde(rename = "health reimbursement arrangement")]
    HealthReimbursementArrangement,
    #[serde(rename = "hsa")]
    Hsa,
    #[serde(rename = "ira")]
    Ira,
    #[serde(rename = "isa")]
    Isa,
    #[serde(rename = "keogh")]
    Keogh,
    #[serde(rename = "lif")]
    Lif,
    #[serde(rename = "life insurance")]
    LifeInsurance,
    #[serde(rename = "lira")]
    Lira,
    #[serde(rename = "lrif")]
    Lrif,
    #[serde(rename = "lrsp")]
    Lrsp,
    #[serde(rename = "mutual fund")]
    MutualFund,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "other annuity")]
    OtherAnnuity,
    #[serde(rename = "other insurance")]
    OtherInsurance,
    #[serde(rename = "pension")]
    Pension,
    #[serde(rename = "prif")]
    Prif,
    #[serde(rename = "profit sharing plan")]
    ProfitSharingPlan,
    #[serde(rename = "qshr")]
    Qshr,
    #[serde(rename = "rdsp")]
    Rdsp,
    #[serde(rename = "resp")]
    Resp,
    #[serde(rename = "retirement")]
    Retirement,
    #[serde(rename = "rlif")]
    Rlif,
    #[serde(rename = "roth")]
    Roth,
    #[serde(rename = "roth 401k")]
    Roth401K,
    #[serde(rename = "rrif")]
    Rrif,
    #[serde(rename = "rrsp")]
    Rrsp,
    #[serde(rename = "sarsep")]
    Sarsep,
    #[serde(rename = "sep ira")]
    SepIra,
    #[serde(rename = "simple ira")]
    SimpleIra,
    #[serde(rename = "sipp")]
    Sipp,
    #[serde(rename = "stock plan")]
    StockPlan,
    #[serde(rename = "tfsa")]
    Tfsa,
    #[serde(rename = "trust")]
    Trust,
    #[serde(rename = "ugma")]
    Ugma,
    #[serde(rename = "utma")]
    Utma,
    #[serde(rename = "variable annuity")]
    VariableAnnuity,
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersSearchRequest {
    #[serde(rename = "query")]
    ///The employer name to be searched for.
    pub query: String,
    #[serde(rename = "products")]
    ///The Plaid products the returned employers should support. Currently, this field must be set to `"deposit_switch"`.
    pub products: Vec<String>,
}
impl std::fmt::Display for EmployersSearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersSearchResponse {
    #[serde(rename = "employers")]
    ///A list of employers matching the search criteria.
    pub employers: Vec<Employer>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EmployersSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Employer {
    #[serde(rename = "employer_id")]
    ///Plaid's unique identifier for the employer.
    pub employer_id: String,
    #[serde(rename = "name")]
    ///The name of the employer
    pub name: String,
    #[serde(rename = "address")]
    ///Data about the components comprising an address.
    pub address: Option<AddressDataNullable>,
    #[serde(rename = "confidence_score")]
    ///A number from 0 to 1 indicating Plaid's level of confidence in the pairing between the employer and the institution (not yet implemented).
    pub confidence_score: f64,
}
impl std::fmt::Display for Employer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequest {
    #[serde(rename = "webhook")]
    ///The URL endpoint to which Plaid should send webhooks related to the progress of the income verification process.
    pub webhook: String,
    #[serde(rename = "precheck_id")]
    ///The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow.
    pub precheck_id: Option<String>,
    #[serde(rename = "options")]
    ///Optional arguments for `/income/verification/create`
    pub options: Option<IncomeVerificationCreateRequestOptions>,
}
impl std::fmt::Display for IncomeVerificationCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequestOptions {
    #[serde(rename = "access_tokens")]
    ///An array of access tokens corresponding to the Items that will be cross-referenced with the product data. Plaid will attempt to correlate transaction history from these Items with data from the user's paystub, such as date and amount. The `verification` status of the paystub as returned by `/income/verification/paystubs/get` will indicate if the verification status was successful, or, if not, why it failed. If the `transactions` product was not initialized for the Items during Link, it will be initialized after this Link session.
    pub access_tokens: Option<Vec<String>>,
}
impl std::fmt::Display for IncomeVerificationCreateRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateResponse {
    #[serde(rename = "income_verification_id")]
    ///ID of the verification. This ID is persisted throughout the lifetime of the verification.
    pub income_verification_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IncomeVerificationCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckRequest {
    #[serde(rename = "user")]
    ///Information about the user whose eligibility is being evaluated.
    pub user: Option<IncomeVerificationPrecheckUser>,
    #[serde(rename = "employer")]
    ///Information about the end user's employer
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    #[serde(rename = "transactions_access_token")]
    pub transactions_access_token: Option<serde_json::Value>,
    #[serde(rename = "transactions_access_tokens")]
    ///An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product.
    pub transactions_access_tokens: Option<Vec<String>>,
    #[serde(rename = "us_military_info")]
    ///Data about military info in the income verification precheck.
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
}
impl std::fmt::Display for IncomeVerificationPrecheckRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployer {
    #[serde(rename = "name")]
    ///The employer's name
    pub name: Option<String>,
    #[serde(rename = "address")]
    ///The address of the employer
    pub address: Option<IncomeVerificationPrecheckEmployerAddress>,
    #[serde(rename = "tax_id")]
    ///The employer's tax id
    pub tax_id: Option<String>,
    #[serde(rename = "url")]
    ///The URL for the employer's public website
    pub url: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddress {}
impl std::fmt::Display for IncomeVerificationPrecheckEmployerAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddressData {
    #[serde(rename = "city")]
    ///The full city name
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    #[serde(rename = "region")]
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckEmployerAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    #[serde(rename = "is_active_duty")]
    ///Is the user currently active duty in the US military
    pub is_active_duty: Option<bool>,
    #[serde(rename = "branch")]
    /**If the user is currently serving in the US military, the branch of the military in which they are serving
Valid values: 'AIR FORCE', 'ARMY', 'COAST GUARD', 'MARINES', 'NAVY', 'UNKNOWN'*/
    pub branch: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPrecheckMilitaryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckUser {
    #[serde(rename = "first_name")]
    ///The user's first name
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    ///The user's last name
    pub last_name: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address
    pub email_address: Option<String>,
    #[serde(rename = "home_address")]
    ///Data about the components comprising an address.
    pub home_address: Option<SignalAddressData>,
}
impl std::fmt::Display for IncomeVerificationPrecheckUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckResponse {
    #[serde(rename = "precheck_id")]
    ///ID of the precheck. Provide this value when calling `/link/token/create` in order to optimize Link conversion.
    pub precheck_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "confidence")]
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: String,
}
impl std::fmt::Display for IncomeVerificationPrecheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationPrecheckConfidence {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerification {
    #[serde(rename = "income_verification_id")]
    ///The `income_verification_id` of the verification instance, as provided by `/income/verification/create`.
    pub income_verification_id: Option<String>,
    #[serde(rename = "asset_report_id")]
    ///The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped.
    pub asset_report_id: Option<String>,
    #[serde(rename = "precheck_id")]
    ///The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow by streamlining the Link interface presented to the end user.
    pub precheck_id: Option<String>,
    #[serde(rename = "access_tokens")]
    /**An array of access tokens corresponding to Items that a user has previously connected with. Data from these institutions will be cross-referenced with document data received during the Document Income flow to help verify that the uploaded documents are accurate. If the `transactions` product was not initialized for these Items during link, it will be initialized after this Link session.

This field should only be used with the `payroll` income source type.*/
    pub access_tokens: Option<Vec<String>>,
    #[serde(rename = "income_source_types")]
    ///The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    pub income_source_types: Option<Vec<String>>,
    #[serde(rename = "bank_income")]
    ///Specifies options for initializing Link for use with Bank Income. This field is required if `income_verification` is included in the `products` array and `bank` is specified in `income_source_types`.
    pub bank_income: Option<LinkTokenCreateRequestIncomeVerificationBankIncome>,
    #[serde(rename = "payroll_income")]
    ///Specifies options for initializing Link for use with Payroll Income. This field is required if `income_verification` is included in the `products` array and `payroll` is specified in `income_source_types`.
    pub payroll_income: Option<LinkTokenCreateRequestIncomeVerificationPayrollIncome>,
    #[serde(rename = "stated_income_sources")]
    ///A list of user stated income sources
    pub stated_income_sources: Option<Vec<LinkTokenCreateRequestUserStatedIncomeSource>>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationSourceType {
    #[serde(rename = "bank")]
    Bank,
    #[serde(rename = "payroll")]
    Payroll,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerificationBankIncome {
    #[serde(rename = "days_requested")]
    ///The number of days of data to request for the Bank Income product
    pub days_requested: Option<i64>,
    #[serde(rename = "enable_multiple_items")]
    ///Whether to enable multiple items to be added in the link session
    pub enable_multiple_items: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    #[serde(rename = "flow_types")]
    ///The types of payroll income verification to enable for the link session. If none are specified, then users will see both document and digital payroll income.
    pub flow_types: Option<Vec<String>>,
    #[serde(rename = "is_update_mode")]
    ///An identifier to indicate whether the income verification link token will be used for an update or not
    pub is_update_mode: Option<bool>,
}
impl std::fmt::Display for LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationPayrollFlowType {
    #[serde(rename = "payroll_digital_income")]
    PayrollDigitalIncome,
    #[serde(rename = "payroll_document_income")]
    PayrollDocumentIncome,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationStatusWebhook {
    #[serde(rename = "webhook_type")]
    ///`"INCOME"`
    pub webhook_type: String,
    #[serde(rename = "webhook_code")]
    ///`INCOME_VERIFICATION`
    pub webhook_code: String,
    #[serde(rename = "item_id")]
    ///The Item ID associated with the verification.
    pub item_id: String,
    #[serde(rename = "user_id")]
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: Option<String>,
    #[serde(rename = "verification_status")]
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.*/
    pub verification_status: String,
}
impl std::fmt::Display for IncomeVerificationStatusWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshRequest {
    #[serde(rename = "income_verification_id")]
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<String>,
}
impl std::fmt::Display for IncomeVerificationRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "verification_refresh_status")]
    /**The verification refresh status. One of the following:

`"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED"` User presence is required to refresh an income verification.
`"VERIFICATION_REFRESH_SUCCESSFUL"` The income verification refresh was successful.
`"VERIFICATION_REFRESH_NOT_FOUND"` No new data was found after the income verification refresh.*/
    pub verification_refresh_status: String,
}
impl std::fmt::Display for IncomeVerificationRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummary {
    #[serde(rename = "employer_name")]
    ///The name of the employer, as reported on the paystub.
    pub employer_name: EmployerIncomeSummaryFieldString,
    #[serde(rename = "employee_name")]
    ///The name of the employee, as reported on the paystub.
    pub employee_name: EmployeeIncomeSummaryFieldString,
    #[serde(rename = "ytd_gross_income")]
    ///Year-to-date pre-tax earnings, as reported on the paystub.
    pub ytd_gross_income: YtdGrossIncomeSummaryFieldNumber,
    #[serde(rename = "ytd_net_income")]
    ///Year-to-date earnings after any tax withholdings, benefit payments or deductions, as reported on the paystub.
    pub ytd_net_income: YtdNetIncomeSummaryFieldNumber,
    #[serde(rename = "pay_frequency")]
    ///The frequency of the pay period.
    pub pay_frequency: Option<PayFrequency>,
    #[serde(rename = "projected_wage")]
    ///The employee's estimated annual salary, as derived from information reported on the paystub.
    pub projected_wage: ProjectedIncomeSummaryFieldNumber,
    #[serde(rename = "verified_transaction")]
    ///Information about the matched direct deposit transaction used to verify a user's payroll information.
    pub verified_transaction: Option<TransactionData>,
}
impl std::fmt::Display for IncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    #[serde(rename = "description")]
    ///The description of the transaction.
    pub description: String,
    #[serde(rename = "amount")]
    ///The amount of the transaction.
    pub amount: f64,
    #[serde(rename = "date")]
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub date: String,
    #[serde(rename = "account_id")]
    ///A unique identifier for the end user's account.
    pub account_id: String,
    #[serde(rename = "transaction_id")]
    ///A unique identifier for the transaction.
    pub transaction_id: String,
}
impl std::fmt::Display for TransactionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummaryFieldString {
    #[serde(rename = "value")]
    ///The value of the field.
    pub value: String,
    #[serde(rename = "verification_status")]
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: String,
}
impl std::fmt::Display for IncomeSummaryFieldString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerIncomeSummaryFieldString(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeIncomeSummaryFieldString(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummaryFieldNumber {
    #[serde(rename = "value")]
    ///The value of the field.
    pub value: f64,
    #[serde(rename = "verification_status")]
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: String,
}
impl std::fmt::Display for IncomeSummaryFieldNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct YtdGrossIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct YtdNetIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectedIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayFrequency {
    #[serde(rename = "value")]
    ///The frequency of the pay period.
    pub value: String,
    #[serde(rename = "verification_status")]
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: String,
}
impl std::fmt::Display for PayFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PayFrequencyValue {
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "semimonthly")]
    Semimonthly,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "biweekly")]
    Biweekly,
    #[serde(rename = "unknown")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationStatus {
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "UNVERIFIED")]
    Unverified,
    #[serde(rename = "NEEDS_INFO")]
    NeedsInfo,
    #[serde(rename = "UNABLE_TO_VERIFY")]
    UnableToVerify,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationRefreshStatus {
    #[serde(rename = "VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED")]
    VerificationRefreshStatusUserPresenceRequired,
    #[serde(rename = "VERIFICATION_REFRESH_SUCCESSFUL")]
    VerificationRefreshSuccessful,
    #[serde(rename = "VERIFICATION_REFRESH_NOT_FOUND")]
    VerificationRefreshNotFound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetRequest {
    #[serde(rename = "income_verification_id")]
    ///The ID of the verification for which to get paystub information.
    pub income_verification_id: Option<String>,
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<String>,
}
impl std::fmt::Display for IncomeVerificationPaystubsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetResponse {
    #[serde(rename = "document_metadata")]
    ///Metadata for an income document.
    pub document_metadata: Option<Vec<DocumentMetadata>>,
    #[serde(rename = "paystubs")]
    pub paystubs: Vec<Paystub>,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IncomeVerificationPaystubsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    #[serde(rename = "name")]
    ///The name of the document.
    pub name: Option<String>,
    #[serde(rename = "status")]
    ///The processing status of the document.
    pub status: Option<String>,
    #[serde(rename = "doc_id")]
    ///An identifier of the document that is also present in the paystub response.
    pub doc_id: Option<String>,
    #[serde(rename = "doc_type")]
    /**The type of document.

`DOCUMENT_TYPE_PAYSTUB`: A paystub.

`DOCUMENT_TYPE_BANK_STATEMENT`: A bank statement.

`DOCUMENT_TYPE_US_TAX_W2`: A W-2 wage and tax statement provided by a US employer reflecting wages earned by the employee.

`DOCUMENT_TYPE_US_MILITARY_ERAS`: An electronic Retirement Account Statement (eRAS) issued by the US military.

`DOCUMENT_TYPE_US_MILITARY_LES`: A Leave and Earnings Statement (LES) issued by the US military.

`DOCUMENT_TYPE_US_MILITARY_CLES`: A Civilian Leave and Earnings Statment (CLES) issued by the US military.

`DOCUMENT_TYPE_GIG`: Used to indicate that the income is related to gig work. Does not necessarily correspond to a specific document type.

`DOCUMENT_TYPE_NONE`: Used to indicate that there is no underlying document for the data.

`UNKNOWN`: Document type could not be determined.*/
    pub doc_type: Option<String>,
}
impl std::fmt::Display for DocumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
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
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Paystub {
    #[serde(rename = "deductions")]
    ///An object with the deduction information found on a paystub.
    pub deductions: Deductions,
    #[serde(rename = "doc_id")]
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: String,
    #[serde(rename = "earnings")]
    ///An object representing both a breakdown of earnings on a paystub and the total earnings.
    pub earnings: Earnings,
    #[serde(rename = "employee")]
    ///Data about the employee.
    pub employee: Employee,
    #[serde(rename = "employer")]
    ///Information about the employer on the paystub
    pub employer: PaystubEmployer,
    #[serde(rename = "employment_details")]
    ///An object representing employment details found on a paystub.
    pub employment_details: Option<EmploymentDetails>,
    #[serde(rename = "net_pay")]
    ///An object representing information about the net pay amount on the paystub.
    pub net_pay: NetPay,
    #[serde(rename = "pay_period_details")]
    ///Details about the pay period.
    pub pay_period_details: PayPeriodDetails,
    #[serde(rename = "paystub_details")]
    ///An object representing details that can be found on the paystub.
    pub paystub_details: Option<PaystubDetails>,
    #[serde(rename = "income_breakdown")]
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    #[serde(rename = "ytd_earnings")]
    ///The amount of income earned year to date, as based on paystub data.
    pub ytd_earnings: Option<PaystubYtdDetails>,
    #[serde(rename = "verification")]
    ///An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.
    pub verification: Option<PaystubVerification>,
}
impl std::fmt::Display for Paystub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Deductions {
    #[serde(rename = "subtotals")]
    pub subtotals: Option<Vec<Total>>,
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<DeductionsBreakdown>,
    #[serde(rename = "totals")]
    pub totals: Option<Vec<Total>>,
    #[serde(rename = "total")]
    ///An object representing the total deductions for the pay period
    pub total: DeductionsTotal,
}
impl std::fmt::Display for Deductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeductionsBreakdown {
    #[serde(rename = "current_amount")]
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    #[serde(rename = "description")]
    ///Description of the deduction line item
    pub description: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date amount of the deduction
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for DeductionsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeductionsTotal {
    #[serde(rename = "current_amount")]
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date total amount of the deductions
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for DeductionsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Total {
    #[serde(rename = "canonical_description")]
    ///Commonly used term to describe the line item.
    pub canonical_description: Option<String>,
    #[serde(rename = "description")]
    ///Text of the line item as printed on the paystub.
    pub description: Option<String>,
    #[serde(rename = "current_pay")]
    ///An object representing a monetary amount.
    pub current_pay: Option<Pay>,
    #[serde(rename = "ytd_pay")]
    ///An object representing a monetary amount.
    pub ytd_pay: Option<Pay>,
}
impl std::fmt::Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TotalCanonicalDescription {
    #[serde(rename = "BONUS")]
    Bonus,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "OVERTIME")]
    Overtime,
    #[serde(rename = "PAID TIME OFF")]
    PaidTimeOff,
    #[serde(rename = "REGULAR PAY")]
    RegularPay,
    #[serde(rename = "VACATION")]
    Vacation,
    #[serde(rename = "EMPLOYEE MEDICARE")]
    EmployeeMedicare,
    #[serde(rename = "FICA")]
    Fica,
    #[serde(rename = "SOCIAL SECURITY EMPLOYEE TAX")]
    SocialSecurityEmployeeTax,
    #[serde(rename = "MEDICAL")]
    Medical,
    #[serde(rename = "VISION")]
    Vision,
    #[serde(rename = "DENTAL")]
    Dental,
    #[serde(rename = "NET PAY")]
    NetPay,
    #[serde(rename = "TAXES")]
    Taxes,
    #[serde(rename = "NOT_FOUND")]
    NotFound,
    #[serde(rename = "OTHER")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pay {
    #[serde(rename = "amount")]
    ///A numerical amount of a specific currency.
    pub amount: Option<f64>,
    #[serde(rename = "currency")]
    ///Currency code, e.g. USD
    pub currency: Option<String>,
}
impl std::fmt::Display for Pay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Earnings {
    #[serde(rename = "subtotals")]
    pub subtotals: Option<Vec<EarningsTotal>>,
    #[serde(rename = "totals")]
    pub totals: Option<Vec<EarningsTotal>>,
    #[serde(rename = "breakdown")]
    pub breakdown: Option<Vec<EarningsBreakdown>>,
    #[serde(rename = "total")]
    ///An object representing both the current pay period and year to date amount for an earning category.
    pub total: Option<EarningsTotal>,
}
impl std::fmt::Display for Earnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsBreakdown {
    #[serde(rename = "canonical_description")]
    ///Commonly used term to describe the earning line item.
    pub canonical_description: Option<String>,
    #[serde(rename = "current_amount")]
    ///Raw amount of the earning line item.
    pub current_amount: Option<f64>,
    #[serde(rename = "description")]
    ///Description of the earning line item.
    pub description: Option<String>,
    #[serde(rename = "hours")]
    ///Number of hours applicable for this earning.
    pub hours: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "rate")]
    ///Hourly rate applicable for this earning.
    pub rate: Option<f64>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date amount of the deduction.
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for EarningsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EarningsBreakdownCanonicalDescription {
    #[serde(rename = "BONUS")]
    Bonus,
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "OVERTIME")]
    Overtime,
    #[serde(rename = "PAID TIME OFF")]
    PaidTimeOff,
    #[serde(rename = "REGULAR PAY")]
    RegularPay,
    #[serde(rename = "VACATION")]
    Vacation,
    #[serde(rename = "BASIC ALLOWANCE HOUSING")]
    BasicAllowanceHousing,
    #[serde(rename = "BASIC ALLOWANCE SUBSISTENCE")]
    BasicAllowanceSubsistence,
    #[serde(rename = "OTHER")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsTotal {
    #[serde(rename = "current_amount")]
    ///Total amount of the earnings for this pay period
    pub current_amount: Option<f64>,
    #[serde(rename = "current_pay")]
    ///An object representing a monetary amount.
    pub current_pay: Option<Pay>,
    #[serde(rename = "ytd_pay")]
    ///An object representing a monetary amount.
    pub ytd_pay: Option<Pay>,
    #[serde(rename = "hours")]
    ///Total number of hours worked for this pay period
    pub hours: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The total year-to-date amount of the earnings
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for EarningsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDetails {
    #[serde(rename = "annual_salary")]
    ///An object representing a monetary amount.
    pub annual_salary: Option<Pay>,
    #[serde(rename = "hire_date")]
    ///Date on which the employee was hired, in the YYYY-MM-DD format.
    pub hire_date: Option<String>,
}
impl std::fmt::Display for EmploymentDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetPay {
    #[serde(rename = "current_amount")]
    ///Raw amount of the net pay for the pay period
    pub current_amount: Option<f64>,
    #[serde(rename = "description")]
    ///Description of the net pay
    pub description: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date amount of the net pay
    pub ytd_amount: Option<f64>,
    #[serde(rename = "total")]
    ///An object representing both the current pay period and year to date amount for a category.
    pub total: Option<Total>,
}
impl std::fmt::Display for NetPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubDetails {
    #[serde(rename = "pay_period_start_date")]
    ///Beginning date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    pub pay_period_start_date: Option<String>,
    #[serde(rename = "pay_period_end_date")]
    ///Ending date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    pub pay_period_end_date: Option<String>,
    #[serde(rename = "pay_date")]
    ///Pay date on the paystub in the 'YYYY-MM-DD' format.
    pub pay_date: Option<String>,
    #[serde(rename = "paystub_provider")]
    ///The name of the payroll provider that generated the paystub, e.g. ADP
    pub paystub_provider: Option<String>,
    #[serde(rename = "pay_frequency")]
    ///The frequency at which the employee is paid. Possible values: `MONTHLY`, `BI-WEEKLY`, `WEEKLY`, `SEMI-MONTHLY`.
    pub pay_frequency: Option<String>,
}
impl std::fmt::Display for PaystubDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaystubPayFrequency {
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "BI-WEEKLY")]
    BiWeekly,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "SEMI-MONTHLY")]
    SemiMonthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeBreakdown {
    #[serde(rename = "type")]
    /**The type of income. Possible values include:
  `"regular"`: regular income
  `"overtime"`: overtime income
  `"bonus"`: bonus income*/
    pub type_: Option<String>,
    #[serde(rename = "rate")]
    ///The hourly rate at which the income is paid.
    pub rate: Option<f64>,
    #[serde(rename = "hours")]
    ///The number of hours logged for this income for this pay period.
    pub hours: Option<f64>,
    #[serde(rename = "total")]
    ///The total pay for this pay period.
    pub total: Option<f64>,
}
impl std::fmt::Display for IncomeBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeBreakdownType {
    #[serde(rename = "bonus")]
    Bonus,
    #[serde(rename = "overtime")]
    Overtime,
    #[serde(rename = "regular")]
    Regular,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    #[serde(rename = "address")]
    ///Address on the paystub
    pub address: PaystubAddress,
    #[serde(rename = "name")]
    ///The name of the employee.
    pub name: Option<String>,
    #[serde(rename = "marital_status")]
    ///Marital status of the employee - either `single` or `married`.
    pub marital_status: Option<String>,
    #[serde(rename = "taxpayer_id")]
    ///Taxpayer ID of the individual receiving the paystub.
    pub taxpayer_id: Option<TaxpayerId>,
}
impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxpayerId {
    #[serde(rename = "id_type")]
    ///Type of ID, e.g. 'SSN'
    pub id_type: Option<String>,
    #[serde(rename = "id_mask")]
    ///ID mask; i.e. last 4 digits of the taxpayer ID
    pub id_mask: Option<String>,
    #[serde(rename = "last_4_digits")]
    ///Last 4 digits of unique number of ID.
    pub last_4_digits: Option<String>,
}
impl std::fmt::Display for TaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubEmployer {
    #[serde(rename = "address")]
    ///Address on the paystub
    pub address: Option<PaystubAddress>,
    #[serde(rename = "name")]
    ///The name of the employer on the paystub.
    pub name: Option<String>,
}
impl std::fmt::Display for PaystubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubAddress {
    #[serde(rename = "city")]
    ///The full city name.
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code of the address.
    pub postal_code: Option<String>,
    #[serde(rename = "region")]
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    ///The full street address.
    pub street: Option<String>,
    #[serde(rename = "line1")]
    ///Street address line 1.
    pub line_1: Option<String>,
    #[serde(rename = "line2")]
    ///Street address line 2.
    pub line_2: Option<String>,
    #[serde(rename = "state_code")]
    /**The region or state
Example: `"NC"`*/
    pub state_code: Option<String>,
}
impl std::fmt::Display for PaystubAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayPeriodDetails {
    #[serde(rename = "check_amount")]
    ///The amount of the paycheck.
    pub check_amount: Option<f64>,
    #[serde(rename = "distribution_breakdown")]
    pub distribution_breakdown: Option<Vec<DistributionBreakdown>>,
    #[serde(rename = "end_date")]
    ///The pay period end date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    pub end_date: Option<String>,
    #[serde(rename = "gross_earnings")]
    ///Total earnings before tax/deductions.
    pub gross_earnings: Option<f64>,
    #[serde(rename = "pay_date")]
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_date: Option<String>,
    #[serde(rename = "pay_frequency")]
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
    #[serde(rename = "pay_day")]
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_day: Option<String>,
    #[serde(rename = "start_date")]
    ///The pay period start date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    pub start_date: Option<String>,
}
impl std::fmt::Display for PayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionBreakdown {
    #[serde(rename = "account_name")]
    ///Name of the account for the given distribution.
    pub account_name: Option<String>,
    #[serde(rename = "bank_name")]
    ///The name of the bank that the payment is being deposited to.
    pub bank_name: Option<String>,
    #[serde(rename = "current_amount")]
    ///The amount distributed to this account.
    pub current_amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "mask")]
    ///The last 2-4 alphanumeric characters of an account's official account number.
    pub mask: Option<String>,
    #[serde(rename = "type")]
    ///Type of the account that the paystub was sent to (e.g. 'checking').
    pub type_: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "current_pay")]
    ///An object representing a monetary amount.
    pub current_pay: Option<Pay>,
}
impl std::fmt::Display for DistributionBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubDeduction {
    #[serde(rename = "type")]
    ///The description of the deduction, as provided on the paystub. For example: `"401(k)"`, `"FICA MED TAX"`.
    pub type_: Option<String>,
    #[serde(rename = "is_pretax")]
    ///`true` if the deduction is pre-tax; `false` otherwise.
    pub is_pretax: Option<bool>,
    #[serde(rename = "total")]
    ///The amount of the deduction.
    pub total: Option<f64>,
}
impl std::fmt::Display for PaystubDeduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubYtdDetails {
    #[serde(rename = "gross_earnings")]
    ///Year-to-date gross earnings.
    pub gross_earnings: Option<f64>,
    #[serde(rename = "net_earnings")]
    ///Year-to-date net (take home) earnings.
    pub net_earnings: Option<f64>,
}
impl std::fmt::Display for PaystubYtdDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubVerification {
    #[serde(rename = "verification_status")]
    ///Derived verification status.
    pub verification_status: Option<String>,
    #[serde(rename = "verification_attributes")]
    pub verification_attributes: Vec<VerificationAttribute>,
}
impl std::fmt::Display for PaystubVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaystubVerificationStatus {
    #[serde(rename = "PAYSTUB_VERIFICATION_STATUS_UNKNOWN")]
    PaystubVerificationStatusUnknown,
    #[serde(rename = "PAYSTUB_VERIFICATION_STATUS_VERIFIED")]
    PaystubVerificationStatusVerified,
    #[serde(rename = "PAYSTUB_VERIFICATION_STATUS_FRAUDULENT")]
    PaystubVerificationStatusFraudulent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationAttribute {
    #[serde(rename = "type")]
    ///Message indicating the reason as to why the verification failed
    pub type_: Option<String>,
}
impl std::fmt::Display for VerificationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationDocumentsDownloadRequest {
    #[serde(rename = "income_verification_id")]
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<String>,
    #[serde(rename = "document_id")]
    ///The document ID to download. If passed, a single document will be returned in the resulting zip file, rather than all document
    pub document_id: Option<String>,
}
impl std::fmt::Display for IncomeVerificationDocumentsDownloadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetRequest {
    #[serde(rename = "income_verification_id")]
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<String>,
}
impl std::fmt::Display for IncomeVerificationTaxformsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<String>,
    #[serde(rename = "document_metadata")]
    pub document_metadata: Vec<DocumentMetadata>,
    #[serde(rename = "taxforms")]
    ///A list of forms.
    pub taxforms: Vec<Taxform>,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for IncomeVerificationTaxformsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Taxform {
    #[serde(rename = "doc_id")]
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: Option<String>,
    #[serde(rename = "document_type")]
    ///The type of tax document. Currently, the only supported value is `w2`.
    pub document_type: String,
    #[serde(rename = "w2")]
    ///W2 is an object that represents income data taken from a W2 tax document.
    pub w_2: Option<W2>,
}
impl std::fmt::Display for Taxform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2 {
    #[serde(rename = "employer")]
    ///Information about the employer on the paystub
    pub employer: Option<PaystubEmployer>,
    #[serde(rename = "employee")]
    ///Data about the employee.
    pub employee: Option<Employee>,
    #[serde(rename = "tax_year")]
    ///The tax year of the W2 document.
    pub tax_year: Option<String>,
    #[serde(rename = "employer_id_number")]
    ///An employee identification number or EIN.
    pub employer_id_number: Option<String>,
    #[serde(rename = "wages_tips_other_comp")]
    ///Wages from tips and other compensation.
    pub wages_tips_other_comp: Option<String>,
    #[serde(rename = "federal_income_tax_withheld")]
    ///Federal income tax withheld for the tax year.
    pub federal_income_tax_withheld: Option<String>,
    #[serde(rename = "social_security_wages")]
    ///Wages from social security.
    pub social_security_wages: Option<String>,
    #[serde(rename = "social_security_tax_withheld")]
    ///Social security tax withheld for the tax year.
    pub social_security_tax_withheld: Option<String>,
    #[serde(rename = "medicare_wages_and_tips")]
    ///Wages and tips from medicare.
    pub medicare_wages_and_tips: Option<String>,
    #[serde(rename = "medicare_tax_withheld")]
    ///Medicare tax withheld for the tax year.
    pub medicare_tax_withheld: Option<String>,
    #[serde(rename = "social_security_tips")]
    ///Tips from social security.
    pub social_security_tips: Option<String>,
    #[serde(rename = "allocated_tips")]
    ///Allocated tips.
    pub allocated_tips: Option<String>,
    #[serde(rename = "box_9")]
    ///Contents from box 9 on the W2.
    pub box_9: Option<String>,
    #[serde(rename = "dependent_care_benefits")]
    ///Dependent care benefits.
    pub dependent_care_benefits: Option<String>,
    #[serde(rename = "nonqualified_plans")]
    ///Nonqualified plans.
    pub nonqualified_plans: Option<String>,
    #[serde(rename = "box_12")]
    pub box_12: Option<Vec<W2Box12>>,
    #[serde(rename = "statutory_employee")]
    ///Statutory employee.
    pub statutory_employee: Option<String>,
    #[serde(rename = "retirement_plan")]
    ///Retirement plan.
    pub retirement_plan: Option<String>,
    #[serde(rename = "third_party_sick_pay")]
    ///Third party sick pay.
    pub third_party_sick_pay: Option<String>,
    #[serde(rename = "other")]
    ///Other.
    pub other: Option<String>,
    #[serde(rename = "state_and_local_wages")]
    pub state_and_local_wages: Option<Vec<W2StateAndLocalWages>>,
}
impl std::fmt::Display for W2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2Box12 {
    #[serde(rename = "code")]
    ///W2 Box 12 code.
    pub code: Option<String>,
    #[serde(rename = "amount")]
    ///W2 Box 12 amount.
    pub amount: Option<String>,
}
impl std::fmt::Display for W2Box12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2StateAndLocalWages {
    #[serde(rename = "state")]
    ///State associated with the wage.
    pub state: Option<String>,
    #[serde(rename = "employer_state_id_number")]
    ///State identification number of the employer.
    pub employer_state_id_number: Option<String>,
    #[serde(rename = "state_wages_tips")]
    ///Wages and tips from the specified state.
    pub state_wages_tips: Option<String>,
    #[serde(rename = "state_income_tax")]
    ///Income tax from the specified state.
    pub state_income_tax: Option<String>,
    #[serde(rename = "local_wages_tips")]
    ///Wages and tips from the locality.
    pub local_wages_tips: Option<String>,
    #[serde(rename = "local_income_tax")]
    ///Income tax from the locality.
    pub local_income_tax: Option<String>,
    #[serde(rename = "locality_name")]
    ///Name of the locality.
    pub locality_name: Option<String>,
}
impl std::fmt::Display for W2StateAndLocalWages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationWebhookStatus {
    #[serde(rename = "id")]
    pub id: String,
}
impl std::fmt::Display for IncomeVerificationWebhookStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for EmploymentVerificationGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationGetResponse {
    #[serde(rename = "employments")]
    ///A list of employment verification summaries.
    pub employments: Vec<EmploymentVerification>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EmploymentVerificationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerification {
    #[serde(rename = "status")]
    ///Current employment status.
    pub status: Option<String>,
    #[serde(rename = "start_date")]
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    pub end_date: Option<String>,
    #[serde(rename = "employer")]
    ///An object containing employer data.
    pub employer: Option<EmployerVerification>,
    #[serde(rename = "title")]
    ///Current title of employee.
    pub title: Option<String>,
    #[serde(rename = "platform_ids")]
    ///An object containing a set of ids related to an employee
    pub platform_ids: Option<PlatformIds>,
}
impl std::fmt::Display for EmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EmploymentVerificationStatus {
    #[serde(rename = "EMPLOYMENT_STATUS_ACTIVE")]
    EmploymentStatusActive,
    #[serde(rename = "EMPLOYMENT_STATUS_INACTIVE")]
    EmploymentStatusInactive,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerVerification {
    #[serde(rename = "name")]
    ///Name of employer.
    pub name: Option<String>,
}
impl std::fmt::Display for EmployerVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformIds {
    #[serde(rename = "employee_id")]
    ///The ID of an employee as given by their employer
    pub employee_id: Option<String>,
    #[serde(rename = "payroll_id")]
    ///The ID of an employee as given by their payroll
    pub payroll_id: Option<String>,
    #[serde(rename = "position_id")]
    ///The ID of the position of the employee
    pub position_id: Option<String>,
}
impl std::fmt::Display for PlatformIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportTransaction(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthIncident {
    #[serde(rename = "start_date")]
    ///The start date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub start_date: String,
    #[serde(rename = "end_date")]
    ///The end date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub end_date: Option<String>,
    #[serde(rename = "title")]
    ///The title of the incident
    pub title: String,
    #[serde(rename = "incident_updates")]
    ///Updates on the health incident.
    pub incident_updates: Vec<IncidentUpdate>,
}
impl std::fmt::Display for HealthIncident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncidentUpdate {
    #[serde(rename = "description")]
    ///The content of the update.
    pub description: Option<String>,
    #[serde(rename = "status")]
    ///The status of the incident.
    pub status: Option<String>,
    #[serde(rename = "updated_date")]
    ///The date when the update was published, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub updated_date: Option<String>,
}
impl std::fmt::Display for IncidentUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    #[serde(rename = "target_account")]
    ///The deposit switch destination account
    pub target_account: DepositSwitchTargetAccount,
    #[serde(rename = "target_user")]
    ///The deposit switch target user
    pub target_user: DepositSwitchTargetUser,
    #[serde(rename = "options")]
    ///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub options: Option<DepositSwitchCreateRequestOptions>,
    #[serde(rename = "country_code")]
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: Option<String>,
}
impl std::fmt::Display for DepositSwitchAltCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateResponse {
    #[serde(rename = "deposit_switch_id")]
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DepositSwitchAltCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTargetAccount {
    #[serde(rename = "account_number")]
    ///Account number for deposit switch destination
    pub account_number: String,
    #[serde(rename = "routing_number")]
    ///Routing number for deposit switch destination
    pub routing_number: String,
    #[serde(rename = "account_name")]
    ///The name of the deposit switch destination account, as it will be displayed to the end user in the Deposit Switch interface. It is not required to match the name used in online banking.
    pub account_name: String,
    #[serde(rename = "account_subtype")]
    ///The account subtype of the account, either `checking` or `savings`.
    pub account_subtype: String,
}
impl std::fmt::Display for DepositSwitchTargetAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTargetUser {
    #[serde(rename = "given_name")]
    ///The given name (first name) of the user.
    pub given_name: String,
    #[serde(rename = "family_name")]
    ///The family name (last name) of the user.
    pub family_name: String,
    #[serde(rename = "phone")]
    ///The phone number of the user. The endpoint can accept a variety of phone number formats, including E.164.
    pub phone: String,
    #[serde(rename = "email")]
    ///The email address of the user.
    pub email: String,
    #[serde(rename = "address")]
    ///The user's address.
    pub address: Option<DepositSwitchAddressData>,
    #[serde(rename = "tax_payer_id")]
    ///The taxpayer ID of the user, generally their SSN, EIN, or TIN.
    pub tax_payer_id: Option<String>,
}
impl std::fmt::Display for DepositSwitchTargetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAddressData {
    #[serde(rename = "city")]
    ///The full city name
    pub city: String,
    #[serde(rename = "region")]
    /**The region or state
Example: `"NC"`*/
    pub region: String,
    #[serde(rename = "street")]
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    #[serde(rename = "postal_code")]
    ///The postal code
    pub postal_code: String,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code
    pub country: String,
}
impl std::fmt::Display for DepositSwitchAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: Option<String>,
    #[serde(rename = "options")]
    ///An optional object for `/credit/bank_income/get` request options.
    pub options: Option<CreditBankIncomeGetRequestOptions>,
}
impl std::fmt::Display for CreditBankIncomeGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequestOptions {
    #[serde(rename = "count")]
    ///How many Bank Income Reports should be fetched. Multiple reports may be available if the report has been re-created or refreshed. If more than one report is available, the most recent reports will be returned first.
    pub count: Option<i64>,
}
impl std::fmt::Display for CreditBankIncomeGetRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeGetResponse {
    #[serde(rename = "bank_income")]
    pub bank_income: Option<Vec<CreditBankIncome>>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditBankIncomeGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomePdfGetRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: String,
}
impl std::fmt::Display for CreditBankIncomePdfGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomePdfGetResponse(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncome {
    #[serde(rename = "bank_income_id")]
    ///The unique identifier associated with the Bank Income Report.
    pub bank_income_id: Option<String>,
    #[serde(rename = "generated_time")]
    ///The time when the Bank Income Report was generated.
    pub generated_time: Option<String>,
    #[serde(rename = "days_requested")]
    ///The number of days requested by the customer for the Bank Income Report.
    pub days_requested: Option<i64>,
    #[serde(rename = "items")]
    ///The list of Items in the report along with the associated metadata about the Item.
    pub items: Option<Vec<CreditBankIncomeItem>>,
    #[serde(rename = "bank_income_summary")]
    ///Summary for bank income across all income sources and items (max history of 730 days).
    pub bank_income_summary: Option<CreditBankIncomeSummary>,
    #[serde(rename = "warnings")]
    ///If data from the Bank Income report was unable to be retrieved, the warnings will contain information about the error that caused the data to be incomplete.
    pub warnings: Option<Vec<CreditBankIncomeWarning>>,
}
impl std::fmt::Display for CreditBankIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeItem {
    #[serde(rename = "bank_income_accounts")]
    ///The Item's accounts that have Bank Income data.
    pub bank_income_accounts: Option<Vec<CreditBankIncomeAccount>>,
    #[serde(rename = "bank_income_sources")]
    ///The income sources for this Item. Each entry in the array is a single income source.
    pub bank_income_sources: Option<Vec<CreditBankIncomeSource>>,
    #[serde(rename = "last_updated_time")]
    ///The time when this Item's data was last retrieved from the financial institution.
    pub last_updated_time: Option<String>,
    #[serde(rename = "institution_id")]
    ///The unique identifier of the institution associated with the Item.
    pub institution_id: Option<String>,
    #[serde(rename = "institution_name")]
    ///The name of the institution associated with the Item.
    pub institution_name: Option<String>,
    #[serde(rename = "item_id")]
    ///The unique identifier for the Item.
    pub item_id: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeAccount {
    #[serde(rename = "account_id")]
    ///Plaid's unique identifier for the account.
    pub account_id: Option<String>,
    #[serde(rename = "mask")]
    /**The last 2-4 alphanumeric characters of an account's official account number.
Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.*/
    pub mask: Option<String>,
    #[serde(rename = "name")]
    ///The name of the bank account.
    pub name: Option<String>,
    #[serde(rename = "official_name")]
    ///The official name of the bank account.
    pub official_name: Option<String>,
    #[serde(rename = "subtype")]
    ///Valid account subtypes for depository accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-depository).
    pub subtype: Option<String>,
    #[serde(rename = "type")]
    ///The account type. This will always be `depository`.
    pub type_: Option<String>,
    #[serde(rename = "owners")]
    pub owners: Option<Vec<Owner>>,
}
impl std::fmt::Display for CreditBankIncomeAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeAccountType {
    #[serde(rename = "depository")]
    Depository,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeSource {
    #[serde(rename = "income_source_id")]
    ///A unique identifier for an income source.
    pub income_source_id: Option<String>,
    #[serde(rename = "income_description")]
    ///The most common name or original description for the underlying income transactions.
    pub income_description: Option<String>,
    #[serde(rename = "income_category")]
    ///The income category.
    pub income_category: Option<String>,
    #[serde(rename = "account_id")]
    ///Plaid's unique idenfier for the account.
    pub account_id: Option<String>,
    #[serde(rename = "start_date")]
    /**Minimum of all dates within the specific income sources in the user's bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    /**Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: Option<String>,
    #[serde(rename = "pay_frequency")]
    ///The income pay frequency.
    pub pay_frequency: Option<String>,
    #[serde(rename = "total_amount")]
    ///Total amount of earnings in the user’s bank account for the specific income source for days requested by the client.
    pub total_amount: Option<f64>,
    #[serde(rename = "transaction_count")]
    ///Number of transactions for the income source within the start and end date.
    pub transaction_count: Option<i64>,
    #[serde(rename = "historical_summary")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
}
impl std::fmt::Display for CreditBankIncomeSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeCategory {
    #[serde(rename = "SALARY")]
    Salary,
    #[serde(rename = "UNEMPLOYMENT")]
    Unemployment,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "GIG_ECONOMY")]
    GigEconomy,
    #[serde(rename = "RENTAL")]
    Rental,
    #[serde(rename = "CHILD_SUPPORT")]
    ChildSupport,
    #[serde(rename = "MILITARY")]
    Military,
    #[serde(rename = "RETIREMENT")]
    Retirement,
    #[serde(rename = "LONG_TERM_DISABILITY")]
    LongTermDisability,
    #[serde(rename = "BANK_INTEREST")]
    BankInterest,
    #[serde(rename = "OTHER")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomePayFrequency {
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(rename = "BIWEEKLY")]
    Biweekly,
    #[serde(rename = "SEMI_MONTHLY")]
    SemiMonthly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditIsoCurrencyCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditUnofficialCurrencyCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeSummary {
    #[serde(rename = "total_amount")]
    ///Total amount of earnings across all the income sources in the end user's Items for the days requested by the client.
    pub total_amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO 4217 currency code of the amount or balance.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "start_date")]
    /**The earliest date within the days requested in which all income sources identified by Plaid appear in a user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    /**The latest date in which all income sources identified by Plaid appear in the user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: Option<String>,
    #[serde(rename = "income_sources_count")]
    ///Number of income sources per end user.
    pub income_sources_count: Option<i64>,
    #[serde(rename = "income_categories_count")]
    ///Number of income categories per end user.
    pub income_categories_count: Option<i64>,
    #[serde(rename = "income_transactions_count")]
    ///Number of income transactions per end user.
    pub income_transactions_count: Option<i64>,
    #[serde(rename = "historical_summary")]
    pub historical_summary: Option<Vec<CreditBankIncomeHistoricalSummary>>,
}
impl std::fmt::Display for CreditBankIncomeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeHistoricalSummary {
    #[serde(rename = "total_amount")]
    ///Total amount of earnings for the income source(s) of the user for the month in the summary.
    pub total_amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO 4217 currency code of the amount or balance.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "start_date")]
    /**The start date of the period covered in this monthly summary.
This date will be the first day of the month, unless the month being covered is a partial month because it is the first month included in the summary and the date range being requested does not begin with the first day of the month.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    /**The end date of the period included in this monthly summary.
This date will be the last day of the month, unless the month being covered is a partial month because it is the last month included in the summary and the date range being requested does not end with the last day of the month.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: Option<String>,
    #[serde(rename = "transactions")]
    pub transactions: Option<Vec<CreditBankIncomeTransaction>>,
}
impl std::fmt::Display for CreditBankIncomeHistoricalSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeTransaction {
    #[serde(rename = "amount")]
    /**The settled value of the transaction, denominated in the transactions's currency as stated in `iso_currency_code` or `unofficial_currency_code`.
Positive values when money moves out of the account; negative values when money moves in.
For example, credit card purchases are positive; credit card payment, direct deposits, and refunds are negative.*/
    pub amount: Option<f64>,
    #[serde(rename = "date")]
    /**For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted.
Both dates are returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub date: Option<String>,
    #[serde(rename = "name")]
    ///The merchant name or transaction description.
    pub name: Option<String>,
    #[serde(rename = "original_description")]
    ///The string returned by the financial institution to describe the transaction.
    pub original_description: Option<String>,
    #[serde(rename = "pending")]
    /**When true, identifies the transaction as pending or unsettled.
Pending transaction details (name, type, amount, category ID) may change before they are settled.*/
    pub pending: Option<bool>,
    #[serde(rename = "transaction_id")]
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: Option<String>,
    #[serde(rename = "check_number")]
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO 4217 currency code of the amount or balance.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: String,
    #[serde(rename = "options")]
    ///An optional object for `/credit/bank_income/refresh` request options.
    pub options: Option<CreditBankIncomeRefreshRequestOptions>,
}
impl std::fmt::Display for CreditBankIncomeRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshRequestOptions {
    #[serde(rename = "days_requested")]
    ///How many days of data to include in the refresh. If not specified, this will default to the days requested in the most recently generated bank income report for the user.
    pub days_requested: Option<i64>,
    #[serde(rename = "webhook")]
    ///The URL where Plaid will send the bank income webhook.
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeRefreshRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditBankIncomeRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: Option<String>,
}
impl std::fmt::Display for CreditPayrollIncomeRiskSignalsGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetResponse {
    #[serde(rename = "items")]
    ///Array of payroll items.
    pub items: Vec<PayrollRiskSignalsItem>,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditPayrollIncomeRiskSignalsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollRiskSignalsItem {
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "verification_risk_signals")]
    ///Array of payroll income document authenticity data retrieved for each of the user's accounts
    pub verification_risk_signals: Vec<DocumentRiskSignalsObject>,
}
impl std::fmt::Display for PayrollRiskSignalsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRiskSignalsObject {
    #[serde(rename = "account_id")]
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    #[serde(rename = "single_document_risk_signals")]
    ///Array of document metadata and associated risk signals per document
    pub single_document_risk_signals: Vec<SingleDocumentRiskSignal>,
    #[serde(rename = "multi_document_risk_signals")]
    ///Array of risk signals computed from a set of uploaded documents and the associated documents' metadata
    pub multi_document_risk_signals: Vec<MultiDocumentRiskSignal>,
}
impl std::fmt::Display for DocumentRiskSignalsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskSignalDocumentReference {
    #[serde(rename = "document_id")]
    ///An identifier of the document referenced by the document metadata.
    pub document_id: Option<String>,
    #[serde(rename = "document_name")]
    ///The name of the document
    pub document_name: Option<String>,
}
impl std::fmt::Display for RiskSignalDocumentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleDocumentRiskSignal {
    #[serde(rename = "document_reference")]
    ///Object containing metadata for the document
    pub document_reference: RiskSignalDocumentReference,
    #[serde(rename = "risk_signals")]
    ///Array of attributes that indicate whether or not there is fraud risk with a document
    pub risk_signals: Vec<DocumentRiskSignal>,
}
impl std::fmt::Display for SingleDocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiDocumentRiskSignal {
    #[serde(rename = "document_references")]
    ///Array of objects containing attributes that could indicate if a document is fraudulent
    pub document_references: Vec<RiskSignalDocumentReference>,
    #[serde(rename = "risk_signals")]
    ///Array of attributes that indicate whether or not there is fraud risk with a set of documents
    pub risk_signals: Vec<DocumentRiskSignal>,
}
impl std::fmt::Display for MultiDocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateRequest {
    #[serde(rename = "report_tokens")]
    ///List of report tokens; can include both Asset Report tokens and Income Report tokens.
    pub report_tokens: Vec<ReportToken>,
    #[serde(rename = "auditor_id")]
    ///The `auditor_id` of the third party with whom you would like to share the Asset Report and/or Income Report.
    pub auditor_id: String,
}
impl std::fmt::Display for CreditAuditCopyTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateResponse {
    #[serde(rename = "audit_copy_token")]
    ///A token that can be shared with a third party auditor to allow them to obtain access to the Asset or Income Report. This token should be stored securely.
    pub audit_copy_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditAuditCopyTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenRemoveRequest {
    #[serde(rename = "audit_copy_token")]
    ///The `audit_copy_token` granting access to the Audit Copy you would like to revoke.
    pub audit_copy_token: String,
}
impl std::fmt::Display for CreditAuditCopyTokenRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenRemoveResponse {
    #[serde(rename = "removed")]
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditAuditCopyTokenRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeGetRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: Option<String>,
}
impl std::fmt::Display for CreditPayrollIncomeGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeGetResponse {
    #[serde(rename = "items")]
    ///Array of payroll items.
    pub items: Vec<PayrollItem>,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditPayrollIncomeGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditDocumentMetadata {
    #[serde(rename = "name")]
    ///The name of the document.
    pub name: String,
    #[serde(rename = "document_type")]
    /**The type of document.

`PAYSTUB`: A paystub.

`BANK_STATEMENT`: A bank statement.

`US_TAX_W2`: A W-2 wage and tax statement provided by a US employer reflecting wages earned by the employee.

`US_MILITARY_ERAS`: An electronic Retirement Account Statement (eRAS) issued by the US military.

`US_MILITARY_LES`: A Leave and Earnings Statement (LES) issued by the US military.

`US_MILITARY_CLES`: A Civilian Leave and Earnings Statment (CLES) issued by the US military.

`GIG`: Used to indicate that the income is related to gig work. Does not necessarily correspond to a specific document type.

`NONE`: Used to indicate that there is no underlying document for the data.

`UNKNOWN`: Document type could not be determined.*/
    pub document_type: Option<String>,
    #[serde(rename = "download_url")]
    ///Signed URL to retrieve the underlying file.
    pub download_url: Option<String>,
    #[serde(rename = "status")]
    ///The processing status of the document.
    pub status: Option<String>,
}
impl std::fmt::Display for CreditDocumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditDocumentType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollItem {
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "accounts")]
    pub accounts: Vec<PayrollIncomeAccountData>,
    #[serde(rename = "payroll_income")]
    pub payroll_income: Vec<PayrollIncomeObject>,
    #[serde(rename = "status")]
    ///Details about the status of the payroll item.
    pub status: Option<PayrollItemStatus>,
    #[serde(rename = "pull_id")]
    ///A reference id to reference what payroll data was returned from this endpoint
    pub pull_id: String,
}
impl std::fmt::Display for PayrollItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollIncomeAccountData {
    #[serde(rename = "account_id")]
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    #[serde(rename = "rate_of_pay")]
    ///An object representing the rate at which an individual is paid.
    pub rate_of_pay: PayrollIncomeRateOfPay,
    #[serde(rename = "pay_frequency")]
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
}
impl std::fmt::Display for PayrollIncomeAccountData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollIncomeObject {
    #[serde(rename = "account_id")]
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    #[serde(rename = "pay_stubs")]
    ///Array of pay stubs for the user.
    pub pay_stubs: Vec<CreditPayStub>,
    #[serde(rename = "w2s")]
    ///Array of tax form W-2s.
    pub w_2_s: Vec<CreditW2>,
    #[serde(rename = "form1099s")]
    ///Array of tax form 1099s.
    pub form_1099_s: Vec<Credit1099>,
}
impl std::fmt::Display for PayrollIncomeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099 {
    #[serde(rename = "document_id")]
    ///An identifier of the document referenced by the document metadata.
    pub document_id: Option<String>,
    #[serde(rename = "document_metadata")]
    ///Object representing metadata pertaining to the document.
    pub document_metadata: Option<CreditDocumentMetadata>,
    #[serde(rename = "form_1099_type")]
    ///Form 1099 Type
    pub form_1099_type: Option<String>,
    #[serde(rename = "recipient")]
    ///An object representing a recipient used in both 1099-K and 1099-MISC tax documents.
    pub recipient: Option<Credit1099Recipient>,
    #[serde(rename = "payer")]
    ///An object representing a payer used by 1099-MISC tax documents.
    pub payer: Option<Credit1099Payer>,
    #[serde(rename = "filer")]
    ///An object representing a filer used by 1099-K tax documents.
    pub filer: Option<Credit1099Filer>,
    #[serde(rename = "tax_year")]
    ///Tax year of the tax form.
    pub tax_year: Option<String>,
    #[serde(rename = "rents")]
    ///Amount in rent by payer.
    pub rents: Option<f64>,
    #[serde(rename = "royalties")]
    ///Amount in royalties by payer.
    pub royalties: Option<f64>,
    #[serde(rename = "other_income")]
    ///Amount in other income by payer.
    pub other_income: Option<f64>,
    #[serde(rename = "federal_income_tax_withheld")]
    ///Amount of federal income tax withheld from payer.
    pub federal_income_tax_withheld: Option<f64>,
    #[serde(rename = "fishing_boat_proceeds")]
    ///Amount of fishing boat proceeds from payer.
    pub fishing_boat_proceeds: Option<f64>,
    #[serde(rename = "medical_and_healthcare_payments")]
    ///Amount of medical and healthcare payments from payer.
    pub medical_and_healthcare_payments: Option<f64>,
    #[serde(rename = "nonemployee_compensation")]
    ///Amount of nonemployee compensation from payer.
    pub nonemployee_compensation: Option<f64>,
    #[serde(rename = "substitute_payments_in_lieu_of_dividends_or_interest")]
    ///Amount of substitute payments made by payer.
    pub substitute_payments_in_lieu_of_dividends_or_interest: Option<f64>,
    #[serde(
        rename = "payer_made_direct_sales_of_5000_or_more_of_consumer_products_to_buyer"
    )]
    ///Whether or not payer made direct sales over $5000 of consumer products.
    pub payer_made_direct_sales_of_5000_or_more_of_consumer_products_to_buyer: Option<
        String,
    >,
    #[serde(rename = "crop_insurance_proceeds")]
    ///Amount of crop insurance proceeds.
    pub crop_insurance_proceeds: Option<f64>,
    #[serde(rename = "excess_golden_parachute_payments")]
    ///Amount of golden parachute payments made by payer.
    pub excess_golden_parachute_payments: Option<f64>,
    #[serde(rename = "gross_proceeds_paid_to_an_attorney")]
    ///Amount of gross proceeds paid to an attorney by payer.
    pub gross_proceeds_paid_to_an_attorney: Option<f64>,
    #[serde(rename = "section_409a_deferrals")]
    ///Amount of 409A deferrals earned by payer.
    pub section_409_a_deferrals: Option<f64>,
    #[serde(rename = "section_409a_income")]
    ///Amount of 409A income earned by payer.
    pub section_409_a_income: Option<f64>,
    #[serde(rename = "state_tax_withheld")]
    ///Amount of state tax withheld of payer for primary state.
    pub state_tax_withheld: Option<f64>,
    #[serde(rename = "state_tax_withheld_lower")]
    ///Amount of state tax withheld of payer for secondary state.
    pub state_tax_withheld_lower: Option<f64>,
    #[serde(rename = "payer_state_number")]
    ///Primary state ID.
    pub payer_state_number: Option<String>,
    #[serde(rename = "payer_state_number_lower")]
    ///Secondary state ID.
    pub payer_state_number_lower: Option<String>,
    #[serde(rename = "state_income")]
    ///State income reported for primary state.
    pub state_income: Option<f64>,
    #[serde(rename = "state_income_lower")]
    ///State income reported for secondary state.
    pub state_income_lower: Option<f64>,
    #[serde(rename = "transactions_reported")]
    ///One of the values will be provided Payment card Third party network
    pub transactions_reported: Option<String>,
    #[serde(rename = "pse_name")]
    ///Name of the PSE (Payment Settlement Entity).
    pub pse_name: Option<String>,
    #[serde(rename = "pse_telephone_number")]
    ///Formatted (XXX) XXX-XXXX. Phone number of the PSE (Payment Settlement Entity).
    pub pse_telephone_number: Option<String>,
    #[serde(rename = "gross_amount")]
    ///Gross amount reported.
    pub gross_amount: Option<f64>,
    #[serde(rename = "card_not_present_transaction")]
    ///Amount in card not present transactions.
    pub card_not_present_transaction: Option<f64>,
    #[serde(rename = "merchant_category_code")]
    ///Merchant category of filer.
    pub merchant_category_code: Option<String>,
    #[serde(rename = "number_of_payment_transactions")]
    ///Number of payment transactions made.
    pub number_of_payment_transactions: Option<String>,
    #[serde(rename = "january_amount")]
    ///Amount reported for January.
    pub january_amount: Option<f64>,
    #[serde(rename = "february_amount")]
    ///Amount reported for February.
    pub february_amount: Option<f64>,
    #[serde(rename = "march_amount")]
    ///Amount reported for March.
    pub march_amount: Option<f64>,
    #[serde(rename = "april_amount")]
    ///Amount reported for April.
    pub april_amount: Option<f64>,
    #[serde(rename = "may_amount")]
    ///Amount reported for May.
    pub may_amount: Option<f64>,
    #[serde(rename = "june_amount")]
    ///Amount reported for June.
    pub june_amount: Option<f64>,
    #[serde(rename = "july_amount")]
    ///Amount reported for July.
    pub july_amount: Option<f64>,
    #[serde(rename = "august_amount")]
    ///Amount reported for August.
    pub august_amount: Option<f64>,
    #[serde(rename = "september_amount")]
    ///Amount reported for September.
    pub september_amount: Option<f64>,
    #[serde(rename = "october_amount")]
    ///Amount reported for October.
    pub october_amount: Option<f64>,
    #[serde(rename = "november_amount")]
    ///Amount reported for November.
    pub november_amount: Option<f64>,
    #[serde(rename = "december_amount")]
    ///Amount reported for December.
    pub december_amount: Option<f64>,
    #[serde(rename = "primary_state")]
    ///Primary state of business.
    pub primary_state: Option<String>,
    #[serde(rename = "secondary_state")]
    ///Secondary state of business.
    pub secondary_state: Option<String>,
    #[serde(rename = "primary_state_id")]
    ///Primary state ID.
    pub primary_state_id: Option<String>,
    #[serde(rename = "secondary_state_id")]
    ///Secondary state ID.
    pub secondary_state_id: Option<String>,
    #[serde(rename = "primary_state_income_tax")]
    ///State income tax reported for primary state.
    pub primary_state_income_tax: Option<f64>,
    #[serde(rename = "secondary_state_income_tax")]
    ///State income tax reported for secondary state.
    pub secondary_state_income_tax: Option<f64>,
}
impl std::fmt::Display for Credit1099 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Form1099Type {
    #[serde(rename = "FORM_1099_TYPE_UNKNOWN")]
    Form1099TypeUnknown,
    #[serde(rename = "FORM_1099_TYPE_MISC")]
    Form1099TypeMisc,
    #[serde(rename = "FORM_1099_TYPE_K")]
    Form1099TypeK,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099Payer {
    #[serde(rename = "address")]
    ///Address on the pay stub.
    pub address: Option<CreditPayStubAddress>,
    #[serde(rename = "name")]
    ///Name of payer.
    pub name: Option<String>,
    #[serde(rename = "tin")]
    ///Tax identification number of payer.
    pub tin: Option<String>,
    #[serde(rename = "telephone_number")]
    ///Telephone number of payer.
    pub telephone_number: Option<String>,
}
impl std::fmt::Display for Credit1099Payer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099Recipient {
    #[serde(rename = "address")]
    ///Address on the pay stub.
    pub address: Option<CreditPayStubAddress>,
    #[serde(rename = "name")]
    ///Name of recipient.
    pub name: Option<String>,
    #[serde(rename = "tin")]
    ///Tax identification number of recipient.
    pub tin: Option<String>,
    #[serde(rename = "account_number")]
    ///Account number number of recipient.
    pub account_number: Option<String>,
    #[serde(rename = "facta_filing_requirement")]
    ///Checked if FACTA is a filing requirement.
    pub facta_filing_requirement: Option<String>,
    #[serde(rename = "second_tin_exists")]
    ///Checked if 2nd TIN exists.
    pub second_tin_exists: Option<String>,
}
impl std::fmt::Display for Credit1099Recipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099Filer {
    #[serde(rename = "address")]
    ///Address on the pay stub.
    pub address: Option<CreditPayStubAddress>,
    #[serde(rename = "name")]
    ///Name of filer.
    pub name: Option<String>,
    #[serde(rename = "tin")]
    ///Tax identification number of filer.
    pub tin: Option<String>,
    #[serde(rename = "type")]
    ///One of the following values will be provided: Payment Settlement Entity (PSE), Electronic Payment Fecilitator (EPF), Other Third Party
    pub type_: Option<String>,
}
impl std::fmt::Display for Credit1099Filer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStub {
    #[serde(rename = "deductions")]
    ///An object with the deduction information found on a pay stub.
    pub deductions: CreditPayStubDeductions,
    #[serde(rename = "document_id")]
    ///An identifier of the document referenced by the document metadata.
    pub document_id: Option<String>,
    #[serde(rename = "document_metadata")]
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    #[serde(rename = "earnings")]
    ///An object representing both a breakdown of earnings on a pay stub and the total earnings.
    pub earnings: CreditPayStubEarnings,
    #[serde(rename = "employee")]
    ///Data about the employee.
    pub employee: CreditPayStubEmployee,
    #[serde(rename = "employer")]
    ///Information about the employer on the pay stub.
    pub employer: CreditPayStubEmployer,
    #[serde(rename = "net_pay")]
    ///An object representing information about the net pay amount on the pay stub.
    pub net_pay: CreditPayStubNetPay,
    #[serde(rename = "pay_period_details")]
    ///Details about the pay period.
    pub pay_period_details: PayStubPayPeriodDetails,
    #[serde(rename = "verification")]
    ///(To be deprecated) Verification info will be moved to a separate endpoint in the future. An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.
    pub verification: Option<CreditPayStubVerification>,
}
impl std::fmt::Display for CreditPayStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubDeductions {
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<PayStubDeductionsBreakdown>,
    #[serde(rename = "total")]
    ///An object representing the total deductions for the pay period
    pub total: PayStubDeductionsTotal,
}
impl std::fmt::Display for CreditPayStubDeductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubDeductionsBreakdown {
    #[serde(rename = "current_amount")]
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    #[serde(rename = "description")]
    ///Description of the deduction line item
    pub description: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date amount of the deduction
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubDeductionsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubDeductionsTotal {
    #[serde(rename = "current_amount")]
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date total amount of the deductions
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubDeductionsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubEarnings {
    #[serde(rename = "breakdown")]
    pub breakdown: Vec<PayStubEarningsBreakdown>,
    #[serde(rename = "total")]
    ///An object representing both the current pay period and year to date amount for an earning category.
    pub total: PayStubEarningsTotal,
}
impl std::fmt::Display for CreditPayStubEarnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubEarningsBreakdown {
    #[serde(rename = "canonical_description")]
    ///Commonly used term to describe the earning line item.
    pub canonical_description: Option<String>,
    #[serde(rename = "current_amount")]
    ///Raw amount of the earning line item.
    pub current_amount: Option<f64>,
    #[serde(rename = "description")]
    ///Description of the earning line item.
    pub description: Option<String>,
    #[serde(rename = "hours")]
    ///Number of hours applicable for this earning.
    pub hours: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "rate")]
    ///Hourly rate applicable for this earning.
    pub rate: Option<f64>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date amount of the deduction.
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubEarningsBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubEarningsBreakdownCanonicalDescription(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubEarningsTotal {
    #[serde(rename = "current_amount")]
    ///Total amount of the earnings for this pay period.
    pub current_amount: Option<f64>,
    #[serde(rename = "hours")]
    ///Total number of hours worked for this pay period.
    pub hours: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The total year-to-date amount of the earnings.
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for PayStubEarningsTotal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubEmployee {
    #[serde(rename = "address")]
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    #[serde(rename = "name")]
    ///The name of the employee.
    pub name: Option<String>,
    #[serde(rename = "marital_status")]
    ///Marital status of the employee - either `SINGLE` or `MARRIED`.
    pub marital_status: Option<String>,
    #[serde(rename = "taxpayer_id")]
    ///Taxpayer ID of the individual receiving the paystub.
    pub taxpayer_id: PayStubTaxpayerId,
}
impl std::fmt::Display for CreditPayStubEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubAddress {
    #[serde(rename = "city")]
    ///The full city name.
    pub city: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code of the address.
    pub postal_code: Option<String>,
    #[serde(rename = "region")]
    /**The region or state.
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    ///The full street address.
    pub street: Option<String>,
}
impl std::fmt::Display for CreditPayStubAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubTaxpayerId {
    #[serde(rename = "id_type")]
    ///Type of ID, e.g. 'SSN'.
    pub id_type: Option<String>,
    #[serde(rename = "id_mask")]
    ///ID mask; i.e. last 4 digits of the taxpayer ID.
    pub id_mask: Option<String>,
}
impl std::fmt::Display for PayStubTaxpayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubEmployer {
    #[serde(rename = "address")]
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    #[serde(rename = "name")]
    ///The name of the employer on the pay stub.
    pub name: Option<String>,
}
impl std::fmt::Display for CreditPayStubEmployer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubNetPay {
    #[serde(rename = "current_amount")]
    ///Raw amount of the net pay for the pay period.
    pub current_amount: Option<f64>,
    #[serde(rename = "description")]
    ///Description of the net pay.
    pub description: Option<String>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    #[serde(rename = "ytd_amount")]
    ///The year-to-date amount of the net pay.
    pub ytd_amount: Option<f64>,
}
impl std::fmt::Display for CreditPayStubNetPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubPayPeriodDetails {
    #[serde(rename = "pay_amount")]
    ///The amount of the paycheck.
    pub pay_amount: Option<f64>,
    #[serde(rename = "distribution_breakdown")]
    pub distribution_breakdown: Vec<PayStubDistributionBreakdown>,
    #[serde(rename = "end_date")]
    ///The date on which the pay period ended, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub end_date: Option<String>,
    #[serde(rename = "gross_earnings")]
    ///Total earnings before tax/deductions.
    pub gross_earnings: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "pay_date")]
    ///The date on which the pay stub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_date: Option<String>,
    #[serde(rename = "pay_frequency")]
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
    #[serde(rename = "start_date")]
    ///The date on which the pay period started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub start_date: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for PayStubPayPeriodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubDistributionBreakdown {
    #[serde(rename = "account_name")]
    ///Name of the account for the given distribution.
    pub account_name: Option<String>,
    #[serde(rename = "bank_name")]
    ///The name of the bank that the payment is being deposited to.
    pub bank_name: Option<String>,
    #[serde(rename = "current_amount")]
    ///The amount distributed to this account.
    pub current_amount: Option<f64>,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "mask")]
    ///The last 2-4 alphanumeric characters of an account's official account number.
    pub mask: Option<String>,
    #[serde(rename = "type")]
    ///Type of the account that the paystub was sent to (e.g. 'checking').
    pub type_: Option<String>,
    #[serde(rename = "unofficial_currency_code")]
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
impl std::fmt::Display for PayStubDistributionBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubVerification {
    #[serde(rename = "verification_status")]
    ///Derived verification status.
    pub verification_status: Option<String>,
    #[serde(rename = "verification_attributes")]
    pub verification_attributes: Vec<PayStubVerificationAttribute>,
}
impl std::fmt::Display for CreditPayStubVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubVerificationStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubVerificationAttribute {
    #[serde(rename = "type")]
    ///Message indicating the reason as to why the verification failed.
    pub type_: Option<String>,
}
impl std::fmt::Display for PayStubVerificationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportToken {
    #[serde(rename = "report_type")]
    ///The report type. It can be `assets` or `income`.
    pub report_type: Option<String>,
    #[serde(rename = "token")]
    ///The report token. It can be an asset report token or an income report token.
    pub token: Option<String>,
}
impl std::fmt::Display for ReportToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ReportType {
    #[serde(rename = "assets")]
    Assets,
    #[serde(rename = "income")]
    Income,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRiskSignal {
    #[serde(rename = "type")]
    ///The result from the risk signal check.
    pub type_: Option<String>,
    #[serde(rename = "field")]
    ///The field which the risk signal was computed for
    pub field: Option<String>,
    #[serde(rename = "has_fraud_risk")]
    ///A flag used to quickly identify if the signal indicates that this field is authentic or fraudulent
    pub has_fraud_risk: Option<bool>,
    #[serde(rename = "institution_metadata")]
    ///An object which contains additional metadata about the institution used to compute the verification attribute
    pub institution_metadata: Option<DocumentRiskSignalInstitutionMetadata>,
    #[serde(rename = "expected_value")]
    ///The expected value of the field, as seen on the document
    pub expected_value: Option<String>,
    #[serde(rename = "actual_value")]
    ///The derived value obtained in the risk signal calculation process for this field
    pub actual_value: Option<String>,
    #[serde(rename = "signal_description")]
    ///A human-readable explanation providing more detail into the particular risk signal
    pub signal_description: Option<String>,
}
impl std::fmt::Display for DocumentRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRiskSignalInstitutionMetadata {
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
}
impl std::fmt::Display for DocumentRiskSignalInstitutionMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollItemStatus {
    #[serde(rename = "processing_status")]
    /**Denotes the processing status for the verification.

`UNKNOWN`: The processing status could not be determined.

`PROCESSING_COMPLETE`: The processing has completed and the user has approved for sharing. The data is available to be retrieved.

`PROCESSING`: The verification is still processing. The data is not available yet.

`FAILED`: The processing failed to complete successfully.

`APPROVAL_STATUS_PENDING`: The processing has completed but the user has not yet approved the sharing of the data.*/
    pub processing_status: Option<String>,
}
impl std::fmt::Display for PayrollItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditW2 {
    #[serde(rename = "document_metadata")]
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    #[serde(rename = "document_id")]
    ///An identifier of the document referenced by the document metadata.
    pub document_id: String,
    #[serde(rename = "employer")]
    ///Information about the employer on the pay stub.
    pub employer: CreditPayStubEmployer,
    #[serde(rename = "employee")]
    ///Data about the employee.
    pub employee: CreditPayStubEmployee,
    #[serde(rename = "tax_year")]
    ///The tax year of the W2 document.
    pub tax_year: Option<String>,
    #[serde(rename = "employer_id_number")]
    ///An employee identification number or EIN.
    pub employer_id_number: Option<String>,
    #[serde(rename = "wages_tips_other_comp")]
    ///Wages from tips and other compensation.
    pub wages_tips_other_comp: Option<String>,
    #[serde(rename = "federal_income_tax_withheld")]
    ///Federal income tax withheld for the tax year.
    pub federal_income_tax_withheld: Option<String>,
    #[serde(rename = "social_security_wages")]
    ///Wages from social security.
    pub social_security_wages: Option<String>,
    #[serde(rename = "social_security_tax_withheld")]
    ///Social security tax withheld for the tax year.
    pub social_security_tax_withheld: Option<String>,
    #[serde(rename = "medicare_wages_and_tips")]
    ///Wages and tips from medicare.
    pub medicare_wages_and_tips: Option<String>,
    #[serde(rename = "medicare_tax_withheld")]
    ///Medicare tax withheld for the tax year.
    pub medicare_tax_withheld: Option<String>,
    #[serde(rename = "social_security_tips")]
    ///Tips from social security.
    pub social_security_tips: Option<String>,
    #[serde(rename = "allocated_tips")]
    ///Allocated tips.
    pub allocated_tips: Option<String>,
    #[serde(rename = "box_9")]
    ///Contents from box 9 on the W2.
    pub box_9: Option<String>,
    #[serde(rename = "dependent_care_benefits")]
    ///Dependent care benefits.
    pub dependent_care_benefits: Option<String>,
    #[serde(rename = "nonqualified_plans")]
    ///Nonqualified plans.
    pub nonqualified_plans: Option<String>,
    #[serde(rename = "box_12")]
    pub box_12: Vec<W2Box12>,
    #[serde(rename = "statutory_employee")]
    ///Statutory employee.
    pub statutory_employee: Option<String>,
    #[serde(rename = "retirement_plan")]
    ///Retirement plan.
    pub retirement_plan: Option<String>,
    #[serde(rename = "third_party_sick_pay")]
    ///Third party sick pay.
    pub third_party_sick_pay: Option<String>,
    #[serde(rename = "other")]
    ///Other.
    pub other: Option<String>,
    #[serde(rename = "state_and_local_wages")]
    pub state_and_local_wages: Vec<W2StateAndLocalWages>,
}
impl std::fmt::Display for CreditW2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollIncomeRateOfPay {
    #[serde(rename = "pay_rate")]
    ///The rate at which an employee is paid.
    pub pay_rate: Option<String>,
    #[serde(rename = "pay_amount")]
    ///The amount at which an employee is paid.
    pub pay_amount: Option<f64>,
}
impl std::fmt::Display for PayrollIncomeRateOfPay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: Option<String>,
    #[serde(rename = "access_tokens")]
    ///An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product.
    pub access_tokens: Option<Vec<String>>,
    #[serde(rename = "employer")]
    ///Information about the end user's employer
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    #[serde(rename = "us_military_info")]
    ///Data about military info in the income verification precheck.
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
}
impl std::fmt::Display for CreditPayrollIncomePrecheckRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "confidence")]
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: String,
}
impl std::fmt::Display for CreditPayrollIncomePrecheckResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: Option<String>,
}
impl std::fmt::Display for CreditPayrollIncomeRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "verification_refresh_status")]
    /**The verification refresh status. One of the following:

`"USER_PRESENCE_REQUIRED"` User presence is required to refresh an income verification.
`"SUCCESSFUL"` The income verification refresh was successful.
`"NOT_FOUND"` No new data was found after the income verification refresh.*/
    pub verification_refresh_status: String,
}
impl std::fmt::Display for CreditPayrollIncomeRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentGetRequest {
    #[serde(rename = "user_token")]
    ///The user token associated with the User data is being requested for.
    pub user_token: String,
}
impl std::fmt::Display for CreditEmploymentGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentGetResponse {
    #[serde(rename = "items")]
    ///Array of employment items.
    pub items: Vec<CreditEmploymentItem>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditEmploymentGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentItem {
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: String,
    #[serde(rename = "employments")]
    pub employments: Vec<CreditEmploymentVerification>,
    #[serde(rename = "pull_id")]
    ///A reference id to reference what payroll data was returned from this endpoint
    pub pull_id: String,
}
impl std::fmt::Display for CreditEmploymentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentVerification {
    #[serde(rename = "account_id")]
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    #[serde(rename = "status")]
    ///Current employment status.
    pub status: Option<String>,
    #[serde(rename = "start_date")]
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    pub end_date: Option<String>,
    #[serde(rename = "employer")]
    ///An object containing employer data.
    pub employer: CreditEmployerVerification,
    #[serde(rename = "title")]
    ///Current title of employee.
    pub title: Option<String>,
    #[serde(rename = "platform_ids")]
    ///The object containing a set of ids related to an employee.
    pub platform_ids: CreditPlatformIds,
    #[serde(rename = "employee_type")]
    /**The type of employment for the individual.
`"FULL_TIME"`: A full-time employee.
`"PART_TIME"`: A part-time employee.
`"CONTRACTOR"`: An employee typically hired externally through a contracting group.
`"TEMPORARY"`: A temporary employee.
`"OTHER"`: The employee type is not one of the above defined types.*/
    pub employee_type: Option<String>,
    #[serde(rename = "last_paystub_date")]
    ///The date of the employee's most recent paystub in ISO 8601 format (YYYY-MM-DD).
    pub last_paystub_date: Option<String>,
}
impl std::fmt::Display for CreditEmploymentVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentEmployeeType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentVerificationStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmployerVerification {
    #[serde(rename = "name")]
    ///Name of employer.
    pub name: Option<String>,
}
impl std::fmt::Display for CreditEmployerVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPlatformIds {
    #[serde(rename = "employee_id")]
    ///The ID of an employee as given by their employer.
    pub employee_id: Option<String>,
    #[serde(rename = "payroll_id")]
    ///The ID of an employee as given by their payroll.
    pub payroll_id: Option<String>,
    #[serde(rename = "position_id")]
    ///The ID of the position of the employee.
    pub position_id: Option<String>,
}
impl std::fmt::Display for CreditPlatformIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeWarning {
    #[serde(rename = "warning_type")]
    ///The warning type which will always be `BANK_INCOME_WARNING`.
    pub warning_type: Option<String>,
    #[serde(rename = "warning_code")]
    /**The warning code identifies a specific kind of warning.
`IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item
`TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item
`ITEM_UNAPPROVED`: User did not grant permission to share income data for the Item
`REPORT_DELETED`: Report deleted due to customer or consumer request*/
    pub warning_code: Option<String>,
    #[serde(rename = "cause")]
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: Option<CreditBankIncomeCause>,
}
impl std::fmt::Display for CreditBankIncomeWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningType {
    #[serde(rename = "BANK_INCOME_WARNING")]
    BankIncomeWarning,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningCode {
    #[serde(rename = "IDENTITY_UNAVAILABLE")]
    IdentityUnavailable,
    #[serde(rename = "TRANSACTIONS_UNAVAILABLE")]
    TransactionsUnavailable,
    #[serde(rename = "ITEM_UNAPPROVED")]
    ItemUnapproved,
    #[serde(rename = "REPORT_DELETED")]
    ReportDeleted,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeCause {
    #[serde(rename = "error_type")]
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: Option<String>,
    #[serde(rename = "error_code")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. Error fields will be `null` if no error has occurred.
    pub error_code: Option<String>,
    #[serde(rename = "error_message")]
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: Option<String>,
    #[serde(rename = "display_message")]
    /**A user-friendly representation of the error code. null if the error is not related to user action.
This may change over time and is not safe for programmatic use.*/
    pub display_message: Option<String>,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this warning.
    pub item_id: Option<String>,
}
impl std::fmt::Display for CreditBankIncomeCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeErrorType {
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[serde(rename = "INSUFFICIENT_CREDENTIALS")]
    InsufficientCredentials,
    #[serde(rename = "ITEM_LOCKED")]
    ItemLocked,
    #[serde(rename = "USER_SETUP_REQUIRED")]
    UserSetupRequired,
    #[serde(rename = "COUNTRY_NOT_SUPPORTED")]
    CountryNotSupported,
    #[serde(rename = "INSTITUTION_DOWN")]
    InstitutionDown,
    #[serde(rename = "INSTITUTION_NO_LONGER_SUPPORTED")]
    InstitutionNoLongerSupported,
    #[serde(rename = "INSTITUTION_NOT_RESPONDING")]
    InstitutionNotResponding,
    #[serde(rename = "INVALID_CREDENTIALS")]
    InvalidCredentials,
    #[serde(rename = "INVALID_MFA")]
    InvalidMfa,
    #[serde(rename = "INVALID_SEND_METHOD")]
    InvalidSendMethod,
    #[serde(rename = "ITEM_LOGIN_REQUIRED")]
    ItemLoginRequired,
    #[serde(rename = "MFA_NOT_SUPPORTED")]
    MfaNotSupported,
    #[serde(rename = "NO_ACCOUNTS")]
    NoAccounts,
    #[serde(rename = "ITEM_NOT_SUPPORTED")]
    ItemNotSupported,
    #[serde(rename = "ACCESS_NOT_GRANTED")]
    AccessNotGranted,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayCreateRequest {
    #[serde(rename = "report_tokens")]
    ///List of report tokens, with at most one token of each report type. Currently only Asset Report token is supported.
    pub report_tokens: Vec<ReportToken>,
    #[serde(rename = "secondary_client_id")]
    ///The `secondary_client_id` is the client id of the third party with whom you would like to share the Relay Token.
    pub secondary_client_id: String,
    #[serde(rename = "webhook")]
    ///URL to which Plaid will send webhooks when the Secondary Client successfully retrieves an Asset Report by calling `/credit/relay/get`.
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditRelayCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayCreateResponse {
    #[serde(rename = "relay_token")]
    ///A token that can be shared with a third party to allow them to access the Asset Report. This token should be stored securely.
    pub relay_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayGetRequest {
    #[serde(rename = "relay_token")]
    ///The `relay_token` granting access to the report you would like to get.
    pub relay_token: String,
    #[serde(rename = "report_type")]
    ///The report type. It can be `assets` or `income`.
    pub report_type: String,
}
impl std::fmt::Display for CreditRelayGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRefreshRequest {
    #[serde(rename = "relay_token")]
    ///The `relay_token` granting access to the report you would like to refresh.
    pub relay_token: String,
    #[serde(rename = "report_type")]
    ///The report type. It can be `assets` or `income`.
    pub report_type: String,
    #[serde(rename = "webhook")]
    ///The URL registered to receive webhooks when the report of a Relay Token has been refreshed.
    pub webhook: Option<String>,
}
impl std::fmt::Display for CreditRelayRefreshRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRefreshResponse {
    #[serde(rename = "relay_token")]
    pub relay_token: String,
    #[serde(rename = "asset_report_id")]
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayRefreshResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRemoveRequest {
    #[serde(rename = "relay_token")]
    ///The `relay_token` you would like to revoke.
    pub relay_token: String,
}
impl std::fmt::Display for CreditRelayRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRemoveResponse {
    #[serde(rename = "removed")]
    ///`true` if the Relay token was successfully removed.
    pub removed: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CreditRelayRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookRequest {
    #[serde(rename = "webhook")]
    ///The URL to which the webhook should be sent.
    pub webhook: String,
}
impl std::fmt::Display for SandboxBankTransferFireWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxBankTransferFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferFireWebhookRequest {
    #[serde(rename = "webhook")]
    ///The URL to which the webhook should be sent.
    pub webhook: String,
}
impl std::fmt::Display for SandboxTransferFireWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferFireWebhookResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxTransferFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "application_id")]
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: String,
    #[serde(rename = "name")]
    ///The name of the application
    pub name: String,
    #[serde(rename = "display_name")]
    ///A human-readable name of the application for display purposes
    pub display_name: Option<String>,
    #[serde(rename = "join_date")]
    ///The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub join_date: String,
    #[serde(rename = "logo_url")]
    ///A URL that links to the application logo image.
    pub logo_url: Option<String>,
    #[serde(rename = "application_url")]
    ///The URL for the application's website
    pub application_url: Option<String>,
    #[serde(rename = "reason_for_access")]
    ///A string provided by the connected app stating why they use their respective enabled products.
    pub reason_for_access: Option<String>,
    #[serde(rename = "use_case")]
    ///A string representing client’s broad use case as assessed by Plaid.
    pub use_case: Option<String>,
    #[serde(rename = "company_legal_name")]
    ///A string representing the name of client’s legal entity.
    pub company_legal_name: Option<String>,
    #[serde(rename = "city")]
    ///A string representing the city of the client’s headquarters.
    pub city: Option<String>,
    #[serde(rename = "region")]
    ///A string representing the region of the client’s headquarters.
    pub region: Option<String>,
    #[serde(rename = "postal_code")]
    ///A string representing the postal code of the client’s headquarters.
    pub postal_code: Option<String>,
    #[serde(rename = "country_code")]
    ///A string representing the country code of the client’s headquarters.
    pub country_code: Option<String>,
}
impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationGetRequest {
    #[serde(rename = "application_id")]
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: String,
}
impl std::fmt::Display for ApplicationGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationGetResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "application")]
    ///Metadata about the application
    pub application: Application,
}
impl std::fmt::Display for ApplicationGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductAccess {
    #[serde(rename = "statements")]
    ///Allow access to statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub statements: Option<bool>,
    #[serde(rename = "identity")]
    ///Allow access to the Identity product (name, email, phone, address). Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub identity: Option<bool>,
    #[serde(rename = "auth")]
    ///Allow access to account number details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub auth: Option<bool>,
    #[serde(rename = "transactions")]
    ///Allow access to transaction details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub transactions: Option<bool>,
    #[serde(rename = "accounts_details_transactions")]
    ///Allow access to "accounts_details_transactions". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_details_transactions: Option<bool>,
    #[serde(rename = "accounts_routing_number")]
    ///Allow access to "accounts_routing_number". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_routing_number: Option<bool>,
    #[serde(rename = "accounts_statements")]
    ///Allow access to "accounts_statements". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_statements: Option<bool>,
    #[serde(rename = "accounts_tax_statements")]
    ///Allow access to "accounts_tax_statements". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_tax_statements: Option<bool>,
    #[serde(rename = "customers_profiles")]
    ///Allow access to "customers_profiles". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub customers_profiles: Option<bool>,
}
impl std::fmt::Display for ProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAccess {
    #[serde(rename = "unique_id")]
    ///The unique account identifier for this account. This value must match that returned by the data access API for this account.
    pub unique_id: String,
    #[serde(rename = "authorized")]
    ///Allow the application to see this account (and associated details, including balance) in the list of accounts  If unset, defaults to `true`.
    pub authorized: Option<bool>,
    #[serde(rename = "account_product_access")]
    ///Allow the application to access specific products on this account
    pub account_product_access: Option<AccountProductAccessNullable>,
}
impl std::fmt::Display for AccountAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProductAccessNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProductAccess {
    #[serde(rename = "account_data")]
    ///Allow the application to access account data. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub account_data: Option<bool>,
    #[serde(rename = "statements")]
    ///Allow the application to access bank statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub statements: Option<bool>,
    #[serde(rename = "tax_documents")]
    ///Allow the application to access tax documents. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub tax_documents: Option<bool>,
}
impl std::fmt::Display for AccountProductAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Scopes {
    #[serde(rename = "product_access")]
    ///The product access being requested. Used to or disallow product access across all accounts. If unset, defaults to all products allowed.
    pub product_access: Option<ProductAccess>,
    #[serde(rename = "accounts")]
    pub accounts: Option<Vec<AccountAccess>>,
    #[serde(rename = "new_accounts")]
    ///Allow access to newly opened accounts as they are opened. If unset, defaults to `true`.
    pub new_accounts: Option<bool>,
}
impl std::fmt::Display for Scopes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesState(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum ScopesContext {
    #[serde(rename = "ENROLLMENT")]
    Enrollment,
    #[serde(rename = "PORTAL")]
    Portal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "application_id")]
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: String,
    #[serde(rename = "scopes")]
    ///The scopes object
    pub scopes: Scopes,
    #[serde(rename = "state")]
    ///When scopes are updated during enrollment, this field must be populated with the state sent to the partner in the OAuth Login URI. This field is required when the context is `ENROLLMENT`.
    pub state: Option<String>,
    #[serde(rename = "context")]
    ///An indicator for when scopes are being updated. When scopes are updated via enrollment (i.e. OAuth), the partner must send `ENROLLMENT`. When scopes are updated in a post-enrollment view, the partner must send `PORTAL`.
    pub context: String,
}
impl std::fmt::Display for ItemApplicationScopesUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemApplicationScopesUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<String>,
}
impl std::fmt::Display for ItemApplicationListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<String>,
    #[serde(rename = "applications")]
    ///A list of connected applications.
    pub applications: Vec<ConnectedApplication>,
}
impl std::fmt::Display for ItemApplicationListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedApplication {
    #[serde(rename = "application_id")]
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: String,
    #[serde(rename = "name")]
    ///The name of the application
    pub name: String,
    #[serde(rename = "display_name")]
    ///A human-readable name of the application for display purposes
    pub display_name: Option<String>,
    #[serde(rename = "logo_url")]
    ///A URL that links to the application logo image.
    pub logo_url: Option<String>,
    #[serde(rename = "application_url")]
    ///The URL for the application's website
    pub application_url: Option<String>,
    #[serde(rename = "reason_for_access")]
    ///A string provided by the connected app stating why they use their respective enabled products.
    pub reason_for_access: Option<String>,
    #[serde(rename = "created_at")]
    ///The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub created_at: String,
    #[serde(rename = "scopes")]
    ///The scopes object
    pub scopes: Option<ScopesNullable>,
}
impl std::fmt::Display for ConnectedApplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountSelectionCardinality {
    #[serde(rename = "SINGLE_SELECT")]
    SingleSelect,
    #[serde(rename = "MULTI_SELECT")]
    MultiSelect,
    #[serde(rename = "ALL")]
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFilter {
    #[serde(rename = "depository")]
    ///A list of account subtypes to be filtered.
    pub depository: Option<AccountFilterSubtypes>,
    #[serde(rename = "credit")]
    ///A list of account subtypes to be filtered.
    pub credit: Option<AccountFilterSubtypes>,
    #[serde(rename = "loan")]
    ///A list of account subtypes to be filtered.
    pub loan: Option<AccountFilterSubtypes>,
    #[serde(rename = "investment")]
    ///A list of account subtypes to be filtered.
    pub investment: Option<AccountFilterSubtypes>,
}
impl std::fmt::Display for AccountFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFilterSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookRequest {
    #[serde(rename = "item_id")]
    ///The Item ID associated with the verification.
    pub item_id: String,
    #[serde(rename = "user_id")]
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: Option<String>,
    #[serde(rename = "webhook")]
    ///The URL to which the webhook should be sent.
    pub webhook: String,
    #[serde(rename = "verification_status")]
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.*/
    pub verification_status: String,
}
impl std::fmt::Display for SandboxIncomeFireWebhookRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SandboxIncomeFireWebhookResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListUserAuth {
    #[serde(rename = "user_id")]
    ///Account username.
    pub user_id: Option<String>,
    #[serde(rename = "fi_username_hash")]
    ///Account username hashed by FI.
    pub fi_username_hash: Option<String>,
}
impl std::fmt::Display for ItemApplicationListUserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "account_id")]
    ///The `account_id` of the account whose verification status is to be modified
    pub account_id: String,
    #[serde(rename = "client_transaction_id")]
    ///The unique ID that you would like to use to refer to this transaction. For your convenience mapping your internal data, you could use your internal ID/identifier for this transaction. The max length for this field is 36 characters.
    pub client_transaction_id: String,
    #[serde(rename = "amount")]
    ///The transaction amount, in USD (e.g. `102.05`)
    pub amount: f64,
    #[serde(rename = "user_present")]
    ///`true` if the end user is present while initiating the ACH transfer and the endpoint is being called; `false` otherwise (for example, when the ACH transfer is scheduled and the end user is not present, or you call this endpoint after the ACH transfer but before submitting the Nacha file for ACH processing).
    pub user_present: Option<bool>,
    #[serde(rename = "client_user_id")]
    ///A unique ID that identifies the end user in your system. This ID is used to correlate requests by a user with multiple Items. The max length for this field is 36 characters.
    pub client_user_id: Option<String>,
    #[serde(rename = "user")]
    ///Details about the end user initiating the transaction (i.e., the account holder).
    pub user: Option<SignalUser>,
    #[serde(rename = "device")]
    ///Details about the end user's device
    pub device: Option<SignalDevice>,
}
impl std::fmt::Display for SignalEvaluateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalUser {
    #[serde(rename = "name")]
    ///The user's legal name
    pub name: Option<SignalPersonName>,
    #[serde(rename = "phone_number")]
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567"
    pub phone_number: Option<String>,
    #[serde(rename = "email_address")]
    ///The user's email address.
    pub email_address: Option<String>,
    #[serde(rename = "address")]
    ///Data about the components comprising an address.
    pub address: Option<SignalAddressData>,
}
impl std::fmt::Display for SignalUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPersonName {
    #[serde(rename = "prefix")]
    ///The user's name prefix (e.g. "Mr.")
    pub prefix: Option<String>,
    #[serde(rename = "given_name")]
    ///The user's given name. If the user has a one-word name, it should be provided in this field.
    pub given_name: Option<String>,
    #[serde(rename = "middle_name")]
    ///The user's middle name
    pub middle_name: Option<String>,
    #[serde(rename = "family_name")]
    ///The user's family name / surname
    pub family_name: Option<String>,
    #[serde(rename = "suffix")]
    ///The user's name suffix (e.g. "II")
    pub suffix: Option<String>,
}
impl std::fmt::Display for SignalPersonName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalAddressData {
    #[serde(rename = "city")]
    ///The full city name
    pub city: Option<String>,
    #[serde(rename = "region")]
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    #[serde(rename = "street")]
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    #[serde(rename = "postal_code")]
    ///The postal code
    pub postal_code: Option<String>,
    #[serde(rename = "country")]
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
impl std::fmt::Display for SignalAddressData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDevice {
    #[serde(rename = "ip_address")]
    ///The IP address of the device that initiated the transaction
    pub ip_address: Option<String>,
    #[serde(rename = "user_agent")]
    ///The user agent of the device that initiated the transaction (e.g. "Mozilla/5.0")
    pub user_agent: Option<String>,
}
impl std::fmt::Display for SignalDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    #[serde(rename = "scores")]
    ///Risk scoring details broken down by risk category.
    pub scores: SignalScores,
    #[serde(rename = "core_attributes")]
    /**The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:

`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
`is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account

For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
    pub core_attributes: Option<SignalEvaluateCoreAttributes>,
}
impl std::fmt::Display for SignalEvaluateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalScores {
    #[serde(rename = "customer_initiated_return_risk")]
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk of an unauthorized debit. Common return codes in this category include: "R05", "R07", "R10", "R11", "R29". These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.
    pub customer_initiated_return_risk: Option<CustomerInitiatedReturnRisk>,
    #[serde(rename = "bank_initiated_return_risk")]
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk because an account is overdrawn or because an ineligible account is used. Common return codes in this category include: "R01", "R02", "R03", "R04", "R06", "R08",  "R09", "R13", "R16", "R17", "R20", "R23". These returns have a turnaround time of 2 banking days.
    pub bank_initiated_return_risk: Option<BankInitiatedReturnRisk>,
}
impl std::fmt::Display for SignalScores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalScore(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInitiatedRiskTier(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInitiatedReturnRisk {
    #[serde(rename = "score")]
    ///A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: i64,
    #[serde(rename = "risk_tier")]
    /**A tier corresponding to the projected likelihood that the transaction, if initiated, will be subject to a return.

In the `customer_initiated_return_risk` object, there are five risk tiers corresponding to the scores:
  1: Predicted customer-initiated return incidence rate between 0.00% - 0.02%
  2: Predicted customer-initiated return incidence rate between 0.02% - 0.05%
  3: Predicted customer-initiated return incidence rate between 0.05% - 0.1%
  4: Predicted customer-initiated return incidence rate between 0.1% - 0.5%
  5: Predicted customer-initiated return incidence rate greater than 0.5%
*/
    pub risk_tier: i64,
}
impl std::fmt::Display for CustomerInitiatedReturnRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankInitiatedRiskTier(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankInitiatedReturnRisk {
    #[serde(rename = "score")]
    ///A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: i64,
    #[serde(rename = "risk_tier")]
    /**In the `bank_initiated_return_risk` object, there are eight risk tiers corresponding to the scores:
  1: Predicted bank-initiated return incidence rate between 0.0% - 0.5%
  2: Predicted bank-initiated return incidence rate between 0.5% - 1.5%
  3: Predicted bank-initiated return incidence rate between 1.5% - 3%
  4: Predicted bank-initiated return incidence rate between 3% - 5%
  5: Predicted bank-initiated return incidence rate between 5% - 10%
  6: Predicted bank-initiated return incidence rate between 10% - 15%
  7: Predicted bank-initiated return incidence rate between 15% and 50%
  8: Predicted bank-initiated return incidence rate greater than 50%
*/
    pub risk_tier: i64,
}
impl std::fmt::Display for BankInitiatedReturnRisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateCoreAttributes {
    #[serde(rename = "unauthorized_transactions_count_7d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited.
    pub unauthorized_transactions_count_7_d: Option<i64>,
    #[serde(rename = "unauthorized_transactions_count_30d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited.
    pub unauthorized_transactions_count_30_d: Option<i64>,
    #[serde(rename = "unauthorized_transactions_count_60d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited.
    pub unauthorized_transactions_count_60_d: Option<i64>,
    #[serde(rename = "unauthorized_transactions_count_90d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited.
    pub unauthorized_transactions_count_90_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_7d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_7_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_30d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_30_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_60d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_60_d: Option<i64>,
    #[serde(rename = "nsf_overdraft_transactions_count_90d")]
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_90_d: Option<i64>,
    #[serde(rename = "days_since_first_plaid_connection")]
    ///The number of days since the first time the Item was connected to an application via Plaid
    pub days_since_first_plaid_connection: Option<i64>,
    #[serde(rename = "plaid_connections_count_7d")]
    ///The number of times the Item has been connected to applications via Plaid over the past 7 days
    pub plaid_connections_count_7_d: Option<i64>,
    #[serde(rename = "plaid_connections_count_30d")]
    ///The number of times the Item has been connected to applications via Plaid over the past 30 days
    pub plaid_connections_count_30_d: Option<i64>,
    #[serde(rename = "total_plaid_connections_count")]
    ///The total number of times the Item has been connected to applications via Plaid
    pub total_plaid_connections_count: Option<i64>,
    #[serde(rename = "is_savings_or_money_market_account")]
    ///Indicates if the ACH transaction funding account is a savings/money market account
    pub is_savings_or_money_market_account: Option<bool>,
    #[serde(rename = "total_credit_transactions_amount_10d")]
    ///The total credit (inflow) transaction amount over the past 10 days from the account that will be debited
    pub total_credit_transactions_amount_10_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_10d")]
    ///The total debit (outflow) transaction amount over the past 10 days from the account that will be debited
    pub total_debit_transactions_amount_10_d: Option<f64>,
    #[serde(rename = "p50_credit_transactions_amount_28d")]
    ///The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_50_credit_transactions_amount_28_d: Option<f64>,
    #[serde(rename = "p50_debit_transactions_amount_28d")]
    ///The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_50_debit_transactions_amount_28_d: Option<f64>,
    #[serde(rename = "p95_credit_transactions_amount_28d")]
    ///The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_95_credit_transactions_amount_28_d: Option<f64>,
    #[serde(rename = "p95_debit_transactions_amount_28d")]
    ///The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_95_debit_transactions_amount_28_d: Option<f64>,
    #[serde(rename = "days_with_negative_balance_count_90d")]
    ///The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance
    pub days_with_negative_balance_count_90_d: Option<i64>,
    #[serde(rename = "p90_eod_balance_30d")]
    ///The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p_90_eod_balance_30_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_60d")]
    ///The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p_90_eod_balance_60_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_90d")]
    ///The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p_90_eod_balance_90_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_30d")]
    ///The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p_10_eod_balance_30_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_60d")]
    ///The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p_10_eod_balance_60_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_90d")]
    ///The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p_10_eod_balance_90_d: Option<f64>,
    #[serde(rename = "available_balance")]
    ///Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.
    pub available_balance: Option<f64>,
    #[serde(rename = "current_balance")]
    ///Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account.
    pub current_balance: Option<f64>,
    #[serde(rename = "balance_last_updated")]
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated.
    pub balance_last_updated: Option<String>,
    #[serde(rename = "phone_change_count_28d")]
    ///The number of times the account's phone numbers on file have changed over the past 28 days
    pub phone_change_count_28_d: Option<i64>,
    #[serde(rename = "phone_change_count_90d")]
    ///The number of times the account's phone numbers on file have changed over the past 90 days
    pub phone_change_count_90_d: Option<i64>,
    #[serde(rename = "email_change_count_28d")]
    ///The number of times the account's email addresses on file have changed over the past 28 days
    pub email_change_count_28_d: Option<i64>,
    #[serde(rename = "email_change_count_90d")]
    ///The number of times the account's email addresses on file have changed over the past 90 days
    pub email_change_count_90_d: Option<i64>,
    #[serde(rename = "address_change_count_28d")]
    ///The number of times the account's addresses on file have changed over the past 28 days
    pub address_change_count_28_d: Option<i64>,
    #[serde(rename = "address_change_count_90d")]
    ///The number of times the account's addresses on file have changed over the past 90 days
    pub address_change_count_90_d: Option<i64>,
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_3d")]
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    pub plaid_non_oauth_authentication_attempts_count_3_d: Option<i64>,
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_7d")]
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    pub plaid_non_oauth_authentication_attempts_count_7_d: Option<i64>,
    #[serde(rename = "plaid_non_oauth_authentication_attempts_count_30d")]
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    pub plaid_non_oauth_authentication_attempts_count_30_d: Option<i64>,
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_3d")]
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    pub failed_plaid_non_oauth_authentication_attempts_count_3_d: Option<i64>,
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_7d")]
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    pub failed_plaid_non_oauth_authentication_attempts_count_7_d: Option<i64>,
    #[serde(rename = "failed_plaid_non_oauth_authentication_attempts_count_30d")]
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    pub failed_plaid_non_oauth_authentication_attempts_count_30_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_10d")]
    ///The total number of debit (outflow) transactions over the past 10 days from the account that will be debited
    pub debit_transactions_count_10_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_10d")]
    ///The total number of credit (inflow) transactions over the past 10 days from the account that will be debited
    pub credit_transactions_count_10_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_30d")]
    ///The total number of debit (outflow) transactions over the past 30 days from the account that will be debited
    pub debit_transactions_count_30_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_30d")]
    ///The total number of credit (inflow) transactions over the past 30 days from the account that will be debited
    pub credit_transactions_count_30_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_60d")]
    ///The total number of debit (outflow) transactions over the past 60 days from the account that will be debited
    pub debit_transactions_count_60_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_60d")]
    ///The total number of credit (inflow) transactions over the past 60 days from the account that will be debited
    pub credit_transactions_count_60_d: Option<i64>,
    #[serde(rename = "debit_transactions_count_90d")]
    ///The total number of debit (outflow) transactions over the past 90 days from the account that will be debited
    pub debit_transactions_count_90_d: Option<i64>,
    #[serde(rename = "credit_transactions_count_90d")]
    ///The total number of credit (inflow) transactions over the past 90 days from the account that will be debited
    pub credit_transactions_count_90_d: Option<i64>,
    #[serde(rename = "total_debit_transactions_amount_30d")]
    ///The total debit (outflow) transaction amount over the past 30 days from the account that will be debited
    pub total_debit_transactions_amount_30_d: Option<f64>,
    #[serde(rename = "total_credit_transactions_amount_30d")]
    ///The total credit (inflow) transaction amount over the past 30 days from the account that will be debited
    pub total_credit_transactions_amount_30_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_60d")]
    ///The total debit (outflow) transaction amount over the past 60 days from the account that will be debited
    pub total_debit_transactions_amount_60_d: Option<f64>,
    #[serde(rename = "total_credit_transactions_amount_60d")]
    ///The total credit (inflow) transaction amount over the past 60 days from the account that will be debited
    pub total_credit_transactions_amount_60_d: Option<f64>,
    #[serde(rename = "total_debit_transactions_amount_90d")]
    ///The total debit (outflow) transaction amount over the past 90 days from the account that will be debited
    pub total_debit_transactions_amount_90_d: Option<f64>,
    #[serde(rename = "total_credit_transactions_amount_90d")]
    ///The total credit (inflow) transaction amount over the past 90 days from the account that will be debited
    pub total_credit_transactions_amount_90_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_30d")]
    ///The 50th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p_50_eod_balance_30_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_60d")]
    ///The 50th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p_50_eod_balance_60_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_90d")]
    ///The 50th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p_50_eod_balance_90_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_31d_to_60d")]
    ///The 50th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    pub p_50_eod_balance_31_d_to_60_d: Option<f64>,
    #[serde(rename = "p50_eod_balance_61d_to_90d")]
    ///The 50th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    pub p_50_eod_balance_61_d_to_90_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_31d_to_60d")]
    ///The 90th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    pub p_90_eod_balance_31_d_to_60_d: Option<f64>,
    #[serde(rename = "p90_eod_balance_61d_to_90d")]
    ///The 90th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    pub p_90_eod_balance_61_d_to_90_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_31d_to_60d")]
    ///The 10th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    pub p_10_eod_balance_31_d_to_60_d: Option<f64>,
    #[serde(rename = "p10_eod_balance_61d_to_90d")]
    ///The 10th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    pub p_10_eod_balance_61_d_to_90_d: Option<f64>,
}
impl std::fmt::Display for SignalEvaluateCoreAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDecisionReportRequest {
    #[serde(rename = "client_transaction_id")]
    ///Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    pub client_transaction_id: String,
    #[serde(rename = "initiated")]
    ///`true` if the ACH transaction was initiated, `false` otherwise.
    pub initiated: bool,
    #[serde(rename = "days_funds_on_hold")]
    ///The actual number of days (hold time) since the ACH debit transaction that you wait before making funds available to your customers. The holding time could affect the ACH return rate. For example, use 0 if you make funds available to your customers instantly or the same day following the debit transaction, or 1 if you make funds available the next day following the debit initialization.
    pub days_funds_on_hold: Option<i64>,
}
impl std::fmt::Display for SignalDecisionReportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDecisionReportResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SignalDecisionReportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalReturnReportRequest {
    #[serde(rename = "client_transaction_id")]
    ///Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    pub client_transaction_id: String,
    #[serde(rename = "return_code")]
    ///Must be a valid ACH return code (e.g. "R01")
    pub return_code: String,
}
impl std::fmt::Display for SignalReturnReportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalReturnReportResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SignalReturnReportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPrepareRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for SignalPrepareRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPrepareResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for SignalPrepareResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsRequest {
    #[serde(rename = "oauth_state_id")]
    pub oauth_state_id: String,
    #[serde(rename = "accounts")]
    pub accounts: Vec<String>,
}
impl std::fmt::Display for SandboxOauthSelectAccountsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsResponse {}
impl std::fmt::Display for SandboxOauthSelectAccountsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccountsAvailableWebhook {
    #[serde(rename = "webhook_type")]
    ///`ITEM`
    pub webhook_type: Option<String>,
    #[serde(rename = "webhook_code")]
    ///`NEW_ACCOUNTS_AVAILABLE`
    pub webhook_code: Option<String>,
    #[serde(rename = "item_id")]
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: Option<String>,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
impl std::fmt::Display for NewAccountsAvailableWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreateRequest {
    #[serde(rename = "iso_currency_code")]
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: String,
}
impl std::fmt::Display for WalletCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreateResponse {}
impl std::fmt::Display for WalletCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGetRequest {
    #[serde(rename = "wallet_id")]
    ///The ID of the e-wallet
    pub wallet_id: String,
}
impl std::fmt::Display for WalletGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGetResponse {}
impl std::fmt::Display for WalletGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletListRequest {
    #[serde(rename = "iso_currency_code")]
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: Option<String>,
    #[serde(rename = "cursor")]
    ///A base64 value representing the latest e-wallet that has already been requested. Set this to `next_cursor` received from the previous `/wallet/list` request. If provided, the response will only contain e-wallets created before that e-wallet. If omitted, the response will contain e-wallets starting from the most recent, and in descending order.
    pub cursor: Option<String>,
    #[serde(rename = "count")]
    ///The number of e-wallets to fetch
    pub count: Option<i64>,
}
impl std::fmt::Display for WalletListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletListResponse {
    #[serde(rename = "wallets")]
    ///An array of e-wallets
    pub wallets: Vec<Wallet>,
    #[serde(rename = "next_cursor")]
    ///Cursor used for fetching e-wallets created before the latest e-wallet provided in this response
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WalletListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(rename = "wallet_id")]
    ///A unique ID identifying the e-wallet
    pub wallet_id: String,
    #[serde(rename = "balance")]
    ///An object representing the e-wallet balance
    pub balance: WalletBalance,
    #[serde(rename = "numbers")]
    ///An object representing the e-wallet account numbers
    pub numbers: WalletNumbers,
}
impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletNumbers {
    #[serde(rename = "bacs")]
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if you need to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacs>,
    #[serde(rename = "international")]
    ///Account numbers using the International Bank Account Number and BIC/SWIFT code format.
    pub international: Option<NumbersInternationalIban>,
}
impl std::fmt::Display for WalletNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalance {
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the balance
    pub iso_currency_code: String,
    #[serde(rename = "current")]
    ///The total amount of funds in the account
    pub current: f64,
}
impl std::fmt::Display for WalletBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WalletIsoCurrencyCode {
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "EUR")]
    Eur,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionExecuteRequest {
    #[serde(rename = "idempotency_key")]
    /**A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: String,
    #[serde(rename = "wallet_id")]
    ///The ID of the e-wallet to debit from
    pub wallet_id: String,
    #[serde(rename = "counterparty")]
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    #[serde(rename = "amount")]
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    #[serde(rename = "reference")]
    ///A reference for the transaction. This must be an alphanumeric string with at most 18 characters and must not contain any special characters or spaces.
    pub reference: String,
}
impl std::fmt::Display for WalletTransactionExecuteRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterparty {
    #[serde(rename = "name")]
    ///The name of the counterparty
    pub name: String,
    #[serde(rename = "numbers")]
    ///The counterparty's bank account numbers. Exactly one of IBAN or BACS data is required.
    pub numbers: WalletTransactionCounterpartyNumbers,
}
impl std::fmt::Display for WalletTransactionCounterparty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyNumbers {
    #[serde(rename = "bacs")]
    ///The account number and sort code of the counterparty's account
    pub bacs: Option<WalletTransactionCounterpartyBacs>,
    #[serde(rename = "international")]
    ///International Bank Account Number for a Wallet Transaction
    pub international: Option<WalletTransactionCounterpartyInternational>,
}
impl std::fmt::Display for WalletTransactionCounterpartyNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyBacs(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyInternational {
    #[serde(rename = "iban")]
    ///International Bank Account Number (IBAN).
    pub iban: Option<String>,
}
impl std::fmt::Display for WalletTransactionCounterpartyInternational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionAmount {
    #[serde(rename = "iso_currency_code")]
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: String,
    #[serde(rename = "value")]
    ///The amount of the transaction. Must contain at most two digits of precision e.g. `1.23`.
    pub value: f64,
}
impl std::fmt::Display for WalletTransactionAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionExecuteResponse {
    #[serde(rename = "transaction_id")]
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    #[serde(rename = "status")]
    /**The status of the transaction.

`INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WalletTransactionExecuteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WalletTransactionStatus {
    #[serde(rename = "INITIATED")]
    Initiated,
    #[serde(rename = "EXECUTED")]
    Executed,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "FAILED")]
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionGetRequest {
    #[serde(rename = "transaction_id")]
    ///The ID of the transaction to fetch
    pub transaction_id: String,
}
impl std::fmt::Display for WalletTransactionGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionGetResponse {}
impl std::fmt::Display for WalletTransactionGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsListRequest {
    #[serde(rename = "wallet_id")]
    ///The ID of the e-wallet to fetch transactions from
    pub wallet_id: String,
    #[serde(rename = "cursor")]
    ///A base64 value representing the latest transaction that has already been requested. Set this to `next_cursor` received from the previous `/wallet/transactions/list` request. If provided, the response will only contain transactions created before that transaction. If omitted, the response will contain transactions starting from the most recent, and in descending order by the `created_at` time.
    pub cursor: Option<String>,
    #[serde(rename = "count")]
    ///The number of transactions to fetch
    pub count: Option<i64>,
}
impl std::fmt::Display for WalletTransactionsListRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsListResponse {
    #[serde(rename = "transactions")]
    ///An array of transactions of an e-wallet, associated with the given `wallet_id`
    pub transactions: Vec<WalletTransaction>,
    #[serde(rename = "next_cursor")]
    ///Cursor used for fetching transactions created before the latest transaction provided in this response
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WalletTransactionsListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransaction {
    #[serde(rename = "transaction_id")]
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    #[serde(rename = "reference")]
    ///A reference for the transaction
    pub reference: String,
    #[serde(rename = "type")]
    /**The type of the transaction. The supported transaction types that are returned are:
`BANK_TRANSFER:` a transaction which credits an e-wallet through an external bank transfer.

`PAYOUT:` a transaction which debits an e-wallet by disbursing funds to a counterparty.

`PIS_PAY_IN:` a payment which credits an e-wallet through Plaid's Payment Initiation Services (PIS) APIs. For more information see the [Payment Initiation endpoints](https://plaid.com/docs/api/products/payment-initiation/).

`REFUND:` a transaction which debits an e-wallet by refunding a previously initated payment made through Plaid's [PIS APIs](https://plaid.com/docs/api/products/payment-initiation/).*/
    pub type_: String,
    #[serde(rename = "amount")]
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    #[serde(rename = "counterparty")]
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    #[serde(rename = "status")]
    /**The status of the transaction.

`INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: String,
    #[serde(rename = "created_at")]
    ///Timestamp when the transaction was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: String,
}
impl std::fmt::Display for WalletTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsEnhanceGetRequest {
    #[serde(rename = "account_type")]
    ///The type of account for the requested transactions (`depository` or `credit`).
    pub account_type: String,
    #[serde(rename = "transactions")]
    ///An array of raw transactions to be enhanced.
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl std::fmt::Display for TransactionsEnhanceGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientProvidedRawTransaction {
    #[serde(rename = "id")]
    ///Unique transaction identifier to tie transactions back to clients' systems.
    pub id: String,
    #[serde(rename = "description")]
    ///The raw description of the transaction.
    pub description: String,
    #[serde(rename = "amount")]
    ///The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the transaction.
    pub iso_currency_code: String,
}
impl std::fmt::Display for ClientProvidedRawTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsEnhanceGetResponse {
    #[serde(rename = "enhanced_transactions")]
    ///An array of enhanced transactions.
    pub enhanced_transactions: Vec<ClientProvidedEnhancedTransaction>,
}
impl std::fmt::Display for TransactionsEnhanceGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientProvidedEnhancedTransaction {
    #[serde(rename = "id")]
    ///Unique transaction identifier to tie transactions back to clients' systems.
    pub id: String,
    #[serde(rename = "description")]
    ///The raw description of the transaction.
    pub description: String,
    #[serde(rename = "amount")]
    ///The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    #[serde(rename = "iso_currency_code")]
    ///The ISO-4217 currency code of the transaction.
    pub iso_currency_code: String,
    #[serde(rename = "enhancements")]
    ///A grouping of the Plaid produced transaction enhancement fields.
    pub enhancements: Enhancements,
}
impl std::fmt::Display for ClientProvidedEnhancedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentChannel {
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "in store")]
    InStore,
    #[serde(rename = "other")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Enhancements {
    #[serde(rename = "merchant_name")]
    ///The merchant name, as extracted by Plaid from the raw description.
    pub merchant_name: Option<String>,
    #[serde(rename = "website")]
    ///The website associated with this transaction.
    pub website: Option<String>,
    #[serde(rename = "logo_url")]
    ///A link to the logo associated with this transaction. The logo will always be 100x100 resolution.
    pub logo_url: Option<String>,
    #[serde(rename = "check_number")]
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
    #[serde(rename = "payment_channel")]
    /**The channel used to make a payment.
`online:` transactions that took place online.

`in store:` transactions that were made at a physical location.

`other:` transactions that relate to banks, e.g. fees or deposits.*/
    pub payment_channel: String,
    #[serde(rename = "category_id")]
    ///The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    pub category_id: Option<String>,
    #[serde(rename = "category")]
    ///A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    pub category: Vec<String>,
    #[serde(rename = "location")]
    ///A representation of where a transaction took place
    pub location: Location,
    #[serde(rename = "personal_finance_category")]
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.*/
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    #[serde(rename = "personal_finance_category_icon_url")]
    ///A link to the icon associated with the primary personal finance category. The logo will always be 100x100 resolution.
    pub personal_finance_category_icon_url: Option<String>,
}
impl std::fmt::Display for Enhancements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileCreateRequest {}
impl std::fmt::Display for PaymentProfileCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileCreateResponse {
    #[serde(rename = "payment_profile_id")]
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentProfileCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileGetRequest {
    #[serde(rename = "payment_profile_id")]
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: String,
}
impl std::fmt::Display for PaymentProfileGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileGetResponse {
    #[serde(rename = "updated_at")]
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time the given Payment Profile was updated at
    pub updated_at: String,
    #[serde(rename = "created_at")]
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Payment Profile was created at
    pub created_at: String,
    #[serde(rename = "status")]
    /**The status of the given Payment Profile.

`READY`: This Payment Profile is ready to be used to create transfers using `/transfer/authorization/create` and /transfer/create`.

`PENDING`: This Payment Profile is not ready to be used. You’ll need to call `/link/token/create` and provide the Payment Profile ID in the `transfer.payment_profile_id` field and go through the account linking flow to activate it.

`REMOVED`: This Payment Profile has been removed.*/
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentProfileGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentProfileStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "READY")]
    Ready,
    #[serde(rename = "REMOVED")]
    Removed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileRemoveRequest {
    #[serde(rename = "payment_profile_id")]
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: String,
}
impl std::fmt::Display for PaymentProfileRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileRemoveResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaymentProfileRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerCustomersCreateRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerCustomersCreateResponse {
    #[serde(rename = "end_customer")]
    ///The end customer details for the newly-created customer client.
    pub end_customer: Option<PartnerEndCustomerClient>,
    #[serde(rename = "production_secret")]
    pub production_secret: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<String>,
}
impl std::fmt::Display for PartnerCustomersCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerEndCustomerClient {
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
}
impl std::fmt::Display for PartnerEndCustomerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AddressPurposeLabel {
    #[serde(rename = "residential")]
    Residential,
    #[serde(rename = "commercial")]
    Commercial,
    #[serde(rename = "no_data")]
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct City(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientUserId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEntityScreeningRequest {
    #[serde(rename = "search_terms")]
    ///Search inputs for creating an entity watchlist screening
    pub search_terms: EntityWatchlistSearchTerms,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
}
impl std::fmt::Display for CreateEntityScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEntityWatchlistScreeningReviewRequest {
    #[serde(rename = "confirmed_hits")]
    ///Hits to mark as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<String>,
    #[serde(rename = "dismissed_hits")]
    ///Hits to mark as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<String>,
    #[serde(rename = "comment")]
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<String>,
    #[serde(rename = "entity_watchlist_screening_id")]
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: String,
}
impl std::fmt::Display for CreateEntityWatchlistScreeningReviewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIndividualWatchlistScreeningReviewRequest {
    #[serde(rename = "confirmed_hits")]
    ///Hits to mark as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<String>,
    #[serde(rename = "dismissed_hits")]
    ///Hits to mark as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<String>,
    #[serde(rename = "comment")]
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<String>,
    #[serde(rename = "watchlist_screening_id")]
    ///ID of the associated screening.
    pub watchlist_screening_id: String,
}
impl std::fmt::Display for CreateIndividualWatchlistScreeningReviewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUser {
    #[serde(rename = "id")]
    ///ID of the associated user.
    pub id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "email_address")]
    ///A valid email address.
    pub email_address: String,
    #[serde(rename = "status")]
    ///The current status of the user.
    pub status: String,
}
impl std::fmt::Display for DashboardUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUserId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUserResponse {
    #[serde(rename = "id")]
    ///ID of the associated user.
    pub id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "email_address")]
    ///A valid email address.
    pub email_address: String,
    #[serde(rename = "status")]
    ///The current status of the user.
    pub status: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for DashboardUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DashboardUserStatus {
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deactivated")]
    Deactivated,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Date(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    #[serde(rename = "beginning")]
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub beginning: String,
    #[serde(rename = "ending")]
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub ending: String,
}
impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentAnalysis {
    #[serde(rename = "authenticity")]
    /**High level summary of whether the document in the provided image matches the formatting rules and security checks for the associated jurisdiction.

For example, most identity documents have formatting rules like the following:


The image of the person's face must have a certain contrast in order to highlight skin tone


The subject in the document's image must remove eye glasses and pose in a certain way


The informational fields (name, date of birth, ID number, etc.) must be colored and aligned according to specific rules


Security features like watermarks and background patterns must be present

So a `match` status for this field indicates that the document in the provided image seems to conform to the various formatting and security rules associated with the detected document.*/
    pub authenticity: String,
    #[serde(rename = "image_quality")]
    /**A high level description of the quality of the image the user submitted.

For example, an image that is blurry, distorted by glare from a nearby light source, or improperly framed might be marked as low or medium quality. Poor quality images are more likely to fail OCR and/or template conformity checks.

Note: By default, Plaid will let a user recapture document images twice before failing the entire session if we attribute the failure to low image quality.*/
    pub image_quality: String,
    #[serde(rename = "extracted_data")]
    ///Analysis of the data extracted from the submitted document.
    pub extracted_data: Option<PhysicalDocumentExtractedDataAnalysis>,
}
impl std::fmt::Display for DocumentAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentAuthenticityMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentDateOfBirthMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageBack(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageCroppedBack(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageCroppedFront(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageFace(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageFront(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentNameMatchCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "manually_approved")]
    ManuallyApproved,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentaryVerification {
    #[serde(rename = "status")]
    ///The outcome status for the associated Identity Verification attempt's `documentary_verification` step. This field will always have the same value as `steps.documentary_verification`.
    pub status: String,
    #[serde(rename = "documents")]
    /**An array of documents submitted to the `documentary_verification` step. Each entry represents one user submission, where each submission will contain both a front and back image, or just a front image, depending on the document type.

Note: Plaid will automatically let a user submit a new set of document images up to three times if we detect that a previous attempt might have failed due to user error. For example, if the first set of document images are blurry or obscured by glare, the user will be asked to capture their documents again, resulting in at least two separate entries within `documents`. If the overall `documentary_verification` is `failed`, the user has exhausted their retry attempts.*/
    pub documents: Vec<DocumentaryVerificationDocument>,
}
impl std::fmt::Display for DocumentaryVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentaryVerificationDocument {
    #[serde(rename = "status")]
    ///An outcome status for this specific document submission. Distinct from the overall `documentary_verification.status` that summarizes the verification outcome from one or more documents.
    pub status: String,
    #[serde(rename = "attempt")]
    ///The `attempt` field begins with 1 and increments with each subsequent document upload.
    pub attempt: f64,
    #[serde(rename = "images")]
    ///URLs for downloading original and cropped images for this document submission. The URLs are designed to only allow downloading, not hot linking, so the URL will only serve the document image for 60 seconds before expiring. The expiration time is 60 seconds after the `GET` request for the associated Identity Verification attempt. A new expiring URL is generated with each request, so you can always rerequest the Identity Verification attempt if one of your URLs expires.
    pub images: PhysicalDocumentImages,
    #[serde(rename = "extracted_data")]
    ///Data extracted from a user-submitted document.
    pub extracted_data: Option<PhysicalDocumentExtractedData>,
    #[serde(rename = "analysis")]
    ///High level descriptions of how the associated document was processed. If a document fails verification, the details in the `analysis` object should help clarify why the document was rejected.
    pub analysis: DocumentAnalysis,
}
impl std::fmt::Display for DocumentaryVerificationDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailAddress(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityDocument {
    #[serde(rename = "type")]
    /**The kind of official document represented by this object.

`bik` - Russian bank code

`business_number` - A number that uniquely identifies the business within a category of businesses

`imo` - Number assigned to the entity by the International Maritime Organization

`other` - Any document not covered by other categories

`swift` - Number identifying a bank and branch.

`tax_id` - Identification issued for the purpose of collecting taxes*/
    pub type_: String,
    #[serde(rename = "number")]
    ///The numeric or alphanumeric identifier associated with this document.
    pub number: String,
}
impl std::fmt::Display for EntityDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityDocumentType {
    #[serde(rename = "bik")]
    Bik,
    #[serde(rename = "business_number")]
    BusinessNumber,
    #[serde(rename = "imo")]
    Imo,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "swift")]
    Swift,
    #[serde(rename = "tax_id")]
    TaxId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitAnalysis {
    #[serde(rename = "documents")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub documents: Option<String>,
    #[serde(rename = "email_addresses")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub email_addresses: Option<String>,
    #[serde(rename = "locations")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub locations: Option<String>,
    #[serde(rename = "names")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub names: Option<String>,
    #[serde(rename = "phone_numbers")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub phone_numbers: Option<String>,
    #[serde(rename = "urls")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub urls: Option<String>,
    #[serde(rename = "search_terms_version")]
    ///The version of the entity screening's `search_terms` that were compared when the entity screening hit was added. entity screening hits are immutable once they have been reviewed. If changes are detected due to updates to the entity screening's `search_terms`, the associated entity program, or the list's source data prior to review, the entity screening hit will be updated to reflect those changes.
    pub search_terms_version: f64,
}
impl std::fmt::Display for EntityScreeningHitAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitData {
    #[serde(rename = "documents")]
    ///Documents associated with the watchlist hit
    pub documents: Option<Vec<EntityScreeningHitDocumentsItems>>,
    #[serde(rename = "email_addresses")]
    ///Email addresses associated with the watchlist hit
    pub email_addresses: Option<Vec<EntityScreeningHitEmailsItems>>,
    #[serde(rename = "locations")]
    ///Locations associated with the watchlist hit
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    #[serde(rename = "names")]
    ///Names associated with the watchlist hit
    pub names: Option<Vec<EntityScreeningHitNamesItems>>,
    #[serde(rename = "phone_numbers")]
    ///Phone numbers associated with the watchlist hit
    pub phone_numbers: Option<Vec<EntityScreeningHitsPhoneNumberItems>>,
    #[serde(rename = "urls")]
    ///URLs associated with the watchlist hit
    pub urls: Option<Vec<EntityScreeningHitUrlsItems>>,
}
impl std::fmt::Display for EntityScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitDocumentsItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///An official document, usually issued by a governing body or institution, with an associated identifier.
    pub data: Option<EntityDocument>,
}
impl std::fmt::Display for EntityScreeningHitDocumentsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitEmails {
    #[serde(rename = "email_address")]
    ///A valid email address.
    pub email_address: String,
}
impl std::fmt::Display for EntityScreeningHitEmails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitEmailsItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///Email address information for the associated entity watchlist hit
    pub data: Option<EntityScreeningHitEmails>,
}
impl std::fmt::Display for EntityScreeningHitEmailsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitNames {
    #[serde(rename = "full")]
    ///The full name of the entity.
    pub full: String,
    #[serde(rename = "is_primary")]
    ///Primary names are those most commonly used to refer to this entity. Only one name will ever be marked as primary.
    pub is_primary: bool,
    #[serde(rename = "weak_alias_determination")]
    ///Names that are explicitly marked as low quality either by their `source` list, or by `plaid` by a series of additional checks done by Plaid. Plaid does not ever surface a hit as a result of a weak name alone. If a name has no quality issues, this value will be `none`.
    pub weak_alias_determination: String,
}
impl std::fmt::Display for EntityScreeningHitNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitNamesItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///Name information for the associated entity watchlist hit
    pub data: Option<EntityScreeningHitNames>,
}
impl std::fmt::Display for EntityScreeningHitNamesItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitPhoneNumbers {
    #[serde(rename = "type")]
    ///An enum indicating whether a phone number is a phone line or a fax line.
    pub type_: String,
    #[serde(rename = "phone_number")]
    ///A phone number in E.164 format.
    pub phone_number: String,
}
impl std::fmt::Display for EntityScreeningHitPhoneNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitUrls {
    #[serde(rename = "url")]
    ///An 'http' or 'https' URL (must begin with either of those).
    pub url: String,
}
impl std::fmt::Display for EntityScreeningHitUrls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitUrlsItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///URLs associated with the entity screening hit
    pub data: Option<EntityScreeningHitUrls>,
}
impl std::fmt::Display for EntityScreeningHitUrlsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitsPhoneNumberItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///Phone number information associated with the entity screening hit
    pub data: Option<EntityScreeningHitPhoneNumbers>,
}
impl std::fmt::Display for EntityScreeningHitsPhoneNumberItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityWatchlistCode {
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_SOE")]
    IzSoe,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "US_CAP")]
    UsCap,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_MBS")]
    UsMbs,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "US_CMC")]
    UsCmc,
    #[serde(rename = "US_UVL")]
    UsUvl,
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "UK_HMC")]
    UkHmc,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistProgram {
    #[serde(rename = "id")]
    ///ID of the associated entity program.
    pub id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "is_rescanning_enabled")]
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    #[serde(rename = "lists_enabled")]
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<String>,
    #[serde(rename = "name")]
    ///A name for the entity program to define its purpose. For example, "High Risk Organizations" or "Applicants".
    pub name: String,
    #[serde(rename = "name_sensitivity")]
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: String,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "is_archived")]
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: bool,
}
impl std::fmt::Display for EntityWatchlistProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistProgramId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistProgramResponse {
    #[serde(rename = "id")]
    ///ID of the associated entity program.
    pub id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "is_rescanning_enabled")]
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    #[serde(rename = "lists_enabled")]
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<String>,
    #[serde(rename = "name")]
    ///A name for the entity program to define its purpose. For example, "High Risk Organizations" or "Applicants".
    pub name: String,
    #[serde(rename = "name_sensitivity")]
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: String,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "is_archived")]
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EntityWatchlistProgramResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreening {
    #[serde(rename = "id")]
    ///ID of the associated entity screening.
    pub id: String,
    #[serde(rename = "search_terms")]
    ///Search terms associated with an entity used for searching against watchlists
    pub search_terms: EntityWatchlistScreeningSearchTerms,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: String,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
impl std::fmt::Display for EntityWatchlistScreening {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningHit {
    #[serde(rename = "id")]
    ///ID of the associated entity screening hit.
    pub id: String,
    #[serde(rename = "review_status")]
    ///The current state of review. All watchlist screening hits begin in a `pending_review` state but can be changed by creating a review. When a hit is in the `pending_review` state, it will always show the latest version of the watchlist data Plaid has available and be compared against the latest customer information saved in the watchlist screening. Once a hit has been marked as `confirmed` or `dismissed` it will no longer be updated so that the state is as it was when the review was first conducted.
    pub review_status: String,
    #[serde(rename = "first_active")]
    ///An ISO8601 formatted timestamp.
    pub first_active: String,
    #[serde(rename = "inactive_since")]
    ///An ISO8601 formatted timestamp.
    pub inactive_since: Option<String>,
    #[serde(rename = "historical_since")]
    ///An ISO8601 formatted timestamp.
    pub historical_since: Option<String>,
    #[serde(rename = "list_code")]
    ///Shorthand identifier for a specific screening list for entities.
    pub list_code: String,
    #[serde(rename = "plaid_uid")]
    ///A universal identifier for a watchlist individual that is stable across searches and updates.
    pub plaid_uid: String,
    #[serde(rename = "source_uid")]
    ///The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    pub source_uid: Option<String>,
    #[serde(rename = "analysis")]
    ///Analysis information describing why a screening hit matched the provided entity information
    pub analysis: Option<EntityScreeningHitAnalysis>,
    #[serde(rename = "data")]
    ///Information associated with the entity watchlist hit
    pub data: Option<EntityScreeningHitData>,
}
impl std::fmt::Display for EntityWatchlistScreeningHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningHitId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningName(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningProgramName(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningResponse {
    #[serde(rename = "id")]
    ///ID of the associated entity screening.
    pub id: String,
    #[serde(rename = "search_terms")]
    ///Search terms associated with an entity used for searching against watchlists
    pub search_terms: EntityWatchlistScreeningSearchTerms,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: String,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EntityWatchlistScreeningResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReview {
    #[serde(rename = "id")]
    ///ID of the associated entity review.
    pub id: String,
    #[serde(rename = "confirmed_hits")]
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<String>,
    #[serde(rename = "dismissed_hits")]
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<String>,
    #[serde(rename = "comment")]
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<String>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
impl std::fmt::Display for EntityWatchlistScreeningReview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReviewId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReviewResponse {
    #[serde(rename = "id")]
    ///ID of the associated entity review.
    pub id: String,
    #[serde(rename = "confirmed_hits")]
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<String>,
    #[serde(rename = "dismissed_hits")]
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<String>,
    #[serde(rename = "comment")]
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<String>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for EntityWatchlistScreeningReviewResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningSearchTerms {
    #[serde(rename = "entity_watchlist_program_id")]
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: String,
    #[serde(rename = "legal_name")]
    ///The name of the organization being screened.
    pub legal_name: String,
    #[serde(rename = "document_number")]
    pub document_number: Option<serde_json::Value>,
    #[serde(rename = "email_address")]
    pub email_address: Option<serde_json::Value>,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
    #[serde(rename = "phone_number")]
    pub phone_number: Option<serde_json::Value>,
    #[serde(rename = "url")]
    pub url: Option<serde_json::Value>,
    #[serde(rename = "version")]
    ///The current version of the search terms. Starts at `1` and increments with each edit to `search_terms`.
    pub version: f64,
}
impl std::fmt::Display for EntityWatchlistScreeningSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistSearchTerms {
    #[serde(rename = "entity_watchlist_program_id")]
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: String,
    #[serde(rename = "legal_name")]
    ///The name of the organization being screened.
    pub legal_name: String,
    #[serde(rename = "document_number")]
    pub document_number: Option<serde_json::Value>,
    #[serde(rename = "email_address")]
    pub email_address: Option<serde_json::Value>,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
    #[serde(rename = "phone_number")]
    pub phone_number: Option<serde_json::Value>,
    #[serde(rename = "url")]
    pub url: Option<serde_json::Value>,
}
impl std::fmt::Display for EntityWatchlistSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ExpirationDate {
    #[serde(rename = "not_expired")]
    NotExpired,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "no_data")]
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyNameField(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericCountryCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericScreeningHitLocationItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///Location information for the associated individual watchlist hit
    pub data: Option<WatchlistScreeningHitLocations>,
}
impl std::fmt::Display for GenericScreeningHitLocationItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDashboardUserRequest {
    #[serde(rename = "dashboard_user_id")]
    ///ID of the associated user.
    pub dashboard_user_id: String,
}
impl std::fmt::Display for GetDashboardUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetEntityWatchlistScreeningRequest {
    #[serde(rename = "entity_watchlist_screening_id")]
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: String,
}
impl std::fmt::Display for GetEntityWatchlistScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetIdentityVerificationRequest {
    #[serde(rename = "identity_verification_id")]
    ///ID of the associated Identity Verification attempt.
    pub identity_verification_id: String,
}
impl std::fmt::Display for GetIdentityVerificationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndividualWatchlistScreeningRequest {
    #[serde(rename = "watchlist_screening_id")]
    ///ID of the associated screening.
    pub watchlist_screening_id: String,
}
impl std::fmt::Display for GetIndividualWatchlistScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWatchlistScreeningEntityProgramRequest {
    #[serde(rename = "entity_watchlist_program_id")]
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: String,
}
impl std::fmt::Display for GetWatchlistScreeningEntityProgramRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWatchlistScreeningIndividualProgramRequest {
    #[serde(rename = "watchlist_program_id")]
    ///ID of the associated program.
    pub watchlist_program_id: String,
}
impl std::fmt::Display for GetWatchlistScreeningIndividualProgramRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GivenNameField(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum IdNumberType {
    #[serde(rename = "ar_dni")]
    ArDni,
    #[serde(rename = "au_drivers_license")]
    AuDriversLicense,
    #[serde(rename = "au_passport")]
    AuPassport,
    #[serde(rename = "br_cpf")]
    BrCpf,
    #[serde(rename = "ca_sin")]
    CaSin,
    #[serde(rename = "cl_run")]
    ClRun,
    #[serde(rename = "cn_resident_card")]
    CnResidentCard,
    #[serde(rename = "co_nit")]
    CoNit,
    #[serde(rename = "dk_cpr")]
    DkCpr,
    #[serde(rename = "eg_national_id")]
    EgNationalId,
    #[serde(rename = "es_dni")]
    EsDni,
    #[serde(rename = "es_nie")]
    EsNie,
    #[serde(rename = "hk_hkid")]
    HkHkid,
    #[serde(rename = "in_pan")]
    InPan,
    #[serde(rename = "it_cf")]
    ItCf,
    #[serde(rename = "jo_civil_id")]
    JoCivilId,
    #[serde(rename = "jp_my_number")]
    JpMyNumber,
    #[serde(rename = "ke_huduma_namba")]
    KeHudumaNamba,
    #[serde(rename = "kw_civil_id")]
    KwCivilId,
    #[serde(rename = "mx_curp")]
    MxCurp,
    #[serde(rename = "mx_rfc")]
    MxRfc,
    #[serde(rename = "my_nric")]
    MyNric,
    #[serde(rename = "ng_nin")]
    NgNin,
    #[serde(rename = "nz_drivers_license")]
    NzDriversLicense,
    #[serde(rename = "om_civil_id")]
    OmCivilId,
    #[serde(rename = "ph_psn")]
    PhPsn,
    #[serde(rename = "pl_pesel")]
    PlPesel,
    #[serde(rename = "ro_cnp")]
    RoCnp,
    #[serde(rename = "sa_national_id")]
    SaNationalId,
    #[serde(rename = "se_pin")]
    SePin,
    #[serde(rename = "sg_nric")]
    SgNric,
    #[serde(rename = "tr_tc_kimlik")]
    TrTcKimlik,
    #[serde(rename = "us_ssn")]
    UsSsn,
    #[serde(rename = "us_ssn_last_4")]
    UsSsnLast4,
    #[serde(rename = "za_smart_id")]
    ZaSmartId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdNumberValue(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IpAddress(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Iso8601Date(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdempotencyFlag(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerification {
    #[serde(rename = "id")]
    ///ID of the associated Identity Verification attempt.
    pub id: String,
    #[serde(rename = "client_user_id")]
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "completed_at")]
    ///An ISO8601 formatted timestamp.
    pub completed_at: Option<String>,
    #[serde(rename = "previous_attempt_id")]
    ///The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt.
    pub previous_attempt_id: Option<String>,
    #[serde(rename = "shareable_url")]
    ///A shareable URL that can be sent directly to the user to complete verification
    pub shareable_url: Option<String>,
    #[serde(rename = "template")]
    ///The resource ID and version number of the template configuring the behavior of a given identity verification.
    pub template: IdentityVerificationTemplateReference,
    #[serde(rename = "user")]
    ///The identity data that was either collected from the user or provided via API in order to perform an identity verification.
    pub user: IdentityVerificationUserData,
    #[serde(rename = "status")]
    /**The status of this Identity Verification attempt.


`active` - The Identity Verification attempt is incomplete. The user may have completed part of the session, but has neither failed or passed.

`success` - The Identity Verification attempt has completed, passing all steps defined to the associated Identity Verification template

`failed` - The user failed one or more steps in the session and was told to contact support.

`expired` - The Identity Verification attempt was active for more than 48 hours without being completed and was automatically marked as expired.

`canceled` - The Identity Verification attempt was canceled, either via the dashboard by a user, or via API. The user may have completed part of the session, but has neither failed or passed.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.*/
    pub status: String,
    #[serde(rename = "steps")]
    /**Each step will be one of the following values:


`active` - This step is the user's current step. They are either in the process of completing this step, or they recently closed their Identity Verification attempt while in the middle of this step. Only one step will be marked as `active` at any given point.

`success` - The Identity Verification attempt has completed this step.

`failed` - The user failed this step. This can either call the user to fail the session as a whole, or cause them to fallback to another step depending on how the Identity Verification template is configured. A failed step does not imply a failed session.

`waiting_for_prerequisite` - The user needs to complete another step first, before they progress to this step. This step may never run, depending on if the user fails an earlier step or if the step is only run as a fallback.

`not_applicable` - This step will not be run for this session.

`skipped` - The retry instructions that created this Identity Verification attempt specified that this step should be skipped.

`expired` - This step had not yet been completed when the Identity Verification attempt as a whole expired.

`canceled` - The Identity Verification attempt was canceled before the user completed this step.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.

`manually_approved` - The step was manually overridden to pass by a team member in the dashboard.

`manually_rejected` - The step was manually overridden to fail by a team member in the dashboard.*/
    pub steps: IdentityVerificationStepSummary,
    #[serde(rename = "documentary_verification")]
    ///data, images, analysis, and results from the `documentary_verification` step.
    pub documentary_verification: Option<DocumentaryVerification>,
    #[serde(rename = "kyc_check")]
    ///The outcome of the `kyc_check` step.
    pub kyc_check: Option<KycCheckDetails>,
    #[serde(rename = "watchlist_screening_id")]
    pub watchlist_screening_id: Option<serde_json::Value>,
}
impl std::fmt::Display for IdentityVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationConsent(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationCreateRequest {
    #[serde(rename = "is_shareable")]
    ///A flag specifying whether you would like Plaid to expose a shareable URL for the verification being created.
    pub is_shareable: bool,
    #[serde(rename = "template_id")]
    ///ID of the associated Identity Verification template.
    pub template_id: String,
    #[serde(rename = "gave_consent")]
    /**A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.

If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.*/
    pub gave_consent: bool,
    #[serde(rename = "user")]
    /**User information collected outside of Link, most likely via your own onboarding process.

Each of the following identity fields are optional:

`email_address`

`phone_number`

`date_of_birth`

`name`

`address`

`id_number`
Specifically, these fields are optional in that they can either be fully provided (satisfying every required field in their subschema)
or omitted from the request entirely by not providing the key or value.
Providing these fields via the API will result in Link skipping the data collection process for the associated user. All verification steps enabled in the associated Identity Verification Template will still be run. Verification steps will either be run immediately, or once the user completes the `accept_tos` step, depending on the value provided to the `gave_consent` field.*/
    pub user: IdentityVerificationRequestUser,
    #[serde(rename = "is_idempotent")]
    /**An optional flag specifying how you would like Plaid to handle attempts to create an Identity Verification when an Identity Verification already exists for the provided `client_user_id` and `template_id`.
If idempotency is enabled, Plaid will return the existing Identity Verification. If idempotency is disabled, Plaid will reject the request with a `400 Bad Request` status code if an Identity Verification already exists for the supplied `client_user_id` and `template_id`.*/
    pub is_idempotent: Option<bool>,
}
impl std::fmt::Display for IdentityVerificationCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRequestUser {
    #[serde(rename = "client_user_id")]
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: String,
    #[serde(rename = "email_address")]
    pub email_address: Option<serde_json::Value>,
    #[serde(rename = "phone_number")]
    ///A phone number in E.164 format.
    pub phone_number: Option<String>,
    #[serde(rename = "date_of_birth")]
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub date_of_birth: Option<String>,
    #[serde(rename = "name")]
    ///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    pub name: Option<UserName>,
    #[serde(rename = "address")]
    ///Home address for the user.
    pub address: Option<UserAddress>,
    #[serde(rename = "id_number")]
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub id_number: Option<UserIdNumber>,
}
impl std::fmt::Display for IdentityVerificationRequestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationResponse {
    #[serde(rename = "id")]
    ///ID of the associated Identity Verification attempt.
    pub id: String,
    #[serde(rename = "client_user_id")]
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "completed_at")]
    ///An ISO8601 formatted timestamp.
    pub completed_at: Option<String>,
    #[serde(rename = "previous_attempt_id")]
    ///The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt.
    pub previous_attempt_id: Option<String>,
    #[serde(rename = "shareable_url")]
    ///A shareable URL that can be sent directly to the user to complete verification
    pub shareable_url: Option<String>,
    #[serde(rename = "template")]
    ///The resource ID and version number of the template configuring the behavior of a given identity verification.
    pub template: IdentityVerificationTemplateReference,
    #[serde(rename = "user")]
    ///The identity data that was either collected from the user or provided via API in order to perform an identity verification.
    pub user: IdentityVerificationUserData,
    #[serde(rename = "status")]
    /**The status of this Identity Verification attempt.


`active` - The Identity Verification attempt is incomplete. The user may have completed part of the session, but has neither failed or passed.

`success` - The Identity Verification attempt has completed, passing all steps defined to the associated Identity Verification template

`failed` - The user failed one or more steps in the session and was told to contact support.

`expired` - The Identity Verification attempt was active for more than 48 hours without being completed and was automatically marked as expired.

`canceled` - The Identity Verification attempt was canceled, either via the dashboard by a user, or via API. The user may have completed part of the session, but has neither failed or passed.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.*/
    pub status: String,
    #[serde(rename = "steps")]
    /**Each step will be one of the following values:


`active` - This step is the user's current step. They are either in the process of completing this step, or they recently closed their Identity Verification attempt while in the middle of this step. Only one step will be marked as `active` at any given point.

`success` - The Identity Verification attempt has completed this step.

`failed` - The user failed this step. This can either call the user to fail the session as a whole, or cause them to fallback to another step depending on how the Identity Verification template is configured. A failed step does not imply a failed session.

`waiting_for_prerequisite` - The user needs to complete another step first, before they progress to this step. This step may never run, depending on if the user fails an earlier step or if the step is only run as a fallback.

`not_applicable` - This step will not be run for this session.

`skipped` - The retry instructions that created this Identity Verification attempt specified that this step should be skipped.

`expired` - This step had not yet been completed when the Identity Verification attempt as a whole expired.

`canceled` - The Identity Verification attempt was canceled before the user completed this step.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.

`manually_approved` - The step was manually overridden to pass by a team member in the dashboard.

`manually_rejected` - The step was manually overridden to fail by a team member in the dashboard.*/
    pub steps: IdentityVerificationStepSummary,
    #[serde(rename = "documentary_verification")]
    ///data, images, analysis, and results from the `documentary_verification` step.
    pub documentary_verification: Option<DocumentaryVerification>,
    #[serde(rename = "kyc_check")]
    ///The outcome of the `kyc_check` step.
    pub kyc_check: Option<KycCheckDetails>,
    #[serde(rename = "watchlist_screening_id")]
    pub watchlist_screening_id: Option<serde_json::Value>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IdentityVerificationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRetryRequest {
    #[serde(rename = "client_user_id")]
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: String,
    #[serde(rename = "template_id")]
    ///ID of the associated Identity Verification template.
    pub template_id: String,
    #[serde(rename = "strategy")]
    /**An instruction specifying what steps the new Identity Verification attempt should require the user to complete:


`reset` - Restart the user at the beginning of the session, regardless of whether they successfully completed part of their previous session.

`incomplete` - Start the new session at the step that the user failed in the previous session, skipping steps that have already been successfully completed.

`infer` - If the most recent Identity Verification attempt associated with the given `client_user_id` has a status of `failed` or `expired`, retry using the `incomplete` strategy. Otherwise, use the `reset` strategy.

`custom` - Start the new session with a custom configuration, specified by the value of the `steps` field

Note:

The `incomplete` strategy cannot be applied if the session's failing step is `screening` or `risk_check`.

The `infer` strategy cannot be applied if the session's status is still `active`*/
    pub strategy: String,
    #[serde(rename = "steps")]
    /**Instructions for the `custom` retry strategy specifying which steps should be required or skipped.


Note:


This field must be provided when the retry strategy is `custom` and must be omitted otherwise.

Custom retries override settings in your Plaid Template. For example, if your Plaid Template has `verify_sms` disabled, a custom retry with `verify_sms` enabled will still require the step.

The `selfie_check` step is currently not supported on the sandbox server. Sandbox requests will silently disable the `selfie_check` step when provided.*/
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
}
impl std::fmt::Display for IdentityVerificationRetryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRetryRequestStepsObject {
    #[serde(rename = "verify_sms")]
    ///A boolean field specifying whether the new session should require or skip the `verify_sms` step.
    pub verify_sms: bool,
    #[serde(rename = "kyc_check")]
    ///A boolean field specifying whether the new session should require or skip the `kyc_check` step.
    pub kyc_check: bool,
    #[serde(rename = "documentary_verification")]
    ///A boolean field specifying whether the new session should require or skip the `documentary_verification` step.
    pub documentary_verification: bool,
    #[serde(rename = "selfie_check")]
    ///A boolean field specifying whether the new session should require or skip the `selfie_check` step.
    pub selfie_check: bool,
}
impl std::fmt::Display for IdentityVerificationRetryRequestStepsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityVerificationStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "pending_review")]
    PendingReview,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityVerificationStepStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "waiting_for_prerequisite")]
    WaitingForPrerequisite,
    #[serde(rename = "not_applicable")]
    NotApplicable,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "manually_approved")]
    ManuallyApproved,
    #[serde(rename = "manually_rejected")]
    ManuallyRejected,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationStepSummary {
    #[serde(rename = "accept_tos")]
    ///The status of a step in the identity verification process.
    pub accept_tos: String,
    #[serde(rename = "verify_sms")]
    ///The status of a step in the identity verification process.
    pub verify_sms: String,
    #[serde(rename = "kyc_check")]
    ///The status of a step in the identity verification process.
    pub kyc_check: String,
    #[serde(rename = "documentary_verification")]
    ///The status of a step in the identity verification process.
    pub documentary_verification: String,
    #[serde(rename = "selfie_check")]
    ///The status of a step in the identity verification process.
    pub selfie_check: String,
    #[serde(rename = "watchlist_screening")]
    ///The status of a step in the identity verification process.
    pub watchlist_screening: String,
    #[serde(rename = "risk_check")]
    ///The status of a step in the identity verification process.
    pub risk_check: String,
}
impl std::fmt::Display for IdentityVerificationStepSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateReference {
    #[serde(rename = "id")]
    ///ID of the associated Identity Verification template.
    pub id: String,
    #[serde(rename = "version")]
    ///Version of the associated Identity Verification template.
    pub version: f64,
}
impl std::fmt::Display for IdentityVerificationTemplateReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateVersion(pub f64);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationUserAddress {
    #[serde(rename = "street")]
    pub street: Option<serde_json::Value>,
    #[serde(rename = "street2")]
    ///Extra street information, like an apartment or suite number.
    pub street_2: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<serde_json::Value>,
    #[serde(rename = "region")]
    pub region: Option<serde_json::Value>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<serde_json::Value>,
    #[serde(rename = "country")]
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
}
impl std::fmt::Display for IdentityVerificationUserAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationUserData {
    #[serde(rename = "phone_number")]
    ///A phone number in E.164 format.
    pub phone_number: Option<String>,
    #[serde(rename = "date_of_birth")]
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub date_of_birth: Option<String>,
    #[serde(rename = "ip_address")]
    ///An IPv4 or IPV6 address.
    pub ip_address: Option<String>,
    #[serde(rename = "email_address")]
    pub email_address: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    pub name: Option<UserName>,
    #[serde(rename = "address")]
    ///Even if an address has been collected, some fields may be null depending on the region's addressing system. For example: * Addresses from the United Kingdom will not include a region * Addresses from Hong Kong will not include postal code
    pub address: Option<IdentityVerificationUserAddress>,
    #[serde(rename = "id_number")]
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub id_number: Option<UserIdNumber>,
}
impl std::fmt::Display for IdentityVerificationUserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationUserPhoneNumber(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum ImageQuality {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "low")]
    Low,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualScreeningHitNames {
    #[serde(rename = "full")]
    ///The full name of the individual, including all parts.
    pub full: String,
    #[serde(rename = "is_primary")]
    ///Primary names are those most commonly used to refer to this person. Only one name will ever be marked as primary.
    pub is_primary: bool,
    #[serde(rename = "weak_alias_determination")]
    ///Names that are explicitly marked as low quality either by their `source` list, or by `plaid` by a series of additional checks done by Plaid. Plaid does not ever surface a hit as a result of a weak name alone. If a name has no quality issues, this value will be `none`.
    pub weak_alias_determination: String,
}
impl std::fmt::Display for IndividualScreeningHitNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IndividualWatchlistCode {
    #[serde(rename = "AU_CON")]
    AuCon,
    #[serde(rename = "CA_CON")]
    CaCon,
    #[serde(rename = "EU_CON")]
    EuCon,
    #[serde(rename = "IZ_CIA")]
    IzCia,
    #[serde(rename = "IZ_IPL")]
    IzIpl,
    #[serde(rename = "IZ_PEP")]
    IzPep,
    #[serde(rename = "IZ_UNC")]
    IzUnc,
    #[serde(rename = "UK_HMC")]
    UkHmc,
    #[serde(rename = "US_DPL")]
    UsDpl,
    #[serde(rename = "US_DTC")]
    UsDtc,
    #[serde(rename = "US_FBI")]
    UsFbi,
    #[serde(rename = "US_FSE")]
    UsFse,
    #[serde(rename = "US_ISN")]
    UsIsn,
    #[serde(rename = "US_MBC")]
    UsMbc,
    #[serde(rename = "US_PLC")]
    UsPlc,
    #[serde(rename = "US_SDN")]
    UsSdn,
    #[serde(rename = "US_SSI")]
    UsSsi,
    #[serde(rename = "SG_SOF")]
    SgSof,
    #[serde(rename = "TR_TWL")]
    TrTwl,
    #[serde(rename = "TR_DFD")]
    TrDfd,
    #[serde(rename = "TR_FOR")]
    TrFor,
    #[serde(rename = "TR_WMD")]
    TrWmd,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualWatchlistProgram {
    #[serde(rename = "id")]
    ///ID of the associated program.
    pub id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "is_rescanning_enabled")]
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    #[serde(rename = "lists_enabled")]
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<String>,
    #[serde(rename = "name")]
    ///A name for the program to define its purpose. For example, "High Risk Individuals", "US Cardholders", or "Applicants".
    pub name: String,
    #[serde(rename = "name_sensitivity")]
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: String,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "is_archived")]
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: bool,
}
impl std::fmt::Display for IndividualWatchlistProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualWatchlistProgramResponse {
    #[serde(rename = "id")]
    ///ID of the associated program.
    pub id: String,
    #[serde(rename = "created_at")]
    ///An ISO8601 formatted timestamp.
    pub created_at: String,
    #[serde(rename = "is_rescanning_enabled")]
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    #[serde(rename = "lists_enabled")]
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<String>,
    #[serde(rename = "name")]
    ///A name for the program to define its purpose. For example, "High Risk Individuals", "US Cardholders", or "Applicants".
    pub name: String,
    #[serde(rename = "name_sensitivity")]
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: String,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "is_archived")]
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: bool,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IndividualWatchlistProgramResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualWatchlistScreeningProgramName(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalUid(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum IssuingCountry {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "no_match")]
    NoMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckAddressSummary {
    #[serde(rename = "summary")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
    #[serde(rename = "po_box")]
    ///Field describing whether the associated address is a post office box. Will be `yes` when a P.O. box is detected, `no` when Plaid confirmed the address is not a P.O. box, and `no_data` when Plaid was not able to determine if the address is a P.O. box.
    pub po_box: String,
    #[serde(rename = "type")]
    /**Field describing whether the associated address is being used for commercial or residential purposes.

Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.*/
    pub type_: String,
}
impl std::fmt::Display for KycCheckAddressSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckDateOfBirthSummary {
    #[serde(rename = "summary")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
}
impl std::fmt::Display for KycCheckDateOfBirthSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckDetails {
    #[serde(rename = "status")]
    ///The outcome status for the associated Identity Verification attempt's `kyc_check` step. This field will always have the same value as `steps.kyc_check`.
    pub status: String,
    #[serde(rename = "address")]
    ///Result summary object specifying how the `address` field matched.
    pub address: KycCheckAddressSummary,
    #[serde(rename = "name")]
    ///Result summary object specifying how the `name` field matched.
    pub name: KycCheckNameSummary,
    #[serde(rename = "date_of_birth")]
    ///Result summary object specifying how the `date_of_birth` field matched.
    pub date_of_birth: KycCheckDateOfBirthSummary,
    #[serde(rename = "id_number")]
    ///Result summary object specifying how the `id_number` field matched.
    pub id_number: KycCheckIdNumberSummary,
    #[serde(rename = "phone_number")]
    ///Result summary object specifying how the `phone` field matched.
    pub phone_number: KycCheckPhoneSummary,
}
impl std::fmt::Display for KycCheckDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckIdNumberSummary {
    #[serde(rename = "summary")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
}
impl std::fmt::Display for KycCheckIdNumberSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckNameSummary {
    #[serde(rename = "summary")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
}
impl std::fmt::Display for KycCheckNameSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckPhoneSummary {
    #[serde(rename = "summary")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
}
impl std::fmt::Display for KycCheckPhoneSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDashboardUserRequest {
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListDashboardUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListEntityWatchlistScreeningRequest {
    #[serde(rename = "entity_watchlist_program_id")]
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: String,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "status")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListEntityWatchlistScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListIdentityVerificationRequest {
    #[serde(rename = "template_id")]
    ///ID of the associated Identity Verification template.
    pub template_id: String,
    #[serde(rename = "client_user_id")]
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListIdentityVerificationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListIndividualWatchlistScreeningRequest {
    #[serde(rename = "watchlist_program_id")]
    ///ID of the associated program.
    pub watchlist_program_id: String,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "status")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListIndividualWatchlistScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityHistoryRequest {
    #[serde(rename = "entity_watchlist_screening_id")]
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningEntityHistoryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityHitRequest {
    #[serde(rename = "entity_watchlist_screening_id")]
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningEntityHitRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityProgramsRequest {
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningEntityProgramsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityReviewsRequest {
    #[serde(rename = "entity_watchlist_screening_id")]
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningEntityReviewsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualHistoryRequest {
    #[serde(rename = "watchlist_screening_id")]
    ///ID of the associated screening.
    pub watchlist_screening_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningIndividualHistoryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualHitRequest {
    #[serde(rename = "watchlist_screening_id")]
    ///ID of the associated screening.
    pub watchlist_screening_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningIndividualHitRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualProgramsRequest {
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningIndividualProgramsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualReviewsRequest {
    #[serde(rename = "watchlist_screening_id")]
    ///ID of the associated screening.
    pub watchlist_screening_id: String,
    #[serde(rename = "cursor")]
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<String>,
}
impl std::fmt::Display for ListWatchlistScreeningIndividualReviewsRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchSummary {
    #[serde(rename = "summary")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: String,
}
impl std::fmt::Display for MatchSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MatchSummaryCode {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "partial_match")]
    PartialMatch,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_data")]
    NoData,
    #[serde(rename = "no_input")]
    NoInput,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PoBoxStatus {
    #[serde(rename = "yes")]
    Yes,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "no_data")]
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedDashboardUserListResponse {
    #[serde(rename = "dashboard_users")]
    ///List of dashboard users
    pub dashboard_users: Vec<DashboardUser>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedDashboardUserListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistProgramListResponse {
    #[serde(rename = "entity_watchlist_programs")]
    ///List of entity watchlist screening programs
    pub entity_watchlist_programs: Vec<EntityWatchlistProgram>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedEntityWatchlistProgramListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistScreeningHitListResponse {
    #[serde(rename = "entity_watchlist_screening_hits")]
    ///List of entity watchlist screening hits
    pub entity_watchlist_screening_hits: Vec<EntityWatchlistScreeningHit>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedEntityWatchlistScreeningHitListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistScreeningListResponse {
    #[serde(rename = "entity_watchlist_screenings")]
    ///List of entity watchlist screening
    pub entity_watchlist_screenings: Vec<EntityWatchlistScreening>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedEntityWatchlistScreeningListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistScreeningReviewListResponse {
    #[serde(rename = "entity_watchlist_screening_reviews")]
    ///List of entity watchlist screening reviews
    pub entity_watchlist_screening_reviews: Vec<EntityWatchlistScreeningReview>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedEntityWatchlistScreeningReviewListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIdentityVerificationListResponse {
    #[serde(rename = "identity_verifications")]
    ///List of Plaid sessions
    pub identity_verifications: Vec<IdentityVerification>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedIdentityVerificationListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistProgramListResponse {
    #[serde(rename = "watchlist_programs")]
    ///List of individual watchlist screening programs
    pub watchlist_programs: Vec<IndividualWatchlistProgram>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedIndividualWatchlistProgramListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistScreeningHitListResponse {
    #[serde(rename = "watchlist_screening_hits")]
    ///List of individual watchlist screening hits
    pub watchlist_screening_hits: Vec<WatchlistScreeningHit>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedIndividualWatchlistScreeningHitListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistScreeningListResponse {
    #[serde(rename = "watchlist_screenings")]
    ///List of individual watchlist screenings
    pub watchlist_screenings: Vec<WatchlistScreeningIndividual>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedIndividualWatchlistScreeningListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistScreeningReviewListResponse {
    #[serde(rename = "watchlist_screening_reviews")]
    ///List of screening reviews
    pub watchlist_screening_reviews: Vec<WatchlistScreeningReview>,
    #[serde(rename = "next_cursor")]
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for PaginatedIndividualWatchlistScreeningReviewListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PhoneType {
    #[serde(rename = "phone")]
    Phone,
    #[serde(rename = "fax")]
    Fax,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PhysicalDocumentCategory {
    #[serde(rename = "drivers_license")]
    DriversLicense,
    #[serde(rename = "id_card")]
    IdCard,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "residence_permit_card")]
    ResidencePermitCard,
    #[serde(rename = "resident_card")]
    ResidentCard,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentExtractedData {
    #[serde(rename = "id_number")]
    ///Alpha-numeric ID number extracted via OCR from the user's document image.
    pub id_number: Option<String>,
    #[serde(rename = "category")]
    /**The type of identity document detected in the images provided. Will always be one of the following values:

  `drivers_license` - A driver's license for the associated country

  `id_card` - A general national identification card, distinct from driver's licenses

  `passport` - A passport for the associated country

  `residence_permit_card` - An identity document permitting a foreign citizen to <em>temporarily</em> reside in the associated country

  `resident_card` - An identity document permitting a foreign citizen to <em>permanently</em> reside in the associated country

Note: This value may be different from the ID type that the user selects within Link. For example, if they select "Driver's License" but then submit a picture of a passport, this field will say `passport`*/
    pub category: String,
    #[serde(rename = "expiration_date")]
    pub expiration_date: Option<serde_json::Value>,
    #[serde(rename = "issuing_country")]
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub issuing_country: String,
}
impl std::fmt::Display for PhysicalDocumentExtractedData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentExtractedDataAnalysis {
    #[serde(rename = "name")]
    ///A match summary describing the cross comparison between the subject's name, extracted from the document image, and the name they separately provided to identity verification attempt.
    pub name: String,
    #[serde(rename = "date_of_birth")]
    ///A match summary describing the cross comparison between the subject's date of birth, extracted from the document image, and the date of birth they separately provided to the identity verification attempt.
    pub date_of_birth: String,
    #[serde(rename = "expiration_date")]
    /**A description of whether the associated document was expired when the verification was performed.

Note: In the case where an expiration date is not present on the document or failed to be extracted, this value will be `no_data`.*/
    pub expiration_date: String,
    #[serde(rename = "issuing_country")]
    /**A binary match indicator specifying whether the country that issued the provided document matches the country that the user separately provided to Plaid.

Note: You can configure whether a `no_match` on `issuing_country` fails the `documentary_verification` by editing your Plaid Template.*/
    pub issuing_country: String,
}
impl std::fmt::Display for PhysicalDocumentExtractedDataAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentIdNumber(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentImages {
    #[serde(rename = "original_front")]
    ///Temporary URL that expires after 60 seconds for downloading the uncropped original image of the front of the document.
    pub original_front: String,
    #[serde(rename = "original_back")]
    ///Temporary URL that expires after 60 seconds for downloading the original image of the back of the document. Might be null if the back of the document was not collected.
    pub original_back: Option<String>,
    #[serde(rename = "cropped_front")]
    ///Temporary URL that expires after 60 seconds for downloading a cropped image containing just the front of the document.
    pub cropped_front: Option<String>,
    #[serde(rename = "cropped_back")]
    ///Temporary URL that expires after 60 seconds for downloading a cropped image containing just the back of the document. Might be null if the back of the document was not collected.
    pub cropped_back: Option<String>,
    #[serde(rename = "face")]
    ///Temporary URL that expires after 60 seconds for downloading a crop of just the user's face from the document image. Might be null if the document does not contain a face photo.
    pub face: Option<String>,
}
impl std::fmt::Display for PhysicalDocumentImages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousIdentityVerificationAttemptId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramArchived(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub enum ProgramNameSensitivity {
    #[serde(rename = "coarse")]
    Coarse,
    #[serde(rename = "balanced")]
    Balanced,
    #[serde(rename = "strict")]
    Strict,
    #[serde(rename = "exact")]
    Exact,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Region(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewComment(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitAnalysis {
    #[serde(rename = "dates_of_birth")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub dates_of_birth: Option<String>,
    #[serde(rename = "documents")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub documents: Option<String>,
    #[serde(rename = "locations")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub locations: Option<String>,
    #[serde(rename = "names")]
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub names: Option<String>,
    #[serde(rename = "search_terms_version")]
    ///The version of the screening's `search_terms` that were compared when the screening hit was added. screening hits are immutable once they have been reviewed. If changes are detected due to updates to the screening's `search_terms`, the associated program, or the list's source data prior to review, the screening hit will be updated to reflect those changes.
    pub search_terms_version: f64,
}
impl std::fmt::Display for ScreeningHitAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitData {
    #[serde(rename = "dates_of_birth")]
    ///Dates of birth associated with the watchlist hit
    pub dates_of_birth: Option<Vec<ScreeningHitDateOfBirthItem>>,
    #[serde(rename = "documents")]
    ///Documents associated with the watchlist hit
    pub documents: Option<Vec<ScreeningHitDocumentsItems>>,
    #[serde(rename = "locations")]
    ///Locations associated with the watchlist hit
    pub locations: Option<Vec<GenericScreeningHitLocationItems>>,
    #[serde(rename = "names")]
    ///Names associated with the watchlist hit
    pub names: Option<Vec<ScreeningHitNamesItems>>,
}
impl std::fmt::Display for ScreeningHitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitDateOfBirthItem {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///A date range with a start and end date
    pub data: Option<DateRange>,
}
impl std::fmt::Display for ScreeningHitDateOfBirthItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitDocumentsItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///An official document, usually issued by a governing body or institution, with an associated identifier.
    pub data: Option<WatchlistScreeningDocument>,
}
impl std::fmt::Display for ScreeningHitDocumentsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitNamesItems {
    #[serde(rename = "analysis")]
    ///Summary object reflecting the match result of the associated data
    pub analysis: Option<MatchSummary>,
    #[serde(rename = "data")]
    ///Name information for the associated individual watchlist hit
    pub data: Option<IndividualScreeningHitNames>,
}
impl std::fmt::Display for ScreeningHitNamesItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShareableUrl(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "dashboard")]
    Dashboard,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "system")]
    System,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceUid(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "reset")]
    Reset,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "infer")]
    Infer,
    #[serde(rename = "custom")]
    Custom,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Street(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Street2(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Timestamp(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampNullable(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Url(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityScreeningRequest {
    #[serde(rename = "entity_watchlist_screening_id")]
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: String,
    #[serde(rename = "search_terms")]
    ///Search terms for editing an entity watchlist screening
    pub search_terms: Option<UpdateEntityScreeningRequestSearchTerms>,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "status")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "reset_fields")]
    ///A list of fields to reset back to null
    pub reset_fields: Option<UpdateEntityScreeningRequestResettableFieldList>,
}
impl std::fmt::Display for UpdateEntityScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateEntityScreeningRequestResettableField {
    #[serde(rename = "assignee")]
    Assignee,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityScreeningRequestResettableFieldList(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityScreeningRequestSearchTerms {
    #[serde(rename = "entity_watchlist_program_id")]
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: String,
    #[serde(rename = "legal_name")]
    pub legal_name: Option<serde_json::Value>,
    #[serde(rename = "document_number")]
    pub document_number: Option<serde_json::Value>,
    #[serde(rename = "email_address")]
    pub email_address: Option<serde_json::Value>,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
    #[serde(rename = "phone_number")]
    pub phone_number: Option<serde_json::Value>,
    #[serde(rename = "url")]
    pub url: Option<serde_json::Value>,
}
impl std::fmt::Display for UpdateEntityScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequest {
    #[serde(rename = "watchlist_screening_id")]
    ///ID of the associated screening.
    pub watchlist_screening_id: String,
    #[serde(rename = "search_terms")]
    ///Search terms for editing an individual watchlist screening
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "status")]
    pub status: Option<serde_json::Value>,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "reset_fields")]
    ///A list of fields to reset back to null
    pub reset_fields: Option<UpdateIndividualScreeningRequestResettableFieldList>,
}
impl std::fmt::Display for UpdateIndividualScreeningRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateIndividualScreeningRequestResettableField {
    #[serde(rename = "assignee")]
    Assignee,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequestResettableFieldList(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequestSearchTerms {
    #[serde(rename = "watchlist_program_id")]
    pub watchlist_program_id: Option<serde_json::Value>,
    #[serde(rename = "legal_name")]
    pub legal_name: Option<serde_json::Value>,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: Option<serde_json::Value>,
    #[serde(rename = "document_number")]
    pub document_number: Option<serde_json::Value>,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
}
impl std::fmt::Display for UpdateIndividualScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddress {
    #[serde(rename = "street")]
    ///The primary street portion of an address. If the user has submitted their address, this field will always be filled.
    pub street: String,
    #[serde(rename = "street2")]
    ///Extra street information, like an apartment or suite number.
    pub street_2: Option<String>,
    #[serde(rename = "city")]
    ///City from the end user's address
    pub city: String,
    #[serde(rename = "region")]
    ///An ISO 3166-2 subdivision code. Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.
    pub region: String,
    #[serde(rename = "postal_code")]
    ///The postal code for the associated address. Between 2 and 10 alphanumeric characters.
    pub postal_code: String,
    #[serde(rename = "country")]
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
}
impl std::fmt::Display for UserAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdNumber {
    #[serde(rename = "value")]
    ///Value of identity document value typed in by user. Alpha-numeric, with all formatting characters stripped.
    pub value: String,
    #[serde(rename = "type")]
    ///A globally unique and human readable ID type, specific to the country and document category. For more context on this field, see [Hybrid Input Validation](https://cognitohq.com/docs/flow/flow-hybrid-input-validation)
    pub type_: String,
}
impl std::fmt::Display for UserIdNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    #[serde(rename = "given_name")]
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub given_name: String,
    #[serde(rename = "family_name")]
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub family_name: String,
}
impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistProgramId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningAuditTrail {
    #[serde(rename = "source")]
    ///A type indicating whether a dashboard user, an API-based user, or Plaid last touched this object.
    pub source: String,
    #[serde(rename = "dashboard_user_id")]
    pub dashboard_user_id: Option<serde_json::Value>,
    #[serde(rename = "timestamp")]
    ///An ISO8601 formatted timestamp.
    pub timestamp: String,
}
impl std::fmt::Display for WatchlistScreeningAuditTrail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningCreateRequest {
    #[serde(rename = "search_terms")]
    ///Search inputs for creating a watchlist screening
    pub search_terms: WatchlistScreeningRequestSearchTerms,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
}
impl std::fmt::Display for WatchlistScreeningCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningDocument {
    #[serde(rename = "type")]
    /**The kind of official document represented by this object.

`birth_certificate` - A certificate of birth

`drivers_license` - A license to operate a motor vehicle

`immigration_number` - Immigration or residence documents

`military_id` - Identification issued by a military group

`other` - Any document not covered by other categories

`passport` - An official passport issue by a government

`personal_identification` - Any generic personal identification that is not covered by other categories

`ration_card` - Identification that entitles the holder to rations

`ssn` - United States Social Security Number

`student_id` - Identification issued by an educational institution

`tax_id` - Identification issued for the purpose of collecting taxes

`travel_document` - Visas, entry permits, refugee documents, etc.

`voter_id` - Identification issued for the purpose of voting*/
    pub type_: String,
    #[serde(rename = "number")]
    ///The numeric or alphanumeric identifier associated with this document.
    pub number: String,
}
impl std::fmt::Display for WatchlistScreeningDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningDocumentType {
    #[serde(rename = "birth_certificate")]
    BirthCertificate,
    #[serde(rename = "drivers_license")]
    DriversLicense,
    #[serde(rename = "immigration_number")]
    ImmigrationNumber,
    #[serde(rename = "military_id")]
    MilitaryId,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "personal_identification")]
    PersonalIdentification,
    #[serde(rename = "ration_card")]
    RationCard,
    #[serde(rename = "ssn")]
    Ssn,
    #[serde(rename = "student_id")]
    StudentId,
    #[serde(rename = "tax_id")]
    TaxId,
    #[serde(rename = "travel_document")]
    TravelDocument,
    #[serde(rename = "voter_id")]
    VoterId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningDocumentValue(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningHit {
    #[serde(rename = "id")]
    ///ID of the associated screening hit.
    pub id: String,
    #[serde(rename = "review_status")]
    ///The current state of review. All watchlist screening hits begin in a `pending_review` state but can be changed by creating a review. When a hit is in the `pending_review` state, it will always show the latest version of the watchlist data Plaid has available and be compared against the latest customer information saved in the watchlist screening. Once a hit has been marked as `confirmed` or `dismissed` it will no longer be updated so that the state is as it was when the review was first conducted.
    pub review_status: String,
    #[serde(rename = "first_active")]
    ///An ISO8601 formatted timestamp.
    pub first_active: String,
    #[serde(rename = "inactive_since")]
    ///An ISO8601 formatted timestamp.
    pub inactive_since: Option<String>,
    #[serde(rename = "historical_since")]
    ///An ISO8601 formatted timestamp.
    pub historical_since: Option<String>,
    #[serde(rename = "list_code")]
    ///Shorthand identifier for a specific screening list for individuals.
    pub list_code: String,
    #[serde(rename = "plaid_uid")]
    ///A universal identifier for a watchlist individual that is stable across searches and updates.
    pub plaid_uid: String,
    #[serde(rename = "source_uid")]
    ///The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    pub source_uid: Option<String>,
    #[serde(rename = "analysis")]
    ///Analysis information describing why a screening hit matched the provided user information
    pub analysis: Option<ScreeningHitAnalysis>,
    #[serde(rename = "data")]
    ///Information associated with the watchlist hit
    pub data: Option<ScreeningHitData>,
}
impl std::fmt::Display for WatchlistScreeningHit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningHitId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningHitLocations {
    #[serde(rename = "full")]
    ///The full location string, potentially including elements like street, city, postal codes and country codes. Note that this is not necessarily a complete or well-formatted address.
    pub full: String,
    #[serde(rename = "country")]
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
}
impl std::fmt::Display for WatchlistScreeningHitLocations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningHitStatus {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "dismissed")]
    Dismissed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividual {
    #[serde(rename = "id")]
    ///ID of the associated screening.
    pub id: String,
    #[serde(rename = "search_terms")]
    ///Search terms for creating an individual watchlist screening
    pub search_terms: WatchlistScreeningSearchTerms,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: String,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
impl std::fmt::Display for WatchlistScreeningIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualName(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualResponse {
    #[serde(rename = "id")]
    ///ID of the associated screening.
    pub id: String,
    #[serde(rename = "search_terms")]
    ///Search terms for creating an individual watchlist screening
    pub search_terms: WatchlistScreeningSearchTerms,
    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,
    #[serde(rename = "status")]
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: String,
    #[serde(rename = "client_user_id")]
    pub client_user_id: Option<serde_json::Value>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningIndividualResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningPhoneNumber(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningRequestSearchTerms {
    #[serde(rename = "watchlist_program_id")]
    ///ID of the associated program.
    pub watchlist_program_id: String,
    #[serde(rename = "legal_name")]
    ///The legal name of the individual being screened.
    pub legal_name: String,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: Option<serde_json::Value>,
    #[serde(rename = "document_number")]
    pub document_number: Option<serde_json::Value>,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
}
impl std::fmt::Display for WatchlistScreeningRequestSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningReview {
    #[serde(rename = "id")]
    ///ID of the associated review.
    pub id: String,
    #[serde(rename = "confirmed_hits")]
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<String>,
    #[serde(rename = "dismissed_hits")]
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<String>,
    #[serde(rename = "comment")]
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<String>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
impl std::fmt::Display for WatchlistScreeningReview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningReviewId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningReviewResponse {
    #[serde(rename = "id")]
    ///ID of the associated review.
    pub id: String,
    #[serde(rename = "confirmed_hits")]
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<String>,
    #[serde(rename = "dismissed_hits")]
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<String>,
    #[serde(rename = "comment")]
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<String>,
    #[serde(rename = "audit_trail")]
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for WatchlistScreeningReviewResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningSearchTerms {
    #[serde(rename = "watchlist_program_id")]
    ///ID of the associated program.
    pub watchlist_program_id: String,
    #[serde(rename = "legal_name")]
    ///The legal name of the individual being screened.
    pub legal_name: String,
    #[serde(rename = "date_of_birth")]
    pub date_of_birth: Option<serde_json::Value>,
    #[serde(rename = "document_number")]
    pub document_number: Option<serde_json::Value>,
    #[serde(rename = "country")]
    pub country: Option<serde_json::Value>,
    #[serde(rename = "version")]
    ///The current version of the search terms. Starts at `1` and increments with each edit to `search_terms`.
    pub version: f64,
}
impl std::fmt::Display for WatchlistScreeningSearchTerms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningStatus {
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "cleared")]
    Cleared,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WeakAliasDetermination {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "source")]
    Source,
    #[serde(rename = "plaid")]
    Plaid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for ItemGetRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetResponse {
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "status")]
    ///Information about the last successful and failed transactions update for the Item.
    pub status: Option<ItemStatusNullable>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRemoveRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for ItemRemoveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRemoveResponse {
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWebhookUpdateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "webhook")]
    ///The new webhook URL to associate with the Item. To remove a webhook from an Item, set to `null`.
    pub webhook: Option<String>,
}
impl std::fmt::Display for ItemWebhookUpdateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWebhookUpdateResponse {
    #[serde(rename = "item")]
    ///Metadata about the Item.
    pub item: Item,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemWebhookUpdateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for ItemAccessTokenInvalidateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateResponse {
    #[serde(rename = "new_access_token")]
    ///The access token associated with the Item data is being requested for.
    pub new_access_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemAccessTokenInvalidateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeRequest {
    #[serde(rename = "public_token")]
    ///Your `public_token`, obtained from the Link `onSuccess` callback or `/sandbox/item/public_token/create`.
    pub public_token: String,
}
impl std::fmt::Display for ItemPublicTokenExchangeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeResponse {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "item_id")]
    ///The `item_id` value of the Item associated with the returned `access_token`
    pub item_id: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemPublicTokenExchangeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateRequest {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
}
impl std::fmt::Display for ItemPublicTokenCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateResponse {
    #[serde(rename = "public_token")]
    ///A `public_token` for the particular Item corresponding to the specified `access_token`
    pub public_token: String,
    #[serde(rename = "expiration")]
    pub expiration: Option<String>,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemPublicTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequest {
    #[serde(rename = "products")]
    ///Array of product strings
    pub products: Vec<String>,
    #[serde(rename = "user_auth")]
    ///Object of user ID and auth token pair, permitting Plaid to aggregate a user’s accounts
    pub user_auth: ItemImportRequestUserAuth,
    #[serde(rename = "options")]
    ///An optional object to configure `/item/import` request.
    pub options: Option<ItemImportRequestOptions>,
}
impl std::fmt::Display for ItemImportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequestOptions {
    #[serde(rename = "webhook")]
    /**Specifies a webhook URL to associate with an Item. Plaid fires a webhook if credentials fail.
*/
    pub webhook: Option<String>,
}
impl std::fmt::Display for ItemImportRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequestUserAuth {
    #[serde(rename = "user_id")]
    ///Opaque user identifier
    pub user_id: String,
    #[serde(rename = "auth_token")]
    ///Authorization token Plaid will use to aggregate this user’s accounts
    pub auth_token: String,
}
impl std::fmt::Display for ItemImportRequestUserAuth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportResponse {
    #[serde(rename = "access_token")]
    ///The access token associated with the Item data is being requested for.
    pub access_token: String,
    #[serde(rename = "request_id")]
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ItemImportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "item_id")]
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    #[serde(rename = "institution_id")]
    ///The Plaid Institution ID associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    pub institution_id: Option<String>,
    #[serde(rename = "webhook")]
    ///The URL registered to receive webhooks for the Item.
    pub webhook: Option<String>,
    #[serde(rename = "error")]
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    #[serde(rename = "available_products")]
    ///A list of products available for the Item that have not yet been accessed. The contents of this array will be mutually exclusive with `billed_products`.
    pub available_products: Vec<String>,
    #[serde(rename = "billed_products")]
    /**A list of products that have been billed for the Item. The contents of this array will be mutually exclusive with `available_products`. Note - `billed_products` is populated in all environments but only requests in Production are billed. Also note that products that are billed on a pay-per-call basis rather than a pay-per-Item basis, such as `balance`, will not appear here.
*/
    pub billed_products: Vec<String>,
    #[serde(rename = "products")]
    /**A list of authorized products for the Item.
*/
    pub products: Option<Vec<String>>,
    #[serde(rename = "consented_products")]
    /**Beta: A list of products that have gone through consent collection for the Item. Only present for those enabled in the beta.
*/
    pub consented_products: Option<Vec<String>>,
    #[serde(rename = "consent_expiration_time")]
    /**The RFC 3339 timestamp after which the consent provided by the end user will expire. Upon consent expiration, the item will enter the `ITEM_LOGIN_REQUIRED` error state. To circumvent the `ITEM_LOGIN_REQUIRED` error and maintain continuous consent, the end user can reauthenticate via Link’s update mode in advance of the consent expiration time.

Note - This is only relevant for certain OAuth-based institutions. For all other institutions, this field will be null.
*/
    pub consent_expiration_time: Option<String>,
    #[serde(rename = "update_type")]
    /**Indicates whether an Item requires user interaction to be updated, which can be the case for Items with some forms of two-factor authentication.

`background` - Item can be updated in the background

`user_present_required` - Item requires user interaction to be updated*/
    pub update_type: String,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatus {
    #[serde(rename = "investments")]
    ///Information about the last successful and failed investments update for the Item.
    pub investments: Option<ItemStatusInvestments>,
    #[serde(rename = "transactions")]
    ///Information about the last successful and failed transactions update for the Item.
    pub transactions: Option<ItemStatusTransactions>,
    #[serde(rename = "last_webhook")]
    ///Information about the last webhook fired for the Item.
    pub last_webhook: Option<ItemStatusLastWebhook>,
}
impl std::fmt::Display for ItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusNullable(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusTransactions {
    #[serde(rename = "last_successful_update")]
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful transactions update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<String>,
    #[serde(rename = "last_failed_update")]
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed transactions update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<String>,
}
impl std::fmt::Display for ItemStatusTransactions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusInvestments {
    #[serde(rename = "last_successful_update")]
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful investments update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<String>,
    #[serde(rename = "last_failed_update")]
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed investments update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<String>,
}
impl std::fmt::Display for ItemStatusInvestments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusLastWebhook {
    #[serde(rename = "sent_at")]
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of when the webhook was fired.
*/
    pub sent_at: Option<String>,
    #[serde(rename = "code_sent")]
    ///The last webhook code sent.
    pub code_sent: Option<String>,
}
impl std::fmt::Display for ItemStatusLastWebhook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
