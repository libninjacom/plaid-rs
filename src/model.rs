use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/auth/get` results.
    pub options: AuthGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetResponse {
    ///The `accounts` for which numbers are being retrieved.
    pub accounts: Vec<AccountBase>,
    ///An object containing identifying numbers used for making electronic transfers to and from the `accounts`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by any `accounts` for which data has been requested, the array for that type will be empty.
    pub numbers: AuthGetNumbers,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetNumbers {
    ///An array of ACH numbers identifying accounts.
    pub ach: Vec<NumbersAch>,
    ///An array of EFT numbers identifying accounts.
    pub eft: Vec<NumbersEft>,
    ///An array of IBAN numbers identifying accounts.
    pub international: Vec<NumbersInternational>,
    ///An array of BACS numbers identifying accounts.
    pub bacs: Vec<NumbersBacs>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequest {
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: TransactionsGetRequestOptions,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The earliest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    ///The latest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
    ///The number of transactions to fetch.
    pub count: i64,
    ///The number of transactions to skip. The default value is 0.
    pub offset: i64,
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    pub include_original_description: Option<bool>,
    ///Please use [`include_personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-get-request-options-include-personal-finance-category) instead.
    pub include_personal_finance_category_beta: bool,
    /**Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-personal-finance-category) object in the response.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.

We’re introducing Category Rules - a new beta endpoint that will enable you to change the `personal_finance_category` for a transaction based on your users’ needs. When rules are set, the selected category will override the Plaid provided category. To learn more, send a note to transactions-feedback@plaid.com.*/
    pub include_personal_finance_category: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetResponse {
    ///An array containing the `accounts` associated with the Item for which transactions are being returned. Each transaction can be mapped to its corresponding account via the `account_id` field.
    pub accounts: Vec<AccountBase>,
    ///An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the `count` parameter.
    pub transactions: Vec<Transaction>,
    ///The total number of transactions available within the date range specified. If `total_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    pub total_transactions: i64,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRefreshRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: TransactionsRecurringGetRequestOptions,
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequestOptions {
    /**Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-personal-finance-category) object for each transaction stream in the response.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.*/
    pub include_personal_finance_category: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetResponse {
    ///An array of depository transaction streams.
    pub inflow_streams: Vec<TransactionStream>,
    ///An array of expense transaction streams.
    pub outflow_streams: Vec<TransactionStream>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time transaction streams for the given account were updated on
    pub updated_datetime: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesCreateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    /**Personal finance detailed category.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.
*/
    pub personal_finance_category: String,
    ///A representation of transactions rule details.
    pub rule_details: TransactionsRuleDetails,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesCreateResponse {
    ///A representation of a transactions category rule.
    pub rule: TransactionsCategoryRule,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesListRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesListResponse {
    ///A list of the Item's transaction rules
    pub rules: Vec<TransactionsCategoryRule>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///A rule's unique identifier
    pub rule_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    /**The cursor value represents the last update requested. Providing it will cause the response to only return changes after this update.
If omitted, the entire history of updates will be returned, starting with the first-added transactions on the item.
Note: The upper-bound length of this cursor is 256 characters of base64.*/
    pub cursor: String,
    ///The number of transaction updates to fetch.
    pub count: i64,
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: TransactionsSyncRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncRequestOptions {
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    pub include_original_description: Option<bool>,
    /**Include the [`personal_finance_category`](https://plaid.com/docs/api/products/transactions/#transactions-sync-response-added-personal-finance-category) object in the response.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.

We’re introducing Category Rules - a new beta endpoint that will enable you to change the `personal_finance_category` for a transaction based on your users’ needs. When rules are set, the selected category will override the Plaid provided category. To learn more, send a note to transactions-feedback@plaid.com.*/
    pub include_personal_finance_category: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncResponse {
    ///Transactions that have been added to the item since `cursor` ordered by ascending last modified time.
    pub added: Vec<Transaction>,
    ///Transactions that have been modified on the item since `cursor` ordered by ascending last modified time.
    pub modified: Vec<Transaction>,
    ///Transactions that have been removed from the item since `cursor` ordered by ascending last modified time.
    pub removed: Vec<RemovedTransaction>,
    ///Cursor used for fetching any future updates after the latest update provided in this response.
    pub next_cursor: String,
    ///Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`.
    pub has_more: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetRequest {
    ///The total number of Institutions to return.
    pub count: i64,
    ///The number of Institutions to skip.
    pub offset: i64,
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard.

In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<CountryCode>,
    ///An optional object to filter `/institutions/get` results.
    pub options: InstitutionsGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetRequestOptions {
    ///Filter the Institutions based on which products they support.
    pub products: Option<Vec<Products>>,
    ///Specify an array of routing numbers to filter institutions. The response will only return institutions that match all of the routing numbers in the array. Routing number records used for this matching are not comprehensive; failure to match a given routing number to an institution does not mean that the institution is unsupported by Plaid.
    pub routing_numbers: Option<Vec<String>>,
    ///Limit results to institutions with or without OAuth login flows.
    pub oauth: Option<bool>,
    /**When `true`, return the institution's homepage URL, logo and primary brand color.

Note that Plaid does not own any of the logos shared by the API, and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    pub include_optional_metadata: bool,
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: bool,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetResponse {
    ///A list of Plaid institutions
    pub institutions: Vec<Institution>,
    ///The total number of institutions available via this endpoint
    pub total: i64,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchRequest {
    ///The search query. Institutions with names matching the query are returned
    pub query: String,
    ///Filter the Institutions based on whether they support all products listed in `products`. Provide `null` to get institutions regardless of supported products. Note that when `auth` is specified as a product, if you are enabled for Instant Match or Automated Micro-deposits, institutions that support those products will be returned even if `auth` is not present in their product array.
    pub products: Option<Vec<Products>>,
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<CountryCode>,
    ///An optional object to filter `/institutions/search` results.
    pub options: InstitutionsSearchRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchRequestOptions {
    ///Limit results to institutions with or without OAuth login flows.
    pub oauth: Option<bool>,
    ///When true, return the institution's homepage URL, logo and primary brand color.
    pub include_optional_metadata: bool,
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
    ///Additional options that will be used to filter institutions by various Payment Initiation configurations.
    pub payment_initiation: Option<InstitutionsSearchPaymentInitiationOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    ///A unique ID identifying the payment
    pub payment_id: Option<String>,
    ///A unique ID identifying the payment consent
    pub consent_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchResponse {
    ///An array of institutions matching the search criteria
    pub institutions: Vec<Institution>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequest {
    ///The ID of the institution to get details about
    pub institution_id: String,
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<CountryCode>,
    ///Specifies optional parameters for `/institutions/get_by_id`. If provided, must not be `null`.
    pub options: InstitutionsGetByIdRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequestOptions {
    /**When `true`, return an institution's logo, brand color, and URL. When available, the bank's logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format. The default value is `false`.

Note that Plaid does not own any of the logos shared by the API and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    pub include_optional_metadata: bool,
    ///If `true`, the response will include status information about the institution. Default value is `false`.
    pub include_status: bool,
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: bool,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdResponse {
    ///Details relating to a specific financial institution
    pub institution: Institution,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/accounts/get` results.
    pub options: AccountsGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequestOptions {
    ///An array of `account_ids` to retrieve for the Account.
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    /**An array of financial institution accounts associated with the Item.
If `/accounts/balance/get` was called, each account will include real-time balance information.*/
    pub accounts: Vec<AccountBase>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesGetRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesGetResponse {
    ///An array of all of the transaction categories used by Plaid.
    pub categories: Vec<Category>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOverridePassword(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOverrideUsername(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequest {
    ///The ID of the institution the Item will be associated with
    pub institution_id: String,
    ///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub options: SandboxProcessorTokenCreateRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    pub override_username: Option<SandboxOverrideUsername>,
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    pub override_password: Option<SandboxOverridePassword>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateResponse {
    ///A processor token that can be used to call the `/processor/` endpoints.
    pub processor_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequest {
    ///The ID of the institution the Item will be associated with
    pub institution_id: String,
    ///The products to initially pull for the Item. May be any products that the specified `institution_id`  supports. This array may not be empty.
    pub initial_products: Vec<Products>,
    ///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub options: SandboxPublicTokenCreateRequestOptions,
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptions {
    ///Specify a webhook to associate with the new Item.
    pub webhook: String,
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    pub override_username: Option<SandboxOverrideUsername>,
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    pub override_password: Option<SandboxOverridePassword>,
    ///An optional set of parameters corresponding to transactions options.
    pub transactions: SandboxPublicTokenCreateRequestOptionsTransactions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptionsTransactions {
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    ///A public token that can be exchanged for an access token using `/item/public_token/exchange`
    pub public_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The webhook types that can be fired by this test endpoint.
    pub webhook_type: WebhookType,
    ///The webhook codes that can be fired by this test endpoint.
    pub webhook_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WebhookType {
    Auth,
    Holdings,
    InvestmentsTransactions,
    Item,
    Liabilities,
    Transactions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookResponse {
    ///Value is `true`  if the test` webhook_code`  was successfully fired.
    pub webhook_fired: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/accounts/balance/get` results.
    pub options: AccountsBalanceGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item. The default value is `null`.

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.

This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.*/
    pub min_last_updated_datetime: MinLastUpdatedDatetime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MinLastUpdatedDatetime(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/identity/get` results.
    pub options: IdentityGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetResponse {
    ///The accounts for which Identity data has been requested
    pub accounts: Vec<AccountIdentity>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The user's legal name, phone number, email address and address used to perform fuzzy match.
    pub user: IdentityMatchUser,
    ///An optional object to filter /identity/match results
    pub options: IdentityMatchRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchRequestOptions {
    ///An array of `account_ids` to perform fuzzy match
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchUser {
    ///The user's full legal name.
    pub legal_name: Option<String>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email_address: Option<String>,
    ///Data about the components comprising an address.
    pub address: Option<AddressDataNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityMatchResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorAuthGetRequest {
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorAuthGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    ///An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.
    pub numbers: ProcessorNumber,
    ///A single account at a financial institution.
    pub account: AccountBase,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateRequest {
    /**A random key provided by the client, per unique bank transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created.*/
    pub idempotency_key: BankTransferIdempotencyKey,
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: BankTransferType,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: BankTransferNetwork,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: BankTransferAmount,
    ///The currency of the transfer amount – should be set to "USD".
    pub iso_currency_code: String,
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    ///An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters.
    pub custom_tag: Option<String>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<BankTransferMetadata>,
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorNumber {
    ///Identifying information for transferring money to or from a US account via ACH or wire transfer.
    pub ach: Option<NumbersAchNullable>,
    ///Identifying information for transferring money to or from a Canadian bank account via EFT.
    pub eft: Option<NumbersEftNullable>,
    ///Identifying information for transferring money to or from an international bank account via wire transfer.
    pub international: Option<NumbersInternationalNullable>,
    ///Identifying information for transferring money to or from a UK bank account via BACS.
    pub bacs: Option<NumbersBacsNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorIdentityGetRequest {
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorIdentityGetResponse {
    ///Identity information about an account
    pub account: AccountIdentity,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequest {
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
    ///An optional object to filter `/processor/balance/get` results.
    pub options: ProcessorBalanceGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequestOptions {
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.

This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.*/
    pub min_last_updated_datetime: MinLastUpdatedDatetime,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetResponse {
    ///A single account at a financial institution.
    pub account: AccountBase,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetRequest {
    ///The key ID ( `kid` ) from the JWT header.
    pub key_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetResponse {
    ///A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
    pub key: JwkPublicKey,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JwkPublicKey {
    ///The alg member identifies the cryptographic algorithm family used with the key.
    pub alg: String,
    ///The crv member identifies the cryptographic curve used with the key.
    pub crv: String,
    ///The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover.
    pub kid: String,
    ///The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC.
    pub kty: String,
    #[serde(rename = "use")]
    ///The use (public key use) parameter identifies the intended use of the public key.
    pub use_: String,
    ///The x member contains the x coordinate for the elliptic curve point.
    pub x: String,
    ///The y member contains the y coordinate for the elliptic curve point.
    pub y: String,
    ///The timestamp when the key was created, in Unix time.
    pub created_at: i64,
    ///The timestamp when the key expired, in Unix time.
    pub expired_at: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/liabilities/get` results. If provided, `options` cannot be null.
    pub options: LiabilitiesGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetRequestOptions {
    /**A list of accounts to retrieve for the Item.

An error will be returned if a provided `account_id` is not associated with the Item*/
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetResponse {
    ///An array of accounts associated with the Item
    pub accounts: Vec<AccountBase>,
    ///Metadata about the Item.
    pub item: Item,
    ///An object containing liability accounts
    pub liabilities: LiabilitiesObject,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateRequest {
    ///The name of the recipient. We recommend using strings of length 18 or less and avoid special characters to ensure compatibility with all institutions.
    pub name: String,
    ///The International Bank Account Number (IBAN) for the recipient. If BACS data is not provided, an IBAN is required.
    pub iban: Option<String>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacsNullable>,
    ///The optional address of the payment recipient.
    pub address: Option<PaymentInitiationAddress>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateResponse {
    ///A unique ID identifying the recipient
    pub recipient_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationRefundStatus {
    Processing,
    Executed,
    Initiated,
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseResponse {
    ///A unique ID identifying the refund
    pub refund_id: String,
    /**The status of the refund.

`PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.

`INITIATED`: The refund has been successfully initiated.

`EXECUTED`: Indicates that the refund has been successfully executed.

`FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved.*/
    pub status: PaymentInitiationRefundStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetRequest {
    ///The ID of the recipient
    pub recipient_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipient {
    ///The ID of the recipient.
    pub recipient_id: String,
    ///The name of the recipient.
    pub name: String,
    ///The optional address of the payment recipient.
    pub address: Option<PaymentInitiationAddress>,
    ///The International Bank Account Number (IBAN) for the recipient.
    pub iban: Option<String>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacsNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListResponse {
    ///An array of payment recipients created for Payment Initiation
    pub recipients: Vec<PaymentInitiationRecipient>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    ///The ID of the recipient the payment is for.
    pub recipient_id: String,
    ///A reference for the payment. This must be an alphanumeric string with at most 18 characters and must not contain any special characters (since not all institutions support them).
    pub reference: String,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    pub schedule: ExternalPaymentScheduleRequest,
    ///Additional payment options
    pub options: Option<ExternalPaymentOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    ///The ID of the payment to reverse
    pub payment_id: String,
    /**A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: WalletTransactionIdempotencyKey,
    ///A reference for the refund. This must be an alphanumeric string with at most 18 characters and must not contain any special characters or spaces.
    pub reference: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentCreateStatus {
    PaymentStatusInputNeeded,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateResponse {
    ///A unique ID identifying the payment
    pub payment_id: String,
    /**For a payment returned by this endpoint, there is only one possible value:

`PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment*/
    pub status: PaymentInitiationPaymentCreateStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemResetLoginRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemResetLoginResponse {
    ///`true` if the call succeeded
    pub reset_login: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` of the account whose verification status is to be modified
    pub account_id: String,
    ///The verification status to set the account to.
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateRequest {
    ///A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`.
    pub client_user_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreateResponse {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: UserId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetRequest {
    ///The `payment_id` returned from `/payment_initiation/payment/create`.
    pub payment_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationPaymentStatus {
    PaymentStatusInputNeeded,
    PaymentStatusProcessing,
    PaymentStatusInitiated,
    PaymentStatusCompleted,
    PaymentStatusInsufficientFunds,
    PaymentStatusFailed,
    PaymentStatusBlocked,
    PaymentStatusUnknown,
    PaymentStatusExecuted,
    PaymentStatusAuthorising,
    PaymentStatusCancelled,
    PaymentStatusEstablished,
    PaymentStatusRejected,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPayment {
    ///The ID of the payment. Like all Plaid identifiers, the `payment_id` is case sensitive.
    pub payment_id: String,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
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
    pub status: PaymentInitiationPaymentStatus,
    ///The ID of the recipient
    pub recipient_id: String,
    ///A reference for the payment.
    pub reference: String,
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    pub adjusted_reference: Option<String>,
    ///The date and time of the last time the `status` was updated, in IS0 8601 format
    pub last_status_update: String,
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    pub schedule: Option<ExternalPaymentScheduleGet>,
    ///Details about external payment refund
    pub refund_details: Option<ExternalPaymentRefundDetails>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<SenderBacsNullable>,
    ///The International Bank Account Number (IBAN) for the sender, if specified in the `/payment_initiation/payment/create` call.
    pub iban: Option<String>,
    ///Refund IDs associated with the payment.
    pub refund_ids: Vec<String>,
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub scheme: Option<PaymentScheme>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub adjusted_scheme: Option<PaymentScheme>,
    ///The payment consent ID that this payment was initiated with. Is present only when payment was initiated using the payment consent.
    pub consent_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateRequest {
    ///The `payment_id` returned from `/payment_initiation/payment/create`.
    pub payment_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateResponse {
    ///A `payment_token` that can be provided to Link initialization to enter the payment initiation flow
    pub payment_token: String,
    ///The date and time at which the token will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `payment_token` expires after 15 minutes.
    pub payment_token_expiration_time: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateRequest {
    ///The ID of the recipient the payment consent is for. The created consent can be used to transfer funds to this recipient only.
    pub recipient_id: String,
    ///A reference for the payment consent. This must be an alphanumeric string with at most 18 characters and must not contain any special characters.
    pub reference: String,
    ///An array of payment consent scopes.
    pub scopes: Vec<PaymentInitiationConsentScope>,
    ///Limitations that will be applied to payments initiated using the payment consent.
    pub constraints: PaymentInitiationConsentConstraints,
    ///Additional payment consent options
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentCreateResponse {
    ///A unique ID identifying the payment consent.
    pub consent_id: String,
    /**The status of the payment consent.

`UNAUTHORISED`: Consent created, but requires user authorisation.

`REJECTED`: Consent authorisation was rejected by the user and/or the bank.

`AUTHORISED`: Consent is active and ready to be used.

`REVOKED`: Consent has been revoked and can no longer be used.

`EXPIRED`: Consent is no longer valid.*/
    pub status: PaymentInitiationConsentStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentGetRequest {
    ///The `consent_id` returned from `/payment_initiation/consent/create`.
    pub consent_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentGetResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsent {
    ///The consent ID.
    pub consent_id: String,
    /**The status of the payment consent.

`UNAUTHORISED`: Consent created, but requires user authorisation.

`REJECTED`: Consent authorisation was rejected by the user and/or the bank.

`AUTHORISED`: Consent is active and ready to be used.

`REVOKED`: Consent has been revoked and can no longer be used.

`EXPIRED`: Consent is no longer valid.*/
    pub status: PaymentInitiationConsentStatus,
    ///Consent creation timestamp, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: String,
    ///The ID of the recipient the payment consent is for.
    pub recipient_id: String,
    ///A reference for the payment consent.
    pub reference: String,
    ///Limitations that will be applied to payments initiated using the payment consent.
    pub constraints: PaymentInitiationConsentConstraints,
    ///An array of payment consent scopes.
    pub scopes: Vec<PaymentInitiationConsentScope>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationConsentStatus {
    Unauthorised,
    Authorised,
    Revoked,
    Rejected,
    Expired,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentRevokeRequest {
    ///The consent ID.
    pub consent_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentRevokeResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteRequest {
    ///The consent ID.
    pub consent_id: String,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    /**A random key provided by the client, per unique consent payment. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a consent payment fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single payment is created. If the request was successfully processed, it will prevent any payment that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: ConsentPaymentIdempotencyKey,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentPaymentExecuteResponse {
    ///A unique ID identifying the payment
    pub payment_id: String,
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
    pub status: PaymentInitiationPaymentStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListRequest {
    ///The maximum number of payments to return. If `count` is not specified, a maximum of 10 payments will be returned, beginning with the most recent payment before the cursor (if specified).
    pub count: Option<i64>,
    ///A string in RFC 3339 format (i.e. "2019-12-06T22:35:49Z"). Only payments created before the cursor will be returned.
    pub cursor: Option<String>,
    ///The consent ID. If specified, only payments, executed using this consent, will be returned.
    pub consent_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListResponse {
    ///An array of payments that have been created, associated with the given `client_id`.
    pub payments: Vec<PaymentInitiationPayment>,
    ///The value that, when used as the optional `cursor` parameter to `/payment_initiation/payment/list`, will return the next unreturned payment as its first payment.
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateRequest {
    ///An array of access tokens corresponding to the Items that will be included in the report. The `assets` product must have been initialized for the Items during link; the Assets product cannot be added after initialization.
    pub access_tokens: Vec<AccessToken>,
    /**The maximum integer number of days of history to include in the Asset Report. If using Fannie Mae Day 1 Certainty, `days_requested` must be at least 61 for new originations or at least 31 for refinancings.

An Asset Report requested with "Additional History" (that is, with more than 61 days of transaction history) will incur an Additional History fee.*/
    pub days_requested: i64,
    ///An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
    pub options: AssetReportCreateRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateRequestOptions {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    pub client_report_id: Option<String>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    pub webhook: Option<String>,
    ///true to return balance and identity earlier as a fast report. Defaults to false if omitted.
    pub include_fast_report: Option<bool>,
    ///Additional information that can be included in the asset report. Possible values: `"investments"`
    pub products: Vec<String>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: AssetReportUser,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateResponse {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshRequest {
    ///The `asset_report_token` returned by the original call to `/asset_report/create`
    pub asset_report_token: AssetReportRefreshAssetReportToken,
    ///The maximum number of days of history to include in the Asset Report. Must be an integer. If not specified, the value from the original call to `/asset_report/create` will be used.
    pub days_requested: Option<i64>,
    ///An optional object to filter `/asset_report/refresh` results. If provided, cannot be `null`. If not specified, the `options` from the original call to `/asset_report/create` will be used.
    pub options: AssetReportRefreshRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshRequestOptions {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    pub client_report_id: Option<String>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    pub webhook: Option<String>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: AssetReportUser,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshResponse {
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRefreshRequest {
    pub asset_relay_token: String,
    ///The URL registered to receive webhooks when the Asset Report of a Relay Token has been refreshed.
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRefreshResponse {
    pub asset_relay_token: String,
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRemoveRequest {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRemoveResponse {
    ///`true` if the Asset Report was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportFilterRequest {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///The accounts to exclude from the Asset Report, identified by `account_id`.
    pub account_ids_to_exclude: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportFilterResponse {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportGetRequest {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///`true` if you would like to retrieve the Asset Report with Insights, `false` otherwise. This field defaults to `false` if omitted.
    pub include_insights: bool,
    ///`true` to fetch "fast" version of asset report. Defaults to false if omitted.
    pub fast_report: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportGetResponse {
    ///An object representing an Asset Report
    pub report: AssetReport,
    ///If the Asset Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    pub warnings: Vec<Warning>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportPdfGetRequest {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportPdfGetResponse(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateRequest {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///The `auditor_id` of the third party with whom you would like to share the Asset Report.
    pub auditor_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateResponse {
    ///A token that can be shared with a third party auditor to allow them to obtain access to the Asset Report. This token should be stored securely.
    pub audit_copy_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveRequest {
    ///The `audit_copy_token` granting access to the Audit Copy you would like to revoke.
    pub audit_copy_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveResponse {
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayCreateRequest {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///The `secondary_client_id` is the client id of the third party with whom you would like to share the Asset Report.
    pub secondary_client_id: String,
    ///URL to which Plaid will send webhooks when the Secondary Client successfully retrieves an Asset Report by calling `asset_report/relay/get`.
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayCreateResponse {
    ///A token that can be shared with a third party to allow them to access the Asset Report. This token should be stored securely.
    pub asset_relay_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayGetRequest {
    ///The `asset_relay_token` granting access to the Asset Report you would like to get.
    pub asset_relay_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRemoveRequest {
    ///The `asset_relay_token` you would like to revoke.
    pub asset_relay_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRelayRemoveResponse {
    ///`true` if the Asset Relay token was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/investments/holdings/get` results. If provided, must not be `null`.
    pub options: InvestmentHoldingsGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentHoldingsGetRequestOptions {
    ///An array of `account_id`s to retrieve for the Item. An error will be returned if a provided `account_id` is not associated with the Item.
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetResponse {
    ///The accounts associated with the Item
    pub accounts: Vec<AccountBase>,
    ///The holdings belonging to investment accounts associated with the Item. Details of the securities in the holdings are provided in the `securities` field.
    pub holdings: Vec<Holding>,
    ///Objects describing the securities held in the accounts associated with the Item.
    pub securities: Vec<Security>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
    ///An optional object to filter `/investments/transactions/get` results. If provided, must be non-`null`.
    pub options: InvestmentsTransactionsGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequestOptions {
    ///An array of `account_ids` to retrieve for the Item.
    pub account_ids: Vec<String>,
    /**The number of transactions to fetch.
*/
    pub count: i64,
    ///The number of transactions to skip when fetching transaction history
    pub offset: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///The accounts for which transaction history is being fetched.
    pub accounts: Vec<AccountBase>,
    ///All securities for which there is a corresponding transaction being fetched.
    pub securities: Vec<Security>,
    ///The transactions being fetched
    pub investment_transactions: Vec<InvestmentTransaction>,
    ///The total number of transactions available within the date range specified. If `total_investment_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.'
    pub total_investment_transactions: i64,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
    ///The processor you are integrating with.
    pub processor: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorTokenCreateResponse {
    ///The `processor_token` that can then be used by the Plaid partner to make API requests
    pub processor_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    ///A token that can be sent to Stripe for use in making API calls to Plaid
    pub stripe_bank_account_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorApexProcessorTokenCreateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequest {
    ///Access token for the target Item, typically provided in the Import Item response.
    pub target_access_token: String,
    ///Plaid Account ID that specifies the target bank account. This account will become the recipient for a user's direct deposit.
    pub target_account_id: String,
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: Option<String>,
    ///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub options: DepositSwitchCreateRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequestOptions {
    /**The URL registered to receive webhooks when the status of a deposit switch request has changed.
*/
    pub webhook: Option<String>,
    ///An array of access tokens corresponding to transaction items to use when attempting to match the user to their Payroll Provider. These tokens must be created by the same client id as the one creating the switch, and have access to the transactions product.
    pub transaction_item_access_tokens: Vec<AccessToken>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateResponse {
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateRequest {
    ///The ID of the deposit switch
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateResponse {
    ///Deposit switch token, used to initialize Link for the Deposit Switch product
    pub deposit_switch_token: String,
    ///Expiration time of the token, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub deposit_switch_token_expiration_time: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetRequest {
    ///A `link_token` from a previous invocation of `/link/token/create`
    pub link_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    ///The name of your application, as it should be displayed in Link. Maximum length of 30 characters. If a value longer than 30 characters is provided, Link will display "This Application" instead.
    pub client_name: String,
    /**The language that Link should be displayed in.

Supported languages are:
- English (`'en'`)
- French (`'fr'`)
- Spanish (`'es'`)
- Dutch (`'nl'`)
- German(`'de'`)

When using a Link customization, the language configured here must match the setting in the customization, or the customization will not be applied.*/
    pub language: String,
    /**Specify an array of Plaid-supported country codes using the ISO-3166-1 alpha-2 country code standard. Institutions from all listed countries will be shown.  Supported country codes are: `US`, `CA`, `DE`, `ES`, `FR`, `GB`, `IE`, `IT`, `NL`. For a complete mapping of supported products by country, see https://plaid.com/global/.

If Link is launched with multiple country codes, only products that you are enabled for in all countries will be used by Link. Note that while all countries are enabled by default in Sandbox and Development, in Production only US and Canada are enabled by default. To gain access to European institutions in the Production environment, [file a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access) via the Plaid dashboard. If you initialize with a European country code, your users will see the European consent panel during the Link flow.

If using a Link customization, make sure the country codes in the customization match those specified in `country_codes`. If both `country_codes` and a Link customization are used, the value in `country_codes` may override the value in the customization.

If using the Auth features Instant Match, Same-day Micro-deposits, or Automated Micro-deposits, `country_codes` must be set to `['US']`.*/
    pub country_codes: Vec<CountryCode>,
    ///An object specifying information about the end user who will be linking their account.
    pub user: LinkTokenCreateRequestUser,
    /**List of Plaid product(s) you wish to use. If launching Link in update mode, should be omitted; required otherwise.

`balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically be initialized when any other product is initialized.

The products specified here will determine which institutions will be available to your users in Link. Only institutions that support *all* requested products can be selected; a if a user attempts to select an institution that does not support a listed product, a "Connectivity not supported" error message will appear in Link. To maximize the number of institutions available, initialize Link with the minimal product set required for your use case. Additional products can be added after Link initialization by calling the relevant endpoints. For details and exceptions, see [Choosing when to initialize products](https://plaid.com/docs/link/best-practices/#choosing-when-to-initialize-products).

Note that, unless you have opted to disable Instant Match support, institutions that support Instant Match will also be shown in Link if `auth` is specified as a product, even though these institutions do not contain `auth` in their product array.

In Production, you will be billed for each product that you specify when initializing Link. Note that a product cannot be removed from an Item once the Item has been initialized with that product. To stop billing on an Item for subscription-based products, such as Liabilities, Investments, and Transactions, remove the Item via `/item/remove`.*/
    pub products: Vec<Products>,
    /**(Beta) This field has no effect unless you are participating in the Product Scope Transparency beta program.
List of additional Plaid product(s) you wish to collect consent for. These products will not be billed until you start using them by calling the relevant endpoints.

`balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically have consent collected.

Institutions that do not support these products will still be shown in Link*/
    pub additional_consented_products: Vec<Products>,
    ///The destination URL to which any webhooks should be sent.
    pub webhook: String,
    ///The `access_token` associated with the Item to update, used when updating or modifying an existing `access_token`. Used when launching Link in update mode, when completing the Same-day (manual) Micro-deposit flow, or (optionally) when initializing Link as part of the Payment Initiation (UK and Europe) flow.
    pub access_token: String,
    ///The name of the Link customization from the Plaid Dashboard to be applied to Link. If not specified, the `default` customization will be used. When using a Link customization, the language in the customization must match the language selected via the `language` parameter, and the countries in the customization should match the country codes selected via `country_codes`.
    pub link_customization_name: String,
    ///A URI indicating the destination where a user should be forwarded after completing the Link flow; used to support OAuth authentication flows when launching Link in the browser or via a webview. The `redirect_uri` should not contain any query parameters. When used in Production or Development, must be an https URI. To specify any subdomain, use `*` as a wildcard character, e.g. `https://*.example.com/oauth.html`. If `android_package_name` is specified, this field should be left blank.  Note that any redirect URI must also be added to the Allowed redirect URIs list in the [developer dashboard](https://dashboard.plaid.com/team/api).
    pub redirect_uri: String,
    ///The name of your app's Android package. Required if using the `link_token` to initialize Link on Android. When creating a `link_token` for initializing Link on other platforms, this field must be left blank. Any package name specified here must also be added to the Allowed Android package names setting on the [developer dashboard](https://dashboard.plaid.com/team/api).
    pub android_package_name: String,
    ///A map containing data used to highlight institutions in Link.
    pub institution_data: LinkTokenCreateInstitutionData,
    /**By default, Link will provide limited account filtering: it will only display Institutions that are compatible with all products supplied in the `products` parameter of `/link/token/create`, and, if `auth` is specified in the `products` array, will also filter out accounts other than `checking` and `savings` accounts on the Account Select pane. You can further limit the accounts shown in Link by using `account_filters` to specify the account subtypes to be shown in Link. Only the specified subtypes will be shown. This filtering applies to both the Account Select view (if enabled) and the Institution Select view. Institutions that do not support the selected subtypes will be omitted from Link. To indicate that all subtypes should be shown, use the value `"all"`. If the `account_filters` filter is used, any account type for which a filter is not specified will be entirely omitted from Link. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).

For institutions using OAuth, the filter will not affect the list of accounts shown by the bank in the OAuth window.
*/
    pub account_filters: LinkTokenAccountFilters,
    ///Configuration parameters for EU flows
    pub eu_config: LinkTokenEuConfig,
    ///Used for certain Europe-only configurations, as well as certain legacy use cases in other regions.
    pub institution_id: String,
    ///Specifies options for initializing Link for use with the Payment Initiation (Europe) product. This field is required if `payment_initiation` is included in the `products` array.
    pub payment_initiation: LinkTokenCreateRequestPaymentInitiation,
    ///Specifies options for initializing Link for use with the Deposit Switch (beta) product. This field is required if `deposit_switch` is included in the `products` array.
    pub deposit_switch: LinkTokenCreateRequestDepositSwitch,
    ///Specifies options for initializing Link for use with the Income (beta) product. This field is required if `income_verification` is included in the `products` array.
    pub income_verification: LinkTokenCreateRequestIncomeVerification,
    ///Specifies options for initializing Link for use with the Auth product. This field can be used to enable or disable extended Auth flows for the resulting Link session. Omitting any field will result in a default that can be configured by your account manager.
    pub auth: LinkTokenCreateRequestAuth,
    ///Specifies options for initializing Link for use with the Transfer product.
    pub transfer: LinkTokenCreateRequestTransfer,
    ///Specifies options for initializing Link for [update mode](https://plaid.com/docs/link/update-mode).
    pub update: LinkTokenCreateRequestUpdate,
    ///Specifies option for initializing Link for use with the Identity Verification product.
    pub identity_verification: LinkTokenCreateRequestIdentityVerification,
    ///A user token generated using `/user/create`. Any item created during the link session will be associated with the user.
    pub user_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenAccountFilters {
    ///A filter to apply to `depository`-type accounts
    pub depository: DepositoryFilter,
    ///A filter to apply to `credit`-type accounts
    pub credit: CreditFilter,
    ///A filter to apply to `loan`-type accounts
    pub loan: LoanFilter,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: InvestmentFilter,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenEuConfig {
    ///If `true`, open Link without an initial UI. Defaults to `false`.
    pub headless: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestPaymentInitiation {
    ///The `payment_id` provided by the `/payment_initiation/payment/create` endpoint.
    pub payment_id: String,
    ///The `consent_id` provided by the `/payment_initiation/consent/create` endpoint.
    pub consent_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestDepositSwitch {
    ///The `deposit_switch_id` provided by the `/deposit_switch/create` endpoint.
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestTransfer {
    ///The `id` returned by the `/transfer/intent/create` endpoint.
    pub intent_id: String,
    ///The `payment_profile_id` returned by the `/payment_profile/create` endpoint.
    pub payment_profile_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUserStatedIncomeSource {
    ///The employer corresponding to an income source specified by the user
    pub employer: String,
    ///The income category for a specified income source
    pub category: UserStatedIncomeSourceCategory,
    ///The income amount paid per cycle for a specified income source
    pub pay_per_cycle: f64,
    ///The income amount paid annually for a specified income source
    pub pay_annual: f64,
    ///The pay type - `GROSS`, `NET`, or `UNKNOWN` for a specified income source
    pub pay_type: UserStatedIncomeSourcePayType,
    ///The pay frequency of a specified income source
    pub pay_frequency: UserStatedIncomeSourceFrequency,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourceCategory {
    Other,
    Salary,
    Unemployment,
    Cash,
    GigEconomy,
    Rental,
    ChildSupport,
    Military,
    Retirement,
    LongTermDisability,
    BankInterest,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourceFrequency {
    Unknown,
    Weekly,
    Biweekly,
    SemiMonthly,
    Monthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatedIncomeSourcePayType {
    Unknown,
    Gross,
    Net,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAuth {
    ///Specifies whether Auth Type Select is enabled for the Link session, allowing the end user to choose between linking instantly or manually prior to selecting their financial institution. Note that this can only be true if `same_day_microdeposits_enabled` is set to true.
    pub auth_type_select_enabled: bool,
    ///Specifies whether the Link session is enabled for the Automated Micro-deposits flow.
    pub automated_microdeposits_enabled: bool,
    ///Specifies whether the Link session is enabled for the Instant Match flow.
    pub instant_match_enabled: bool,
    ///Specifies whether the Link session is enabled for the Same Day Micro-deposits flow.
    pub same_day_microdeposits_enabled: bool,
    ///This field has been deprecated in favor of `auth_type_select_enabled`.
    pub flow_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIdentityVerification {
    ///ID of the associated Identity Verification template.
    pub template_id: IdentityVerificationTemplateId,
    pub consent: serde_json::Value,
    /**A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.

If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.*/
    pub gave_consent: IdentityVerificationConsent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateInstitutionData {
    ///The routing number of the bank to highlight.
    pub routing_number: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUser {
    ///A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`. It is currently used as a means of searching logs for the given user in the Plaid Dashboard.
    pub client_user_id: String,
    ///The user's full legal name. Currently used only to support certain legacy flows.
    pub legal_name: String,
    ///The user's full name. Optional if using the [Identity Verification](https://plaid.com/docs/api/products/identity-verification) product; if not using Identity Verification, this field is not allowed. Users will not be asked for their name when this field is provided.
    pub name: serde_json::Value,
    ///The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. This field is optional, but required to enable the [returning user experience](https://plaid.com/docs/link/returning-user).
    pub phone_number: String,
    /**The date and time the phone number was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This field is optional, but required to enable any [returning user experience](https://plaid.com/docs/link/returning-user).

 Only pass a verification time for a phone number that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.

 Example: `2020-01-01T00:00:00Z`
*/
    pub phone_number_verified_time: String,
    ///The user's email address. This field is optional, but required to enable the [pre-authenticated returning user flow](https://plaid.com/docs/link/returning-user/#enabling-the-returning-user-experience).
    pub email_address: String,
    /**The date and time the email address was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This is an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user).

 Only pass a verification time for an email address that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.

 Example: `2020-01-01T00:00:00Z`*/
    pub email_address_verified_time: String,
    ///To be provided in the format "ddd-dd-dddd". Not currently used.
    pub ssn: String,
    ///To be provided in the format "yyyy-mm-dd". Not currently used.
    pub date_of_birth: String,
    ///Home address for the user.
    pub address: Option<UserAddress>,
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub id_number: Option<UserIdNumber>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUpdate {
    ///If `true`, enables [update mode with Account Select](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).
    pub account_selection_enabled: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAccountSubtypes {
    ///A filter to apply to `depository`-type accounts
    pub depository: LinkTokenCreateDepositoryFilter,
    ///A filter to apply to `credit`-type accounts
    pub credit: LinkTokenCreateCreditFilter,
    ///A filter to apply to `loan`-type accounts
    pub loan: LinkTokenCreateLoanFilter,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: LinkTokenCreateInvestmentFilter,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateDepositoryFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: DepositoryAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateCreditFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: CreditAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateLoanFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: LoanAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateInvestmentFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: InvestmentAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetResponse {
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    ///The creation timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: Option<String>,
    ///The expiration timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub expiration: Option<String>,
    ///An object specifying the arguments originally provided to the `/link/token/create` call.
    pub metadata: LinkTokenGetMetadataResponse,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetMetadataResponse {
    ///The `products` specified in the `/link/token/create` call.
    pub initial_products: Vec<Products>,
    ///The `webhook` specified in the `/link/token/create` call.
    pub webhook: Option<String>,
    ///The `country_codes` specified in the `/link/token/create` call.
    pub country_codes: Vec<CountryCode>,
    ///The `language` specified in the `/link/token/create` call.
    pub language: Option<String>,
    ///A map containing data used to highlight institutions in Link.
    pub institution_data: LinkTokenCreateInstitutionData,
    /**The `account_filters` specified in the original call to `/link/token/create`.
*/
    pub account_filters: AccountFiltersResponse,
    ///The `redirect_uri` specified in the `/link/token/create` call.
    pub redirect_uri: Option<String>,
    ///The `client_name` specified in the `/link/token/create` call.
    pub client_name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateResponse {
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    ///The expiration date for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `link_token` created to generate a `public_token` that will be exchanged for a new `access_token` expires after 4 hours. A `link_token` created for an existing Item (such as when updating an existing `access_token` by launching Link in update mode) expires after 30 minutes.
    pub expiration: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidError(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: String,
    ///The particular error code. Safe for programmatic use.
    pub error_code: String,
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    /**A user-friendly representation of the error code. `null` if the error is not related to user action.

This may change over time and is not safe for programmatic use.*/
    pub display_message: Option<String>,
    ///A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    pub request_id: String,
    /**In the Assets product, a request can pertain to more than one Item. If an error is returned for such a request, `causes` will return an array of errors containing a breakdown of these errors on the individual Item level, if any can be identified.

`causes` will only be provided for the `error_type` `ASSET_REPORT_ERROR`. `causes` will also not be populated inside an error nested within a `warning` object.*/
    pub causes: Vec<serde_json::Value>,
    ///The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook.
    pub status: Option<f64>,
    ///The URL of a Plaid documentation page with more information about the error
    pub documentation_url: String,
    ///Suggested steps for resolving the error
    pub suggested_action: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Investment,
    Credit,
    Depository,
    Loan,
    Brokerage,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum OverrideAccountType {
    Investment,
    Credit,
    Depository,
    Loan,
    Payroll,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBase {
    /**Plaid’s unique identifier for the account. This value will not change unless Plaid can't reconcile the account with the data returned by the financial institution. This may occur, for example, when the name of the account changes. If this happens a new `account_id` will be assigned to the account.

The `account_id` can also change if the `access_token` is deleted and the same credentials that were used to generate that `access_token` are used to generate a new `access_token` on a later date. In that case, the new `account_id` will be different from the old `account_id`.

If an account with a specific `account_id` disappears instead of changing, the account is likely closed. Closed accounts are not returned by the Plaid API.

Like all Plaid identifiers, the `account_id` is case sensitive.*/
    pub account_id: String,
    ///A set of fields describing the balance for an account. Balance information may be cached unless the balance object was returned by `/accounts/balance/get`.
    pub balances: AccountBalance,
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    pub mask: Option<String>,
    ///The name of the account, either assigned by the user or by the financial institution itself
    pub name: String,
    ///The official name of the account as given by the financial institution
    pub official_name: Option<String>,
    #[serde(rename = "type")]
    /**`investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    pub type_: AccountType,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    pub subtype: Option<AccountSubtype>,
    /**The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.

`pending_automatic_verification`: The Item is pending automatic verification

`pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the two amounts.

`automatically_verified`: The Item has successfully been automatically verified	

`manually_verified`: The Item has successfully been manually verified

`verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.

`verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.	
	*/
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    /**The amount of funds available to be withdrawn from the account, as determined by the financial institution.

For `credit`-type accounts, the `available` balance typically equals the `limit` less the `current` balance, less any pending outflows plus any pending inflows.

For `depository`-type accounts, the `available` balance typically equals the `current` balance less any pending outflows plus any pending inflows. For `depository`-type accounts, the `available` balance does not include the overdraft limit.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the `available` balance is the total cash available to withdraw as presented by the institution.

Note that not all institutions calculate the `available`  balance. In the event that `available` balance is unavailable, Plaid will return an `available` balance value of `null`.

Available balance may be cached and is not guaranteed to be up-to-date in realtime unless the value was returned by `/accounts/balance/get`.

If `current` is `null` this field is guaranteed not to be `null`.*/
    pub available: Option<f64>,
    /**The total amount of funds in or owed by the account.

For `credit`-type accounts, a positive balance indicates the amount owed; a negative amount indicates the lender owing the account holder.

For `loan`-type accounts, the current balance is the principal remaining on the loan, except in the case of student loan accounts at Sallie Mae (`ins_116944`). For Sallie Mae student loans, the account's balance includes both principal and any outstanding interest.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the current balance is the total value of assets as presented by the institution.

Note that balance information may be cached unless the value was returned by `/accounts/balance/get`; if the Item is enabled for Transactions, the balance will be at least as recent as the most recent Transaction update. If you require realtime balance information, use the `available` balance as provided by `/accounts/balance/get`.

When returned by `/accounts/balance/get`, this field may be `null`. When this happens, `available` is guaranteed not to be `null`.*/
    pub current: Option<f64>,
    /**For `credit`-type accounts, this represents the credit limit.

For `depository`-type accounts, this represents the pre-arranged overdraft limit, which is common for current (checking) accounts in Europe.

In North America, this field is typically only available for `credit`-type accounts.*/
    pub limit: Option<f64>,
    ///The ISO-4217 currency code of the balance. Always null if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the balance. Always null if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `unofficial_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time that the balance for the given account has been updated

This is currently only provided when the `min_last_updated_datetime` is passed when calling `/accounts/balance/get` for `ins_128026` (Capital One).*/
    pub last_updated_datetime: Option<String>,
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
    Brokerage,
    CashIsa,
    CryptoExchange,
    EducationSavingsAccount,
    Ebt,
    FixedAnnuity,
    Gic,
    HealthReimbursementArrangement,
    Hsa,
    Isa,
    Ira,
    Lif,
    LifeInsurance,
    Lira,
    Lrif,
    Lrsp,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    Other,
    OtherInsurance,
    OtherAnnuity,
    Prif,
    Rdsp,
    Resp,
    Rlif,
    Rrif,
    Pension,
    ProfitSharingPlan,
    Retirement,
    Roth,
    Roth401K,
    Rrsp,
    SepIra,
    SimpleIra,
    Sipp,
    StockPlan,
    ThriftSavingsPlan,
    Tfsa,
    Trust,
    Ugma,
    Utma,
    VariableAnnuity,
    CreditCard,
    Paypal,
    Cd,
    Checking,
    Savings,
    MoneyMarket,
    Prepaid,
    Auto,
    Business,
    Commercial,
    Construction,
    Consumer,
    HomeEquity,
    Loan,
    Mortgage,
    Overdraft,
    LineOfCredit,
    Student,
    CashManagement,
    Keogh,
    MutualFund,
    Recurring,
    Rewards,
    SafeDeposit,
    Sarsep,
    Payroll,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersAch {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    /**The ACH account number for the account.

Note that when using OAuth with Chase Bank (`ins_56`), Chase will issue "tokenized" routing and account numbers, which are not the user's actual account and routing numbers. These tokenized numbers should work identically to normal account and routing numbers. The digits returned in the `mask` field will continue to reflect the actual account number, rather than the tokenized account number; for this reason, when displaying account numbers to the user to help them identify their account in your UI, always use the `mask` rather than truncating the `account` number. If a user revokes their permissions to your app, the tokenized numbers will continue to work for ACH deposits, but not withdrawals.*/
    pub account: String,
    ///The ACH routing number for the account. If the institution is `ins_56`, this may be a tokenized routing number. For more information, see the description of the `account` field.
    pub routing: String,
    ///The wire transfer routing number for the account, if available
    pub wire_routing: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersAchNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersEft {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The EFT account number for the account
    pub account: String,
    ///The EFT institution number for the account
    pub institution: String,
    ///The EFT branch number for the account
    pub branch: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersEftNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternational {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The International Bank Account Number (IBAN) for the account
    pub iban: String,
    ///The Bank Identifier Code (BIC) for the account
    pub bic: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternationalNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersBacs {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The BACS account number for the account
    pub account: String,
    ///The BACS sort code for the account
    pub sort_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersBacsNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternationalIban {
    ///International Bank Account Number (IBAN).
    pub iban: NumbersIban,
    ///The Business Identifier Code, also known as SWIFT code, for this bank account.
    pub bic: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersIban(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersIbanNullable(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientBacs {
    ///The account number of the account. Maximum of 10 characters.
    pub account: String,
    ///The 6-character sort code of the account.
    pub sort_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientBacsNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SenderBacsNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationOptionalRestrictionBacs(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPullId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovedTransaction {
    ///The ID of the removed transaction.
    pub transaction_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRuleDetails {
    ///Transaction field for which the rule is defined.
    pub field: TransactionsRuleField,
    #[serde(rename = "type")]
    /**Transaction rule's match type. For TRANSACTION_ID field, EXACT_MATCH is available.
Matches are case sensitive.
*/
    pub type_: TransactionsRuleType,
    /**For TRANSACTION_ID field, provide transaction_id. For NAME field, provide a string pattern.
*/
    pub query: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionsRuleField {
    TransactionId,
    Name,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionsRuleType {
    ExactMatch,
    SubstringMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsCategoryRule {
    ///A unique identifier of the rule created
    pub id: String,
    ///A unique identifier of the item the rule was created for
    pub item_id: String,
    /**Date and time when a rule was created in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DDTHH:mm:ssZ` ).
*/
    pub created_at: String,
    /**Personal finance category unique identifier.

In the personal finance category taxonomy, this field is represented by the detailed category field.
*/
    pub personal_finance_category: String,
    ///A representation of transactions rule details.
    pub rule_details: TransactionsRuleDetails,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionBase {
    /**Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.

`digital:` transactions that took place online.

`place:` transactions that were made at a physical location.

`special:` transactions that relate to banks, e.g. fees or deposits.

`unresolved:` transactions that do not fit into the other three types.
*/
    pub transaction_type: String,
    ///The ID of a posted transaction's associated pending transaction, where applicable.
    pub pending_transaction_id: Option<String>,
    /**The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub category_id: Option<String>,
    /**A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).

If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub category: Option<Vec<String>>,
    ///A representation of where a transaction took place
    pub location: Location,
    /**Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub payment_meta: PaymentMeta,
    ///The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    pub account_owner: Option<String>,
    /**The merchant name or transaction description.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub name: String,
    ///The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`.
    pub original_description: Option<String>,
    ///The ID of the account in which this transaction occurred.
    pub account_id: String,
    ///The settled value of the transaction, denominated in the transactions's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    pub date: String,
    ///When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    pub pending: bool,
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: String,
    ///The merchant name, as extracted by Plaid from the `name` field.
    pub merchant_name: Option<String>,
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    ///The street address where the transaction occurred.
    pub address: Option<String>,
    ///The city where the transaction occurred.
    pub city: Option<String>,
    ///The region or state where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `state`.
    pub region: Option<String>,
    ///The postal code where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code where the transaction occurred.
    pub country: Option<String>,
    ///The latitude where the transaction occurred.
    pub lat: Option<f64>,
    ///The longitude where the transaction occurred.
    pub lon: Option<f64>,
    ///The merchant defined store number where the transaction occurred.
    pub store_number: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStream {
    ///The ID of the account to which the stream belongs
    pub account_id: String,
    ///A unique id for the stream
    pub stream_id: String,
    ///The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    pub category_id: String,
    ///A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    pub category: Vec<String>,
    ///A description of the transaction stream.
    pub description: String,
    ///The merchant associated with the transaction stream.
    pub merchant_name: Option<String>,
    ///The posted date of the earliest transaction in the stream.
    pub first_date: String,
    ///The posted date of the latest transaction in the stream.
    pub last_date: String,
    /**Describes the frequency of the transaction stream.

`WEEKLY`: Assigned to a transaction stream that occurs approximately every week.

`BIWEEKLY`: Assigned to a transaction stream that occurs approximately every 2 weeks.

`SEMI_MONTHLY`: Assigned to a transaction stream that occurs approximately twice per month. This frequency is typically seen for inflow transaction streams.

`MONTHLY`: Assigned to a transaction stream that occurs approximately every month.

`UNKNOWN`: Assigned to a transaction stream that does not fit any of the pre-defined frequencies.*/
    pub frequency: RecurringTransactionFrequency,
    ///An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    pub transaction_ids: Vec<String>,
    ///Object with data pertaining to an amount on the transaction stream.
    pub average_amount: TransactionStreamAmount,
    ///Object with data pertaining to an amount on the transaction stream.
    pub last_amount: TransactionStreamAmount,
    ///Indicates whether the transaction stream is still live.
    pub is_active: bool,
    /**The current status of the transaction stream.

`MATURE`: A `MATURE` recurring stream should have at least 3 transactions and happen on a regular cadence.

`EARLY_DETECTION`: When a recurring transaction first appears in the transaction history and before it fulfills the requirement of a mature stream, the status will be `EARLY_DETECTION`.

`TOMBSTONED`: A stream that was previously in the `EARLY_DETECTION` status will move to the `TOMBSTONED` status when no further transactions were found at the next expected date.

`UNKNOWN`: A stream is assigned an `UNKNOWN` status when none of the other statuses are applicable.*/
    pub status: TransactionStreamStatus,
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.*/
    pub personal_finance_category: Option<PersonalFinanceCategory>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStreamAmount {
    ///Represents the numerical value of an amount.
    pub amount: f64,
    /**The ISO-4217 currency code of the amount. Always `null` if `unofficial_currency_code` is non-`null`.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub iso_currency_code: Option<String>,
    ///The unofficial currency code of the amount. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecurringTransactionFrequency {
    Unknown,
    Weekly,
    Biweekly,
    SemiMonthly,
    Monthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionStreamStatus {
    Unknown,
    Mature,
    EarlyDetection,
    Tombstoned,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    ///Unique identifier for the institution
    pub institution_id: String,
    ///The official name of the institution
    pub name: String,
    ///A list of the Plaid products supported by the institution. Note that only institutions that support Instant Auth will return `auth` in the product array; institutions that do not list `auth` may still support other Auth methods such as Instant Match or Automated Micro-deposit Verification. To identify institutions that support those methods, use the `auth_metadata` object. For more details, see [Full Auth coverage](https://plaid.com/docs/auth/coverage/).
    pub products: Vec<Products>,
    ///A list of the country codes supported by the institution.
    pub country_codes: Vec<CountryCode>,
    ///The URL for the institution's website
    pub url: Option<String>,
    ///Hexadecimal representation of the primary color used by the institution
    pub primary_color: Option<String>,
    ///Base64 encoded representation of the institution's logo
    pub logo: Option<String>,
    ///A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution.
    pub routing_numbers: Vec<String>,
    ///Indicates that the institution has an OAuth login flow.
    pub oauth: bool,
    /**The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.

Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment.
*/
    pub status: Option<InstitutionStatus>,
    ///Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
    pub payment_initiation_metadata: Option<PaymentInitiationMetadata>,
    ///Metadata that captures information about the Auth features of an institution.
    pub auth_metadata: Option<AuthMetadata>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionStatus {
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub item_logins: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub transactions_updates: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub auth: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub identity: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub investments_updates: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub liabilities_updates: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub liabilities: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub investments: ProductStatus,
    ///Details of recent health incidents associated with the institution.
    pub health_incidents: Option<Vec<HealthIncident>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CountryCode {
    Us,
    Gb,
    Es,
    Nl,
    Fr,
    Ie,
    Ca,
    De,
    It,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMeta {
    ///The transaction reference number supplied by the financial institution.
    pub reference_number: Option<String>,
    ///The ACH PPD ID for the payer.
    pub ppd_id: Option<String>,
    ///For transfers, the party that is receiving the transaction.
    pub payee: Option<String>,
    ///The party initiating a wire transfer. Will be `null` if the transaction is not a wire transfer.
    pub by_order_of: Option<String>,
    ///For transfers, the party that is paying the transaction.
    pub payer: Option<String>,
    ///The type of transfer, e.g. 'ACH'
    pub payment_method: Option<String>,
    ///The name of the payment processor
    pub payment_processor: Option<String>,
    ///The payer-supplied description of the transfer.
    pub reason: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionCode {
    Adjustment,
    Atm,
    BankCharge,
    BillPayment,
    Cash,
    Cashback,
    Cheque,
    DirectDebit,
    Interest,
    Purchase,
    StandingOrder,
    Transfer,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    ///An identifying number for the category. `category_id` is a Plaid-specific identifier and does not necessarily correspond to merchant category codes.
    pub category_id: String,
    ///`place` for physical transactions or `special` for other transactions such as bank charges.
    pub group: String,
    ///A hierarchical array of the categories to which this `category_id` belongs.
    pub hierarchy: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalFinanceCategory {
    ///A high level category that communicates the broad category of the transaction.
    pub primary: String,
    ///A granular category conveying the transaction's intent. This field can also be used as a unique identifier for the category.
    pub detailed: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenNullable(pub Option<String>);
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
    ///`SCREENING`
    pub webhook_type: String,
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    ///The ID of the associated screening.
    pub screening_id: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningStatusUpdatedWebhook {
    ///`ENTITY_SCREENING`
    pub webhook_type: String,
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    ///The ID of the associated screening.
    pub screening_id: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationStepUpdatedWebhook {
    ///`IDENTITY_VERIFCATION`
    pub webhook_type: String,
    ///`STEP_UPDATED`
    pub webhook_code: String,
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRetriedWebhook {
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
    ///`RETRIED`
    pub webhook_code: String,
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationStatusUpdatedWebhook {
    ///`IDENTITY_VERIFICATION`
    pub webhook_type: String,
    ///`STATUS_UPDATED`
    pub webhook_code: String,
    ///The ID of the associated Identity Verification attempt.
    pub identity_verification_id: serde_json::Value,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRemovedWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`TRANSACTIONS_REMOVED`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///An array of `transaction_ids` corresponding to the removed transactions
    pub removed_transactions: Vec<String>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new transactions detected since the last time this webhook was fired.
    pub new_transactions: f64,
    ///The `item_id` of the Item the webhook relates to.
    pub item_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncUpdatesAvailableWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`SYNC_UPDATES_AVAILABLE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///Indicates if initial pull information is available.
    pub initial_update_complete: bool,
    ///Indicates if historical pull information is available.
    pub historical_update_complete: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringTransactionsUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`RECURRING_TRANSACTIONS_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///A list of `account_ids` for accounts that have new or updated recurring transactions data.
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityDefaultUpdateWebhook {
    ///`IDENTITY`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    /**An object with keys of `account_id`'s that are mapped to their respective identity attributes that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["PHONES"] }`
*/
    pub account_ids_with_updated_identity: AccountIdsWithUpdatedIdentity,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountIdsWithUpdatedIdentity {}
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityUpdateTypes {
    Phones,
    Addresses,
    Emails,
    Names,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`HISTORICAL_UPDATE`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new, unfetched transactions available
    pub new_transactions: f64,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`INITIAL_UPDATE`
    pub webhook_code: String,
    ///The error code associated with the webhook.
    pub error: Option<String>,
    ///The number of new, unfetched transactions available.
    pub new_transactions: f64,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    ///The phone number.
    pub data: String,
    ///When `true`, identifies the phone number as the primary number on an account.
    pub primary: bool,
    #[serde(rename = "type")]
    ///The type of phone number.
    pub type_: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    ///The email address.
    pub data: String,
    ///When `true`, identifies the email address as the primary email on an account.
    pub primary: bool,
    #[serde(rename = "type")]
    ///The type of email account as described by the financial institution.
    pub type_: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    ///Data about the components comprising an address.
    pub data: AddressData,
    ///When `true`, identifies the address as the primary address on an account.
    pub primary: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDataNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressData {
    ///The full city name
    pub city: String,
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBalance {
    ///The date of the calculated historical balance, in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD)
    pub date: String,
    /**The total amount of funds in the account, calculated from the `current` balance in the `balance` object by subtracting inflows and adding back outflows according to the posted date of each transaction.

If the account has any pending transactions, historical balance amounts on or after the date of the earliest pending transaction may differ if retrieved in subsequent Asset Reports as a result of those pending transactions posting.*/
    pub current: f64,
    ///The ISO-4217 currency code of the balance. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the balance. Always `null` if `iso_currency_code` is non-`null`.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    /**A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. If the name of a business is reported, please contact Plaid Support. In the case of a joint account, Plaid will make a best effort to report the names of all account holders.

If an Item contains multiple accounts with different owner names, some institutions will report all names associated with the Item in each account's `names` array.*/
    pub names: Vec<String>,
    ///A list of phone numbers associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub phone_numbers: Vec<PhoneNumber>,
    ///A list of email addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub emails: Vec<Email>,
    ///Data about the various addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub addresses: Vec<Address>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerOverride {
    ///A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. Note that the same name data will be used for all accounts associated with an Item.
    pub names: Vec<String>,
    ///A list of phone numbers associated with the account.
    pub phone_numbers: Vec<PhoneNumber>,
    ///A list of email addresses associated with the account.
    pub emails: Vec<Email>,
    ///Data about the various addresses associated with the account.
    pub addresses: Vec<Address>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesObject {
    ///The credit accounts returned.
    pub credit: Option<Vec<CreditCardLiability>>,
    ///The mortgage accounts returned.
    pub mortgage: Option<Vec<MortgageLiability>>,
    ///The student loan accounts returned.
    pub student: Option<Vec<StudentLoan>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoan {
    ///The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    ///The account number of the loan. For some institutions, this may be a masked version of the number (e.g., the last 4 digits instead of the entire number).
    pub account_number: Option<String>,
    ///The dates on which loaned funds were disbursed or will be disbursed. These are often in the past. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub disbursement_dates: Option<Vec<String>>,
    ///The date when the student loan is expected to be paid off. Availability for this field is limited. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub expected_payoff_date: Option<String>,
    ///The guarantor of the student loan.
    pub guarantor: Option<String>,
    ///The interest rate on the loan as a percentage.
    pub interest_rate_percentage: f64,
    ///`true` if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_payment_date: Option<String>,
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_statement_issue_date: Option<String>,
    ///The type of loan, e.g., "Consolidation Loans".
    pub loan_name: Option<String>,
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    /**The minimum payment due for the next billing cycle. There are some exceptions:
Some institutions require a minimum payment across all loans associated with an account number. Our API presents that same minimum payment amount on each loan. The institutions that do this are: Great Lakes ( `ins_116861`), Firstmark (`ins_116295`), Commonbond Firstmark Services (`ins_116950`), Nelnet (`ins_116528`), EdFinancial Services (`ins_116304`), Granite State (`ins_116308`), and Oklahoma Student Loan Authority (`ins_116945`).
Firstmark (`ins_116295` ) and Navient (`ins_116248`) will display as $0 if there is an autopay program in effect.*/
    pub minimum_payment_amount: Option<f64>,
    ///The due date for the next payment. The due date is `null` if a payment is not expected. A payment is not expected if `loan_status.type` is `deferment`, `in_school`, `consolidated`, `paid in full`, or `transferred`. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
    /**The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
*/
    pub origination_date: Option<String>,
    ///The original principal balance of the loan.
    pub origination_principal_amount: Option<f64>,
    ///The total dollar amount of the accrued interest balance. For Sallie Mae ( `ins_116944`), this amount is included in the current balance of the loan, so this field will return as `null`.
    pub outstanding_interest_amount: Option<f64>,
    ///The relevant account number that should be used to reference this loan for payments. In the majority of cases, `payment_reference_number` will match a`ccount_number,` but in some institutions, such as Great Lakes (`ins_116861`), it will be different.
    pub payment_reference_number: Option<String>,
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (`ins_116527`).
    pub pslf_status: PslfStatus,
    ///An object representing the repayment plan for the student loan
    pub repayment_plan: StudentRepaymentPlan,
    ///The sequence number of the student loan. Heartland ECSI (`ins_116948`) does not make this field available.
    pub sequence_number: Option<String>,
    ///The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
    pub servicer_address: ServicerAddressData,
    ///The year to date (YTD) interest paid. Availability for this field is limited.
    pub ytd_interest_paid: Option<f64>,
    ///The year to date (YTD) principal paid. Availability for this field is limited.
    pub ytd_principal_paid: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardLiability {
    ///The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    ///The various interest rates that apply to the account. APR information is not provided by all card issuers; if APR data is not available, this array will be empty.
    pub aprs: Vec<Apr>,
    ///true if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Availability for this field is limited.
    pub last_payment_date: Option<String>,
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_statement_issue_date: Option<String>,
    ///The total amount owed as of the last statement issued
    pub last_statement_balance: Option<f64>,
    ///The minimum payment due for the next billing cycle.
    pub minimum_payment_amount: Option<f64>,
    ///The due date for the next payment. The due date is `null` if a payment is not expected. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageLiability {
    ///The ID of the account that this liability belongs to.
    pub account_id: String,
    ///The account number of the loan.
    pub account_number: String,
    ///The current outstanding amount charged for late payment.
    pub current_late_fee: Option<f64>,
    ///Total amount held in escrow to pay taxes and insurance on behalf of the borrower.
    pub escrow_balance: Option<f64>,
    ///Indicates whether the borrower has private mortgage insurance in effect.
    pub has_pmi: Option<bool>,
    ///Indicates whether the borrower will pay a penalty for early payoff of mortgage.
    pub has_prepayment_penalty: Option<bool>,
    ///Object containing metadata about the interest rate for the mortgage.
    pub interest_rate: MortgageInterestRate,
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_payment_date: Option<String>,
    ///Description of the type of loan, for example `conventional`, `fixed`, or `variable`. This field is provided directly from the loan servicer and does not have an enumerated set of possible values.
    pub loan_type_description: Option<String>,
    ///Full duration of mortgage as at origination (e.g. `10 year`).
    pub loan_term: Option<String>,
    ///Original date on which mortgage is due in full. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub maturity_date: Option<String>,
    ///The amount of the next payment.
    pub next_monthly_payment: Option<f64>,
    ///The due date for the next payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
    ///The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub origination_date: Option<String>,
    ///The original principal balance of the mortgage.
    pub origination_principal_amount: Option<f64>,
    ///Amount of loan (principal + interest) past due for payment.
    pub past_due_amount: Option<f64>,
    ///Object containing fields describing property address.
    pub property_address: MortgagePropertyAddress,
    ///The year to date (YTD) interest paid.
    pub ytd_interest_paid: Option<f64>,
    ///The YTD principal paid.
    pub ytd_principal_paid: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageInterestRate {
    ///Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    pub percentage: Option<f64>,
    #[serde(rename = "type")]
    ///The type of interest charged (fixed or variable).
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgagePropertyAddress {
    ///The city name.
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    ///The five or nine digit postal code.
    pub postal_code: Option<String>,
    ///The region or state (example "NC").
    pub region: Option<String>,
    ///The full street address (example "564 Main Street, Apt 15").
    pub street: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoanStatus {
    /**The date until which the loan will be in its current status. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
*/
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    ///The status type of the student loan
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentRepaymentPlan {
    ///The description of the repayment plan as provided by the servicer.
    pub description: Option<String>,
    #[serde(rename = "type")]
    ///The type of the repayment plan.
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PslfStatus {
    ///The estimated date borrower will have completed 120 qualifying monthly payments. Returned in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub estimated_eligibility_date: Option<String>,
    ///The number of qualifying payments that have been made.
    pub payments_made: Option<f64>,
    ///The number of qualifying payments remaining.
    pub payments_remaining: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServicerAddressData {
    ///The full city name
    pub city: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    ///The postal code
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Apr {
    /**Annual Percentage Rate applied.
*/
    pub apr_percentage: f64,
    ///The type of balance to which the APR applies.
    pub apr_type: String,
    ///Amount of money that is subjected to the APR if a balance was carried beyond payment due date. How it is calculated can vary by card issuer. It is often calculated as an average daily balance.
    pub balance_subject_to_apr: Option<f64>,
    ///Amount of money charged due to interest from last statement.
    pub interest_charge_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMetadata {
    ///Metadata specifically related to which auth methods an institution supports.
    pub supported_methods: Option<AuthSupportedMethods>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSupportedMethods {
    ///Indicates if instant auth is supported.
    pub instant_auth: bool,
    ///Indicates if instant match is supported.
    pub instant_match: bool,
    ///Indicates if automated microdeposits are supported.
    pub automated_micro_deposits: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationMetadata {
    ///Indicates whether the institution supports payments from a different country.
    pub supports_international_payments: bool,
    ///Indicates whether the institution supports SEPA Instant payments.
    pub supports_sepa_instant: bool,
    /**A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.

Example: `{"GBP": "10000"}`
*/
    pub maximum_payment_amount: PaymentInitiationMaximumPaymentAmount,
    ///Indicates whether the institution supports returning refund details when initiating a payment.
    pub supports_refund_details: bool,
    ///Metadata specifically related to valid Payment Initiation standing order configurations for the institution.
    pub standing_order_metadata: Option<PaymentInitiationStandingOrderMetadata>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationMaximumPaymentAmount {}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationStandingOrderMetadata {
    ///Indicates whether the institution supports closed-ended standing orders by providing an end date.
    pub supports_standing_order_end_date: bool,
    ///This is only applicable to `MONTHLY` standing orders. Indicates whether the institution supports negative integers (-1 to -5) for setting up a `MONTHLY` standing order relative to the end of the month.
    pub supports_standing_order_negative_execution_days: bool,
    ///A list of the valid standing order intervals supported by the institution.
    pub valid_standing_order_intervals: Vec<PaymentScheduleInterval>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationAddress {
    ///An array of length 1-2 representing the street address where the recipient is located. Maximum of 70 characters.
    pub street: Vec<String>,
    ///The city where the recipient is located. Maximum of 35 characters.
    pub city: String,
    ///The postal code where the recipient is located. Maximum of 16 characters.
    pub postal_code: String,
    ///The ISO 3166-1 alpha-2 country code where the recipient is located.
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleBase {
    ///The frequency interval of the payment.
    pub interval: PaymentScheduleInterval,
    /**The day of the interval on which to schedule the payment.

If the payment interval is weekly, `interval_execution_day` should be an integer from 1 (Monday) to 7 (Sunday).

If the payment interval is monthly, `interval_execution_day` should be an integer indicating which day of the month to make the payment on. Integers from 1 to 28 can be used to make a payment on that day of the month. Negative integers from -1 to -5 can be used to make a payment relative to the end of the month. To make a payment on the last day of the month, use -1; to make the payment on the second-to-last day, use -2, and so on.*/
    pub interval_execution_day: i64,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will begin on the first `interval_execution_day` on or after the `start_date`.

If the first `interval_execution_day` on or after the start date is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make the first payment on that day, but it is not guaranteed to do so.*/
    pub start_date: String,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will end on the last `interval_execution_day` on or before the `end_date`.
If the only `interval_execution_day` between the start date and the end date (inclusive) is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make a payment on that day, but it is not guaranteed to do so.*/
    pub end_date: Option<String>,
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, this field will be `null`.
    pub adjusted_start_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentScheduleInterval {
    Weekly,
    Monthly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentScheme {
    FasterPayments,
    SepaCreditTransfer,
    SepaCreditTransferInstant,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentInitiationConsentScope {
    MeToMe,
    External,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentInitiationConsentOptions {
    ///The EMI (E-Money Institution) wallet that this payment consent is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    pub request_refund_details: Option<bool>,
    ///The International Bank Account Number (IBAN) for the payer's account. If provided, the end user will be able to set up payment consent using only the specified bank account.
    pub iban: Option<String>,
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    pub bacs: Option<PaymentInitiationOptionalRestrictionBacs>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationConsentConstraints {
    ///Life span for the payment consent. After the `to` date the payment consent expires and can no longer be used for payment initiation.
    pub valid_date_time: Option<PaymentConsentValidDateTime>,
    ///Maximum amount of a single payment initiated using the payment consent.
    pub max_payment_amount: PaymentConsentMaxPaymentAmount,
    ///A list of amount limitations per period of time.
    pub periodic_amounts: Vec<PaymentConsentPeriodicAmount>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentMaxPaymentAmount(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsentPaymentIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentOptions {
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    pub request_refund_details: Option<bool>,
    ///The International Bank Account Number (IBAN) for the payer's account. If provided, the end user will be able to send payments only from the specified bank account.
    pub iban: Option<String>,
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    pub bacs: Option<PaymentInitiationOptionalRestrictionBacs>,
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub scheme: Option<PaymentScheme>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentRefundDetails {
    ///The name of the account holder.
    pub name: String,
    ///The International Bank Account Number (IBAN) for the account.
    pub iban: Option<String>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacsNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleGet(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub enum Products {
    Assets,
    Auth,
    Balance,
    Identity,
    Investments,
    Liabilities,
    PaymentInitiation,
    IdentityVerification,
    Transactions,
    CreditDetails,
    Income,
    IncomeVerification,
    DepositSwitch,
    StandingOrders,
    Transfer,
    Employment,
    RecurringTransactions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStatus {
    /**This field is deprecated in favor of the `breakdown` object, which provides more granular institution health data.

`HEALTHY`: the majority of requests are successful
`DEGRADED`: only some requests are successful
`DOWN`: all requests are failing*/
    pub status: String,
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) formatted timestamp of the last status change for the institution.
*/
    pub last_status_change: String,
    ///A detailed breakdown of the institution's performance for a request type. The values for `success`, `error_plaid`, and `error_institution` sum to 1.
    pub breakdown: ProductStatusBreakdown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStatusBreakdown {
    ///The percentage of login attempts that are successful, expressed as a decimal.
    pub success: f64,
    /**The percentage of logins that are failing due to an internal Plaid issue, expressed as a decimal.
*/
    pub error_plaid: f64,
    ///The percentage of logins that are failing due to an issue in the institution's system, expressed as a decimal.
    pub error_institution: f64,
    ///The `refresh_interval` may be `DELAYED` or `STOPPED` even when the success rate is high. This value is only returned for Transactions status breakdowns.
    pub refresh_interval: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCustomPassword {
    ///The version of the password schema to use, possible values are 1 or 2. The default value is 2. You should only specify 1 if you know it is necessary for your test suite.
    pub version: Option<String>,
    /**A seed, in the form of a string, that will be used to randomly generate account and transaction data, if this data is not specified using the `override_accounts` argument. If no seed is specified, the randomly generated data will be different each time.

Note that transactions data is generated relative to the Item's creation date. Different Items created on different dates with the same seed for transactions data will have different dates for the transactions. The number of days between each transaction and the Item creation will remain constant. For example, an Item created on December 15 might show a transaction on December 14. An Item created on December 20, using the same seed, would show that same transaction occurring on December 19.*/
    pub seed: String,
    ///An array of account overrides to configure the accounts for the Item. By default, if no override is specified, transactions and account data will be randomly generated based on the account type and subtype, and other products will have fixed or empty data.
    pub override_accounts: Vec<OverrideAccounts>,
    ///Specifies the multi-factor authentication settings to use with this test account
    pub mfa: Mfa,
    ///You may trigger a reCAPTCHA in Plaid Link in the Sandbox environment by using the recaptcha field. Possible values are `good` or `bad`. A value of `good` will result in successful Item creation and `bad` will result in a `RECAPTCHA_BAD` error to simulate a failed reCAPTCHA. Both values require the reCAPTCHA to be manually solved within Plaid Link.
    pub recaptcha: String,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Mfa {
    #[serde(rename = "type")]
    /**Possible values are `device`, `selections`, or `questions`.

If value is `device`, the MFA answer is `1234`.

If value is `selections`, the MFA answer is always the first option.

If value is `questions`, the MFA answer is  `answer_<i>_<j>` for the j-th question in the i-th round, starting from 0. For example, the answer to the first question in the second round is `answer_1_0`.*/
    pub type_: String,
    ///Number of rounds of questions. Required if value of `type` is `questions`.
    pub question_rounds: f64,
    ///Number of questions per round. Required if value of `type` is `questions`. If value of type is `selections`, default value is 2.
    pub questions_per_round: f64,
    ///Number of rounds of selections, used if `type` is `selections`. Defaults to 1.
    pub selection_rounds: f64,
    /**Number of available answers per question, used if `type` is `selection`. Defaults to 2.
*/
    pub selections_per_question: f64,
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
    pub type_: OverrideAccountType,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    pub subtype: Option<AccountSubtype>,
    /**If provided, the account will start with this amount as the current balance.
*/
    pub starting_balance: f64,
    ///If provided, the account will always have this amount as its  available balance, regardless of current balance or changes in transactions over time.
    pub force_available_balance: f64,
    ///ISO-4217 currency code. If provided, the account will be denominated in the given currency. Transactions will also be in this currency by default.
    pub currency: String,
    ///Allows specifying the metadata of the test account
    pub meta: Meta,
    ///Account and bank identifier number data used to configure the test account. All values are optional.
    pub numbers: Numbers,
    ///Specify the list of transactions on the account.
    pub transactions: Vec<TransactionOverride>,
    ///Specify the holdings on the account.
    pub holdings: HoldingsOverride,
    ///Specify the list of investments transactions on the account.
    pub investment_transactions: InvestmentsTransactionsOverride,
    ///Data about the owner or owners of an account. Any fields not specified will be filled in with default Sandbox information.
    pub identity: OwnerOverride,
    ///Used to configure Sandbox test data for the Liabilities product
    pub liability: LiabilityOverride,
    ///The `inflow_model` allows you to model a test account that receives regular income or make regular payments on a loan. Any transactions generated by the `inflow_model` will appear in addition to randomly generated test data or transactions specified by `override_accounts`.
    pub inflow_model: InflowModel,
    ///Specify payroll data on the account.
    pub income: IncomeOverride,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    ///The account's name
    pub name: String,
    ///The account's official name
    pub official_name: String,
    ///The account's limit
    pub limit: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Numbers {
    ///Will be used for the account number.
    pub account: String,
    ///Must be a valid ACH routing number.
    pub ach_routing: String,
    ///Must be a valid wire transfer routing number.
    pub ach_wire_routing: String,
    ///EFT institution number. Must be specified alongside `eft_branch`.
    pub eft_institution: String,
    ///EFT branch number. Must be specified alongside `eft_institution`.
    pub eft_branch: String,
    ///Bank identifier code (BIC). Must be specified alongside `international_iban`.
    pub international_bic: String,
    ///International bank account number (IBAN). If no account number is specified via `account`, will also be used as the account number by default. Must be specified alongside `international_bic`.
    pub international_iban: String,
    ///BACS sort code
    pub bacs_sort_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOverride {
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Transactions in Sandbox will move from pending to posted once their transaction date has been reached. If a `date_transacted` is not provided by the institution, a transaction date may be available in the [`authorized_date`](https://plaid.com/docs/api/products/transactions/#transactions-get-response-transactions-authorized-date) field.
    pub date_transacted: String,
    ///The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Posted dates in the past or present will result in posted transactions; posted dates in the future will result in pending transactions.
    pub date_posted: String,
    ///The transaction amount. Can be negative.
    pub amount: f64,
    ///The transaction description.
    pub description: String,
    ///The ISO-4217 format currency code for the transaction.
    pub currency: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityOverride {
    ///12-character ISIN, a globally unique securities identifier.
    pub isin: String,
    ///9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: String,
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: String,
    ///A descriptive name for the security, suitable for display.
    pub name: String,
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: String,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HoldingsOverride {
    ///The last price given by the institution for this security
    pub institution_price: f64,
    ///The date at which `institution_price` was current. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub institution_price_as_of: String,
    ///The average original value of the holding. Multiple cost basis values for the same security purchased at different prices are not supported.
    pub cost_basis: f64,
    ///The total quantity of the asset held, as reported by the financial institution.
    pub quantity: f64,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: SecurityOverride,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsOverride {
    ///Posting date for the transaction. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub date: String,
    ///The institution's description of the transaction.
    pub name: String,
    ///The number of units of the security involved in this transaction. Must be positive if the type is a buy and negative if the type is a sell.
    pub quantity: f64,
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    ///The combined value of all fees applied to this transaction.
    pub fees: f64,
    #[serde(rename = "type")]
    /**The type of the investment transaction. Possible values are:
`buy`: Buying an investment
`sell`: Selling an investment
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer*/
    pub type_: String,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: SecurityOverride,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityOverride {
    #[serde(rename = "type")]
    ///The type of the liability object, either `credit` or `student`. Mortgages are not currently supported in the custom Sandbox.
    pub type_: String,
    ///The purchase APR percentage value. For simplicity, this is the only interest rate used to calculate interest charges. Can only be set if `type` is `credit`.
    pub purchase_apr: f64,
    ///The cash APR percentage value. Can only be set if `type` is `credit`.
    pub cash_apr: f64,
    ///The balance transfer APR percentage value. Can only be set if `type` is `credit`. Can only be set if `type` is `credit`.
    pub balance_transfer_apr: f64,
    ///The special APR percentage value. Can only be set if `type` is `credit`.
    pub special_apr: f64,
    ///Override the `last_payment_amount` field. Can only be set if `type` is `credit`.
    pub last_payment_amount: f64,
    ///Override the `minimum_payment_amount` field. Can only be set if `type` is `credit` or `student`.
    pub minimum_payment_amount: f64,
    ///Override the `is_overdue` field
    pub is_overdue: bool,
    ///The date on which the loan was initially lent, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Can only be set if `type` is `student`.
    pub origination_date: String,
    ///The original loan principal. Can only be set if `type` is `student`.
    pub principal: f64,
    ///The interest rate on the loan as a percentage. Can only be set if `type` is `student`.
    pub nominal_apr: f64,
    ///If set, interest capitalization begins at the given number of months after loan origination. By default interest is never capitalized. Can only be set if `type` is `student`.
    pub interest_capitalization_grace_period_months: f64,
    ///Student loan repayment information used to configure Sandbox test data for the Liabilities product
    pub repayment_model: StudentLoanRepaymentModel,
    ///Override the `expected_payoff_date` field. Can only be set if `type` is `student`.
    pub expected_payoff_date: String,
    ///Override the `guarantor` field. Can only be set if `type` is `student`.
    pub guarantor: String,
    ///Override the `is_federal` field. Can only be set if `type` is `student`.
    pub is_federal: bool,
    ///Override the `loan_name` field. Can only be set if `type` is `student`.
    pub loan_name: String,
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    ///Override the `payment_reference_number` field. Can only be set if `type` is `student`.
    pub payment_reference_number: String,
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (`ins_116527`).
    pub pslf_status: PslfStatus,
    ///Override the `repayment_plan.description` field. Can only be set if `type` is `student`.
    pub repayment_plan_description: String,
    ///Override the `repayment_plan.type` field. Can only be set if `type` is `student`. Possible values are: `"extended graduated"`, `"extended standard"`, `"graduated"`, `"income-contingent repayment"`, `"income-based repayment"`, `"interest only"`, `"other"`, `"pay as you earn"`, `"revised pay as you earn"`, or `"standard"`.
    pub repayment_plan_type: String,
    ///Override the `sequence_number` field. Can only be set if `type` is `student`.
    pub sequence_number: String,
    ///A physical mailing address.
    pub servicer_address: Address,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoanRepaymentModel {
    #[serde(rename = "type")]
    ///The only currently supported value for this field is `standard`.
    pub type_: String,
    ///Configures the number of months before repayment starts.
    pub non_repayment_months: f64,
    ///Configures the number of months of repayments before the loan is paid off.
    pub repayment_months: f64,
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
    ///Amount of income per month. This value is required if `type` is `monthly-income`.
    pub income_amount: f64,
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the income transaction will appear. The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub payment_day_of_month: f64,
    ///The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub transaction_name: String,
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the balance is calculated for the next payment. The name of the income transaction. This field is required if `type` is `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub statement_day_of_month: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeOverride {
    ///A list of paystubs associated with the account.
    pub paystubs: Vec<PaystubOverride>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverride {
    ///The employer on the paystub.
    pub employer: PaystubOverrideEmployer,
    ///The employee on the paystub.
    pub employee: PaystubOverrideEmployee,
    pub income_breakdown: Vec<IncomeBreakdown>,
    ///Details about the pay period.
    pub pay_period_details: PayPeriodDetails,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployer {
    ///The name of the employer.
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployee {
    ///The name of the employee.
    pub name: String,
    ///The address of the employee.
    pub address: PaystubOverrideEmployeeAddress,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployeeAddress {
    ///The full city name.
    pub city: String,
    /**The region or state
Example: `"NC"`*/
    pub region: String,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    ///5 digit postal code.
    pub postal_code: String,
    ///The country of the address.
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticallyVerifiedWebhook {
    ///`AUTH`
    pub webhook_type: String,
    ///`AUTOMATICALLY_VERIFIED`
    pub webhook_code: String,
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtHeader {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationExpiredWebhook {
    ///`AUTH`
    pub webhook_type: String,
    ///`VERIFICATION_EXPIRED`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookUpdateAcknowledgedWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`WEBHOOK_UPDATE_ACKNOWLEDGED`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The new webhook URL
    pub new_webhook_url: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PendingExpirationWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`PENDING_EXPIRATION`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The date and time at which the Item's access consent will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub consent_expiration_time: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemErrorWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`ERROR`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemProductReadyWebhook {
    ///`INCOME`
    pub webhook_type: String,
    ///`PRODUCT_READY`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecaptchaRequiredError {
    ///RECAPTCHA_ERROR
    pub error_type: String,
    ///RECAPTCHA_REQUIRED
    pub error_code: String,
    pub display_message: String,
    ///400
    pub http_code: String,
    ///Your user will be prompted to solve a Google reCAPTCHA challenge in the Link Recaptcha pane. If they solve the challenge successfully, the user's request is resubmitted and they are directed to the next Item creation step.
    pub link_user_experience: String,
    ///Plaid's fraud system detects abusive traffic and considers a variety of parameters throughout Item creation requests. When a request is considered risky or possibly fraudulent, Link presents a reCAPTCHA for the user to solve.
    pub common_causes: String,
    /**Link will automatically guide your user through reCAPTCHA verification. As a general rule, we recommend instrumenting basic fraud monitoring to detect and protect your website from spam and abuse.

If your user cannot verify their session, please submit a Support ticket with the following identifiers: `link_session_id` or `request_id`*/
    pub troubleshooting_steps: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransfersEventsUpdateWebhook {
    ///`BANK_TRANSFERS`
    pub webhook_type: String,
    ///`BANK_TRANSFERS_EVENTS_UPDATE`
    pub webhook_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventsUpdateWebhook {
    ///`TRANSFER`
    pub webhook_type: String,
    ///`TRANSFER_EVENTS_UPDATE`
    pub webhook_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsDefaultUpdateWebhook {
    ///`INVESTMENTS_TRANSACTIONS`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new transactions reported since the last time this webhook was fired.
    pub new_investments_transactions: f64,
    ///The number of canceled transactions reported since the last time this webhook was fired.
    pub canceled_investments_transactions: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HoldingsDefaultUpdateWebhook {
    ///`HOLDINGS`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new holdings reported since the last time this webhook was fired.
    pub new_holdings: f64,
    ///The number of updated holdings reported since the last time this webhook was fired.
    pub updated_holdings: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesDefaultUpdateWebhook {
    ///`LIABILITIES`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///An array of `account_id`'s for accounts that contain new liabilities.'
    pub account_ids_with_new_liabilities: Vec<String>,
    /**An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["past_amount_due"] }`
*/
    pub account_ids_with_updated_liabilities: LiabilitiesAccountIdsWithUpdatedLiabilities,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesAccountIdsWithUpdatedLiabilities {}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsProductReadyWebhook {
    ///`ASSETS`
    pub webhook_type: String,
    ///`PRODUCT_READY`
    pub webhook_code: String,
    ///The `asset_report_id` that can be provided to `/asset_report/get` to retrieve the Asset Report.
    pub asset_report_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsErrorWebhook {
    ///`ASSETS`
    pub webhook_type: String,
    ///`ERROR`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The ID associated with the Asset Report.
    pub asset_report_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsRelayWebhook {
    ///`ASSETS`
    pub webhook_type: String,
    ///`RELAY_EVENT`
    pub webhook_code: String,
    ///The webhook code indicating which endpoint was called. It can be one of `GET_CALLED`, `REFRESH_CALLED` or `AUDIT_COPY_CREATE_CALLED`.
    pub relay_event: RelayEvent,
    ///The id of the client with whom the Asset Report is being shared.
    pub secondary_client_id: String,
    ///The `asset_relay_token` that was created by calling `/asset_report/relay/create.
    pub asset_relay_token: String,
    ///The `asset_report_id` that can be provided to `/asset_report/relay/get` to retrieve the Asset Report.
    pub asset_report_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RelayEvent {
    GetCalled,
    RefreshCalled,
    AuditCopyCreateCalled,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cause {}
#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    ///The warning type, which will always be `ASSET_REPORT_WARNING`
    pub warning_type: String,
    ///The warning code identifies a specific kind of warning. Currently, the only possible warning code is `OWNERS_UNAVAILABLE`, which indicates that account-owner information is not available.
    pub warning_code: String,
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: Cause,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentAmountCurrency {
    Gbp,
    Eur,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentAmount {
    ///The ISO-4217 currency code of the payment. For standing orders and payment consents, `"GBP"` must be used.
    pub currency: PaymentAmountCurrency,
    ///The amount of the payment. Must contain at most two digits of precision e.g. `1.23`. Minimum accepted value is `1`.
    pub value: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentValidDateTime {
    ///The date and time from which the consent should be active, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub from: Option<String>,
    ///The date and time at which the consent expires, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub to: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmount {
    ///Maximum cumulative amount for all payments in the specified interval.
    pub amount: PaymentConsentPeriodicAmountAmount,
    ///Payment consent periodic interval.
    pub interval: PaymentConsentPeriodicInterval,
    /**Where the payment consent period should start.

`CALENDAR`: line up with a calendar.

`CONSENT`: on the date of consent creation.*/
    pub alignment: PaymentConsentPeriodicAlignment,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConsentPeriodicAmountAmount(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicInterval {
    Day,
    Week,
    Month,
    Year,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentConsentPeriodicAlignment {
    Calendar,
    Consent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportUser {
    ///An identifier you determine and submit for the user.
    pub client_user_id: Option<String>,
    ///The user's first name. Required for the Fannie Mae Day 1 Certainty™ program.
    pub first_name: Option<String>,
    ///The user's middle name
    pub middle_name: Option<String>,
    ///The user's last name.  Required for the Fannie Mae Day 1 Certainty™ program.
    pub last_name: Option<String>,
    /**The user's Social Security Number. Required for the Fannie Mae Day 1 Certainty™ program.

Format: "ddd-dd-dddd"*/
    pub ssn: Option<String>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshAssetReportToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneCurrencyCodeList {
    ///Plaid supports all ISO 4217 currency codes.
    pub iso_currency_code: String,
    ///List of unofficial currency codes
    pub unofficial_currency_code: UnofficialCurrencyCodeList,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnofficialCurrencyCodeList(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneAccountType {
    ///An account type holding cash, in which funds are deposited. Supported products for `depository` accounts are: Auth (`checking` and `savings` types only), Balance, Transactions, Identity, Payment Initiation, and Assets.
    pub depository: DepositoryAccount,
    ///A credit card type account. Supported products for `credit` accounts are: Balance, Transactions, Identity, and Liabilities.
    pub credit: CreditAccount,
    ///A loan type account. Supported products for `loan` accounts are: Balance, Liabilities, and Transactions.
    pub loan: LoanAccount,
    ///An investment account. Supported products for `investment` accounts are: Balance and Investments. In API versions 2018-05-22 and earlier, this type is called `brokerage`.
    pub investment: InvestmentAccountSubtypeStandalone,
    ///Other or unknown account type. Supported products for `other` accounts are: Balance, Transactions, Identity, and Assets.
    pub other: String,
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
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///An identifier you determine and submit for the Asset Report.
    pub client_report_id: Option<String>,
    ///The date and time when the Asset Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub date_generated: String,
    ///The duration of transaction history you requested
    pub days_requested: f64,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: AssetReportUser,
    ///Data returned by Plaid about each of the Items included in the Asset Report.
    pub items: Vec<AssetReportItem>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportItem {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The full financial institution name associated with the Item.
    pub institution_name: String,
    ///The id of the financial institution associated with the Item.
    pub institution_id: String,
    ///The date and time when this Item’s data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub date_last_updated: String,
    ///Data about each of the accounts open on the Item.
    pub accounts: Vec<AccountAssets>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStatusUpdateWebhook {
    ///`PAYMENT_INITIATION`
    pub webhook_type: String,
    ///`PAYMENT_STATUS_UPDATE`
    pub webhook_code: String,
    ///The `payment_id` for the payment being updated
    pub payment_id: String,
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
    pub new_payment_status: PaymentInitiationPaymentStatus,
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
    pub old_payment_status: PaymentInitiationPaymentStatus,
    ///The original value of the reference when creating the payment.
    pub original_reference: Option<String>,
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    pub adjusted_reference: Option<String>,
    ///The original value of the `start_date` provided during the creation of a standing order. If the payment is not a standing order, this field will be `null`.
    pub original_start_date: Option<String>,
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, or if the payment is not a standing order, this field will be `null`.
    pub adjusted_start_date: Option<String>,
    ///The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2017-09-14T14:42:19.350Z"`
    pub timestamp: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Holding {
    ///The Plaid `account_id` associated with the holding.
    pub account_id: String,
    ///The Plaid `security_id` associated with the holding.
    pub security_id: String,
    ///The last price given by the institution for this security.
    pub institution_price: f64,
    ///The date at which `institution_price` was current.
    pub institution_price_as_of: Option<String>,
    /**Date and time at which `institution_price` was current, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ).

This field is returned for select financial institutions and comes as provided by the institution. It may contain default time values (such as 00:00:00).
*/
    pub institution_price_datetime: Option<String>,
    ///The value of the holding, as reported by the institution.
    pub institution_value: f64,
    ///The original total value or the purchase price per share of the holding. This field is an aggregate on a per holding basis and dependent on the information provided by the institution.
    pub cost_basis: Option<f64>,
    ///The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts.
    pub quantity: f64,
    ///The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    ///A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive.
    pub security_id: String,
    ///12-character ISIN, a globally unique securities identifier.
    pub isin: Option<String>,
    ///9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: Option<String>,
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: Option<String>,
    ///An identifier given to the security by the institution
    pub institution_security_id: Option<String>,
    ///If `institution_security_id` is present, this field indicates the Plaid `institution_id` of the institution to whom the identifier belongs.
    pub institution_id: Option<String>,
    ///In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security.
    pub proxy_security_id: Option<String>,
    ///A descriptive name for the security, suitable for display.
    pub name: Option<String>,
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: Option<String>,
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
    /**Price of the security at the close of the previous trading session. Null for non-public securities.

If the security is a foreign currency this field will be updated daily and will be priced in USD.

If the security is a cryptocurrency, this field will be updated multiple times a day. As crypto prices can fluctuate quickly and data may become stale sooner than other asset classes, please refer to update_datetime with the time when the price was last updated.
*/
    pub close_price: Option<f64>,
    ///Date for which `close_price` is accurate. Always `null` if `close_price` is `null`.
    pub close_price_as_of: Option<String>,
    ///Date and time at which close_price is accurate, in ISO 8601 format (YYYY-MM-DDTHH:mm:ssZ). Always null if close_price is null.
    pub update_datetime: Option<String>,
    ///The ISO-4217 currency code of the price given. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentTransactionType {
    Buy,
    Sell,
    Cancel,
    Cash,
    Fee,
    Transfer,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum InvestmentTransactionSubtype {
    AccountFee,
    Adjustment,
    Assignment,
    Buy,
    BuyToCover,
    Contribution,
    Deposit,
    Distribution,
    Dividend,
    DividendReinvestment,
    Exercise,
    Expire,
    FundFee,
    Interest,
    InterestReceivable,
    InterestReinvestment,
    LegalFee,
    LoanPayment,
    #[serde(rename = "long-term capital gain")]
    LongTermCapitalGain,
    #[serde(rename = "long-term capital gain reinvestment")]
    LongTermCapitalGainReinvestment,
    ManagementFee,
    MarginExpense,
    Merger,
    MiscellaneousFee,
    #[serde(rename = "non-qualified dividend")]
    NonQualifiedDividend,
    #[serde(rename = "non-resident tax")]
    NonResidentTax,
    PendingCredit,
    PendingDebit,
    QualifiedDividend,
    Rebalance,
    ReturnOfPrincipal,
    Request,
    Sell,
    SellShort,
    Send,
    #[serde(rename = "short-term capital gain")]
    ShortTermCapitalGain,
    #[serde(rename = "short-term capital gain reinvestment")]
    ShortTermCapitalGainReinvestment,
    SpinOff,
    Split,
    StockDistribution,
    Tax,
    TaxWithheld,
    Transfer,
    TransferFee,
    TrustFee,
    UnqualifiedGain,
    Withdrawal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    ///The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive.
    pub investment_transaction_id: String,
    ///A legacy field formerly used internally by Plaid to identify certain canceled transactions.
    pub cancel_transaction_id: Option<String>,
    ///The `account_id` of the account against which this transaction posted.
    pub account_id: String,
    ///The `security_id` to which this transaction is related.
    pub security_id: Option<String>,
    ///The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction.
    pub date: String,
    ///The institution’s description of the transaction.
    pub name: String,
    ///The number of units of the security involved in this transaction.
    pub quantity: f64,
    ///The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities.
    pub amount: f64,
    ///The price of the security at which this transaction occurred.
    pub price: f64,
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
    pub type_: InvestmentTransactionType,
    ///For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
    pub subtype: InvestmentTransactionSubtype,
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionType {
    ///Buying an investment
    pub buy: StandaloneInvestmentTransactionBuyType,
    ///Selling an investment
    pub sell: StandaloneInvestmentTransactionSellType,
    ///A cancellation of a pending transaction
    pub cancel: String,
    ///Activity that modifies a cash position
    pub cash: StandaloneInvestmentTransactionCashType,
    ///Fees on the account, e.g. commission, bookkeeping, options-related.
    pub fee: StandaloneInvestmentTransactionFeeType,
    ///Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer
    pub transfer: StandaloneInvestmentTransactionTransferType,
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
pub struct AccountSubtypes(pub Vec<Option<AccountSubtype>>);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermissionRevokedWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`USER_PERMISSION_REVOKED`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchGetRequest {
    ///The ID of the deposit switch
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchGetResponse {
    ///The ID of the deposit switch.
    pub deposit_switch_id: String,
    ///The ID of the bank account the direct deposit was switched to.
    pub target_account_id: Option<String>,
    ///The ID of the Item the direct deposit was switched to.
    pub target_item_id: Option<String>,
    /**
The state, or status, of the deposit switch.

- `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

- `processing` – The deposit switch request has been submitted and is being processed.

- `completed` – The user's employer has fulfilled the deposit switch request.

- `error` – There was an error processing the deposit switch request.*/
    pub state: String,
    /**The method used to make the deposit switch.

- `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.

- `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.

- `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.'*/
    pub switch_method: Option<String>,
    ///When `true`, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always `null` if the deposit switch has not been completed.
    pub account_has_multiple_allocations: Option<bool>,
    ///When `true`, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When `false`, user’s direct deposit is allocated as a percent or amount. Always `null` if the deposit switch has not been completed.
    pub is_allocated_remainder: Option<bool>,
    ///The percentage of direct deposit allocated to the target account. Always `null` if the target account is not allocated a percentage or if the deposit switch has not been completed or if `is_allocated_remainder` is true.
    pub percent_allocated: Option<f64>,
    ///The dollar amount of direct deposit allocated to the target account. Always `null` if the target account is not allocated an amount or if the deposit switch has not been completed.
    pub amount_allocated: Option<f64>,
    ///The name of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    pub employer_name: Option<String>,
    ///The ID of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    pub employer_id: Option<String>,
    ///The name of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    pub institution_name: Option<String>,
    ///The ID of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    pub institution_id: Option<String>,
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was created.
*/
    pub date_created: String,
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was completed. Always `null` if the deposit switch has not been completed.
*/
    pub date_completed: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchStateUpdateWebhook {
    ///`"DEPOSIT_SWITCH"`
    pub webhook_type: String,
    ///`"SWITCH_STATE_UPDATE"`
    pub webhook_code: String,
    /**
The state, or status, of the deposit switch.

`initialized`: The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

`processing`: The deposit switch request has been submitted and is being processed.

`completed`: The user's employer has fulfilled and completed the deposit switch request.

`error`: There was an error processing the deposit switch request.

For more information, see the [Deposit Switch API reference](/docs/deposit-switch/reference#deposit_switchget).*/
    pub state: String,
    ///The ID of the deposit switch.
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyGetRequest {
    ///The `audit_copy_token` granting access to the Audit Copy you would like to get.
    pub audit_copy_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferGetRequest {
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferGetRequest {
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferGetResponse {
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferGetResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepId(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    ///Plaid’s unique identifier for a transfer.
    pub id: TransferId,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The account ID that should be credited/debited for this transfer.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///The description of the transfer.
    pub description: String,
    ///The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: String,
    ///The status of the transfer.
    pub status: TransferStatus,
    /**The status of the sweep for the transfer.
`unswept`: The transfer hasn't been swept yet.
`swept`: The transfer was swept to the sweep account.
`return_swept`: The transfer was returned, funds were pulled back or pushed back to the sweep account.
`null`: The transfer will never be swept (e.g. if the transfer is cancelled or returned before being swept)*/
    pub sweep_status: Option<TransferSweepStatus>,
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`. The cutoff for same-day transfers is 7:45 AM Pacific Time and the cutoff for next-day transfers is 5:45 PM Pacific Time. It is recommended to submit a transfer at least 15 minutes before the cutoff time in order to ensure that it will be processed before the cutoff. Any transfer that is indicated as `same-day-ach` and that misses the same-day cutoff, but is submitted in time for the next-day cutoff, will be sent over next-day rails and will not incur same-day charges. Note that both legs of the transfer will be downgraded if applicable.
    pub network: TransferNetwork,
    ///When `true`, you can still cancel this transfer.
    pub cancellable: bool,
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    pub failure_reason: Option<TransferFailure>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransfer {
    ///Plaid’s unique identifier for a bank transfer.
    pub id: BankTransferId,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The account ID that should be credited/debited for this bank transfer.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: BankTransferType,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: BankTransferAmount,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    ///The description of the transfer.
    pub description: String,
    ///The datetime when this bank transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: String,
    ///The status of the transfer.
    pub status: BankTransferStatus,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: BankTransferNetwork,
    ///When `true`, you can still cancel this bank transfer.
    pub cancellable: bool,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: Option<BankTransferFailure>,
    ///A string containing the custom tag provided by the client in the create request. Will be null if not provided.
    pub custom_tag: Option<String>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<BankTransferMetadata>,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: Option<BankTransferDirection>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AchClass {
    Ccd,
    Ppd,
    Tel,
    Web,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAmount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepAmount(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetFailureReason {
    ///A broad categorization of the error.
    pub error_type: String,
    /**A code representing the reason for a failed transfer intent (i.e., an API error or the authorization being declined).

For a full listing of bank transfer errors, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/).*/
    pub error_code: String,
    ///A human-readable description of the code associated with a failed transfer intent.
    pub error_message: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentCreateMode {
    Payment,
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
    ///The user's legal name.
    pub legal_name: String,
    ///The user's phone number. In order to qualify for a guaranteed transfer, at least one of `phone_number` or `email_address` must be provided.
    pub phone_number: String,
    ///The user's email address. In order to qualify for a guaranteed transfer, at least one of `phone_number` or `email_address` must be provided.
    pub email_address: String,
    ///The address associated with the account holder. Providing this data will improve the likelihood that Plaid will be able to guarantee the transfer, if applicable.
    pub address: TransferUserAddressInRequest,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserInRequest {
    ///The user's legal name.
    pub legal_name: String,
    ///The user's phone number.
    pub phone_number: String,
    ///The user's email address.
    pub email_address: String,
    ///The address associated with the account holder. Providing this data will improve the likelihood that Plaid will be able to guarantee the transfer, if applicable.
    pub address: TransferUserAddressInRequest,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserInResponse {
    ///The user's legal name.
    pub legal_name: String,
    ///The user's phone number.
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email_address: Option<String>,
    ///The address associated with the account holder.
    pub address: Option<TransferUserAddressInResponse>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserAddressInRequest {
    ///The street number and name (i.e., "100 Market St.").
    pub street: String,
    ///Ex. "San Francisco"
    pub city: String,
    ///The state or province (e.g., "CA").
    pub region: String,
    ///The postal code (e.g., "94103").
    pub postal_code: String,
    ///A two-letter country code (e.g., "US").
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserAddressInResponse {
    ///The street number and name (i.e., "100 Market St.").
    pub street: Option<String>,
    ///Ex. "San Francisco"
    pub city: Option<String>,
    ///The state or province (e.g., "CA").
    pub region: Option<String>,
    ///The postal code (e.g., "94103").
    pub postal_code: Option<String>,
    ///A two-letter country code (e.g., "US").
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferUser {
    ///The account holder’s full legal name. If the transfer `ach_class` is `ccd`, this should be the business name of the account holder.
    pub legal_name: String,
    ///The account holder’s email.
    pub email_address: Option<String>,
    ///The account holder's routing number. This field is only used in response data. Do not provide this field when making requests.
    pub routing_number: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationDecisionRationaleCode {
    Nsf,
    Risk,
    TransferLimitReached,
    ManuallyVerifiedItem,
    LoginRequired,
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationDecisionRationale {
    /**A code representing the rationale for approving or declining the proposed transfer. Possible values are:

`MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available. Plaid will offer `approved` as a transaction decision.

`LOGIN_REQUIRED` – Unable to collect the account information due to Item staleness. Can be rectified using Link in update mode. Plaid will offer `approved` as a transaction decision.

`ERROR` – Unable to collect the account information due to an error. Plaid will offer `approved` as a transaction decision.

`NSF` – Transaction likely to result in a return due to insufficient funds. Plaid will offer `declined` as a transaction decision.

`RISK` - Transaction is high-risk. Plaid will offer `declined` as a transaction decision.

`TRANSFER_LIMIT_REACHED` - One or several transfer limits are reached, e.g. monthly transfer limit. Plaid will offer `declined` as a transaction decision.*/
    pub code: TransferAuthorizationDecisionRationaleCode,
    ///A human-readable description of the code associated with a transfer approval or transfer decline.
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecision {
    Guaranteed,
    NotGuaranteed,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationGuaranteeDecisionRationaleCode {
    ReturnBank,
    ReturnCustomer,
    GuaranteeLimitReached,
    RiskEstimateUnavailable,
    RequiredParamMissing,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationGuaranteeDecisionRationale {
    /**A code representing the reason Plaid declined to guarantee this transfer:

`RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.

`RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.

`GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guaranteed ACH has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.

`RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.

`REQUIRED_PARAM_MISSING`: Required fields are missing.*/
    pub code: TransferAuthorizationGuaranteeDecisionRationaleCode,
    ///A human-readable description of why the transfer cannot be guaranteed.
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///The network or rails used for the transfer.
    pub network: String,
    ///Plaid's unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationDevice {
    ///The IP address of the device being used to initiate the authorization. Required for guaranteed ACH customers.
    pub ip_address: String,
    ///The user agent of the device being used to initiate the authorization. Required for guaranteed ACH customers.
    pub user_agent: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMetadata {}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMetadata {}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferType {
    Debit,
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferType {
    Debit,
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    Posted,
    Cancelled,
    Failed,
    Returned,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferSweepStatus {
    Unswept,
    Swept,
    ReverseSwept,
    ReturnSwept,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferStatus {
    Pending,
    Posted,
    Cancelled,
    Failed,
    Reversed,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferNetwork {
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferNetwork {
    Ach,
    #[serde(rename = "same-day-ach")]
    SameDayAch,
    Wire,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferFailure {
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `returned`. For a full listing of ACH return codes, see [Transfer errors](https://plaid.com/docs/errors/transfer/#ach-return-codes).
    pub ach_return_code: Option<String>,
    ///A human-readable description of the reason for the failure or reversal.
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferFailure {
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/#ach-return-codes).
    pub ach_return_code: Option<String>,
    ///A human-readable description of the reason for the failure or reversal.
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequest {
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: TransferAccessToken,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`. The cutoff for same-day transfers is 7:45 AM Pacific Time and the cutoff for next-day transfers is 5:45 PM Pacific Time. It is recommended to submit a transfer at least 15 minutes before the cutoff time in order to ensure that it will be processed before the cutoff. Any transfer that is indicated as `same-day-ach` and that misses the same-day cutoff, but is submitted in time for the next-day cutoff, will be sent over next-day rails and will not incur same-day charges. Note that both legs of the transfer will be downgraded if applicable.
    pub network: TransferNetwork,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The legal name and other information for the account holder.
    pub user: TransferAuthorizationUserInRequest,
    ///Information about the device being used to initiate the authorization.
    pub device: TransferAuthorizationDevice,
    ///Plaid's unique identifier for the origination account for this authorization. If not specified, the default account will be used.
    pub origination_account_id: String,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: String,
    ///Required for guaranteed ACH customers. If the end user is initiating the specific transfer themselves via an interactive UI, this should be `true`; for automatic recurring payments where the end user is not actually initiating each individual transfer, it should be `false`.
    pub user_present: Option<bool>,
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: PaymentProfileId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateRequest {
    /**Deprecated. `authorization_id` is now used as idempotency instead.

A random key provided by the client, per unique transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single transfer is created.*/
    pub idempotency_key: TransferCreateIdempotencyKey,
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: TransferAccessToken,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    ///Plaid’s unique identifier for a transfer authorization. This parameter also serves the purpose of acting as an idempotency identifier.
    pub authorization_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`. The cutoff for same-day transfers is 7:45 AM Pacific Time and the cutoff for next-day transfers is 5:45 PM Pacific Time. It is recommended to submit a transfer at least 15 minutes before the cutoff time in order to ensure that it will be processed before the cutoff. Any transfer that is indicated as `same-day-ach` and that misses the same-day cutoff, but is submitted in time for the next-day cutoff, will be sent over next-day rails and will not incur same-day charges. Note that both legs of the transfer will be downgraded if applicable.
    pub network: TransferNetwork,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    pub origination_account_id: Option<String>,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: String,
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: PaymentProfileId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCreateRequest {
    /**A random key provided by the client, per unique bank transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created.*/
    pub idempotency_key: BankTransferIdempotencyKey,
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: BankTransferAccessToken,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: BankTransferType,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: BankTransferNetwork,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: BankTransferAmount,
    ///The currency of the transfer amount – should be set to "USD".
    pub iso_currency_code: String,
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    ///An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters.
    pub custom_tag: Option<String>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<BankTransferMetadata>,
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateResponse {
    ///Contains the authorization decision for a proposed transfer
    pub authorization: TransferAuthorization,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferAuthorizationDecision {
    Approved,
    Declined,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorization {
    ///Plaid’s unique identifier for a transfer authorization.
    pub id: TransferAuthorizationId,
    ///The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`.
    pub created: String,
    /**
A decision regarding the proposed transfer.

`approved` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub decision: TransferAuthorizationDecision,
    ///The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    pub decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
    ///Details regarding the proposed transfer.
    pub proposed_transfer: TransferAuthorizationProposedTransfer,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateResponse {
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCreateResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferListRequest {
    ///The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///The maximum number of transfers to return.
    pub count: i64,
    ///The number of transfers to skip before returning results.
    pub offset: i64,
    ///Filter transfers to only those originated through the specified origination account.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferListRequest {
    ///The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///The maximum number of bank transfers to return.
    pub count: i64,
    ///The number of bank transfers to skip before returning results.
    pub offset: i64,
    ///Filter bank transfers to only those originated through the specified origination account.
    pub origination_account_id: Option<String>,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: Option<BankTransferDirection>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferListResponse {
    pub transfers: Vec<Transfer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferListResponse {
    pub bank_transfers: Vec<BankTransfer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferDirection {
    Outbound,
    Inbound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCancelRequest {
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCancelRequest {
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCancelResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCancelResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferEventListTransferType {
    Debit,
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventListRequest {
    ///The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: Option<String>,
    ///The account ID to get events for all transactions to/from an account.
    pub account_id: Option<String>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
    pub transfer_type: Option<TransferEventListTransferType>,
    ///Filter events by event type.
    pub event_types: Vec<TransferEventType>,
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: String,
    ///The maximum number of transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    pub count: Option<i64>,
    ///The offset into the list of transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned.
    pub offset: Option<i64>,
    ///The origination account ID to get events for transfers from a specific origination account.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferEventListBankTransferType {
    Debit,
    Credit,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferEventListDirection {
    Inbound,
    Outbound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventListRequest {
    ///The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: Option<String>,
    ///The account ID to get events for all transactions to/from an account.
    pub account_id: Option<String>,
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
    pub bank_transfer_type: Option<BankTransferEventListBankTransferType>,
    ///Filter events by event type.
    pub event_types: Vec<BankTransferEventType>,
    ///The maximum number of bank transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    pub count: Option<i64>,
    ///The offset into the list of bank transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned.
    pub offset: Option<i64>,
    ///The origination account ID to get events for transfers from a specific origination account.
    pub origination_account_id: Option<String>,
    /**Indicates the direction of the transfer: `outbound`: for API-initiated transfers
`inbound`: for payments received by the FBO account.*/
    pub direction: Option<BankTransferEventListDirection>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferEventType {
    Pending,
    Cancelled,
    Failed,
    Posted,
    Returned,
    Swept,
    ReverseSwept,
    ReturnSwept,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BankTransferEventType {
    Pending,
    Cancelled,
    Failed,
    Posted,
    Reversed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEvent {
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: String,
    /**The type of event that this transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`returned`: A posted transfer was returned.

`swept`: The transfer was swept to / from the sweep account.

`return_swept`: Due to the transfer being returned, funds were pulled from or pushed back to the sweep account.*/
    pub event_type: TransferEventType,
    ///The account ID associated with the transfer.
    pub account_id: String,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferId,
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub transfer_type: TransferType,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub transfer_amount: TransferAmount,
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    pub failure_reason: Option<TransferFailure>,
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: Option<TransferSweepId>,
    ///A signed amount of how much was `swept` or `return_swept` (decimal string with two digits of precision e.g. "-5.50").
    pub sweep_amount: Option<TransferSweepAmount>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEvent {
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: String,
    /**The type of event that this bank transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`reversed`: A posted transfer was reversed.*/
    pub event_type: BankTransferEventType,
    ///The account ID associated with the bank transfer.
    pub account_id: String,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferId,
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub bank_transfer_type: BankTransferType,
    ///The bank transfer amount.
    pub bank_transfer_amount: String,
    ///The currency of the bank transfer amount.
    pub bank_transfer_iso_currency_code: String,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: Option<BankTransferFailure>,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: Option<BankTransferDirection>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventListResponse {
    pub transfer_events: Vec<TransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventListResponse {
    pub bank_transfer_events: Vec<BankTransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventSyncRequest {
    ///The latest (largest) `event_id` fetched via the sync endpoint, or 0 initially.
    pub after_id: i64,
    ///The maximum number of bank transfer events to return.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventSyncRequest {
    ///The latest (largest) `event_id` fetched via the sync endpoint, or 0 initially.
    pub after_id: i64,
    ///The maximum number of transfer events to return.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventSyncResponse {
    pub bank_transfer_events: Vec<BankTransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventSyncResponse {
    pub transfer_events: Vec<TransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepGetRequest {
    ///Identifier of the sweep.
    pub sweep_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepGetRequest {
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepGetResponse {
    ///BankTransferSweep describes a sweep transfer.
    pub sweep: BankTransferSweep,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepGetResponse {
    /**Describes a sweep of funds to / from the sweep account.

A sweep is associated with many sweep events (events of type `swept` or `return_swept`) which can be retrieved by invoking the `/transfer/event/list` endpoint with the corresponding `sweep_id`.

`swept` events occur when the transfer amount is credited or debited from your sweep account, depending on the `type` of the transfer. `return_swept` events occur when a transfer is returned and Plaid undoes the credit or debit.

The total sum of the `swept` and `return_swept` events is equal to the `amount` of the sweep Plaid creates and matches the amount of the entry on your sweep account ledger.*/
    pub sweep: TransferSweep,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepListRequest {
    ///If multiple origination accounts are available, `origination_account_id` must be used to specify the account that the sweeps belong to.
    pub origination_account_id: Option<String>,
    ///The start datetime of sweeps to return (RFC 3339 format).
    pub start_time: Option<String>,
    ///The end datetime of sweeps to return (RFC 3339 format).
    pub end_time: Option<String>,
    ///The maximum number of sweeps to return.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepListRequest {
    ///The start datetime of sweeps to return (RFC 3339 format).
    pub start_date: Option<String>,
    ///The end datetime of sweeps to return (RFC 3339 format).
    pub end_date: Option<String>,
    ///The maximum number of sweeps to return.
    pub count: Option<i64>,
    ///The number of sweeps to skip before returning results.
    pub offset: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepListResponse {
    pub sweeps: Vec<TransferSweep>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepListResponse {
    pub sweeps: Vec<BankTransferSweep>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweep {
    ///Identifier of the sweep.
    pub id: String,
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created_at: String,
    ///The amount of the sweep.
    pub amount: String,
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweep {
    ///Identifier of the sweep.
    pub id: String,
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created: String,
    /**Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. "-10.00")

If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced.*/
    pub amount: String,
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulatedTransferSweep {}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalanceGetRequest {
    ///If multiple origination accounts are available, `origination_account_id` must be used to specify the account for which balance will be returned.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalanceGetResponse {
    ///Information about the balance of a bank transfer
    pub balance: BankTransferBalance,
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalance {
    ///The total available balance - the sum of all successful debit transfer amounts minus all credit transfer amounts.
    pub available: String,
    ///The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.
    pub transactable: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountRequest {
    ///The user's account number.
    pub account_number: String,
    ///The user's routing number.
    pub routing_number: String,
    ///The user's wire transfer routing number. This is the ABA number; for some institutions, this may differ from the ACH number used in `routing_number`.
    pub wire_routing_number: String,
    ///The type of the bank account (`checking` or `savings`).
    pub account_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountResponse {
    ///The Plaid `access_token` for the newly created Item.
    pub access_token: String,
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMigrateAccountRequest {
    ///The user's account number.
    pub account_number: String,
    ///The user's routing number.
    pub routing_number: String,
    ///The user's wire transfer routing number. This is the ABA number; for some institutions, this may differ from the ACH number used in `routing_number`.
    pub wire_routing_number: String,
    ///The type of the bank account (`checking` or `savings`).
    pub account_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMigrateAccountResponse {
    ///The Plaid `access_token` for the newly created Item.
    pub access_token: String,
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentListRequest {
    ///The start datetime of repayments to return (RFC 3339 format).
    pub start_date: Option<String>,
    ///The end datetime of repayments to return (RFC 3339 format).
    pub end_date: Option<String>,
    ///The maximum number of repayments to return.
    pub count: Option<i64>,
    ///The number of repayments to skip before returning results.
    pub offset: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentListResponse {
    pub repayments: Vec<TransferRepayment>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepayment {
    ///Identifier of the repayment.
    pub repayment_id: String,
    ///The datetime when the repayment occurred, in RFC 3339 format.
    pub created: String,
    ///Decimal amount of the repayment as it appears on your account ledger.
    pub amount: String,
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListRequest {
    ///Identifier of the repayment to query.
    pub repayment_id: String,
    ///The maximum number of repayments to return.
    pub count: Option<i64>,
    ///The number of repayments to skip before returning results.
    pub offset: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListResponse {
    pub repayment_returns: Vec<TransferRepaymentReturn>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturn {
    ///The unique identifier of the guaranteed transfer that was returned.
    pub transfer_id: String,
    ///The unique identifier of the corresponding `returned` transfer event.
    pub event_id: i64,
    ///The value of the returned transfer.
    pub amount: String,
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: Option<String>,
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: TransferIntentCreateMode,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: Option<String>,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guaranteed ACH customers only).
    pub require_guarantee: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentStatus {
    Pending,
    Succeeded,
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    ///Plaid's unique identifier for the transfer intent object.
    pub id: String,
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: String,
    /**The status of the transfer intent.

- `PENDING` – The transfer intent is pending.
- `SUCCEEDED` – The transfer intent was successfully created.
- `FAILED` – The transfer intent was unable to be created.*/
    pub status: TransferIntentStatus,
    ///The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    pub account_id: Option<String>,
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: String,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: TransferIntentCreateMode,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guaranteed ACH customers only).
    pub require_guarantee: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateResponse {
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentCreate,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetRequest {
    ///Plaid's unique identifier for a transfer intent object.
    pub transfer_intent_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TransferIntentAuthorizationDecision {
    Approved,
    Declined,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGet {
    ///Plaid's unique identifier for a transfer intent object.
    pub id: String,
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: String,
    /**The status of the transfer intent.

- `PENDING` – The transfer intent is pending.
- `SUCCEEDED` – The transfer intent was successfully created.
- `FAILED` – The transfer intent was unable to be created.*/
    pub status: TransferIntentStatus,
    ///Plaid's unique identifier for the transfer created through the UI. Returned only if the transfer was successfully created. Null value otherwise.
    pub transfer_id: Option<String>,
    ///The reason for a failed transfer intent. Returned only if the transfer intent status is `failed`. Null otherwise.
    pub failure_reason: Option<TransferIntentGetFailureReason>,
    /**
A decision regarding the proposed transfer.

`APPROVED` – The proposed transfer has received the end user's consent and has been approved for processing by Plaid. The `decision_rationale` field is set if Plaid was unable to fetch the account information. You may proceed with the transfer, but further review is recommended (i.e., use Link in update to re-authenticate your user when `decision_rationale.code` is `LOGIN_REQUIRED`). Refer to the `code` field in the `decision_rationale` object for details.

`DECLINED` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub authorization_decision: Option<TransferIntentAuthorizationDecision>,
    ///The rationale for Plaid's decision regarding a proposed transfer. It is always set for `declined` decisions, and may or may not be null for `approved` decisions.
    pub authorization_decision_rationale: Option<TransferAuthorizationDecisionRationale>,
    ///The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    pub account_id: Option<String>,
    ///Plaid’s unique identifier for the origination account used for the transfer.
    pub origination_account_id: String,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: TransferIntentCreateMode,
    /**Specifies the use case of the transfer. Required for transfers on an ACH network.

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: AchClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: Option<TransferMetadata>,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    ///When `true`, the transfer requires a `GUARANTEED` decision by Plaid to proceed (Guaranteed ACH customers only).
    pub require_guarantee: Option<bool>,
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: Option<TransferAuthorizationGuaranteeDecision>,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: Option<
        TransferAuthorizationGuaranteeDecisionRationale,
    >,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetResponse {
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentGet,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateRequest {
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferId,
    /**The asynchronous event to be simulated. May be: `posted`, `failed`, or `reversed`.

An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:

`pending` --> `failed`

`pending` --> `posted`

`posted` --> `reversed`
*/
    pub event_type: String,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: Option<BankTransferFailure>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSimulateRequest {
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferId,
    /**The asynchronous event to be simulated. May be: `posted`, `failed`, or `returned`.

An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:

`pending` --> `failed`

`pending` --> `posted`

`posted` --> `returned`
*/
    pub event_type: String,
    ///The failure reason if the event type for a transfer is `"failed"` or `"returned"`. Null value otherwise.
    pub failure_reason: Option<TransferFailure>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateResponse {
    /**A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint.
Can be null if there are no transfers to include in a sweep.*/
    pub sweep: SimulatedTransferSweep,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFiltersResponse {
    ///A filter to apply to `depository`-type accounts
    pub depository: DepositoryFilter,
    ///A filter to apply to `credit`-type accounts
    pub credit: CreditFilter,
    ///A filter to apply to `loan`-type accounts
    pub loan: LoanFilter,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: InvestmentFilter,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchAccountFilter {
    pub loan: Vec<Option<AccountSubtype>>,
    pub depository: Vec<Option<AccountSubtype>>,
    pub credit: Vec<Option<AccountSubtype>>,
    pub investment: Vec<Option<AccountSubtype>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountIdentity(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAssets(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: DepositoryAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: CreditAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: LoanAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).
    pub account_subtypes: InvestmentAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryAccountSubtypes(pub Vec<DepositoryAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAccountSubtypes(pub Vec<CreditAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAccountSubtypes(pub Vec<LoanAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentAccountSubtypes(pub Vec<InvestmentAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub enum DepositoryAccountSubtype {
    Checking,
    Savings,
    Hsa,
    Cd,
    MoneyMarket,
    Paypal,
    Prepaid,
    CashManagement,
    Ebt,
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditAccountSubtype {
    CreditCard,
    Paypal,
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LoanAccountSubtype {
    Auto,
    Business,
    Commercial,
    Construction,
    Consumer,
    HomeEquity,
    Loan,
    Mortgage,
    LineOfCredit,
    Student,
    Other,
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
    Brokerage,
    CashIsa,
    CryptoExchange,
    EducationSavingsAccount,
    FixedAnnuity,
    Gic,
    HealthReimbursementArrangement,
    Hsa,
    Ira,
    Isa,
    Keogh,
    Lif,
    LifeInsurance,
    Lira,
    Lrif,
    Lrsp,
    MutualFund,
    #[serde(rename = "non-taxable brokerage account")]
    NonTaxableBrokerageAccount,
    Other,
    OtherAnnuity,
    OtherInsurance,
    Pension,
    Prif,
    ProfitSharingPlan,
    Qshr,
    Rdsp,
    Resp,
    Retirement,
    Rlif,
    Roth,
    Roth401K,
    Rrif,
    Rrsp,
    Sarsep,
    SepIra,
    SimpleIra,
    Sipp,
    StockPlan,
    Tfsa,
    Trust,
    Ugma,
    Utma,
    VariableAnnuity,
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersSearchRequest {
    ///The employer name to be searched for.
    pub query: String,
    ///The Plaid products the returned employers should support. Currently, this field must be set to `"deposit_switch"`.
    pub products: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersSearchResponse {
    ///A list of employers matching the search criteria.
    pub employers: Vec<Employer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Employer {
    ///Plaid's unique identifier for the employer.
    pub employer_id: String,
    ///The name of the employer
    pub name: String,
    ///Data about the components comprising an address.
    pub address: Option<AddressDataNullable>,
    ///A number from 0 to 1 indicating Plaid's level of confidence in the pairing between the employer and the institution (not yet implemented).
    pub confidence_score: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequest {
    ///The URL endpoint to which Plaid should send webhooks related to the progress of the income verification process.
    pub webhook: String,
    ///The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow.
    pub precheck_id: String,
    ///Optional arguments for `/income/verification/create`
    pub options: IncomeVerificationCreateRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequestOptions {
    ///An array of access tokens corresponding to the Items that will be cross-referenced with the product data. Plaid will attempt to correlate transaction history from these Items with data from the user's paystub, such as date and amount. The `verification` status of the paystub as returned by `/income/verification/paystubs/get` will indicate if the verification status was successful, or, if not, why it failed. If the `transactions` product was not initialized for the Items during Link, it will be initialized after this Link session.
    pub access_tokens: Vec<AccessToken>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateResponse {
    ///ID of the verification. This ID is persisted throughout the lifetime of the verification.
    pub income_verification_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckRequest {
    ///Information about the user whose eligibility is being evaluated.
    pub user: Option<IncomeVerificationPrecheckUser>,
    ///Information about the end user's employer
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub transactions_access_token: serde_json::Value,
    ///An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product.
    pub transactions_access_tokens: Vec<AccessToken>,
    ///Data about military info in the income verification precheck.
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployer {
    ///The employer's name
    pub name: Option<String>,
    ///The address of the employer
    pub address: Option<IncomeVerificationPrecheckEmployerAddress>,
    ///The employer's tax id
    pub tax_id: Option<String>,
    ///The URL for the employer's public website
    pub url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddress {}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddressData {
    ///The full city name
    pub city: String,
    ///The ISO 3166-1 alpha-2 country code
    pub country: String,
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: String,
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    pub region: String,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    ///Is the user currently active duty in the US military
    pub is_active_duty: Option<bool>,
    /**If the user is currently serving in the US military, the branch of the military in which they are serving
Valid values: 'AIR FORCE', 'ARMY', 'COAST GUARD', 'MARINES', 'NAVY', 'UNKNOWN'*/
    pub branch: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckUser {
    ///The user's first name
    pub first_name: Option<String>,
    ///The user's last name
    pub last_name: Option<String>,
    ///The user's email address
    pub email_address: Option<String>,
    ///Data about the components comprising an address.
    pub home_address: Option<SignalAddressData>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckResponse {
    ///ID of the precheck. Provide this value when calling `/link/token/create` in order to optimize Link conversion.
    pub precheck_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: IncomeVerificationPrecheckConfidence,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationPrecheckConfidence {
    High,
    Low,
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerification {
    ///The `income_verification_id` of the verification instance, as provided by `/income/verification/create`.
    pub income_verification_id: String,
    ///The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped.
    pub asset_report_id: String,
    ///The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow by streamlining the Link interface presented to the end user.
    pub precheck_id: String,
    /**An array of access tokens corresponding to Items that a user has previously connected with. Data from these institutions will be cross-referenced with document data received during the Document Income flow to help verify that the uploaded documents are accurate. If the `transactions` product was not initialized for these Items during link, it will be initialized after this Link session.

This field should only be used with the `payroll` income source type.*/
    pub access_tokens: Vec<AccessToken>,
    ///The types of source income data that users will be permitted to share. Options include `bank` and `payroll`. Currently you can only specify one of these options.
    pub income_source_types: Vec<IncomeVerificationSourceType>,
    ///Specifies options for initializing Link for use with Bank Income. This field is required if `income_verification` is included in the `products` array and `bank` is specified in `income_source_types`.
    pub bank_income: LinkTokenCreateRequestIncomeVerificationBankIncome,
    ///Specifies options for initializing Link for use with Payroll Income. This field is required if `income_verification` is included in the `products` array and `payroll` is specified in `income_source_types`.
    pub payroll_income: LinkTokenCreateRequestIncomeVerificationPayrollIncome,
    ///A list of user stated income sources
    pub stated_income_sources: Vec<LinkTokenCreateRequestUserStatedIncomeSource>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationSourceType {
    Bank,
    Payroll,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerificationBankIncome {
    ///The number of days of data to request for the Bank Income product
    pub days_requested: i64,
    ///Whether to enable multiple items to be added in the link session
    pub enable_multiple_items: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerificationPayrollIncome {
    ///The types of payroll income verification to enable for the link session. If none are specified, then users will see both document and digital payroll income.
    pub flow_types: Option<Vec<IncomeVerificationPayrollFlowType>>,
    ///An identifier to indicate whether the income verification link token will be used for an update or not
    pub is_update_mode: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeVerificationPayrollFlowType {
    PayrollDigitalIncome,
    PayrollDocumentIncome,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationStatusWebhook {
    ///`"INCOME"`
    pub webhook_type: String,
    ///`INCOME_VERIFICATION`
    pub webhook_code: String,
    ///The Item ID associated with the verification.
    pub item_id: String,
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: UserId,
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.*/
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshRequest {
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<AccessTokenNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    /**The verification refresh status. One of the following:

`"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED"` User presence is required to refresh an income verification.
`"VERIFICATION_REFRESH_SUCCESSFUL"` The income verification refresh was successful.
`"VERIFICATION_REFRESH_NOT_FOUND"` No new data was found after the income verification refresh.*/
    pub verification_refresh_status: VerificationRefreshStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummary {
    ///The name of the employer, as reported on the paystub.
    pub employer_name: EmployerIncomeSummaryFieldString,
    ///The name of the employee, as reported on the paystub.
    pub employee_name: EmployeeIncomeSummaryFieldString,
    ///Year-to-date pre-tax earnings, as reported on the paystub.
    pub ytd_gross_income: YtdGrossIncomeSummaryFieldNumber,
    ///Year-to-date earnings after any tax withholdings, benefit payments or deductions, as reported on the paystub.
    pub ytd_net_income: YtdNetIncomeSummaryFieldNumber,
    ///The frequency of the pay period.
    pub pay_frequency: Option<PayFrequency>,
    ///The employee's estimated annual salary, as derived from information reported on the paystub.
    pub projected_wage: ProjectedIncomeSummaryFieldNumber,
    ///Information about the matched direct deposit transaction used to verify a user's payroll information.
    pub verified_transaction: Option<TransactionData>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    ///The description of the transaction.
    pub description: String,
    ///The amount of the transaction.
    pub amount: f64,
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub date: String,
    ///A unique identifier for the end user's account.
    pub account_id: String,
    ///A unique identifier for the transaction.
    pub transaction_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummaryFieldString {
    ///The value of the field.
    pub value: String,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerIncomeSummaryFieldString(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeIncomeSummaryFieldString(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummaryFieldNumber {
    ///The value of the field.
    pub value: f64,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct YtdGrossIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct YtdNetIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectedIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayFrequency {
    ///The frequency of the pay period.
    pub value: PayFrequencyValue,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PayFrequencyValue {
    Monthly,
    Semimonthly,
    Weekly,
    Biweekly,
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationStatus {
    Verified,
    Unverified,
    NeedsInfo,
    UnableToVerify,
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum VerificationRefreshStatus {
    VerificationRefreshStatusUserPresenceRequired,
    VerificationRefreshSuccessful,
    VerificationRefreshNotFound,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetRequest {
    ///The ID of the verification for which to get paystub information.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<AccessTokenNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetResponse {
    ///Metadata for an income document.
    pub document_metadata: Vec<DocumentMetadata>,
    pub paystubs: Vec<Paystub>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    ///The name of the document.
    pub name: String,
    ///The processing status of the document.
    pub status: String,
    ///An identifier of the document that is also present in the paystub response.
    pub doc_id: String,
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
    pub doc_type: DocType,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocType {
    Unknown,
    DocumentTypePaystub,
    DocumentTypeBankStatement,
    DocumentTypeUsTaxW2,
    DocumentTypeUsMilitaryEras,
    DocumentTypeUsMilitaryLes,
    DocumentTypeUsMilitaryCles,
    DocumentTypeGig,
    DocumentTypeNone,
    DocumentTypeUsTax1099Misc,
    DocumentTypeUsTax1099K,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Paystub {
    ///An object with the deduction information found on a paystub.
    pub deductions: Deductions,
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: String,
    ///An object representing both a breakdown of earnings on a paystub and the total earnings.
    pub earnings: Earnings,
    ///Data about the employee.
    pub employee: Employee,
    ///Information about the employer on the paystub
    pub employer: PaystubEmployer,
    ///An object representing employment details found on a paystub.
    pub employment_details: EmploymentDetails,
    ///An object representing information about the net pay amount on the paystub.
    pub net_pay: NetPay,
    ///Details about the pay period.
    pub pay_period_details: PayPeriodDetails,
    ///An object representing details that can be found on the paystub.
    pub paystub_details: PaystubDetails,
    pub income_breakdown: Vec<IncomeBreakdown>,
    ///The amount of income earned year to date, as based on paystub data.
    pub ytd_earnings: PaystubYtdDetails,
    ///An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.
    pub verification: Option<PaystubVerification>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Deductions {
    pub subtotals: Vec<Total>,
    pub breakdown: Vec<DeductionsBreakdown>,
    pub totals: Vec<Total>,
    ///An object representing the total deductions for the pay period
    pub total: DeductionsTotal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeductionsBreakdown {
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    ///Description of the deduction line item
    pub description: Option<String>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeductionsTotal {
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date total amount of the deductions
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Total {
    ///Commonly used term to describe the line item.
    pub canonical_description: Option<TotalCanonicalDescription>,
    ///Text of the line item as printed on the paystub.
    pub description: Option<String>,
    ///An object representing a monetary amount.
    pub current_pay: Pay,
    ///An object representing a monetary amount.
    pub ytd_pay: Pay,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum TotalCanonicalDescription {
    Bonus,
    Commission,
    Overtime,
    PaidTimeOff,
    RegularPay,
    Vacation,
    EmployeeMedicare,
    Fica,
    SocialSecurityEmployeeTax,
    Medical,
    Vision,
    Dental,
    NetPay,
    Taxes,
    NotFound,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Pay {
    ///A numerical amount of a specific currency.
    pub amount: Option<f64>,
    ///Currency code, e.g. USD
    pub currency: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Earnings {
    pub subtotals: Vec<EarningsTotal>,
    pub totals: Vec<EarningsTotal>,
    pub breakdown: Vec<EarningsBreakdown>,
    ///An object representing both the current pay period and year to date amount for an earning category.
    pub total: EarningsTotal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsBreakdown {
    ///Commonly used term to describe the earning line item.
    pub canonical_description: Option<EarningsBreakdownCanonicalDescription>,
    ///Raw amount of the earning line item.
    pub current_amount: Option<f64>,
    ///Description of the earning line item.
    pub description: Option<String>,
    ///Number of hours applicable for this earning.
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///Hourly rate applicable for this earning.
    pub rate: Option<f64>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction.
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EarningsBreakdownCanonicalDescription {
    Bonus,
    Commission,
    Overtime,
    PaidTimeOff,
    RegularPay,
    Vacation,
    BasicAllowanceHousing,
    BasicAllowanceSubsistence,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsTotal {
    ///Total amount of the earnings for this pay period
    pub current_amount: Option<f64>,
    ///An object representing a monetary amount.
    pub current_pay: Pay,
    ///An object representing a monetary amount.
    pub ytd_pay: Pay,
    ///Total number of hours worked for this pay period
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The total year-to-date amount of the earnings
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDetails {
    ///An object representing a monetary amount.
    pub annual_salary: Pay,
    ///Date on which the employee was hired, in the YYYY-MM-DD format.
    pub hire_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetPay {
    ///Raw amount of the net pay for the pay period
    pub current_amount: Option<f64>,
    ///Description of the net pay
    pub description: Option<String>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the net pay
    pub ytd_amount: Option<f64>,
    ///An object representing both the current pay period and year to date amount for a category.
    pub total: Total,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubDetails {
    ///Beginning date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    pub pay_period_start_date: Option<String>,
    ///Ending date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    pub pay_period_end_date: Option<String>,
    ///Pay date on the paystub in the 'YYYY-MM-DD' format.
    pub pay_date: Option<String>,
    ///The name of the payroll provider that generated the paystub, e.g. ADP
    pub paystub_provider: Option<String>,
    ///The frequency at which the employee is paid. Possible values: `MONTHLY`, `BI-WEEKLY`, `WEEKLY`, `SEMI-MONTHLY`.
    pub pay_frequency: Option<PaystubPayFrequency>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaystubPayFrequency {
    Monthly,
    #[serde(rename = "BI-WEEKLY")]
    BiWeekly,
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
    pub type_: Option<IncomeBreakdownType>,
    ///The hourly rate at which the income is paid.
    pub rate: Option<f64>,
    ///The number of hours logged for this income for this pay period.
    pub hours: Option<f64>,
    ///The total pay for this pay period.
    pub total: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IncomeBreakdownType {
    Bonus,
    Overtime,
    Regular,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    ///Address on the paystub
    pub address: PaystubAddress,
    ///The name of the employee.
    pub name: Option<String>,
    ///Marital status of the employee - either `single` or `married`.
    pub marital_status: Option<String>,
    ///Taxpayer ID of the individual receiving the paystub.
    pub taxpayer_id: TaxpayerId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxpayerId {
    ///Type of ID, e.g. 'SSN'
    pub id_type: Option<String>,
    ///ID mask; i.e. last 4 digits of the taxpayer ID
    pub id_mask: Option<String>,
    ///Last 4 digits of unique number of ID.
    pub last_4_digits: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubEmployer {
    ///Address on the paystub
    pub address: PaystubAddress,
    ///The name of the employer on the paystub.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubAddress {
    ///The full city name.
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    ///The postal code of the address.
    pub postal_code: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    ///The full street address.
    pub street: Option<String>,
    ///Street address line 1.
    pub line_1: Option<String>,
    ///Street address line 2.
    pub line_2: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub state_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayPeriodDetails {
    ///The amount of the paycheck.
    pub check_amount: Option<f64>,
    pub distribution_breakdown: Vec<DistributionBreakdown>,
    ///The pay period end date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    pub end_date: Option<String>,
    ///Total earnings before tax/deductions.
    pub gross_earnings: Option<f64>,
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_date: Option<String>,
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_day: Option<String>,
    ///The pay period start date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    pub start_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionBreakdown {
    ///Name of the account for the given distribution.
    pub account_name: Option<String>,
    ///The name of the bank that the payment is being deposited to.
    pub bank_name: Option<String>,
    ///The amount distributed to this account.
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///The last 2-4 alphanumeric characters of an account's official account number.
    pub mask: Option<String>,
    #[serde(rename = "type")]
    ///Type of the account that the paystub was sent to (e.g. 'checking').
    pub type_: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///An object representing a monetary amount.
    pub current_pay: Pay,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubDeduction {
    #[serde(rename = "type")]
    ///The description of the deduction, as provided on the paystub. For example: `"401(k)"`, `"FICA MED TAX"`.
    pub type_: Option<String>,
    ///`true` if the deduction is pre-tax; `false` otherwise.
    pub is_pretax: Option<bool>,
    ///The amount of the deduction.
    pub total: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubYtdDetails {
    ///Year-to-date gross earnings.
    pub gross_earnings: Option<f64>,
    ///Year-to-date net (take home) earnings.
    pub net_earnings: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubVerification {
    ///Derived verification status.
    pub verification_status: Option<PaystubVerificationStatus>,
    pub verification_attributes: Vec<Option<VerificationAttribute>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaystubVerificationStatus {
    PaystubVerificationStatusUnknown,
    PaystubVerificationStatusVerified,
    PaystubVerificationStatusFraudulent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationAttribute {
    #[serde(rename = "type")]
    ///Message indicating the reason as to why the verification failed
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationDocumentsDownloadRequest {
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<AccessTokenNullable>,
    ///The document ID to download. If passed, a single document will be returned in the resulting zip file, rather than all document
    pub document_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetRequest {
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<AccessTokenNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    pub document_metadata: Vec<DocumentMetadata>,
    ///A list of forms.
    pub taxforms: Vec<Taxform>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Taxform {
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: String,
    ///The type of tax document. Currently, the only supported value is `w2`.
    pub document_type: String,
    ///W2 is an object that represents income data taken from a W2 tax document.
    pub w_2: W2,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2 {
    ///Information about the employer on the paystub
    pub employer: PaystubEmployer,
    ///Data about the employee.
    pub employee: Employee,
    ///The tax year of the W2 document.
    pub tax_year: Option<String>,
    ///An employee identification number or EIN.
    pub employer_id_number: Option<String>,
    ///Wages from tips and other compensation.
    pub wages_tips_other_comp: Option<String>,
    ///Federal income tax withheld for the tax year.
    pub federal_income_tax_withheld: Option<String>,
    ///Wages from social security.
    pub social_security_wages: Option<String>,
    ///Social security tax withheld for the tax year.
    pub social_security_tax_withheld: Option<String>,
    ///Wages and tips from medicare.
    pub medicare_wages_and_tips: Option<String>,
    ///Medicare tax withheld for the tax year.
    pub medicare_tax_withheld: Option<String>,
    ///Tips from social security.
    pub social_security_tips: Option<String>,
    ///Allocated tips.
    pub allocated_tips: Option<String>,
    ///Contents from box 9 on the W2.
    pub box_9: Option<String>,
    ///Dependent care benefits.
    pub dependent_care_benefits: Option<String>,
    ///Nonqualified plans.
    pub nonqualified_plans: Option<String>,
    pub box_12: Vec<W2Box12>,
    ///Statutory employee.
    pub statutory_employee: Option<String>,
    ///Retirement plan.
    pub retirement_plan: Option<String>,
    ///Third party sick pay.
    pub third_party_sick_pay: Option<String>,
    ///Other.
    pub other: Option<String>,
    pub state_and_local_wages: Vec<W2StateAndLocalWages>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2Box12 {
    ///W2 Box 12 code.
    pub code: Option<String>,
    ///W2 Box 12 amount.
    pub amount: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2StateAndLocalWages {
    ///State associated with the wage.
    pub state: Option<String>,
    ///State identification number of the employer.
    pub employer_state_id_number: Option<String>,
    ///Wages and tips from the specified state.
    pub state_wages_tips: Option<String>,
    ///Income tax from the specified state.
    pub state_income_tax: Option<String>,
    ///Wages and tips from the locality.
    pub local_wages_tips: Option<String>,
    ///Income tax from the locality.
    pub local_income_tax: Option<String>,
    ///Name of the locality.
    pub locality_name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationWebhookStatus {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationGetResponse {
    ///A list of employment verification summaries.
    pub employments: Vec<EmploymentVerification>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerification {
    ///Current employment status.
    pub status: Option<EmploymentVerificationStatus>,
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    pub start_date: Option<String>,
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    pub end_date: Option<String>,
    ///An object containing employer data.
    pub employer: EmployerVerification,
    ///Current title of employee.
    pub title: Option<String>,
    ///An object containing a set of ids related to an employee
    pub platform_ids: PlatformIds,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EmploymentVerificationStatus {
    EmploymentStatusActive,
    EmploymentStatusInactive,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerVerification {
    ///Name of employer.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformIds {
    ///The ID of an employee as given by their employer
    pub employee_id: Option<String>,
    ///The ID of an employee as given by their payroll
    pub payroll_id: Option<String>,
    ///The ID of the position of the employee
    pub position_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportTransaction(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthIncident {
    ///The start date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub start_date: String,
    ///The end date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub end_date: String,
    ///The title of the incident
    pub title: String,
    ///Updates on the health incident.
    pub incident_updates: Vec<IncidentUpdate>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncidentUpdate {
    ///The content of the update.
    pub description: String,
    ///The status of the incident.
    pub status: String,
    ///The date when the update was published, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub updated_date: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    ///The deposit switch destination account
    pub target_account: DepositSwitchTargetAccount,
    ///The deposit switch target user
    pub target_user: DepositSwitchTargetUser,
    ///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub options: DepositSwitchCreateRequestOptions,
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateResponse {
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTargetAccount {
    ///Account number for deposit switch destination
    pub account_number: String,
    ///Routing number for deposit switch destination
    pub routing_number: String,
    ///The name of the deposit switch destination account, as it will be displayed to the end user in the Deposit Switch interface. It is not required to match the name used in online banking.
    pub account_name: String,
    ///The account subtype of the account, either `checking` or `savings`.
    pub account_subtype: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTargetUser {
    ///The given name (first name) of the user.
    pub given_name: String,
    ///The family name (last name) of the user.
    pub family_name: String,
    ///The phone number of the user. The endpoint can accept a variety of phone number formats, including E.164.
    pub phone: String,
    ///The email address of the user.
    pub email: String,
    ///The user's address.
    pub address: DepositSwitchAddressData,
    ///The taxpayer ID of the user, generally their SSN, EIN, or TIN.
    pub tax_payer_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAddressData {
    ///The full city name
    pub city: String,
    /**The region or state
Example: `"NC"`*/
    pub region: String,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    ///The postal code
    pub postal_code: String,
    ///The ISO 3166-1 alpha-2 country code
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
    ///An optional object for `/credit/bank_income/get` request options.
    pub options: CreditBankIncomeGetRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeGetRequestOptions {
    ///How many Bank Income Reports should be fetched. Multiple reports may be available if the report has been re-created or refreshed. If more than one report is available, the most recent reports will be returned first.
    pub count: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeGetResponse {
    pub bank_income: Vec<CreditBankIncome>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomePdfGetRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomePdfGetResponse(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncome {
    ///The unique identifier associated with the Bank Income Report.
    pub bank_income_id: String,
    ///The time when the Bank Income Report was generated.
    pub generated_time: String,
    ///The number of days requested by the customer for the Bank Income Report.
    pub days_requested: i64,
    ///The list of Items in the report along with the associated metadata about the Item.
    pub items: Vec<CreditBankIncomeItem>,
    ///Summary for bank income across all income sources and items (max history of 730 days).
    pub bank_income_summary: CreditBankIncomeSummary,
    ///If data from the Bank Income report was unable to be retrieved, the warnings will contain information about the error that caused the data to be incomplete.
    pub warnings: Vec<CreditBankIncomeWarning>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeItem {
    ///The Item's accounts that have Bank Income data.
    pub bank_income_accounts: Vec<CreditBankIncomeAccount>,
    ///The income sources for this Item. Each entry in the array is a single income source.
    pub bank_income_sources: Vec<CreditBankIncomeSource>,
    ///The time when this Item's data was last retrieved from the financial institution.
    pub last_updated_time: String,
    ///The unique identifier of the institution associated with the Item.
    pub institution_id: String,
    ///The name of the institution associated with the Item.
    pub institution_name: String,
    ///The unique identifier for the Item.
    pub item_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeAccount {
    ///Plaid's unique identifier for the account.
    pub account_id: String,
    /**The last 2-4 alphanumeric characters of an account's official account number.
Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.*/
    pub mask: Option<String>,
    ///The name of the bank account.
    pub name: String,
    ///The official name of the bank account.
    pub official_name: Option<String>,
    ///Valid account subtypes for depository accounts. For a list containing descriptions of each subtype, see [Account schemas](https://plaid.com/docs/api/accounts/#StandaloneAccountType-depository).
    pub subtype: DepositoryAccountSubtype,
    #[serde(rename = "type")]
    ///The account type. This will always be `depository`.
    pub type_: CreditBankIncomeAccountType,
    pub owners: Vec<Owner>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeAccountType {
    Depository,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeSource {
    ///A unique identifier for an income source.
    pub income_source_id: String,
    ///The most common name or original description for the underlying income transactions.
    pub income_description: String,
    ///The income category.
    pub income_category: CreditBankIncomeCategory,
    ///Plaid's unique idenfier for the account.
    pub account_id: String,
    /**Minimum of all dates within the specific income sources in the user's bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: String,
    /**Maximum of all dates within the specific income sources in the user’s bank account for days requested by the client.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: String,
    ///The income pay frequency.
    pub pay_frequency: CreditBankIncomePayFrequency,
    ///Total amount of earnings in the user’s bank account for the specific income source for days requested by the client.
    pub total_amount: f64,
    ///Number of transactions for the income source within the start and end date.
    pub transaction_count: i64,
    pub historical_summary: Vec<CreditBankIncomeHistoricalSummary>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeCategory {
    Salary,
    Unemployment,
    Cash,
    GigEconomy,
    Rental,
    ChildSupport,
    Military,
    Retirement,
    LongTermDisability,
    BankInterest,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomePayFrequency {
    Weekly,
    Biweekly,
    SemiMonthly,
    Monthly,
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditIsoCurrencyCode(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditUnofficialCurrencyCode(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeSummary {
    ///Total amount of earnings across all the income sources in the end user's Items for the days requested by the client.
    pub total_amount: f64,
    ///The ISO 4217 currency code of the amount or balance.
    pub iso_currency_code: Option<CreditIsoCurrencyCode>,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    pub unofficial_currency_code: Option<CreditUnofficialCurrencyCode>,
    /**The earliest date within the days requested in which all income sources identified by Plaid appear in a user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: String,
    /**The latest date in which all income sources identified by Plaid appear in the user's account.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: String,
    ///Number of income sources per end user.
    pub income_sources_count: i64,
    ///Number of income categories per end user.
    pub income_categories_count: i64,
    ///Number of income transactions per end user.
    pub income_transactions_count: i64,
    pub historical_summary: Vec<CreditBankIncomeHistoricalSummary>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeHistoricalSummary {
    ///Total amount of earnings for the income source(s) of the user for the month in the summary.
    pub total_amount: f64,
    ///The ISO 4217 currency code of the amount or balance.
    pub iso_currency_code: Option<CreditIsoCurrencyCode>,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    pub unofficial_currency_code: Option<CreditUnofficialCurrencyCode>,
    /**The start date of the period covered in this monthly summary.
This date will be the first day of the month, unless the month being covered is a partial month because it is the first month included in the summary and the date range being requested does not begin with the first day of the month.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub start_date: String,
    /**The end date of the period included in this monthly summary.
This date will be the last day of the month, unless the month being covered is a partial month because it is the last month included in the summary and the date range being requested does not end with the last day of the month.
The date will be returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub end_date: String,
    pub transactions: Vec<CreditBankIncomeTransaction>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeTransaction {
    /**The settled value of the transaction, denominated in the transactions's currency as stated in `iso_currency_code` or `unofficial_currency_code`.
Positive values when money moves out of the account; negative values when money moves in.
For example, credit card purchases are positive; credit card payment, direct deposits, and refunds are negative.*/
    pub amount: f64,
    /**For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted.
Both dates are returned in an ISO 8601 format (YYYY-MM-DD).*/
    pub date: String,
    ///The merchant name or transaction description.
    pub name: String,
    ///The string returned by the financial institution to describe the transaction.
    pub original_description: Option<String>,
    /**When true, identifies the transaction as pending or unsettled.
Pending transaction details (name, type, amount, category ID) may change before they are settled.*/
    pub pending: bool,
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: String,
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
    ///The ISO 4217 currency code of the amount or balance.
    pub iso_currency_code: Option<CreditIsoCurrencyCode>,
    /**The unofficial currency code associated with the amount or balance. Always `null` if `iso_currency_code` is non-null.
Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.*/
    pub unofficial_currency_code: Option<CreditUnofficialCurrencyCode>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
    ///An optional object for `/credit/bank_income/refresh` request options.
    pub options: CreditBankIncomeRefreshRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshRequestOptions {
    ///How many days of data to include in the refresh. If not specified, this will default to the days requested in the most recently generated bank income report for the user.
    pub days_requested: i64,
    ///The URL where Plaid will send the bank income webhook.
    pub webhook: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRiskSignalsGetResponse {
    ///Array of payroll items.
    pub items: Vec<PayrollRiskSignalsItem>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollRiskSignalsItem {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///Array of payroll income document authenticity data retrieved for each of the user's accounts
    pub verification_risk_signals: Vec<DocumentRiskSignalsObject>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRiskSignalsObject {
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    ///Array of document metadata and associated risk signals per document
    pub single_document_risk_signals: Vec<SingleDocumentRiskSignal>,
    ///Array of risk signals computed from a set of uploaded documents and the associated documents' metadata
    pub multi_document_risk_signals: Vec<MultiDocumentRiskSignal>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskSignalDocumentReference {
    ///An identifier of the document referenced by the document metadata.
    pub document_id: Option<String>,
    ///The name of the document
    pub document_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleDocumentRiskSignal {
    ///Object containing metadata for the document
    pub document_reference: RiskSignalDocumentReference,
    ///Array of attributes that indicate whether or not there is fraud risk with a document
    pub risk_signals: Vec<Option<DocumentRiskSignal>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MultiDocumentRiskSignal {
    ///Array of objects containing attributes that could indicate if a document is fraudulent
    pub document_references: Vec<RiskSignalDocumentReference>,
    ///Array of attributes that indicate whether or not there is fraud risk with a set of documents
    pub risk_signals: Vec<Option<DocumentRiskSignal>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateRequest {
    ///List of report tokens; can include both Asset Report tokens and Income Report tokens.
    pub report_tokens: Vec<ReportToken>,
    ///The `auditor_id` of the third party with whom you would like to share the Asset Report and/or Income Report.
    pub auditor_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenCreateResponse {
    ///A token that can be shared with a third party auditor to allow them to obtain access to the Asset or Income Report. This token should be stored securely.
    pub audit_copy_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenRemoveRequest {
    ///The `audit_copy_token` granting access to the Audit Copy you would like to revoke.
    pub audit_copy_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAuditCopyTokenRemoveResponse {
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeGetRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeGetResponse {
    ///Array of payroll items.
    pub items: Vec<PayrollItem>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditDocumentMetadata {
    ///The name of the document.
    pub name: String,
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
    pub document_type: Option<CreditDocumentType>,
    ///Signed URL to retrieve the underlying file.
    pub download_url: Option<String>,
    ///The processing status of the document.
    pub status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditDocumentType(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollItem {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    pub accounts: Vec<Option<PayrollIncomeAccountData>>,
    pub payroll_income: Vec<PayrollIncomeObject>,
    ///Details about the status of the payroll item.
    pub status: Option<PayrollItemStatus>,
    ///A reference id to reference what payroll data was returned from this endpoint
    pub pull_id: CreditPullId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollIncomeAccountData {
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    ///An object representing the rate at which an individual is paid.
    pub rate_of_pay: PayrollIncomeRateOfPay,
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollIncomeObject {
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    ///Array of pay stubs for the user.
    pub pay_stubs: Vec<CreditPayStub>,
    ///Array of tax form W-2s.
    pub w_2_s: Vec<CreditW2>,
    ///Array of tax form 1099s.
    pub form_1099_s: Vec<Credit1099>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099 {
    ///An identifier of the document referenced by the document metadata.
    pub document_id: Option<String>,
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    ///Form 1099 Type
    pub form_1099_type: Form1099Type,
    ///An object representing a recipient used in both 1099-K and 1099-MISC tax documents.
    pub recipient: Credit1099Recipient,
    ///An object representing a payer used by 1099-MISC tax documents.
    pub payer: Credit1099Payer,
    ///An object representing a filer used by 1099-K tax documents.
    pub filer: Credit1099Filer,
    ///Tax year of the tax form.
    pub tax_year: Option<String>,
    ///Amount in rent by payer.
    pub rents: Option<f64>,
    ///Amount in royalties by payer.
    pub royalties: Option<f64>,
    ///Amount in other income by payer.
    pub other_income: Option<f64>,
    ///Amount of federal income tax withheld from payer.
    pub federal_income_tax_withheld: Option<f64>,
    ///Amount of fishing boat proceeds from payer.
    pub fishing_boat_proceeds: Option<f64>,
    ///Amount of medical and healthcare payments from payer.
    pub medical_and_healthcare_payments: Option<f64>,
    ///Amount of nonemployee compensation from payer.
    pub nonemployee_compensation: Option<f64>,
    ///Amount of substitute payments made by payer.
    pub substitute_payments_in_lieu_of_dividends_or_interest: Option<f64>,
    ///Whether or not payer made direct sales over $5000 of consumer products.
    pub payer_made_direct_sales_of_5000_or_more_of_consumer_products_to_buyer: Option<
        String,
    >,
    ///Amount of crop insurance proceeds.
    pub crop_insurance_proceeds: Option<f64>,
    ///Amount of golden parachute payments made by payer.
    pub excess_golden_parachute_payments: Option<f64>,
    ///Amount of gross proceeds paid to an attorney by payer.
    pub gross_proceeds_paid_to_an_attorney: Option<f64>,
    ///Amount of 409A deferrals earned by payer.
    pub section_409_a_deferrals: Option<f64>,
    ///Amount of 409A income earned by payer.
    pub section_409_a_income: Option<f64>,
    ///Amount of state tax withheld of payer for primary state.
    pub state_tax_withheld: Option<f64>,
    ///Amount of state tax withheld of payer for secondary state.
    pub state_tax_withheld_lower: Option<f64>,
    ///Primary state ID.
    pub payer_state_number: Option<String>,
    ///Secondary state ID.
    pub payer_state_number_lower: Option<String>,
    ///State income reported for primary state.
    pub state_income: Option<f64>,
    ///State income reported for secondary state.
    pub state_income_lower: Option<f64>,
    ///One of the values will be provided Payment card Third party network
    pub transactions_reported: Option<String>,
    ///Name of the PSE (Payment Settlement Entity).
    pub pse_name: Option<String>,
    ///Formatted (XXX) XXX-XXXX. Phone number of the PSE (Payment Settlement Entity).
    pub pse_telephone_number: Option<String>,
    ///Gross amount reported.
    pub gross_amount: Option<f64>,
    ///Amount in card not present transactions.
    pub card_not_present_transaction: Option<f64>,
    ///Merchant category of filer.
    pub merchant_category_code: Option<String>,
    ///Number of payment transactions made.
    pub number_of_payment_transactions: Option<String>,
    ///Amount reported for January.
    pub january_amount: Option<f64>,
    ///Amount reported for February.
    pub february_amount: Option<f64>,
    ///Amount reported for March.
    pub march_amount: Option<f64>,
    ///Amount reported for April.
    pub april_amount: Option<f64>,
    ///Amount reported for May.
    pub may_amount: Option<f64>,
    ///Amount reported for June.
    pub june_amount: Option<f64>,
    ///Amount reported for July.
    pub july_amount: Option<f64>,
    ///Amount reported for August.
    pub august_amount: Option<f64>,
    ///Amount reported for September.
    pub september_amount: Option<f64>,
    ///Amount reported for October.
    pub october_amount: Option<f64>,
    ///Amount reported for November.
    pub november_amount: Option<f64>,
    ///Amount reported for December.
    pub december_amount: Option<f64>,
    ///Primary state of business.
    pub primary_state: Option<String>,
    ///Secondary state of business.
    pub secondary_state: Option<String>,
    ///Primary state ID.
    pub primary_state_id: Option<String>,
    ///Secondary state ID.
    pub secondary_state_id: Option<String>,
    ///State income tax reported for primary state.
    pub primary_state_income_tax: Option<f64>,
    ///State income tax reported for secondary state.
    pub secondary_state_income_tax: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Form1099Type {
    Form1099TypeUnknown,
    Form1099TypeMisc,
    Form1099TypeK,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099Payer {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///Name of payer.
    pub name: Option<String>,
    ///Tax identification number of payer.
    pub tin: Option<String>,
    ///Telephone number of payer.
    pub telephone_number: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099Recipient {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///Name of recipient.
    pub name: Option<String>,
    ///Tax identification number of recipient.
    pub tin: Option<String>,
    ///Account number number of recipient.
    pub account_number: Option<String>,
    ///Checked if FACTA is a filing requirement.
    pub facta_filing_requirement: Option<String>,
    ///Checked if 2nd TIN exists.
    pub second_tin_exists: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Credit1099Filer {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///Name of filer.
    pub name: Option<String>,
    ///Tax identification number of filer.
    pub tin: Option<String>,
    #[serde(rename = "type")]
    ///One of the following values will be provided: Payment Settlement Entity (PSE), Electronic Payment Fecilitator (EPF), Other Third Party
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStub {
    ///An object with the deduction information found on a pay stub.
    pub deductions: CreditPayStubDeductions,
    ///An identifier of the document referenced by the document metadata.
    pub document_id: Option<String>,
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    ///An object representing both a breakdown of earnings on a pay stub and the total earnings.
    pub earnings: CreditPayStubEarnings,
    ///Data about the employee.
    pub employee: CreditPayStubEmployee,
    ///Information about the employer on the pay stub.
    pub employer: CreditPayStubEmployer,
    ///An object representing information about the net pay amount on the pay stub.
    pub net_pay: CreditPayStubNetPay,
    ///Details about the pay period.
    pub pay_period_details: PayStubPayPeriodDetails,
    ///(To be deprecated) Verification info will be moved to a separate endpoint in the future. An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.
    pub verification: Option<CreditPayStubVerification>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubDeductions {
    pub breakdown: Vec<PayStubDeductionsBreakdown>,
    ///An object representing the total deductions for the pay period
    pub total: PayStubDeductionsTotal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubDeductionsBreakdown {
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    ///Description of the deduction line item
    pub description: Option<String>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubDeductionsTotal {
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date total amount of the deductions
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubEarnings {
    pub breakdown: Vec<PayStubEarningsBreakdown>,
    ///An object representing both the current pay period and year to date amount for an earning category.
    pub total: PayStubEarningsTotal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubEarningsBreakdown {
    ///Commonly used term to describe the earning line item.
    pub canonical_description: Option<PayStubEarningsBreakdownCanonicalDescription>,
    ///Raw amount of the earning line item.
    pub current_amount: Option<f64>,
    ///Description of the earning line item.
    pub description: Option<String>,
    ///Number of hours applicable for this earning.
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///Hourly rate applicable for this earning.
    pub rate: Option<f64>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction.
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubEarningsBreakdownCanonicalDescription(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubEarningsTotal {
    ///Total amount of the earnings for this pay period.
    pub current_amount: Option<f64>,
    ///Total number of hours worked for this pay period.
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The total year-to-date amount of the earnings.
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubEmployee {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///The name of the employee.
    pub name: Option<String>,
    ///Marital status of the employee - either `SINGLE` or `MARRIED`.
    pub marital_status: Option<String>,
    ///Taxpayer ID of the individual receiving the paystub.
    pub taxpayer_id: PayStubTaxpayerId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubAddress {
    ///The full city name.
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    ///The postal code of the address.
    pub postal_code: Option<String>,
    /**The region or state.
Example: `"NC"`*/
    pub region: Option<String>,
    ///The full street address.
    pub street: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubTaxpayerId {
    ///Type of ID, e.g. 'SSN'.
    pub id_type: Option<String>,
    ///ID mask; i.e. last 4 digits of the taxpayer ID.
    pub id_mask: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubEmployer {
    ///Address on the pay stub.
    pub address: CreditPayStubAddress,
    ///The name of the employer on the pay stub.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubNetPay {
    ///Raw amount of the net pay for the pay period.
    pub current_amount: Option<f64>,
    ///Description of the net pay.
    pub description: Option<String>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the net pay.
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubPayPeriodDetails {
    ///The amount of the paycheck.
    pub pay_amount: Option<f64>,
    pub distribution_breakdown: Vec<PayStubDistributionBreakdown>,
    ///The date on which the pay period ended, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub end_date: Option<String>,
    ///Total earnings before tax/deductions.
    pub gross_earnings: Option<f64>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///The date on which the pay stub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_date: Option<String>,
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
    ///The date on which the pay period started, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub start_date: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubDistributionBreakdown {
    ///Name of the account for the given distribution.
    pub account_name: Option<String>,
    ///The name of the bank that the payment is being deposited to.
    pub bank_name: Option<String>,
    ///The amount distributed to this account.
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///The last 2-4 alphanumeric characters of an account's official account number.
    pub mask: Option<String>,
    #[serde(rename = "type")]
    ///Type of the account that the paystub was sent to (e.g. 'checking').
    pub type_: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubVerification {
    ///Derived verification status.
    pub verification_status: Option<CreditPayStubVerificationStatus>,
    pub verification_attributes: Vec<Option<PayStubVerificationAttribute>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayStubVerificationStatus(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayStubVerificationAttribute {
    #[serde(rename = "type")]
    ///Message indicating the reason as to why the verification failed.
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportToken {
    ///The report type. It can be `assets` or `income`.
    pub report_type: ReportType,
    ///The report token. It can be an asset report token or an income report token.
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ReportType {
    Assets,
    Income,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRiskSignal {
    #[serde(rename = "type")]
    ///The result from the risk signal check.
    pub type_: Option<String>,
    ///The field which the risk signal was computed for
    pub field: Option<String>,
    ///A flag used to quickly identify if the signal indicates that this field is authentic or fraudulent
    pub has_fraud_risk: Option<bool>,
    ///An object which contains additional metadata about the institution used to compute the verification attribute
    pub institution_metadata: Option<DocumentRiskSignalInstitutionMetadata>,
    ///The expected value of the field, as seen on the document
    pub expected_value: Option<String>,
    ///The derived value obtained in the risk signal calculation process for this field
    pub actual_value: Option<String>,
    ///A human-readable explanation providing more detail into the particular risk signal
    pub signal_description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRiskSignalInstitutionMetadata {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollItemStatus {
    /**Denotes the processing status for the verification.

`UNKNOWN`: The processing status could not be determined.

`PROCESSING_COMPLETE`: The processing has completed and the user has approved for sharing. The data is available to be retrieved.

`PROCESSING`: The verification is still processing. The data is not available yet.

`FAILED`: The processing failed to complete successfully.

`APPROVAL_STATUS_PENDING`: The processing has completed but the user has not yet approved the sharing of the data.*/
    pub processing_status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditW2 {
    ///Object representing metadata pertaining to the document.
    pub document_metadata: CreditDocumentMetadata,
    ///An identifier of the document referenced by the document metadata.
    pub document_id: String,
    ///Information about the employer on the pay stub.
    pub employer: CreditPayStubEmployer,
    ///Data about the employee.
    pub employee: CreditPayStubEmployee,
    ///The tax year of the W2 document.
    pub tax_year: Option<String>,
    ///An employee identification number or EIN.
    pub employer_id_number: Option<String>,
    ///Wages from tips and other compensation.
    pub wages_tips_other_comp: Option<String>,
    ///Federal income tax withheld for the tax year.
    pub federal_income_tax_withheld: Option<String>,
    ///Wages from social security.
    pub social_security_wages: Option<String>,
    ///Social security tax withheld for the tax year.
    pub social_security_tax_withheld: Option<String>,
    ///Wages and tips from medicare.
    pub medicare_wages_and_tips: Option<String>,
    ///Medicare tax withheld for the tax year.
    pub medicare_tax_withheld: Option<String>,
    ///Tips from social security.
    pub social_security_tips: Option<String>,
    ///Allocated tips.
    pub allocated_tips: Option<String>,
    ///Contents from box 9 on the W2.
    pub box_9: Option<String>,
    ///Dependent care benefits.
    pub dependent_care_benefits: Option<String>,
    ///Nonqualified plans.
    pub nonqualified_plans: Option<String>,
    pub box_12: Vec<W2Box12>,
    ///Statutory employee.
    pub statutory_employee: Option<String>,
    ///Retirement plan.
    pub retirement_plan: Option<String>,
    ///Third party sick pay.
    pub third_party_sick_pay: Option<String>,
    ///Other.
    pub other: Option<String>,
    pub state_and_local_wages: Vec<W2StateAndLocalWages>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayrollIncomeRateOfPay {
    ///The rate at which an employee is paid.
    pub pay_rate: Option<String>,
    ///The amount at which an employee is paid.
    pub pay_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
    ///An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product.
    pub access_tokens: Vec<AccessToken>,
    ///Information about the end user's employer
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    ///Data about military info in the income verification precheck.
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomePrecheckResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: IncomeVerificationPrecheckConfidence,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPayrollIncomeRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    /**The verification refresh status. One of the following:

`"USER_PRESENCE_REQUIRED"` User presence is required to refresh an income verification.
`"SUCCESSFUL"` The income verification refresh was successful.
`"NOT_FOUND"` No new data was found after the income verification refresh.*/
    pub verification_refresh_status: CreditPayrollIncomeRefreshStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentGetRequest {
    ///The user token associated with the User data is being requested for.
    pub user_token: UserToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentGetResponse {
    ///Array of employment items.
    pub items: Vec<CreditEmploymentItem>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentItem {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    pub employments: Vec<CreditEmploymentVerification>,
    ///A reference id to reference what payroll data was returned from this endpoint
    pub pull_id: CreditPullId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentVerification {
    ///ID of the payroll provider account.
    pub account_id: Option<String>,
    ///Current employment status.
    pub status: Option<CreditEmploymentVerificationStatus>,
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    pub start_date: Option<String>,
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    pub end_date: Option<String>,
    ///An object containing employer data.
    pub employer: CreditEmployerVerification,
    ///Current title of employee.
    pub title: Option<String>,
    ///The object containing a set of ids related to an employee.
    pub platform_ids: CreditPlatformIds,
    /**The type of employment for the individual.
`"FULL_TIME"`: A full-time employee.
`"PART_TIME"`: A part-time employee.
`"CONTRACTOR"`: An employee typically hired externally through a contracting group.
`"TEMPORARY"`: A temporary employee.
`"OTHER"`: The employee type is not one of the above defined types.*/
    pub employee_type: Option<CreditEmploymentEmployeeType>,
    ///The date of the employee's most recent paystub in ISO 8601 format (YYYY-MM-DD).
    pub last_paystub_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentEmployeeType(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmploymentVerificationStatus(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditEmployerVerification {
    ///Name of employer.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPlatformIds {
    ///The ID of an employee as given by their employer.
    pub employee_id: Option<String>,
    ///The ID of an employee as given by their payroll.
    pub payroll_id: Option<String>,
    ///The ID of the position of the employee.
    pub position_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeWarning {
    ///The warning type which will always be `BANK_INCOME_WARNING`.
    pub warning_type: CreditBankIncomeWarningType,
    /**The warning code identifies a specific kind of warning.
`IDENTITY_UNAVAILABLE`: Unable to extract identity for the Item
`TRANSACTIONS_UNAVAILABLE`: Unable to extract transactions for the Item
`ITEM_UNAPPROVED`: User did not grant permission to share income data for the Item
`REPORT_DELETED`: Report deleted due to customer or consumer request*/
    pub warning_code: CreditBankIncomeWarningCode,
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: CreditBankIncomeCause,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningType {
    BankIncomeWarning,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeWarningCode {
    IdentityUnavailable,
    TransactionsUnavailable,
    ItemUnapproved,
    ReportDeleted,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditBankIncomeCause {
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: CreditBankIncomeErrorType,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues. Error fields will be `null` if no error has occurred.
    pub error_code: String,
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    /**A user-friendly representation of the error code. null if the error is not related to user action.
This may change over time and is not safe for programmatic use.*/
    pub display_message: String,
    ///The `item_id` of the Item associated with this warning.
    pub item_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum CreditBankIncomeErrorType {
    InternalServerError,
    InsufficientCredentials,
    ItemLocked,
    UserSetupRequired,
    CountryNotSupported,
    InstitutionDown,
    InstitutionNoLongerSupported,
    InstitutionNotResponding,
    InvalidCredentials,
    InvalidMfa,
    InvalidSendMethod,
    ItemLoginRequired,
    MfaNotSupported,
    NoAccounts,
    ItemNotSupported,
    AccessNotGranted,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayCreateRequest {
    ///List of report tokens, with at most one token of each report type. Currently only Asset Report token is supported.
    pub report_tokens: Vec<ReportToken>,
    ///The `secondary_client_id` is the client id of the third party with whom you would like to share the Relay Token.
    pub secondary_client_id: String,
    ///URL to which Plaid will send webhooks when the Secondary Client successfully retrieves an Asset Report by calling `/credit/relay/get`.
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayCreateResponse {
    ///A token that can be shared with a third party to allow them to access the Asset Report. This token should be stored securely.
    pub relay_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayGetRequest {
    ///The `relay_token` granting access to the report you would like to get.
    pub relay_token: String,
    ///The report type. It can be `assets` or `income`.
    pub report_type: ReportType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRefreshRequest {
    ///The `relay_token` granting access to the report you would like to refresh.
    pub relay_token: String,
    ///The report type. It can be `assets` or `income`.
    pub report_type: ReportType,
    ///The URL registered to receive webhooks when the report of a Relay Token has been refreshed.
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRefreshResponse {
    pub relay_token: String,
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRemoveRequest {
    ///The `relay_token` you would like to revoke.
    pub relay_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditRelayRemoveResponse {
    ///`true` if the Relay token was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookRequest {
    ///The URL to which the webhook should be sent.
    pub webhook: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferFireWebhookRequest {
    ///The URL to which the webhook should be sent.
    pub webhook: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationId,
    ///The name of the application
    pub name: String,
    ///A human-readable name of the application for display purposes
    pub display_name: Option<String>,
    ///The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub join_date: String,
    ///A URL that links to the application logo image.
    pub logo_url: Option<String>,
    ///The URL for the application's website
    pub application_url: Option<String>,
    ///A string provided by the connected app stating why they use their respective enabled products.
    pub reason_for_access: Option<String>,
    ///A string representing client’s broad use case as assessed by Plaid.
    pub use_case: Option<String>,
    ///A string representing the name of client’s legal entity.
    pub company_legal_name: Option<String>,
    ///A string representing the city of the client’s headquarters.
    pub city: Option<String>,
    ///A string representing the region of the client’s headquarters.
    pub region: Option<String>,
    ///A string representing the postal code of the client’s headquarters.
    pub postal_code: Option<String>,
    ///A string representing the country code of the client’s headquarters.
    pub country_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationGetRequest {
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    ///Metadata about the application
    pub application: Application,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductAccess {
    ///Allow access to statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub statements: Option<bool>,
    ///Allow access to the Identity product (name, email, phone, address). Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub identity: Option<bool>,
    ///Allow access to account number details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub auth: Option<bool>,
    ///Allow access to transaction details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub transactions: Option<bool>,
    ///Allow access to "accounts_details_transactions". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_details_transactions: Option<bool>,
    ///Allow access to "accounts_routing_number". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_routing_number: Option<bool>,
    ///Allow access to "accounts_statements". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_statements: Option<bool>,
    ///Allow access to "accounts_tax_statements". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub accounts_tax_statements: Option<bool>,
    ///Allow access to "customers_profiles". Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub customers_profiles: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAccess {
    ///The unique account identifier for this account. This value must match that returned by the data access API for this account.
    pub unique_id: String,
    ///Allow the application to see this account (and associated details, including balance) in the list of accounts  If unset, defaults to `true`.
    pub authorized: Option<bool>,
    ///Allow the application to access specific products on this account
    pub account_product_access: Option<AccountProductAccessNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProductAccessNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProductAccess {
    ///Allow the application to access account data. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub account_data: Option<bool>,
    ///Allow the application to access bank statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub statements: Option<bool>,
    ///Allow the application to access tax documents. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub tax_documents: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Scopes {
    ///The product access being requested. Used to or disallow product access across all accounts. If unset, defaults to all products allowed.
    pub product_access: ProductAccess,
    pub accounts: Vec<AccountAccess>,
    ///Allow access to newly opened accounts as they are opened. If unset, defaults to `true`.
    pub new_accounts: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesState(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum ScopesContext {
    Enrollment,
    Portal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationId,
    ///The scopes object
    pub scopes: Scopes,
    ///When scopes are updated during enrollment, this field must be populated with the state sent to the partner in the OAuth Login URI. This field is required when the context is `ENROLLMENT`.
    pub state: ScopesState,
    ///An indicator for when scopes are being updated. When scopes are updated via enrollment (i.e. OAuth), the partner must send `ENROLLMENT`. When scopes are updated in a post-enrollment view, the partner must send `PORTAL`.
    pub context: ScopesContext,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: Option<AccessTokenNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    ///A list of connected applications.
    pub applications: Vec<ConnectedApplication>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedApplication {
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationId,
    ///The name of the application
    pub name: String,
    ///A human-readable name of the application for display purposes
    pub display_name: Option<String>,
    ///A URL that links to the application logo image.
    pub logo_url: Option<String>,
    ///The URL for the application's website
    pub application_url: Option<String>,
    ///A string provided by the connected app stating why they use their respective enabled products.
    pub reason_for_access: Option<String>,
    ///The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub created_at: String,
    ///The scopes object
    pub scopes: Option<ScopesNullable>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountSelectionCardinality {
    SingleSelect,
    MultiSelect,
    All,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFilter {
    ///A list of account subtypes to be filtered.
    pub depository: AccountFilterSubtypes,
    ///A list of account subtypes to be filtered.
    pub credit: AccountFilterSubtypes,
    ///A list of account subtypes to be filtered.
    pub loan: AccountFilterSubtypes,
    ///A list of account subtypes to be filtered.
    pub investment: AccountFilterSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFilterSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookRequest {
    ///The Item ID associated with the verification.
    pub item_id: String,
    ///The Plaid `user_id` of the User associated with this webhook, warning, or error.
    pub user_id: UserId,
    ///The URL to which the webhook should be sent.
    pub webhook: String,
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.*/
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListUserAuth {
    ///Account username.
    pub user_id: Option<String>,
    ///Account username hashed by FI.
    pub fi_username_hash: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` of the account whose verification status is to be modified
    pub account_id: String,
    ///The unique ID that you would like to use to refer to this transaction. For your convenience mapping your internal data, you could use your internal ID/identifier for this transaction. The max length for this field is 36 characters.
    pub client_transaction_id: String,
    ///The transaction amount, in USD (e.g. `102.05`)
    pub amount: f64,
    ///`true` if the end user is present while initiating the ACH transfer and the endpoint is being called; `false` otherwise (for example, when the ACH transfer is scheduled and the end user is not present, or you call this endpoint after the ACH transfer but before submitting the Nacha file for ACH processing).
    pub user_present: Option<bool>,
    ///A unique ID that identifies the end user in your system. This ID is used to correlate requests by a user with multiple Items. The max length for this field is 36 characters.
    pub client_user_id: String,
    ///Details about the end user initiating the transaction (i.e., the account holder).
    pub user: SignalUser,
    ///Details about the end user's device
    pub device: SignalDevice,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalUser {
    ///The user's legal name
    pub name: Option<SignalPersonName>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567"
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email_address: Option<String>,
    ///Data about the components comprising an address.
    pub address: Option<SignalAddressData>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPersonName {
    ///The user's name prefix (e.g. "Mr.")
    pub prefix: Option<String>,
    ///The user's given name. If the user has a one-word name, it should be provided in this field.
    pub given_name: Option<String>,
    ///The user's middle name
    pub middle_name: Option<String>,
    ///The user's family name / surname
    pub family_name: Option<String>,
    ///The user's name suffix (e.g. "II")
    pub suffix: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalAddressData {
    ///The full city name
    pub city: String,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    ///The postal code
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDevice {
    ///The IP address of the device that initiated the transaction
    pub ip_address: Option<String>,
    ///The user agent of the device that initiated the transaction (e.g. "Mozilla/5.0")
    pub user_agent: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
    ///Risk scoring details broken down by risk category.
    pub scores: SignalScores,
    /**The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:

`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
`is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account

For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
    pub core_attributes: SignalEvaluateCoreAttributes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalScores {
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk of an unauthorized debit. Common return codes in this category include: "R05", "R07", "R10", "R11", "R29". These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.
    pub customer_initiated_return_risk: CustomerInitiatedReturnRisk,
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk because an account is overdrawn or because an ineligible account is used. Common return codes in this category include: "R01", "R02", "R03", "R04", "R06", "R08",  "R09", "R13", "R16", "R17", "R20", "R23". These returns have a turnaround time of 2 banking days.
    pub bank_initiated_return_risk: BankInitiatedReturnRisk,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalScore(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInitiatedRiskTier(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInitiatedReturnRisk {
    ///A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: SignalScore,
    /**A tier corresponding to the projected likelihood that the transaction, if initiated, will be subject to a return.

In the `customer_initiated_return_risk` object, there are five risk tiers corresponding to the scores:
  1: Predicted customer-initiated return incidence rate between 0.00% - 0.02%
  2: Predicted customer-initiated return incidence rate between 0.02% - 0.05%
  3: Predicted customer-initiated return incidence rate between 0.05% - 0.1%
  4: Predicted customer-initiated return incidence rate between 0.1% - 0.5%
  5: Predicted customer-initiated return incidence rate greater than 0.5%
*/
    pub risk_tier: CustomerInitiatedRiskTier,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankInitiatedRiskTier(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankInitiatedReturnRisk {
    ///A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: SignalScore,
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
    pub risk_tier: BankInitiatedRiskTier,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateCoreAttributes {
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited.
    pub unauthorized_transactions_count_7_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited.
    pub unauthorized_transactions_count_30_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited.
    pub unauthorized_transactions_count_60_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited.
    pub unauthorized_transactions_count_90_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_7_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_30_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_60_d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_90_d: Option<i64>,
    ///The number of days since the first time the Item was connected to an application via Plaid
    pub days_since_first_plaid_connection: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 7 days
    pub plaid_connections_count_7_d: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 30 days
    pub plaid_connections_count_30_d: Option<i64>,
    ///The total number of times the Item has been connected to applications via Plaid
    pub total_plaid_connections_count: Option<i64>,
    ///Indicates if the ACH transaction funding account is a savings/money market account
    pub is_savings_or_money_market_account: bool,
    ///The total credit (inflow) transaction amount over the past 10 days from the account that will be debited
    pub total_credit_transactions_amount_10_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 10 days from the account that will be debited
    pub total_debit_transactions_amount_10_d: Option<f64>,
    ///The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_50_credit_transactions_amount_28_d: Option<f64>,
    ///The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_50_debit_transactions_amount_28_d: Option<f64>,
    ///The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_95_credit_transactions_amount_28_d: Option<f64>,
    ///The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    pub p_95_debit_transactions_amount_28_d: Option<f64>,
    ///The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance
    pub days_with_negative_balance_count_90_d: Option<i64>,
    ///The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p_90_eod_balance_30_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p_90_eod_balance_60_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p_90_eod_balance_90_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p_10_eod_balance_30_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p_10_eod_balance_60_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p_10_eod_balance_90_d: Option<f64>,
    ///Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.
    pub available_balance: Option<f64>,
    ///Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account.
    pub current_balance: Option<f64>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated.
    pub balance_last_updated: Option<String>,
    ///The number of times the account's phone numbers on file have changed over the past 28 days
    pub phone_change_count_28_d: Option<i64>,
    ///The number of times the account's phone numbers on file have changed over the past 90 days
    pub phone_change_count_90_d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 28 days
    pub email_change_count_28_d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 90 days
    pub email_change_count_90_d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 28 days
    pub address_change_count_28_d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 90 days
    pub address_change_count_90_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    pub plaid_non_oauth_authentication_attempts_count_3_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    pub plaid_non_oauth_authentication_attempts_count_7_d: Option<i64>,
    ///The number of non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    pub plaid_non_oauth_authentication_attempts_count_30_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 3 days
    pub failed_plaid_non_oauth_authentication_attempts_count_3_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 7 days
    pub failed_plaid_non_oauth_authentication_attempts_count_7_d: Option<i64>,
    ///The number of failed non-OAuth authentication attempts via Plaid for this bank account over the past 30 days
    pub failed_plaid_non_oauth_authentication_attempts_count_30_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 10 days from the account that will be debited
    pub debit_transactions_count_10_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 10 days from the account that will be debited
    pub credit_transactions_count_10_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 30 days from the account that will be debited
    pub debit_transactions_count_30_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 30 days from the account that will be debited
    pub credit_transactions_count_30_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 60 days from the account that will be debited
    pub debit_transactions_count_60_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 60 days from the account that will be debited
    pub credit_transactions_count_60_d: Option<i64>,
    ///The total number of debit (outflow) transactions over the past 90 days from the account that will be debited
    pub debit_transactions_count_90_d: Option<i64>,
    ///The total number of credit (inflow) transactions over the past 90 days from the account that will be debited
    pub credit_transactions_count_90_d: Option<i64>,
    ///The total debit (outflow) transaction amount over the past 30 days from the account that will be debited
    pub total_debit_transactions_amount_30_d: Option<f64>,
    ///The total credit (inflow) transaction amount over the past 30 days from the account that will be debited
    pub total_credit_transactions_amount_30_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 60 days from the account that will be debited
    pub total_debit_transactions_amount_60_d: Option<f64>,
    ///The total credit (inflow) transaction amount over the past 60 days from the account that will be debited
    pub total_credit_transactions_amount_60_d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 90 days from the account that will be debited
    pub total_debit_transactions_amount_90_d: Option<f64>,
    ///The total credit (inflow) transaction amount over the past 90 days from the account that will be debited
    pub total_credit_transactions_amount_90_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p_50_eod_balance_30_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p_50_eod_balance_60_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p_50_eod_balance_90_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    pub p_50_eod_balance_31_d_to_60_d: Option<f64>,
    ///The 50th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    pub p_50_eod_balance_61_d_to_90_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    pub p_90_eod_balance_31_d_to_60_d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    pub p_90_eod_balance_61_d_to_90_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance between day 31 and day 60 over the past 60 days of the account that will be debited
    pub p_10_eod_balance_31_d_to_60_d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance between day 61 and day 90 over the past 60 days of the account that will be debited
    pub p_10_eod_balance_61_d_to_90_d: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDecisionReportRequest {
    ///Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    pub client_transaction_id: String,
    ///`true` if the ACH transaction was initiated, `false` otherwise.
    pub initiated: bool,
    ///The actual number of days (hold time) since the ACH debit transaction that you wait before making funds available to your customers. The holding time could affect the ACH return rate. For example, use 0 if you make funds available to your customers instantly or the same day following the debit transaction, or 1 if you make funds available the next day following the debit initialization.
    pub days_funds_on_hold: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDecisionReportResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalReturnReportRequest {
    ///Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    pub client_transaction_id: String,
    ///Must be a valid ACH return code (e.g. "R01")
    pub return_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalReturnReportResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPrepareRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPrepareResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsRequest {
    pub oauth_state_id: String,
    pub accounts: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccountsAvailableWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`NEW_ACCOUNTS_AVAILABLE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreateRequest {
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: WalletIsoCurrencyCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreateResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGetRequest {
    ///The ID of the e-wallet
    pub wallet_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGetResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletListRequest {
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: WalletIsoCurrencyCode,
    ///A base64 value representing the latest e-wallet that has already been requested. Set this to `next_cursor` received from the previous `/wallet/list` request. If provided, the response will only contain e-wallets created before that e-wallet. If omitted, the response will contain e-wallets starting from the most recent, and in descending order.
    pub cursor: String,
    ///The number of e-wallets to fetch
    pub count: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletListResponse {
    ///An array of e-wallets
    pub wallets: Vec<Wallet>,
    ///Cursor used for fetching e-wallets created before the latest e-wallet provided in this response
    pub next_cursor: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    ///A unique ID identifying the e-wallet
    pub wallet_id: String,
    ///An object representing the e-wallet balance
    pub balance: WalletBalance,
    ///An object representing the e-wallet account numbers
    pub numbers: WalletNumbers,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletNumbers {
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if you need to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: Option<RecipientBacs>,
    ///Account numbers using the International Bank Account Number and BIC/SWIFT code format.
    pub international: Option<NumbersInternationalIban>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalance {
    ///The ISO-4217 currency code of the balance
    pub iso_currency_code: String,
    ///The total amount of funds in the account
    pub current: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WalletIsoCurrencyCode {
    Gbp,
    Eur,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionExecuteRequest {
    /**A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: WalletTransactionIdempotencyKey,
    ///The ID of the e-wallet to debit from
    pub wallet_id: String,
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    ///A reference for the transaction. This must be an alphanumeric string with at most 18 characters and must not contain any special characters or spaces.
    pub reference: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterparty {
    ///The name of the counterparty
    pub name: String,
    ///The counterparty's bank account numbers. Exactly one of IBAN or BACS data is required.
    pub numbers: WalletTransactionCounterpartyNumbers,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyNumbers {
    ///The account number and sort code of the counterparty's account
    pub bacs: WalletTransactionCounterpartyBacs,
    ///International Bank Account Number for a Wallet Transaction
    pub international: Option<WalletTransactionCounterpartyInternational>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyBacs(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyInternational {
    ///International Bank Account Number (IBAN).
    pub iban: NumbersIban,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionAmount {
    ///An ISO-4217 currency code, used with e-wallets and transactions.
    pub iso_currency_code: WalletIsoCurrencyCode,
    ///The amount of the transaction. Must contain at most two digits of precision e.g. `1.23`.
    pub value: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionExecuteResponse {
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    /**The status of the transaction.

`INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: WalletTransactionStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WalletTransactionStatus {
    Initiated,
    Executed,
    Blocked,
    Failed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionGetRequest {
    ///The ID of the transaction to fetch
    pub transaction_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionGetResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsListRequest {
    ///The ID of the e-wallet to fetch transactions from
    pub wallet_id: String,
    ///A base64 value representing the latest transaction that has already been requested. Set this to `next_cursor` received from the previous `/wallet/transactions/list` request. If provided, the response will only contain transactions created before that transaction. If omitted, the response will contain transactions starting from the most recent, and in descending order by the `created_at` time.
    pub cursor: String,
    ///The number of transactions to fetch
    pub count: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsListResponse {
    ///An array of transactions of an e-wallet, associated with the given `wallet_id`
    pub transactions: Vec<WalletTransaction>,
    ///Cursor used for fetching transactions created before the latest transaction provided in this response
    pub next_cursor: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransaction {
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    ///A reference for the transaction
    pub reference: String,
    #[serde(rename = "type")]
    /**The type of the transaction. The supported transaction types that are returned are:
`BANK_TRANSFER:` a transaction which credits an e-wallet through an external bank transfer.

`PAYOUT:` a transaction which debits an e-wallet by disbursing funds to a counterparty.

`PIS_PAY_IN:` a payment which credits an e-wallet through Plaid's Payment Initiation Services (PIS) APIs. For more information see the [Payment Initiation endpoints](https://plaid.com/docs/api/products/payment-initiation/).

`REFUND:` a transaction which debits an e-wallet by refunding a previously initated payment made through Plaid's [PIS APIs](https://plaid.com/docs/api/products/payment-initiation/).*/
    pub type_: String,
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    /**The status of the transaction.

`INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: WalletTransactionStatus,
    ///Timestamp when the transaction was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsEnhanceGetRequest {
    ///The type of account for the requested transactions (`depository` or `credit`).
    pub account_type: String,
    ///An array of raw transactions to be enhanced.
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientProvidedRawTransaction {
    ///Unique transaction identifier to tie transactions back to clients' systems.
    pub id: String,
    ///The raw description of the transaction.
    pub description: String,
    ///The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    ///The ISO-4217 currency code of the transaction.
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsEnhanceGetResponse {
    ///An array of enhanced transactions.
    pub enhanced_transactions: Vec<ClientProvidedEnhancedTransaction>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientProvidedEnhancedTransaction {
    ///Unique transaction identifier to tie transactions back to clients' systems.
    pub id: String,
    ///The raw description of the transaction.
    pub description: String,
    ///The value of the transaction, denominated in the account's currency, as stated in `iso_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    ///The ISO-4217 currency code of the transaction.
    pub iso_currency_code: String,
    ///A grouping of the Plaid produced transaction enhancement fields.
    pub enhancements: Enhancements,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentChannel {
    Online,
    InStore,
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Enhancements {
    ///The merchant name, as extracted by Plaid from the raw description.
    pub merchant_name: Option<String>,
    ///The website associated with this transaction.
    pub website: Option<String>,
    ///A link to the logo associated with this transaction. The logo will always be 100x100 resolution.
    pub logo_url: Option<String>,
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
    /**The channel used to make a payment.
`online:` transactions that took place online.

`in store:` transactions that were made at a physical location.

`other:` transactions that relate to banks, e.g. fees or deposits.*/
    pub payment_channel: PaymentChannel,
    ///The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    pub category_id: Option<String>,
    ///A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/transactions/#categoriesget).
    pub category: Vec<String>,
    ///A representation of where a transaction took place
    pub location: Location,
    /**Information describing the intent of the transaction. Most relevant for personal finance use cases, but not limited to such use cases.

See the [`taxonomy csv file`](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) for a full list of personal finance categories.*/
    pub personal_finance_category: Option<PersonalFinanceCategory>,
    ///A link to the icon associated with the primary personal finance category. The logo will always be 100x100 resolution.
    pub personal_finance_category_icon_url: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileCreateRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileCreateResponse {
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: PaymentProfileId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileGetRequest {
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: PaymentProfileId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileGetResponse {
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time the given Payment Profile was updated at
    pub updated_at: String,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the time the given Payment Profile was created at
    pub created_at: String,
    /**The status of the given Payment Profile.

`READY`: This Payment Profile is ready to be used to create transfers using `/transfer/authorization/create` and /transfer/create`.

`PENDING`: This Payment Profile is not ready to be used. You’ll need to call `/link/token/create` and provide the Payment Profile ID in the `transfer.payment_profile_id` field and go through the account linking flow to activate it.

`REMOVED`: This Payment Profile has been removed.*/
    pub status: PaymentProfileStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentProfileStatus {
    Pending,
    Ready,
    Removed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileRemoveRequest {
    ///Plaid’s unique identifier for a payment profile.
    pub payment_profile_id: PaymentProfileId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentProfileRemoveResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerCustomersCreateRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerCustomersCreateResponse {
    ///The end customer details for the newly-created customer client.
    pub end_customer: PartnerEndCustomerClient,
    pub production_secret: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartnerEndCustomerClient {
    pub company_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AddressPurposeLabel {
    Residential,
    Commercial,
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct City(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientUserId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEntityScreeningRequest {
    ///Search inputs for creating an entity watchlist screening
    pub search_terms: EntityWatchlistSearchTerms,
    pub client_user_id: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEntityWatchlistScreeningReviewRequest {
    ///Hits to mark as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<EntityWatchlistScreeningHitId>,
    ///Hits to mark as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<EntityWatchlistScreeningHitId>,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<ReviewComment>,
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: EntityWatchlistScreeningId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIndividualWatchlistScreeningReviewRequest {
    ///Hits to mark as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<WatchlistScreeningHitId>,
    ///Hits to mark as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<WatchlistScreeningHitId>,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<ReviewComment>,
    ///ID of the associated screening.
    pub watchlist_screening_id: WatchlistScreeningIndividualId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cursor(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUser {
    ///ID of the associated user.
    pub id: DashboardUserId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///A valid email address.
    pub email_address: EmailAddress,
    ///The current status of the user.
    pub status: DashboardUserStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUserId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardUserResponse {
    ///ID of the associated user.
    pub id: DashboardUserId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///A valid email address.
    pub email_address: EmailAddress,
    ///The current status of the user.
    pub status: DashboardUserStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DashboardUserStatus {
    Invited,
    Active,
    Deactivated,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Date(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRange {
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub beginning: Date,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub ending: Date,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentAnalysis {
    /**High level summary of whether the document in the provided image matches the formatting rules and security checks for the associated jurisdiction.

For example, most identity documents have formatting rules like the following:


The image of the person's face must have a certain contrast in order to highlight skin tone


The subject in the document's image must remove eye glasses and pose in a certain way


The informational fields (name, date of birth, ID number, etc.) must be colored and aligned according to specific rules


Security features like watermarks and background patterns must be present

So a `match` status for this field indicates that the document in the provided image seems to conform to the various formatting and security rules associated with the detected document.*/
    pub authenticity: DocumentAuthenticityMatchCode,
    /**A high level description of the quality of the image the user submitted.

For example, an image that is blurry, distorted by glare from a nearby light source, or improperly framed might be marked as low or medium quality. Poor quality images are more likely to fail OCR and/or template conformity checks.

Note: By default, Plaid will let a user recapture document images twice before failing the entire session if we attribute the failure to low image quality.*/
    pub image_quality: ImageQuality,
    ///Analysis of the data extracted from the submitted document.
    pub extracted_data: Option<PhysicalDocumentExtractedDataAnalysis>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentAuthenticityMatchCode {
    Match,
    PartialMatch,
    NoMatch,
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentDateOfBirthMatchCode {
    Match,
    PartialMatch,
    NoMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageBack(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageCroppedBack(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageCroppedFront(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageFace(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentImageFront(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentNameMatchCode {
    Match,
    PartialMatch,
    NoMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentStatus {
    Success,
    Failed,
    ManuallyApproved,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentaryVerification {
    ///The outcome status for the associated Identity Verification attempt's `documentary_verification` step. This field will always have the same value as `steps.documentary_verification`.
    pub status: String,
    /**An array of documents submitted to the `documentary_verification` step. Each entry represents one user submission, where each submission will contain both a front and back image, or just a front image, depending on the document type.

Note: Plaid will automatically let a user submit a new set of document images up to three times if we detect that a previous attempt might have failed due to user error. For example, if the first set of document images are blurry or obscured by glare, the user will be asked to capture their documents again, resulting in at least two separate entries within `documents`. If the overall `documentary_verification` is `failed`, the user has exhausted their retry attempts.*/
    pub documents: Vec<DocumentaryVerificationDocument>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentaryVerificationDocument {
    ///An outcome status for this specific document submission. Distinct from the overall `documentary_verification.status` that summarizes the verification outcome from one or more documents.
    pub status: DocumentStatus,
    ///The `attempt` field begins with 1 and increments with each subsequent document upload.
    pub attempt: f64,
    ///URLs for downloading original and cropped images for this document submission. The URLs are designed to only allow downloading, not hot linking, so the URL will only serve the document image for 60 seconds before expiring. The expiration time is 60 seconds after the `GET` request for the associated Identity Verification attempt. A new expiring URL is generated with each request, so you can always rerequest the Identity Verification attempt if one of your URLs expires.
    pub images: PhysicalDocumentImages,
    ///Data extracted from a user-submitted document.
    pub extracted_data: Option<PhysicalDocumentExtractedData>,
    ///High level descriptions of how the associated document was processed. If a document fails verification, the details in the `analysis` object should help clarify why the document was rejected.
    pub analysis: DocumentAnalysis,
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
    pub type_: EntityDocumentType,
    ///The numeric or alphanumeric identifier associated with this document.
    pub number: WatchlistScreeningDocumentValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityDocumentType {
    Bik,
    BusinessNumber,
    Imo,
    Other,
    Swift,
    TaxId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitAnalysis {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub documents: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub email_addresses: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub locations: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub names: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub phone_numbers: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub urls: MatchSummaryCode,
    ///The version of the entity screening's `search_terms` that were compared when the entity screening hit was added. entity screening hits are immutable once they have been reviewed. If changes are detected due to updates to the entity screening's `search_terms`, the associated entity program, or the list's source data prior to review, the entity screening hit will be updated to reflect those changes.
    pub search_terms_version: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitData {
    ///Documents associated with the watchlist hit
    pub documents: Vec<EntityScreeningHitDocumentsItems>,
    ///Email addresses associated with the watchlist hit
    pub email_addresses: Vec<EntityScreeningHitEmailsItems>,
    ///Locations associated with the watchlist hit
    pub locations: Vec<GenericScreeningHitLocationItems>,
    ///Names associated with the watchlist hit
    pub names: Vec<EntityScreeningHitNamesItems>,
    ///Phone numbers associated with the watchlist hit
    pub phone_numbers: Vec<EntityScreeningHitsPhoneNumberItems>,
    ///URLs associated with the watchlist hit
    pub urls: Vec<EntityScreeningHitUrlsItems>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitDocumentsItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///An official document, usually issued by a governing body or institution, with an associated identifier.
    pub data: EntityDocument,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitEmails {
    ///A valid email address.
    pub email_address: EmailAddress,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitEmailsItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///Email address information for the associated entity watchlist hit
    pub data: EntityScreeningHitEmails,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitNames {
    ///The full name of the entity.
    pub full: String,
    ///Primary names are those most commonly used to refer to this entity. Only one name will ever be marked as primary.
    pub is_primary: bool,
    ///Names that are explicitly marked as low quality either by their `source` list, or by `plaid` by a series of additional checks done by Plaid. Plaid does not ever surface a hit as a result of a weak name alone. If a name has no quality issues, this value will be `none`.
    pub weak_alias_determination: WeakAliasDetermination,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitNamesItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///Name information for the associated entity watchlist hit
    pub data: EntityScreeningHitNames,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitPhoneNumbers {
    #[serde(rename = "type")]
    ///An enum indicating whether a phone number is a phone line or a fax line.
    pub type_: PhoneType,
    ///A phone number in E.164 format.
    pub phone_number: WatchlistScreeningPhoneNumber,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitUrls {
    ///An 'http' or 'https' URL (must begin with either of those).
    pub url: Url,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitUrlsItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///URLs associated with the entity screening hit
    pub data: EntityScreeningHitUrls,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityScreeningHitsPhoneNumberItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///Phone number information associated with the entity screening hit
    pub data: EntityScreeningHitPhoneNumbers,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityWatchlistCode {
    CaCon,
    EuCon,
    IzSoe,
    IzUnc,
    UsCap,
    UsFse,
    UsMbs,
    UsSdn,
    UsSsi,
    UsCmc,
    UsUvl,
    AuCon,
    UkHmc,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistProgram {
    ///ID of the associated entity program.
    pub id: EntityWatchlistProgramId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<EntityWatchlistCode>,
    ///A name for the entity program to define its purpose. For example, "High Risk Organizations" or "Applicants".
    pub name: EntityWatchlistScreeningProgramName,
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: ProgramNameSensitivity,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: ProgramArchived,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistProgramId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistProgramResponse {
    ///ID of the associated entity program.
    pub id: EntityWatchlistProgramId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<EntityWatchlistCode>,
    ///A name for the entity program to define its purpose. For example, "High Risk Organizations" or "Applicants".
    pub name: EntityWatchlistScreeningProgramName,
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: ProgramNameSensitivity,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: ProgramArchived,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreening {
    ///ID of the associated entity screening.
    pub id: EntityWatchlistScreeningId,
    ///Search terms associated with an entity used for searching against watchlists
    pub search_terms: EntityWatchlistScreeningSearchTerms,
    pub assignee: Option<serde_json::Value>,
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: WatchlistScreeningStatus,
    pub client_user_id: Option<serde_json::Value>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningHit {
    ///ID of the associated entity screening hit.
    pub id: EntityWatchlistScreeningHitId,
    ///The current state of review. All watchlist screening hits begin in a `pending_review` state but can be changed by creating a review. When a hit is in the `pending_review` state, it will always show the latest version of the watchlist data Plaid has available and be compared against the latest customer information saved in the watchlist screening. Once a hit has been marked as `confirmed` or `dismissed` it will no longer be updated so that the state is as it was when the review was first conducted.
    pub review_status: WatchlistScreeningHitStatus,
    ///An ISO8601 formatted timestamp.
    pub first_active: Timestamp,
    ///An ISO8601 formatted timestamp.
    pub inactive_since: Option<TimestampNullable>,
    ///An ISO8601 formatted timestamp.
    pub historical_since: Option<TimestampNullable>,
    ///Shorthand identifier for a specific screening list for entities.
    pub list_code: EntityWatchlistCode,
    ///A universal identifier for a watchlist individual that is stable across searches and updates.
    pub plaid_uid: InternalUid,
    ///The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    pub source_uid: Option<SourceUid>,
    ///Analysis information describing why a screening hit matched the provided entity information
    pub analysis: EntityScreeningHitAnalysis,
    ///Information associated with the entity watchlist hit
    pub data: EntityScreeningHitData,
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
    ///ID of the associated entity screening.
    pub id: EntityWatchlistScreeningId,
    ///Search terms associated with an entity used for searching against watchlists
    pub search_terms: EntityWatchlistScreeningSearchTerms,
    pub assignee: Option<serde_json::Value>,
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: WatchlistScreeningStatus,
    pub client_user_id: Option<serde_json::Value>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReview {
    ///ID of the associated entity review.
    pub id: EntityWatchlistScreeningReviewId,
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<EntityWatchlistScreeningHitId>,
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<EntityWatchlistScreeningHitId>,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<ReviewComment>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReviewId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningReviewResponse {
    ///ID of the associated entity review.
    pub id: EntityWatchlistScreeningReviewId,
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<EntityWatchlistScreeningHitId>,
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<EntityWatchlistScreeningHitId>,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<ReviewComment>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistScreeningSearchTerms {
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: EntityWatchlistProgramId,
    ///The name of the organization being screened.
    pub legal_name: EntityWatchlistScreeningName,
    pub document_number: Option<serde_json::Value>,
    pub email_address: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
    pub phone_number: Option<serde_json::Value>,
    pub url: Option<serde_json::Value>,
    ///The current version of the search terms. Starts at `1` and increments with each edit to `search_terms`.
    pub version: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityWatchlistSearchTerms {
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: EntityWatchlistProgramId,
    ///The name of the organization being screened.
    pub legal_name: EntityWatchlistScreeningName,
    pub document_number: Option<serde_json::Value>,
    pub email_address: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
    pub phone_number: Option<serde_json::Value>,
    pub url: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ExpirationDate {
    NotExpired,
    Expired,
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyNameField(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericCountryCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericScreeningHitLocationItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///Location information for the associated individual watchlist hit
    pub data: WatchlistScreeningHitLocations,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetDashboardUserRequest {
    ///ID of the associated user.
    pub dashboard_user_id: DashboardUserId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetEntityWatchlistScreeningRequest {
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: EntityWatchlistScreeningId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetIdentityVerificationRequest {
    ///ID of the associated Identity Verification attempt.
    pub identity_verification_id: IdentityVerificationId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetIndividualWatchlistScreeningRequest {
    ///ID of the associated screening.
    pub watchlist_screening_id: WatchlistScreeningIndividualId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWatchlistScreeningEntityProgramRequest {
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: EntityWatchlistProgramId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetWatchlistScreeningIndividualProgramRequest {
    ///ID of the associated program.
    pub watchlist_program_id: WatchlistProgramId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GivenNameField(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum IdNumberType {
    ArDni,
    AuDriversLicense,
    AuPassport,
    BrCpf,
    CaSin,
    ClRun,
    CnResidentCard,
    CoNit,
    DkCpr,
    EgNationalId,
    EsDni,
    EsNie,
    HkHkid,
    InPan,
    ItCf,
    JoCivilId,
    JpMyNumber,
    KeHudumaNamba,
    KwCivilId,
    MxCurp,
    MxRfc,
    MyNric,
    NgNin,
    NzDriversLicense,
    OmCivilId,
    PhPsn,
    PlPesel,
    RoCnp,
    SaNationalId,
    SePin,
    SgNric,
    TrTcKimlik,
    UsSsn,
    UsSsnLast4,
    ZaSmartId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdNumberValue(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IpAddress(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Iso8601Date(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdempotencyFlag(pub Option<bool>);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerification {
    ///ID of the associated Identity Verification attempt.
    pub id: IdentityVerificationId,
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: ClientUserId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///An ISO8601 formatted timestamp.
    pub completed_at: Option<TimestampNullable>,
    ///The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt.
    pub previous_attempt_id: Option<PreviousIdentityVerificationAttemptId>,
    ///A shareable URL that can be sent directly to the user to complete verification
    pub shareable_url: Option<ShareableUrl>,
    ///The resource ID and version number of the template configuring the behavior of a given identity verification.
    pub template: IdentityVerificationTemplateReference,
    ///The identity data that was either collected from the user or provided via API in order to perform an identity verification.
    pub user: IdentityVerificationUserData,
    /**The status of this Identity Verification attempt.


`active` - The Identity Verification attempt is incomplete. The user may have completed part of the session, but has neither failed or passed.

`success` - The Identity Verification attempt has completed, passing all steps defined to the associated Identity Verification template

`failed` - The user failed one or more steps in the session and was told to contact support.

`expired` - The Identity Verification attempt was active for more than 48 hours without being completed and was automatically marked as expired.

`canceled` - The Identity Verification attempt was canceled, either via the dashboard by a user, or via API. The user may have completed part of the session, but has neither failed or passed.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.*/
    pub status: IdentityVerificationStatus,
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
    ///data, images, analysis, and results from the `documentary_verification` step.
    pub documentary_verification: Option<DocumentaryVerification>,
    ///The outcome of the `kyc_check` step.
    pub kyc_check: Option<KycCheckDetails>,
    pub watchlist_screening_id: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationConsent(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationCreateRequest {
    ///A flag specifying whether you would like Plaid to expose a shareable URL for the verification being created.
    pub is_shareable: bool,
    ///ID of the associated Identity Verification template.
    pub template_id: IdentityVerificationTemplateId,
    /**A flag specifying whether the end user has already agreed to a privacy policy specifying that their data will be shared with Plaid for verification purposes.

If `gave_consent` is set to `true`, the `accept_tos` step will be marked as `skipped` and the end user's session will start at the next step requirement.*/
    pub gave_consent: IdentityVerificationConsent,
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
    /**An optional flag specifying how you would like Plaid to handle attempts to create an Identity Verification when an Identity Verification already exists for the provided `client_user_id` and `template_id`.
If idempotency is enabled, Plaid will return the existing Identity Verification. If idempotency is disabled, Plaid will reject the request with a `400 Bad Request` status code if an Identity Verification already exists for the supplied `client_user_id` and `template_id`.*/
    pub is_idempotent: Option<IdempotencyFlag>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRequestUser {
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: ClientUserId,
    pub email_address: Option<serde_json::Value>,
    ///A phone number in E.164 format.
    pub phone_number: Option<IdentityVerificationUserPhoneNumber>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub date_of_birth: Option<Iso8601Date>,
    ///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    pub name: Option<UserName>,
    ///Home address for the user.
    pub address: Option<UserAddress>,
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub id_number: Option<UserIdNumber>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationResponse {
    ///ID of the associated Identity Verification attempt.
    pub id: IdentityVerificationId,
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: ClientUserId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///An ISO8601 formatted timestamp.
    pub completed_at: Option<TimestampNullable>,
    ///The ID for the Identity Verification preceding this session. This field will only be filled if the current Identity Verification is a retry of a previous attempt.
    pub previous_attempt_id: Option<PreviousIdentityVerificationAttemptId>,
    ///A shareable URL that can be sent directly to the user to complete verification
    pub shareable_url: Option<ShareableUrl>,
    ///The resource ID and version number of the template configuring the behavior of a given identity verification.
    pub template: IdentityVerificationTemplateReference,
    ///The identity data that was either collected from the user or provided via API in order to perform an identity verification.
    pub user: IdentityVerificationUserData,
    /**The status of this Identity Verification attempt.


`active` - The Identity Verification attempt is incomplete. The user may have completed part of the session, but has neither failed or passed.

`success` - The Identity Verification attempt has completed, passing all steps defined to the associated Identity Verification template

`failed` - The user failed one or more steps in the session and was told to contact support.

`expired` - The Identity Verification attempt was active for more than 48 hours without being completed and was automatically marked as expired.

`canceled` - The Identity Verification attempt was canceled, either via the dashboard by a user, or via API. The user may have completed part of the session, but has neither failed or passed.

`pending_review` - The Identity Verification attempt template was configured to perform a screening that had one or more hits needing review.*/
    pub status: IdentityVerificationStatus,
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
    ///data, images, analysis, and results from the `documentary_verification` step.
    pub documentary_verification: Option<DocumentaryVerification>,
    ///The outcome of the `kyc_check` step.
    pub kyc_check: Option<KycCheckDetails>,
    pub watchlist_screening_id: Option<serde_json::Value>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRetryRequest {
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: ClientUserId,
    ///ID of the associated Identity Verification template.
    pub template_id: IdentityVerificationTemplateId,
    /**An instruction specifying what steps the new Identity Verification attempt should require the user to complete:


`reset` - Restart the user at the beginning of the session, regardless of whether they successfully completed part of their previous session.

`incomplete` - Start the new session at the step that the user failed in the previous session, skipping steps that have already been successfully completed.

`infer` - If the most recent Identity Verification attempt associated with the given `client_user_id` has a status of `failed` or `expired`, retry using the `incomplete` strategy. Otherwise, use the `reset` strategy.

`custom` - Start the new session with a custom configuration, specified by the value of the `steps` field

Note:

The `incomplete` strategy cannot be applied if the session's failing step is `screening` or `risk_check`.

The `infer` strategy cannot be applied if the session's status is still `active`*/
    pub strategy: Strategy,
    /**Instructions for the `custom` retry strategy specifying which steps should be required or skipped.


Note:


This field must be provided when the retry strategy is `custom` and must be omitted otherwise.

Custom retries override settings in your Plaid Template. For example, if your Plaid Template has `verify_sms` disabled, a custom retry with `verify_sms` enabled will still require the step.

The `selfie_check` step is currently not supported on the sandbox server. Sandbox requests will silently disable the `selfie_check` step when provided.*/
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationRetryRequestStepsObject {
    ///A boolean field specifying whether the new session should require or skip the `verify_sms` step.
    pub verify_sms: bool,
    ///A boolean field specifying whether the new session should require or skip the `kyc_check` step.
    pub kyc_check: bool,
    ///A boolean field specifying whether the new session should require or skip the `documentary_verification` step.
    pub documentary_verification: bool,
    ///A boolean field specifying whether the new session should require or skip the `selfie_check` step.
    pub selfie_check: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityVerificationStatus {
    Active,
    Success,
    Failed,
    Expired,
    Canceled,
    PendingReview,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IdentityVerificationStepStatus {
    Success,
    Active,
    Failed,
    WaitingForPrerequisite,
    NotApplicable,
    Skipped,
    Expired,
    Canceled,
    PendingReview,
    ManuallyApproved,
    ManuallyRejected,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationStepSummary {
    ///The status of a step in the identity verification process.
    pub accept_tos: IdentityVerificationStepStatus,
    ///The status of a step in the identity verification process.
    pub verify_sms: IdentityVerificationStepStatus,
    ///The status of a step in the identity verification process.
    pub kyc_check: IdentityVerificationStepStatus,
    ///The status of a step in the identity verification process.
    pub documentary_verification: IdentityVerificationStepStatus,
    ///The status of a step in the identity verification process.
    pub selfie_check: IdentityVerificationStepStatus,
    ///The status of a step in the identity verification process.
    pub watchlist_screening: IdentityVerificationStepStatus,
    ///The status of a step in the identity verification process.
    pub risk_check: IdentityVerificationStepStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateReference {
    ///ID of the associated Identity Verification template.
    pub id: IdentityVerificationTemplateId,
    ///Version of the associated Identity Verification template.
    pub version: IdentityVerificationTemplateVersion,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationTemplateVersion(pub f64);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationUserAddress {
    pub street: Option<serde_json::Value>,
    ///Extra street information, like an apartment or suite number.
    pub street_2: Option<Street2>,
    pub city: Option<serde_json::Value>,
    pub region: Option<serde_json::Value>,
    pub postal_code: Option<serde_json::Value>,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: GenericCountryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationUserData {
    ///A phone number in E.164 format.
    pub phone_number: Option<IdentityVerificationUserPhoneNumber>,
    ///A date in the format YYYY-MM-DD (RFC 3339 Section 5.6).
    pub date_of_birth: Option<Iso8601Date>,
    ///An IPv4 or IPV6 address.
    pub ip_address: Option<IpAddress>,
    pub email_address: Option<serde_json::Value>,
    ///The full name provided by the user. If the user has not submitted their name, this field will be null. Otherwise, both fields are guaranteed to be filled.
    pub name: Option<UserName>,
    ///Even if an address has been collected, some fields may be null depending on the region's addressing system. For example: * Addresses from the United Kingdom will not include a region * Addresses from Hong Kong will not include postal code
    pub address: Option<IdentityVerificationUserAddress>,
    ///ID number submitted by the user, currently used only for the Identity Verification product. If the user has not submitted this data yet, this field will be `null`. Otherwise, both fields are guaranteed to be filled.
    pub id_number: Option<UserIdNumber>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationUserPhoneNumber(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub enum ImageQuality {
    High,
    Medium,
    Low,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualScreeningHitNames {
    ///The full name of the individual, including all parts.
    pub full: String,
    ///Primary names are those most commonly used to refer to this person. Only one name will ever be marked as primary.
    pub is_primary: bool,
    ///Names that are explicitly marked as low quality either by their `source` list, or by `plaid` by a series of additional checks done by Plaid. Plaid does not ever surface a hit as a result of a weak name alone. If a name has no quality issues, this value will be `none`.
    pub weak_alias_determination: WeakAliasDetermination,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum IndividualWatchlistCode {
    AuCon,
    CaCon,
    EuCon,
    IzCia,
    IzIpl,
    IzPep,
    IzUnc,
    UkHmc,
    UsDpl,
    UsDtc,
    UsFbi,
    UsFse,
    UsIsn,
    UsMbc,
    UsPlc,
    UsSdn,
    UsSsi,
    SgSof,
    TrTwl,
    TrDfd,
    TrFor,
    TrWmd,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualWatchlistProgram {
    ///ID of the associated program.
    pub id: WatchlistProgramId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<IndividualWatchlistCode>,
    ///A name for the program to define its purpose. For example, "High Risk Individuals", "US Cardholders", or "Applicants".
    pub name: IndividualWatchlistScreeningProgramName,
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: ProgramNameSensitivity,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: ProgramArchived,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualWatchlistProgramResponse {
    ///ID of the associated program.
    pub id: WatchlistProgramId,
    ///An ISO8601 formatted timestamp.
    pub created_at: Timestamp,
    ///Indicator specifying whether the program is enabled and will perform daily rescans.
    pub is_rescanning_enabled: bool,
    ///Watchlists enabled for the associated program
    pub lists_enabled: Vec<IndividualWatchlistCode>,
    ///A name for the program to define its purpose. For example, "High Risk Individuals", "US Cardholders", or "Applicants".
    pub name: IndividualWatchlistScreeningProgramName,
    /**The valid name matching sensitivity configurations for a screening program. Note that while certain matching techniques may be more prevalent on less strict settings, all matching algorithms are enabled for every sensitivity.

`coarse` - See more potential matches. This sensitivity will see more broad phonetic matches across alphabets that make missing a potential hit very unlikely. This setting is noisier and will require more manual review.

`balanced` - A good default for most companies. This sensitivity is balanced to show high quality hits with reduced noise.

`strict` - Aggressive false positive reduction. This sensitivity will require names to be more similar than `coarse` and `balanced` settings, relying less on phonetics, while still accounting for character transpositions, missing tokens, and other common permutations.

`exact` - Matches must be nearly exact. This sensitivity will only show hits with exact or nearly exact name matches with only basic correction such as extraneous symbols and capitalization. This setting is generally not recommended unless you have a very specific use case.*/
    pub name_sensitivity: ProgramNameSensitivity,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///Archived programs are read-only and cannot screen new customers nor participate in ongoing monitoring.
    pub is_archived: ProgramArchived,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IndividualWatchlistScreeningProgramName(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalUid(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum IssuingCountry {
    Match,
    NoMatch,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckAddressSummary {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: MatchSummaryCode,
    ///Field describing whether the associated address is a post office box. Will be `yes` when a P.O. box is detected, `no` when Plaid confirmed the address is not a P.O. box, and `no_data` when Plaid was not able to determine if the address is a P.O. box.
    pub po_box: PoBoxStatus,
    #[serde(rename = "type")]
    /**Field describing whether the associated address is being used for commercial or residential purposes.

Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.*/
    pub type_: AddressPurposeLabel,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckDateOfBirthSummary {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: MatchSummaryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckDetails {
    ///The outcome status for the associated Identity Verification attempt's `kyc_check` step. This field will always have the same value as `steps.kyc_check`.
    pub status: String,
    ///Result summary object specifying how the `address` field matched.
    pub address: KycCheckAddressSummary,
    ///Result summary object specifying how the `name` field matched.
    pub name: KycCheckNameSummary,
    ///Result summary object specifying how the `date_of_birth` field matched.
    pub date_of_birth: KycCheckDateOfBirthSummary,
    ///Result summary object specifying how the `id_number` field matched.
    pub id_number: KycCheckIdNumberSummary,
    ///Result summary object specifying how the `phone` field matched.
    pub phone_number: KycCheckPhoneSummary,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckIdNumberSummary {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: MatchSummaryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckNameSummary {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: MatchSummaryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KycCheckPhoneSummary {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: MatchSummaryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDashboardUserRequest {
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListEntityWatchlistScreeningRequest {
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: EntityWatchlistProgramId,
    pub client_user_id: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListIdentityVerificationRequest {
    ///ID of the associated Identity Verification template.
    pub template_id: IdentityVerificationTemplateId,
    ///An identifier to help you connect this object to your internal systems. For example, your database ID corresponding to this object.
    pub client_user_id: ClientUserId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListIndividualWatchlistScreeningRequest {
    ///ID of the associated program.
    pub watchlist_program_id: WatchlistProgramId,
    pub client_user_id: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityHistoryRequest {
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: EntityWatchlistScreeningId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityHitRequest {
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: EntityWatchlistScreeningId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityProgramsRequest {
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningEntityReviewsRequest {
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: EntityWatchlistScreeningId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualHistoryRequest {
    ///ID of the associated screening.
    pub watchlist_screening_id: WatchlistScreeningIndividualId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualHitRequest {
    ///ID of the associated screening.
    pub watchlist_screening_id: WatchlistScreeningIndividualId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualProgramsRequest {
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListWatchlistScreeningIndividualReviewsRequest {
    ///ID of the associated screening.
    pub watchlist_screening_id: WatchlistScreeningIndividualId,
    ///An identifier that determines which page of results you receive.
    pub cursor: Option<Cursor>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MatchSummary {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub summary: MatchSummaryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum MatchSummaryCode {
    Match,
    PartialMatch,
    NoMatch,
    NoData,
    NoInput,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PoBoxStatus {
    Yes,
    No,
    NoData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedDashboardUserListResponse {
    ///List of dashboard users
    pub dashboard_users: Vec<DashboardUser>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistProgramListResponse {
    ///List of entity watchlist screening programs
    pub entity_watchlist_programs: Vec<EntityWatchlistProgram>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistScreeningHitListResponse {
    ///List of entity watchlist screening hits
    pub entity_watchlist_screening_hits: Vec<EntityWatchlistScreeningHit>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistScreeningListResponse {
    ///List of entity watchlist screening
    pub entity_watchlist_screenings: Vec<EntityWatchlistScreening>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedEntityWatchlistScreeningReviewListResponse {
    ///List of entity watchlist screening reviews
    pub entity_watchlist_screening_reviews: Vec<EntityWatchlistScreeningReview>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIdentityVerificationListResponse {
    ///List of Plaid sessions
    pub identity_verifications: Vec<IdentityVerification>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistProgramListResponse {
    ///List of individual watchlist screening programs
    pub watchlist_programs: Vec<IndividualWatchlistProgram>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistScreeningHitListResponse {
    ///List of individual watchlist screening hits
    pub watchlist_screening_hits: Vec<WatchlistScreeningHit>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistScreeningListResponse {
    ///List of individual watchlist screenings
    pub watchlist_screenings: Vec<WatchlistScreeningIndividual>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedIndividualWatchlistScreeningReviewListResponse {
    ///List of screening reviews
    pub watchlist_screening_reviews: Vec<WatchlistScreeningReview>,
    ///An identifier that determines which page of results you receive.
    pub next_cursor: Option<Cursor>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PhoneType {
    Phone,
    Fax,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PhysicalDocumentCategory {
    DriversLicense,
    IdCard,
    Passport,
    ResidencePermitCard,
    ResidentCard,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentExtractedData {
    ///Alpha-numeric ID number extracted via OCR from the user's document image.
    pub id_number: Option<PhysicalDocumentIdNumber>,
    /**The type of identity document detected in the images provided. Will always be one of the following values:

  `drivers_license` - A driver's license for the associated country

  `id_card` - A general national identification card, distinct from driver's licenses

  `passport` - A passport for the associated country

  `residence_permit_card` - An identity document permitting a foreign citizen to <em>temporarily</em> reside in the associated country

  `resident_card` - An identity document permitting a foreign citizen to <em>permanently</em> reside in the associated country

Note: This value may be different from the ID type that the user selects within Link. For example, if they select "Driver's License" but then submit a picture of a passport, this field will say `passport`*/
    pub category: PhysicalDocumentCategory,
    pub expiration_date: Option<serde_json::Value>,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub issuing_country: GenericCountryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentExtractedDataAnalysis {
    ///A match summary describing the cross comparison between the subject's name, extracted from the document image, and the name they separately provided to identity verification attempt.
    pub name: DocumentNameMatchCode,
    ///A match summary describing the cross comparison between the subject's date of birth, extracted from the document image, and the date of birth they separately provided to the identity verification attempt.
    pub date_of_birth: DocumentDateOfBirthMatchCode,
    /**A description of whether the associated document was expired when the verification was performed.

Note: In the case where an expiration date is not present on the document or failed to be extracted, this value will be `no_data`.*/
    pub expiration_date: ExpirationDate,
    /**A binary match indicator specifying whether the country that issued the provided document matches the country that the user separately provided to Plaid.

Note: You can configure whether a `no_match` on `issuing_country` fails the `documentary_verification` by editing your Plaid Template.*/
    pub issuing_country: IssuingCountry,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentIdNumber(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PhysicalDocumentImages {
    ///Temporary URL that expires after 60 seconds for downloading the uncropped original image of the front of the document.
    pub original_front: DocumentImageFront,
    ///Temporary URL that expires after 60 seconds for downloading the original image of the back of the document. Might be null if the back of the document was not collected.
    pub original_back: Option<DocumentImageBack>,
    ///Temporary URL that expires after 60 seconds for downloading a cropped image containing just the front of the document.
    pub cropped_front: Option<DocumentImageCroppedFront>,
    ///Temporary URL that expires after 60 seconds for downloading a cropped image containing just the back of the document. Might be null if the back of the document was not collected.
    pub cropped_back: Option<DocumentImageCroppedBack>,
    ///Temporary URL that expires after 60 seconds for downloading a crop of just the user's face from the document image. Might be null if the document does not contain a face photo.
    pub face: Option<DocumentImageFace>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostalCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousIdentityVerificationAttemptId(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramArchived(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub enum ProgramNameSensitivity {
    Coarse,
    Balanced,
    Strict,
    Exact,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Region(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewComment(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitAnalysis {
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub dates_of_birth: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub documents: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub locations: MatchSummaryCode,
    /**An enum indicating the match type between data provided by user and data checked against an external data source.


`match` indicates that the provided input data was a strong match against external data.

`partial_match` indicates the data approximately matched against external data. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to perform a check against an external data source and it did not match the provided input data.

`no_data` indicates that Plaid was unable to find external data to compare against the provided input data.

`no_input` indicates that Plaid was unable to perform a check because no information was provided for this field by the end user.*/
    pub names: MatchSummaryCode,
    ///The version of the screening's `search_terms` that were compared when the screening hit was added. screening hits are immutable once they have been reviewed. If changes are detected due to updates to the screening's `search_terms`, the associated program, or the list's source data prior to review, the screening hit will be updated to reflect those changes.
    pub search_terms_version: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitData {
    ///Dates of birth associated with the watchlist hit
    pub dates_of_birth: Vec<ScreeningHitDateOfBirthItem>,
    ///Documents associated with the watchlist hit
    pub documents: Vec<ScreeningHitDocumentsItems>,
    ///Locations associated with the watchlist hit
    pub locations: Vec<GenericScreeningHitLocationItems>,
    ///Names associated with the watchlist hit
    pub names: Vec<ScreeningHitNamesItems>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitDateOfBirthItem {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///A date range with a start and end date
    pub data: DateRange,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitDocumentsItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///An official document, usually issued by a governing body or institution, with an associated identifier.
    pub data: WatchlistScreeningDocument,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScreeningHitNamesItems {
    ///Summary object reflecting the match result of the associated data
    pub analysis: MatchSummary,
    ///Name information for the associated individual watchlist hit
    pub data: IndividualScreeningHitNames,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ShareableUrl(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    Dashboard,
    Link,
    Api,
    System,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SourceUid(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub enum Strategy {
    Reset,
    Incomplete,
    Infer,
    Custom,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Street(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Street2(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Timestamp(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TimestampNullable(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Url(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityScreeningRequest {
    ///ID of the associated entity screening.
    pub entity_watchlist_screening_id: EntityWatchlistScreeningId,
    ///Search terms for editing an entity watchlist screening
    pub search_terms: Option<UpdateEntityScreeningRequestSearchTerms>,
    pub assignee: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub client_user_id: Option<serde_json::Value>,
    ///A list of fields to reset back to null
    pub reset_fields: Option<UpdateEntityScreeningRequestResettableFieldList>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateEntityScreeningRequestResettableField {
    Assignee,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityScreeningRequestResettableFieldList(
    pub Option<Vec<UpdateEntityScreeningRequestResettableField>>,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntityScreeningRequestSearchTerms {
    ///ID of the associated entity program.
    pub entity_watchlist_program_id: EntityWatchlistProgramId,
    pub legal_name: Option<serde_json::Value>,
    pub document_number: Option<serde_json::Value>,
    pub email_address: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
    pub phone_number: Option<serde_json::Value>,
    pub url: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequest {
    ///ID of the associated screening.
    pub watchlist_screening_id: WatchlistScreeningIndividualId,
    ///Search terms for editing an individual watchlist screening
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    pub assignee: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub client_user_id: Option<serde_json::Value>,
    ///A list of fields to reset back to null
    pub reset_fields: Option<UpdateIndividualScreeningRequestResettableFieldList>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateIndividualScreeningRequestResettableField {
    Assignee,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequestResettableFieldList(
    pub Option<Vec<UpdateIndividualScreeningRequestResettableField>>,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateIndividualScreeningRequestSearchTerms {
    pub watchlist_program_id: Option<serde_json::Value>,
    pub legal_name: Option<serde_json::Value>,
    pub date_of_birth: Option<serde_json::Value>,
    pub document_number: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddress {
    ///The primary street portion of an address. If the user has submitted their address, this field will always be filled.
    pub street: Street,
    ///Extra street information, like an apartment or suite number.
    pub street_2: Option<Street2>,
    ///City from the end user's address
    pub city: City,
    ///An ISO 3166-2 subdivision code. Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.
    pub region: Region,
    ///The postal code for the associated address. Between 2 and 10 alphanumeric characters.
    pub postal_code: PostalCode,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: GenericCountryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserIdNumber {
    ///Value of identity document value typed in by user. Alpha-numeric, with all formatting characters stripped.
    pub value: IdNumberValue,
    #[serde(rename = "type")]
    ///A globally unique and human readable ID type, specific to the country and document category. For more context on this field, see [Hybrid Input Validation](https://cognitohq.com/docs/flow/flow-hybrid-input-validation)
    pub type_: IdNumberType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserName {
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub given_name: GivenNameField,
    ///A string with at least one non-whitespace character, with a max length of 100 characters.
    pub family_name: FamilyNameField,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistProgramId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningAuditTrail {
    ///A type indicating whether a dashboard user, an API-based user, or Plaid last touched this object.
    pub source: Source,
    pub dashboard_user_id: Option<serde_json::Value>,
    ///An ISO8601 formatted timestamp.
    pub timestamp: Timestamp,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningCreateRequest {
    ///Search inputs for creating a watchlist screening
    pub search_terms: WatchlistScreeningRequestSearchTerms,
    pub client_user_id: Option<serde_json::Value>,
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
    pub type_: WatchlistScreeningDocumentType,
    ///The numeric or alphanumeric identifier associated with this document.
    pub number: WatchlistScreeningDocumentValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningDocumentType {
    BirthCertificate,
    DriversLicense,
    ImmigrationNumber,
    MilitaryId,
    Other,
    Passport,
    PersonalIdentification,
    RationCard,
    Ssn,
    StudentId,
    TaxId,
    TravelDocument,
    VoterId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningDocumentValue(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningHit {
    ///ID of the associated screening hit.
    pub id: WatchlistScreeningHitId,
    ///The current state of review. All watchlist screening hits begin in a `pending_review` state but can be changed by creating a review. When a hit is in the `pending_review` state, it will always show the latest version of the watchlist data Plaid has available and be compared against the latest customer information saved in the watchlist screening. Once a hit has been marked as `confirmed` or `dismissed` it will no longer be updated so that the state is as it was when the review was first conducted.
    pub review_status: WatchlistScreeningHitStatus,
    ///An ISO8601 formatted timestamp.
    pub first_active: Timestamp,
    ///An ISO8601 formatted timestamp.
    pub inactive_since: Option<TimestampNullable>,
    ///An ISO8601 formatted timestamp.
    pub historical_since: Option<TimestampNullable>,
    ///Shorthand identifier for a specific screening list for individuals.
    pub list_code: IndividualWatchlistCode,
    ///A universal identifier for a watchlist individual that is stable across searches and updates.
    pub plaid_uid: InternalUid,
    ///The identifier provided by the source sanction or watchlist. When one is not provided by the source, this is `null`.
    pub source_uid: Option<SourceUid>,
    ///Analysis information describing why a screening hit matched the provided user information
    pub analysis: ScreeningHitAnalysis,
    ///Information associated with the watchlist hit
    pub data: ScreeningHitData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningHitId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningHitLocations {
    ///The full location string, potentially including elements like street, city, postal codes and country codes. Note that this is not necessarily a complete or well-formatted address.
    pub full: String,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: GenericCountryCode,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningHitStatus {
    Confirmed,
    PendingReview,
    Dismissed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividual {
    ///ID of the associated screening.
    pub id: WatchlistScreeningIndividualId,
    ///Search terms for creating an individual watchlist screening
    pub search_terms: WatchlistScreeningSearchTerms,
    pub assignee: Option<serde_json::Value>,
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: WatchlistScreeningStatus,
    pub client_user_id: Option<serde_json::Value>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualName(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualResponse {
    ///ID of the associated screening.
    pub id: WatchlistScreeningIndividualId,
    ///Search terms for creating an individual watchlist screening
    pub search_terms: WatchlistScreeningSearchTerms,
    pub assignee: Option<serde_json::Value>,
    ///A status enum indicating whether a screening is still pending review, has been rejected, or has been cleared.
    pub status: WatchlistScreeningStatus,
    pub client_user_id: Option<serde_json::Value>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningPhoneNumber(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningRequestSearchTerms {
    ///ID of the associated program.
    pub watchlist_program_id: WatchlistProgramId,
    ///The legal name of the individual being screened.
    pub legal_name: WatchlistScreeningIndividualName,
    pub date_of_birth: Option<serde_json::Value>,
    pub document_number: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningReview {
    ///ID of the associated review.
    pub id: WatchlistScreeningReviewId,
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<WatchlistScreeningHitId>,
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<WatchlistScreeningHitId>,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<ReviewComment>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningReviewId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningReviewResponse {
    ///ID of the associated review.
    pub id: WatchlistScreeningReviewId,
    ///Hits marked as a true positive after thorough manual review. These hits will never recur or be updated once dismissed. In most cases, confirmed hits indicate that the customer should be rejected.
    pub confirmed_hits: Vec<WatchlistScreeningHitId>,
    ///Hits marked as a false positive after thorough manual review. These hits will never recur or be updated once dismissed.
    pub dismissed_hits: Vec<WatchlistScreeningHitId>,
    ///A comment submitted by a team member as part of reviewing a watchlist screening.
    pub comment: Option<ReviewComment>,
    ///Information about the last change made to the parent object specifying what caused the change as well as when it occurred.
    pub audit_trail: WatchlistScreeningAuditTrail,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchlistScreeningSearchTerms {
    ///ID of the associated program.
    pub watchlist_program_id: WatchlistProgramId,
    ///The legal name of the individual being screened.
    pub legal_name: WatchlistScreeningIndividualName,
    pub date_of_birth: Option<serde_json::Value>,
    pub document_number: Option<serde_json::Value>,
    pub country: Option<serde_json::Value>,
    ///The current version of the search terms. Starts at `1` and increments with each edit to `search_terms`.
    pub version: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WatchlistScreeningStatus {
    Rejected,
    PendingReview,
    Cleared,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum WeakAliasDetermination {
    None,
    Source,
    Plaid,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///Information about the last successful and failed transactions update for the Item.
    pub status: Option<ItemStatusNullable>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRemoveRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRemoveResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWebhookUpdateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The new webhook URL to associate with the Item. To remove a webhook from an Item, set to `null`.
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWebhookUpdateResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateResponse {
    ///The access token associated with the Item data is being requested for.
    pub new_access_token: AccessToken,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeRequest {
    ///Your `public_token`, obtained from the Link `onSuccess` callback or `/sandbox/item/public_token/create`.
    pub public_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeResponse {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `item_id` value of the Item associated with the returned `access_token`
    pub item_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateResponse {
    ///A `public_token` for the particular Item corresponding to the specified `access_token`
    pub public_token: String,
    pub expiration: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequest {
    ///Array of product strings
    pub products: Vec<Products>,
    ///Object of user ID and auth token pair, permitting Plaid to aggregate a user’s accounts
    pub user_auth: ItemImportRequestUserAuth,
    ///An optional object to configure `/item/import` request.
    pub options: ItemImportRequestOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequestOptions {
    /**Specifies a webhook URL to associate with an Item. Plaid fires a webhook if credentials fail.
*/
    pub webhook: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequestUserAuth {
    ///Opaque user identifier
    pub user_id: String,
    ///Authorization token Plaid will use to aggregate this user’s accounts
    pub auth_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportResponse {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    ///The Plaid Institution ID associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    pub institution_id: Option<String>,
    ///The URL registered to receive webhooks for the Item.
    pub webhook: Option<String>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A list of products available for the Item that have not yet been accessed. The contents of this array will be mutually exclusive with `billed_products`.
    pub available_products: Vec<Products>,
    /**A list of products that have been billed for the Item. The contents of this array will be mutually exclusive with `available_products`. Note - `billed_products` is populated in all environments but only requests in Production are billed. Also note that products that are billed on a pay-per-call basis rather than a pay-per-Item basis, such as `balance`, will not appear here.
*/
    pub billed_products: Vec<Products>,
    /**A list of authorized products for the Item.
*/
    pub products: Vec<Products>,
    /**Beta: A list of products that have gone through consent collection for the Item. Only present for those enabled in the beta.
*/
    pub consented_products: Vec<Products>,
    /**The RFC 3339 timestamp after which the consent provided by the end user will expire. Upon consent expiration, the item will enter the `ITEM_LOGIN_REQUIRED` error state. To circumvent the `ITEM_LOGIN_REQUIRED` error and maintain continuous consent, the end user can reauthenticate via Link’s update mode in advance of the consent expiration time.

Note - This is only relevant for certain OAuth-based institutions. For all other institutions, this field will be null.
*/
    pub consent_expiration_time: Option<String>,
    /**Indicates whether an Item requires user interaction to be updated, which can be the case for Items with some forms of two-factor authentication.

`background` - Item can be updated in the background

`user_present_required` - Item requires user interaction to be updated*/
    pub update_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatus {
    ///Information about the last successful and failed investments update for the Item.
    pub investments: Option<ItemStatusInvestments>,
    ///Information about the last successful and failed transactions update for the Item.
    pub transactions: Option<ItemStatusTransactions>,
    ///Information about the last webhook fired for the Item.
    pub last_webhook: Option<ItemStatusLastWebhook>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusTransactions {
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful transactions update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<String>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed transactions update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusInvestments {
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful investments update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<String>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed investments update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusLastWebhook {
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of when the webhook was fired.
*/
    pub sent_at: Option<String>,
    ///The last webhook code sent.
    pub code_sent: Option<String>,
}
