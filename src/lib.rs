//! The [`PlaidClient`] is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]

use serde_json::json;
use httpclient::RequestBuilder;
pub mod model;
pub mod request_model;
use crate::model::*;

impl PlaidClient {
    pub fn from_env() -> PlaidClient {
        let url = format!(
            "https://{}.plaid.com",
            std::env::var("PLAID_ENV")
                .expect("Environment variable PLAID_ENV is not set."),
        );
        PlaidClient::new(&url)
            .with_authentication(PlaidAuthentication::from_env())
    }
}
pub struct PlaidClient {
    pub(crate) client: httpclient::Client,
    authentication: Option<PlaidAuthentication>,
}
impl PlaidClient {}
impl PlaidClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        let authentication = None;
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: PlaidAuthentication) -> Self {
        self.authentication = Some(authentication);
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        if let Some(ref auth) = self.authentication {
            match auth {
                PlaidAuthentication::ClientId { client_id, secret, .. } => {
                    r = r
                        .push_json(
                            json!(
                                { "client_id" : client_id, "secret" : secret
                                }
                            ),
                        );
                }
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    ///List a user’s connected applications
    pub fn item_application_list(
        &self,
        access_token: Option<String>,
    ) -> request_model::ItemApplicationListRequest {
        request_model::ItemApplicationListRequest {
            client: &self,
            access_token,
        }
    }
    /**Update the scopes of access for a particular application

Enable consumers to update product access on selected accounts for an application.*/
    pub fn item_application_scopes_update(
        &self,
        access_token: String,
        application_id: String,
        scopes: serde_json::Value,
        state: String,
        context: String,
    ) -> request_model::ItemApplicationScopesUpdateRequest {
        request_model::ItemApplicationScopesUpdateRequest {
            client: &self,
            access_token,
            application_id,
            scopes,
            state,
            context,
        }
    }
    /**Retrieve information about a Plaid application

Allows financial institutions to retrieve information about Plaid clients for the purpose of building control-tower experiences*/
    pub fn application_get(
        &self,
        application_id: String,
    ) -> request_model::ApplicationGetRequest {
        request_model::ApplicationGetRequest {
            client: &self,
            application_id,
        }
    }
    /**Retrieve an Item

Returns information about the status of an Item.

See endpoint docs at <https://plaid.com/docs/api/items/#itemget>.*/
    pub fn item_get(&self, access_token: String) -> request_model::ItemGetRequest {
        request_model::ItemGetRequest {
            client: &self,
            access_token,
        }
    }
    /**Retrieve auth data

The `/auth/get` endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.

Note: This request may take some time to complete if `auth` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

Also note that `/auth/get` will not return data for any new accounts opened after the Item was created. To obtain data for new accounts, create a new Item.

Versioning note: In API version 2017-03-08, the schema of the `numbers` object returned by this endpoint is substantially different. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2018-05-22).

See endpoint docs at <https://plaid.com/docs/api/products/#authget>.*/
    pub fn auth_get(
        &self,
        access_token: String,
        options: serde_json::Value,
    ) -> request_model::AuthGetRequest {
        request_model::AuthGetRequest {
            client: &self,
            access_token,
            options,
        }
    }
    /**Get transaction data

The `/transactions/get` endpoint allows developers to receive user-authorized transaction data for credit, depository, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from investments accounts, use the [Investments endpoint](https://plaid.com/docs/api/products#investments) instead. Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.  Transactions are not immutable and can also be removed altogether by the institution; a removed transaction will no longer appear in `/transactions/get`.  For more details, see [Pending and posted transactions](https://plaid.com/docs/transactions/transactions-data/#pending-and-posted-transactions).

Due to the potentially large number of transactions associated with an Item, results are paginated. Manipulate the `count` and `offset` parameters in conjunction with the `total_transactions` response body field to fetch all available transactions.

Data returned by `/transactions/get` will be the data available for the Item as of the most recent successful check for new transactions. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. An Item's `status.transactions.last_successful_update` field will show the timestamp of the most recent successful update. To force Plaid to check for new transactions, you can use the `/transactions/refresh` endpoint.

Note that data may not be immediately available to `/transactions/get`. Plaid will begin to prepare transactions data upon Item link, if Link was initialized with `transactions`, or upon the first call to `/transactions/get`, if it wasn't. To be alerted when transaction data is ready to be fetched, listen for the [`INITIAL_UPDATE`](https://plaid.com/docs/api/webhooks#transactions-initial_update) and [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/webhooks#transactions-historical_update) webhooks. If no transaction history is ready when `/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

See endpoint docs at <https://plaid.com/docs/api/products/#transactionsget>.*/
    pub fn transactions_get(
        &self,
        options: serde_json::Value,
        access_token: String,
        start_date: String,
        end_date: String,
    ) -> request_model::TransactionsGetRequest {
        request_model::TransactionsGetRequest {
            client: &self,
            options,
            access_token,
            start_date,
            end_date,
        }
    }
    /**Refresh transaction data

`/transactions/refresh` is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled Item. If changes to transactions are discovered after calling `/transactions/refresh`, Plaid will fire a webhook: [`TRANSACTIONS_REMOVED`](https://plaid.com/docs/api/webhooks#deleted-transactions-detected) will be fired if any removed transactions are detected, and [`DEFAULT_UPDATE`](https://plaid.com/docs/api/webhooks#transactions-default_update) will be fired if any new transactions are detected. New transactions can be fetched by calling `/transactions/get`.

Access to `/transactions/refresh` in Production is specific to certain pricing plans. If you cannot access `/transactions/refresh` in Production, [contact Sales](https://www.plaid.com/contact) for assistance.

See endpoint docs at <https://plaid.com/docs/api/products/#transactionsrefresh>.*/
    pub fn transactions_refresh(
        &self,
        access_token: String,
    ) -> request_model::TransactionsRefreshRequest {
        request_model::TransactionsRefreshRequest {
            client: &self,
            access_token,
        }
    }
    /**Get streams of recurring transactions

The `/transactions/recurring/get` endpoint identifies and returns groups of transactions that occur on a regular basis for the inputted Item and accounts.

The product is currently in beta. To request access, contact transactions-feedback@plaid.com.*/
    pub fn transactions_recurring_get(
        &self,
        access_token: String,
        account_ids: Vec<String>,
    ) -> request_model::TransactionsRecurringGetRequest {
        request_model::TransactionsRecurringGetRequest {
            client: &self,
            access_token,
            account_ids,
        }
    }
    /**Get incremental transaction updates on an Item

The `/transactions/sync` endpoint returns item transactions as a set of delta updates.
Subsequent calls to the endpoint using the cursor returned in the response will return new added, modified, and removed transactions since the last call to the endpoint

The product is currently in beta. To request access, contact transactions-feedback@plaid.com.

See endpoint docs at <https://plaid.com/docs/api/products/#transactionssync>.*/
    pub fn transactions_sync(
        &self,
        access_token: String,
        cursor: String,
        count: i64,
    ) -> request_model::TransactionsSyncRequest {
        request_model::TransactionsSyncRequest {
            client: &self,
            access_token,
            cursor,
            count,
        }
    }
    /**Get details of all supported institutions

Returns a JSON response containing details on all financial institutions currently supported by Plaid. Because Plaid supports thousands of institutions, results are paginated.

If there is no overlap between an institution’s enabled products and a client’s enabled products, then the institution will be filtered out from the response. As a result, the number of institutions returned may not match the count specified in the call.

See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget>.*/
    pub fn institutions_get(
        &self,
        count: i64,
        offset: i64,
        country_codes: Vec<CountryCode>,
        options: serde_json::Value,
    ) -> request_model::InstitutionsGetRequest {
        request_model::InstitutionsGetRequest {
            client: &self,
            count,
            offset,
            country_codes,
            options,
        }
    }
    /**Search institutions

Returns a JSON response containing details for institutions that match the query parameters, up to a maximum of ten institutions per query.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` parameters to authenticate to this endpoint. The `public_key` parameter has since been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionssearch>.*/
    pub fn institutions_search(
        &self,
        query: String,
        products: Option<Vec<Products>>,
        country_codes: Vec<CountryCode>,
        options: serde_json::Value,
    ) -> request_model::InstitutionsSearchRequest {
        request_model::InstitutionsSearchRequest {
            client: &self,
            query,
            products,
            country_codes,
            options,
        }
    }
    /**Get details of an institution

Returns a JSON response containing details on a specified financial institution currently supported by Plaid.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` to authenticate to this endpoint. The `public_key` has been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget_by_id>.*/
    pub fn institutions_get_by_id(
        &self,
        institution_id: String,
        country_codes: Vec<CountryCode>,
        options: serde_json::Value,
    ) -> request_model::InstitutionsGetByIdRequest {
        request_model::InstitutionsGetByIdRequest {
            client: &self,
            institution_id,
            country_codes,
            options,
        }
    }
    /**Remove an Item

The `/item/remove`  endpoint allows you to remove an Item. Once removed, the `access_token`  associated with the Item is no longer valid and cannot be used to access any data that was associated with the Item.

Note that in the Development environment, issuing an `/item/remove`  request will not decrement your live credential count. To increase your credential account in Development, contact Support.

Also note that for certain OAuth-based institutions, an Item removed via `/item/remove` may still show as an active connection in the institution's OAuth permission manager.

API versions 2019-05-29 and earlier return a `removed` boolean as part of the response.

See endpoint docs at <https://plaid.com/docs/api/items/#itemremove>.*/
    pub fn item_remove(&self, access_token: String) -> request_model::ItemRemoveRequest {
        request_model::ItemRemoveRequest {
            client: &self,
            access_token,
        }
    }
    /**Retrieve accounts

The `/accounts/get` endpoint can be used to retrieve a list of accounts associated with any linked Item. Plaid will only return active bank accounts — that is, accounts that are not closed and are capable of carrying a balance.

This endpoint only returns accounts that were permissioned by the user when they initially created the Item. If a user creates a new account after the initial link, you can capture this event through the [`NEW_ACCOUNTS_AVAILABLE`](https://plaid.com/docs/api/webhooks/#item-new_accounts_available) webhook and then use Link's [update mode](https://plaid.com/docs/link/update-mode/) to request that the user share this new account with you.

This endpoint retrieves cached information, rather than extracting fresh information from the institution. As a result, balances returned may not be up-to-date; for realtime balance information, use `/accounts/balance/get` instead. Note that some information is nullable.

See endpoint docs at <https://plaid.com/docs/api/accounts/#accountsget>.*/
    pub fn accounts_get(
        &self,
        access_token: String,
        options: serde_json::Value,
    ) -> request_model::AccountsGetRequest {
        request_model::AccountsGetRequest {
            client: &self,
            access_token,
            options,
        }
    }
    /**Get Categories

Send a request to the `/categories/get` endpoint to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

See endpoint docs at <https://plaid.com/docs/api/products/#categoriesget>.*/
    pub fn categories_get(&self) -> request_model::CategoriesGetRequest {
        request_model::CategoriesGetRequest {
            client: &self,
        }
    }
    /**Create a test Item and processor token

Use the `/sandbox/processor_token/create` endpoint to create a valid `processor_token` for an arbitrary institution ID and test credentials. The created `processor_token` corresponds to a new Sandbox Item. You can then use this `processor_token` with the `/processor/` API endpoints in Sandbox. You can also use `/sandbox/processor_token/create` with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxprocessor_tokencreate>.*/
    pub fn sandbox_processor_token_create(
        &self,
        institution_id: String,
        options: serde_json::Value,
    ) -> request_model::SandboxProcessorTokenCreateRequest {
        request_model::SandboxProcessorTokenCreateRequest {
            client: &self,
            institution_id,
            options,
        }
    }
    /**Create a test Item

Use the `/sandbox/public_token/create` endpoint to create a valid `public_token`  for an arbitrary institution ID, initial products, and test credentials. The created `public_token` maps to a new Sandbox Item. You can then call `/item/public_token/exchange` to exchange the `public_token` for an `access_token` and perform all API actions. `/sandbox/public_token/create` can also be used with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data. `/sandbox/public_token/create` cannot be used with OAuth institutions.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate>.*/
    pub fn sandbox_public_token_create(
        &self,
        institution_id: String,
        initial_products: Vec<Products>,
        options: serde_json::Value,
    ) -> request_model::SandboxPublicTokenCreateRequest {
        request_model::SandboxPublicTokenCreateRequest {
            client: &self,
            institution_id,
            initial_products,
            options,
        }
    }
    /**Fire a test webhook

The `/sandbox/item/fire_webhook` endpoint is used to test that code correctly handles webhooks. This endpoint can trigger a Transactions `DEFAULT_UPDATE` webhook to be fired for a given Sandbox Item. If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result. This endpoint can also trigger a `NEW_ACCOUNTS_AVAILABLE` webhook to be fired for a given Sandbox Item created with Account Select v2. Note that this endpoint is provided for developer ease-of-use and is not required for testing webhooks; webhooks will also fire in Sandbox under the same conditions that they would in Production or Development.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemfire_webhook>.*/
    pub fn sandbox_item_fire_webhook(
        &self,
        access_token: String,
        webhook_code: String,
    ) -> request_model::SandboxItemFireWebhookRequest {
        request_model::SandboxItemFireWebhookRequest {
            client: &self,
            access_token,
            webhook_code,
        }
    }
    /**Retrieve real-time balance data

The `/accounts/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/accounts/balance/get` forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, `balance` itself is not a product that can be used to initialize Link.

See endpoint docs at <https://plaid.com/docs/api/products/#accountsbalanceget>.*/
    pub fn accounts_balance_get(
        &self,
        access_token: String,
        options: serde_json::Value,
    ) -> request_model::AccountsBalanceGetRequest {
        request_model::AccountsBalanceGetRequest {
            client: &self,
            access_token,
            options,
        }
    }
    /**Retrieve identity data

The `/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses. Only name data is guaranteed to be returned; other fields will be empty arrays if not provided by the institution.

Note: This request may take some time to complete if identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

See endpoint docs at <https://plaid.com/docs/api/products/#identityget>.*/
    pub fn identity_get(
        &self,
        access_token: String,
        options: serde_json::Value,
    ) -> request_model::IdentityGetRequest {
        request_model::IdentityGetRequest {
            client: &self,
            access_token,
            options,
        }
    }
    /**Retrieve Auth data

The `/processor/auth/get` endpoint returns the bank account and bank identification number (such as the routing number, for US accounts), for a checking or savings account that''s associated with a given `processor_token`. The endpoint also returns high-level account data and balances when available.

Versioning note: API versions 2019-05-29 and earlier use a different schema for the `numbers` object returned by this endpoint. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2020-09-14).


See endpoint docs at <https://plaid.com/docs/api/processors/#processorauthget>.*/
    pub fn processor_auth_get(
        &self,
        processor_token: String,
    ) -> request_model::ProcessorAuthGetRequest {
        request_model::ProcessorAuthGetRequest {
            client: &self,
            processor_token,
        }
    }
    /**Create a bank transfer as a processor

Use the `/processor/bank_transfer/create` endpoint to initiate a new bank transfer as a processor

See endpoint docs at <https://plaid.com/docs/api/processors/#bank_transfercreate>.*/
    pub fn processor_bank_transfer_create(
        &self,
        idempotency_key: String,
        processor_token: String,
        type_: String,
        network: String,
        amount: String,
        iso_currency_code: String,
        description: String,
        ach_class: String,
        user: serde_json::Value,
        custom_tag: Option<String>,
        metadata: Option<serde_json::Value>,
        origination_account_id: Option<String>,
    ) -> request_model::ProcessorBankTransferCreateRequest {
        request_model::ProcessorBankTransferCreateRequest {
            client: &self,
            idempotency_key,
            processor_token,
            type_,
            network,
            amount,
            iso_currency_code,
            description,
            ach_class,
            user,
            custom_tag,
            metadata,
            origination_account_id,
        }
    }
    /**Retrieve Identity data

The `/processor/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses.

See endpoint docs at <https://plaid.com/docs/api/processors/#processoridentityget>.*/
    pub fn processor_identity_get(
        &self,
        processor_token: String,
    ) -> request_model::ProcessorIdentityGetRequest {
        request_model::ProcessorIdentityGetRequest {
            client: &self,
            processor_token,
        }
    }
    /**Retrieve Balance data

The `/processor/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/processor/balance/get` forces the available and current balance fields to be refreshed rather than cached.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorbalanceget>.*/
    pub fn processor_balance_get(
        &self,
        processor_token: String,
        options: serde_json::Value,
    ) -> request_model::ProcessorBalanceGetRequest {
        request_model::ProcessorBalanceGetRequest {
            client: &self,
            processor_token,
            options,
        }
    }
    /**Update Webhook URL

The POST `/item/webhook/update` allows you to update the webhook URL associated with an Item. This request triggers a [`WEBHOOK_UPDATE_ACKNOWLEDGED`](https://plaid.com/docs/api/webhooks/#item-webhook-update-acknowledged) webhook to the newly specified webhook URL.

See endpoint docs at <https://plaid.com/docs/api/items/#itemwebhookupdate>.*/
    pub fn item_webhook_update(
        &self,
        access_token: String,
        webhook: Option<String>,
    ) -> request_model::ItemWebhookUpdateRequest {
        request_model::ItemWebhookUpdateRequest {
            client: &self,
            access_token,
            webhook,
        }
    }
    /**Invalidate access_token

By default, the `access_token` associated with an Item does not expire and should be stored in a persistent, secure manner.

You can use the `/item/access_token/invalidate` endpoint to rotate the `access_token` associated with an Item. The endpoint returns a new `access_token` and immediately invalidates the previous `access_token`.


See endpoint docs at <https://plaid.com/docs/api/tokens/#itemaccess_tokeninvalidate>.*/
    pub fn item_access_token_invalidate(
        &self,
        access_token: String,
    ) -> request_model::ItemAccessTokenInvalidateRequest {
        request_model::ItemAccessTokenInvalidateRequest {
            client: &self,
            access_token,
        }
    }
    /**Get webhook verification key

Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the `Plaid-Verification` header.

The `/webhook_verification_key/get` endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.

See endpoint docs at <https://plaid.com/docs/api/webhooks/webhook-verification/#webhook_verification_keyget>.*/
    pub fn webhook_verification_key_get(
        &self,
        key_id: String,
    ) -> request_model::WebhookVerificationKeyGetRequest {
        request_model::WebhookVerificationKeyGetRequest {
            client: &self,
            key_id,
        }
    }
    /**Retrieve Liabilities data

The `/liabilities/get` endpoint returns various details about an Item with loan or credit accounts. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`. To limit accounts listed in Link to types and subtypes supported by Liabilities, you can use the `account_filters` parameter when [creating a Link token](https://plaid.com/docs/api/tokens/#linktokencreate).

The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/liabilities/get`.

Note: This request may take some time to complete if `liabilities` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the additional data.

See endpoint docs at <https://plaid.com/docs/api/products/#liabilitiesget>.*/
    pub fn liabilities_get(
        &self,
        access_token: String,
        options: serde_json::Value,
    ) -> request_model::LiabilitiesGetRequest {
        request_model::LiabilitiesGetRequest {
            client: &self,
            access_token,
            options,
        }
    }
    /**Create payment recipient

Create a payment recipient for payment initiation.  The recipient must be in Europe, within a country that is a member of the Single Euro Payment Area (SEPA).  For a standing order (recurring) payment, the recipient must be in the UK.

The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same `recipient_id`.


See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationrecipientcreate>.*/
    pub fn payment_initiation_recipient_create(
        &self,
        name: String,
        iban: Option<String>,
        bacs: Option<serde_json::Value>,
        address: Option<serde_json::Value>,
    ) -> request_model::PaymentInitiationRecipientCreateRequest {
        request_model::PaymentInitiationRecipientCreateRequest {
            client: &self,
            name,
            iban,
            bacs,
            address,
        }
    }
    /**Reverse an existing payment

Reverse a previously initiated payment.

A payment can only be reversed once and will be refunded to the original sender's account.


See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationpaymentreverse>.*/
    pub fn payment_initiation_payment_reverse(
        &self,
        payment_id: String,
    ) -> request_model::PaymentInitiationPaymentReverseRequest {
        request_model::PaymentInitiationPaymentReverseRequest {
            client: &self,
            payment_id,
        }
    }
    /**Get payment recipient

Get details about a payment recipient you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationrecipientget>.*/
    pub fn payment_initiation_recipient_get(
        &self,
        recipient_id: String,
    ) -> request_model::PaymentInitiationRecipientGetRequest {
        request_model::PaymentInitiationRecipientGetRequest {
            client: &self,
            recipient_id,
        }
    }
    /**List payment recipients

The `/payment_initiation/recipient/list` endpoint list the payment recipients that you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationrecipientlist>.*/
    pub fn payment_initiation_recipient_list(
        &self,
    ) -> request_model::PaymentInitiationRecipientListRequest {
        request_model::PaymentInitiationRecipientListRequest {
            client: &self,
        }
    }
    /**Create a payment

After creating a payment recipient, you can use the `/payment_initiation/payment/create` endpoint to create a payment to that recipient.  Payments can be one-time or standing order (recurring) and can be denominated in either EUR or GBP.  If making domestic GBP-denominated payments, your recipient must have been created with BACS numbers. In general, EUR-denominated payments will be sent via SEPA Credit Transfer and GBP-denominated payments will be sent via the Faster Payments network, but the payment network used will be determined by the institution. Payments sent via Faster Payments will typically arrive immediately, while payments sent via SEPA Credit Transfer will typically arrive in one business day.

Standing orders (recurring payments) must be denominated in GBP and can only be sent to recipients in the UK. Once created, standing order payments cannot be modified or canceled via the API. An end user can cancel or modify a standing order directly on their banking application or website, or by contacting the bank. Standing orders will follow the payment rules of the underlying rails (Faster Payments in UK). Payments can be sent Monday to Friday, excluding bank holidays. If the pre-arranged date falls on a weekend or bank holiday, the payment is made on the next working day. It is not possible to guarantee the exact time the payment will reach the recipient’s account, although at least 90% of standing order payments are sent by 6am.

In the Development environment, payments must be below 5 GBP / EUR. For details on any payment limits in Production, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationpaymentcreate>.*/
    pub fn payment_initiation_payment_create(
        &self,
        recipient_id: String,
        reference: String,
        amount: serde_json::Value,
        schedule: serde_json::Value,
        options: Option<serde_json::Value>,
    ) -> request_model::PaymentInitiationPaymentCreateRequest {
        request_model::PaymentInitiationPaymentCreateRequest {
            client: &self,
            recipient_id,
            reference,
            amount,
            schedule,
            options,
        }
    }
    /**Create payment token

The `/payment_initiation/payment/token/create` endpoint has been deprecated. New Plaid customers will be unable to use this endpoint, and existing customers are encouraged to migrate to the newer, `link_token`-based flow. The recommended flow is to provide the `payment_id` to `/link/token/create`, which returns a `link_token` used to initialize Link.

The `/payment_initiation/payment/token/create` is used to create a `payment_token`, which can then be used in Link initialization to enter a payment initiation flow. You can only use a `payment_token` once. If this attempt fails, the end user aborts the flow, or the token expires, you will need to create a new payment token. Creating a new payment token does not require end user input.

See endpoint docs at <https://plaid.com/docs/link/maintain-legacy-integration/#creating-a-payment-token>.*/
    pub fn create_payment_token(
        &self,
        payment_id: String,
    ) -> request_model::CreatePaymentTokenRequest {
        request_model::CreatePaymentTokenRequest {
            client: &self,
            payment_id,
        }
    }
    /**Force a Sandbox Item into an error state

`/sandbox/item/reset_login/` forces an Item into an `ITEM_LOGIN_REQUIRED` state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's [update mode](https://plaid.com/docs/link/update-mode) flow in the Sandbox environment.  After calling `/sandbox/item/reset_login`, You can then use Plaid Link update mode to restore the Item to a good state. An `ITEM_LOGIN_REQUIRED` webhook will also be fired after a call to this endpoint, if one is associated with the Item.


In the Sandbox, Items will transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days, even if this endpoint is not called.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemreset_login>.*/
    pub fn sandbox_item_reset_login(
        &self,
        access_token: String,
    ) -> request_model::SandboxItemResetLoginRequest {
        request_model::SandboxItemResetLoginRequest {
            client: &self,
            access_token,
        }
    }
    /**Set verification status for Sandbox account

The `/sandbox/item/set_verification_status` endpoint can be used to change the verification status of an Item in in the Sandbox in order to simulate the Automated Micro-deposit flow.

Note that not all Plaid developer accounts are enabled for micro-deposit based verification by default. Your account must be enabled for this feature in order to test it in Sandbox. To enable this features or check your status, contact your account manager or [submit a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access).

For more information on testing Automated Micro-deposits in Sandbox, see [Auth full coverage testing](https://plaid.com/docs/auth/coverage/testing#).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemset_verification_status>.*/
    pub fn sandbox_item_set_verification_status(
        &self,
        access_token: String,
        account_id: String,
        verification_status: String,
    ) -> request_model::SandboxItemSetVerificationStatusRequest {
        request_model::SandboxItemSetVerificationStatusRequest {
            client: &self,
            access_token,
            account_id,
            verification_status,
        }
    }
    /**Exchange public token for an access token

Exchange a Link `public_token` for an API `access_token`. Link hands off the `public_token` client-side via the `onSuccess` callback once a user has successfully created an Item. The `public_token` is ephemeral and expires after 30 minutes.

The response also includes an `item_id` that should be stored with the `access_token`. The `item_id` is used to identify an Item in a webhook. The `item_id` can also be retrieved by making an `/item/get` request.

See endpoint docs at <https://plaid.com/docs/api/tokens/#itempublic_tokenexchange>.*/
    pub fn item_public_token_exchange(
        &self,
        public_token: String,
    ) -> request_model::ItemPublicTokenExchangeRequest {
        request_model::ItemPublicTokenExchangeRequest {
            client: &self,
            public_token,
        }
    }
    /**Create public token

Note: As of July 2020, the `/item/public_token/create` endpoint is deprecated. Instead, use `/link/token/create` with an `access_token` to create a Link token for use with [update mode](https://plaid.com/docs/link/update-mode).

If you need your user to take action to restore or resolve an error associated with an Item, generate a public token with the `/item/public_token/create` endpoint and then initialize Link with that `public_token`.

A `public_token` is one-time use and expires after 30 minutes. You use a `public_token` to initialize Link in [update mode](https://plaid.com/docs/link/update-mode) for a particular Item. You can generate a `public_token` for an Item even if you did not use Link to create the Item originally.

The `/item/public_token/create` endpoint is **not** used to create your initial `public_token`. If you have not already received an `access_token` for a specific Item, use Link to obtain your `public_token` instead. See the [Quickstart](https://plaid.com/docs/quickstart) for more information.

See endpoint docs at <https://plaid.com/docs/api/tokens/#itempublic_tokencreate>.*/
    pub fn item_create_public_token(
        &self,
        access_token: String,
    ) -> request_model::ItemCreatePublicTokenRequest {
        request_model::ItemCreatePublicTokenRequest {
            client: &self,
            access_token,
        }
    }
    /**Get payment details

The `/payment_initiation/payment/get` endpoint can be used to check the status of a payment, as well as to receive basic information such as recipient and payment amount. In the case of standing orders, the `/payment_initiation/payment/get` endpoint will provide information about the status of the overall standing order itself; the API cannot be used to retrieve payment status for individual payments within a standing order.

See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationpaymentget>.*/
    pub fn payment_initiation_payment_get(
        &self,
        payment_id: String,
    ) -> request_model::PaymentInitiationPaymentGetRequest {
        request_model::PaymentInitiationPaymentGetRequest {
            client: &self,
            payment_id,
        }
    }
    /**List payments

The `/payment_initiation/payment/list` endpoint can be used to retrieve all created payments. By default, the 10 most recent payments are returned. You can request more payments and paginate through the results using the optional `count` and `cursor` parameters.

See endpoint docs at <https://plaid.com/docs/api/products/#payment_initiationpaymentlist>.*/
    pub fn payment_initiation_payment_list(
        &self,
        count: Option<i64>,
        cursor: Option<String>,
    ) -> request_model::PaymentInitiationPaymentListRequest {
        request_model::PaymentInitiationPaymentListRequest {
            client: &self,
            count,
            cursor,
        }
    }
    /**Create an Asset Report

The `/asset_report/create` endpoint initiates the process of creating an Asset Report, which can then be retrieved by passing the `asset_report_token` return value to the `/asset_report/get` or `/asset_report/pdf/get` endpoints.

The Asset Report takes some time to be created and is not available immediately after calling `/asset_report/create`. When the Asset Report is ready to be retrieved using `/asset_report/get` or `/asset_report/pdf/get`, Plaid will fire a `PRODUCT_READY` webhook. For full details of the webhook schema, see [Asset Report webhooks](https://plaid.com/docs/api/webhooks/#assets-webhooks).

The `/asset_report/create` endpoint creates an Asset Report at a moment in time. Asset Reports are immutable. To get an updated Asset Report, use the `/asset_report/refresh` endpoint.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportcreate>.*/
    pub fn asset_report_create(
        &self,
        access_tokens: Vec<AccessToken>,
        days_requested: i64,
        options: serde_json::Value,
    ) -> request_model::AssetReportCreateRequest {
        request_model::AssetReportCreateRequest {
            client: &self,
            access_tokens,
            days_requested,
            options,
        }
    }
    /**Refresh an Asset Report

An Asset Report is an immutable snapshot of a user's assets. In order to "refresh" an Asset Report you created previously, you can use the `/asset_report/refresh` endpoint to create a new Asset Report based on the old one, but with the most recent data available.

The new Asset Report will contain the same Items as the original Report, as well as the same filters applied by any call to `/asset_report/filter`. By default, the new Asset Report will also use the same parameters you submitted with your original `/asset_report/create` request, but the original `days_requested` value and the values of any parameters in the `options` object can be overridden with new values. To change these arguments, simply supply new values for them in your request to `/asset_report/refresh`. Submit an empty string ("") for any previously-populated fields you would like set as empty.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportrefresh>.*/
    pub fn asset_report_refresh(
        &self,
        asset_report_token: String,
        days_requested: Option<i64>,
        options: serde_json::Value,
    ) -> request_model::AssetReportRefreshRequest {
        request_model::AssetReportRefreshRequest {
            client: &self,
            asset_report_token,
            days_requested,
            options,
        }
    }
    /**Delete an Asset Report

The `/item/remove` endpoint allows you to invalidate an `access_token`, meaning you will not be able to create new Asset Reports with it. Removing an Item does not affect any Asset Reports or Audit Copies you have already created, which will remain accessible until you remove them specifically.

The `/asset_report/remove` endpoint allows you to remove an Asset Report. Removing an Asset Report invalidates its `asset_report_token`, meaning you will no longer be able to use it to access Report data or create new Audit Copies. Removing an Asset Report does not affect the underlying Items, but does invalidate any `audit_copy_tokens` associated with the Asset Report.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportremove>.*/
    pub fn asset_report_remove(
        &self,
        asset_report_token: String,
    ) -> request_model::AssetReportRemoveRequest {
        request_model::AssetReportRemoveRequest {
            client: &self,
            asset_report_token,
        }
    }
    /**Filter Asset Report

By default, an Asset Report will contain all of the accounts on a given Item. In some cases, you may not want the Asset Report to contain all accounts. For example, you might have the end user choose which accounts are relevant in Link using the Account Select view, which you can enable in the dashboard. Or, you might always exclude certain account types or subtypes, which you can identify by using the `/accounts/get` endpoint. To narrow an Asset Report to only a subset of accounts, use the `/asset_report/filter` endpoint.

To exclude certain Accounts from an Asset Report, first use the `/asset_report/create` endpoint to create the report, then send the `asset_report_token` along with a list of `account_ids` to exclude to the `/asset_report/filter` endpoint, to create a new Asset Report which contains only a subset of the original Asset Report's data.

Because Asset Reports are immutable, calling `/asset_report/filter` does not alter the original Asset Report in any way; rather, `/asset_report/filter` creates a new Asset Report with a new token and id. Asset Reports created via `/asset_report/filter` do not contain new Asset data, and are not billed.

Plaid will fire a [`PRODUCT_READY`](https://plaid.com/docs/api/webhooks) webhook once generation of the filtered Asset Report has completed.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportfilter>.*/
    pub fn asset_report_filter(
        &self,
        asset_report_token: String,
        account_ids_to_exclude: Vec<String>,
    ) -> request_model::AssetReportFilterRequest {
        request_model::AssetReportFilterRequest {
            client: &self,
            asset_report_token,
            account_ids_to_exclude,
        }
    }
    /**Retrieve an Asset Report

The `/asset_report/get` endpoint retrieves the Asset Report in JSON format. Before calling `/asset_report/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/webhooks) webhook to fire, indicating that the Report is ready to be retrieved.

By default, an Asset Report includes transaction descriptions as returned by the bank, as opposed to parsed and categorized by Plaid. You can also receive cleaned and categorized transactions, as well as additional insights like merchant name or location information. We call this an Asset Report with Insights. An Asset Report with Insights provides transaction category, location, and merchant information in addition to the transaction strings provided in a standard Asset Report.

To retrieve an Asset Report with Insights, call the `/asset_report/get` endpoint with `include_insights` set to `true`.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportget>.*/
    pub fn asset_report_get(
        &self,
        asset_report_token: String,
        include_insights: bool,
    ) -> request_model::AssetReportGetRequest {
        request_model::AssetReportGetRequest {
            client: &self,
            asset_report_token,
            include_insights,
        }
    }
    /**Create Asset Report Audit Copy

Plaid can provide an Audit Copy of any Asset Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy contains the same underlying data as the Asset Report.

To grant access to an Audit Copy, use the `/asset_report/audit_copy/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportaudit_copycreate>.*/
    pub fn asset_report_audit_copy_create(
        &self,
        asset_report_token: String,
        auditor_id: String,
    ) -> request_model::AssetReportAuditCopyCreateRequest {
        request_model::AssetReportAuditCopyCreateRequest {
            client: &self,
            asset_report_token,
            auditor_id,
        }
    }
    /**Remove Asset Report Audit Copy

The `/asset_report/audit_copy/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Asset Report, the Asset Report itself and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportaudit_copyremove>.*/
    pub fn asset_report_audit_copy_remove(
        &self,
        audit_copy_token: String,
    ) -> request_model::AssetReportAuditCopyRemoveRequest {
        request_model::AssetReportAuditCopyRemoveRequest {
            client: &self,
            audit_copy_token,
        }
    }
    /**Get Investment holdings

The `/investments/holdings/get` endpoint allows developers to receive user-authorized stock position data for `investment`-type accounts.

See endpoint docs at <https://plaid.com/docs/api/products/#investmentsholdingsget>.*/
    pub fn investments_holdings_get(
        &self,
        access_token: String,
        options: serde_json::Value,
    ) -> request_model::InvestmentsHoldingsGetRequest {
        request_model::InvestmentsHoldingsGetRequest {
            client: &self,
            access_token,
            options,
        }
    }
    /**Get investment transactions

The `/investments/transactions/get` endpoint allows developers to retrieve user-authorized transaction data for investment accounts.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.

Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the `total_investment_transactions` response body field to fetch all available investment transactions.

See endpoint docs at <https://plaid.com/docs/api/products/#investmentstransactionsget>.*/
    pub fn investments_transactions_get(
        &self,
        access_token: String,
        start_date: String,
        end_date: String,
        options: serde_json::Value,
    ) -> request_model::InvestmentsTransactionsGetRequest {
        request_model::InvestmentsTransactionsGetRequest {
            client: &self,
            access_token,
            start_date,
            end_date,
            options,
        }
    }
    /**Create processor token

Used to create a token suitable for sending to one of Plaid's partners to enable integrations. Note that Stripe partnerships use bank account tokens instead; see `/processor/stripe/bank_account_token/create` for creating tokens for use with Stripe integrations.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokencreate>.*/
    pub fn processor_token_create(
        &self,
        access_token: String,
        account_id: String,
        processor: String,
    ) -> request_model::ProcessorTokenCreateRequest {
        request_model::ProcessorTokenCreateRequest {
            client: &self,
            access_token,
            account_id,
            processor,
        }
    }
    /**Create Stripe bank account token

Used to create a token suitable for sending to Stripe to enable Plaid-Stripe integrations. For a detailed guide on integrating Stripe, see [Add Stripe to your app](https://plaid.com/docs/auth/partnerships/stripe/).

See endpoint docs at <https://plaid.com/docs/api/processors/#processorstripebank_account_tokencreate>.*/
    pub fn processor_stripe_bank_account_token_create(
        &self,
        access_token: String,
        account_id: String,
    ) -> request_model::ProcessorStripeBankAccountTokenCreateRequest {
        request_model::ProcessorStripeBankAccountTokenCreateRequest {
            client: &self,
            access_token,
            account_id,
        }
    }
    /**Create Apex bank account token

Used to create a token suitable for sending to Apex to enable Plaid-Apex integrations.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn processor_apex_processor_token_create(
        &self,
        access_token: String,
        account_id: String,
    ) -> request_model::ProcessorApexProcessorTokenCreateRequest {
        request_model::ProcessorApexProcessorTokenCreateRequest {
            client: &self,
            access_token,
            account_id,
        }
    }
    /**Create a deposit switch

This endpoint creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchcreate>.*/
    pub fn deposit_switch_create(
        &self,
        target_access_token: String,
        target_account_id: String,
        country_code: Option<String>,
        options: serde_json::Value,
    ) -> request_model::DepositSwitchCreateRequest {
        request_model::DepositSwitchCreateRequest {
            client: &self,
            target_access_token,
            target_account_id,
            country_code,
            options,
        }
    }
    /**Import Item

`/item/import` creates an Item via your Plaid Exchange Integration and returns an `access_token`. As part of an `/item/import` request, you will include a User ID (`user_auth.user_id`) and Authentication Token (`user_auth.auth_token`) that enable data aggregation through your Plaid Exchange API endpoints. These authentication principals are to be chosen by you.

Upon creating an Item via `/item/import`, Plaid will automatically begin an extraction of that Item through the Plaid Exchange infrastructure you have already integrated. This will automatically generate the Plaid native account ID for the account the user will switch their direct deposit to (`target_account_id`).*/
    pub fn item_import(
        &self,
        products: Vec<Products>,
        user_auth: serde_json::Value,
        options: serde_json::Value,
    ) -> request_model::ItemImportRequest {
        request_model::ItemImportRequest {
            client: &self,
            products,
            user_auth,
            options,
        }
    }
    /**Create a deposit switch token

In order for the end user to take action, you will need to create a public token representing the deposit switch. This token is used to initialize Link. It can be used one time and expires after 30 minutes.


See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchtokencreate>.*/
    pub fn deposit_switch_token_create(
        &self,
        deposit_switch_id: String,
    ) -> request_model::DepositSwitchTokenCreateRequest {
        request_model::DepositSwitchTokenCreateRequest {
            client: &self,
            deposit_switch_id,
        }
    }
    /**Create Link Token

The `/link/token/create` endpoint creates a `link_token`, which is required as a parameter when initializing Link. Once Link has been initialized, it returns a `public_token`, which can then be exchanged for an `access_token` via `/item/public_token/exchange` as part of the main Link flow.

A `link_token` generated by `/link/token/create` is also used to initialize other Link flows, such as the update mode flow for tokens with expired credentials, or the Payment Initiation (Europe) flow.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokencreate>.*/
    pub fn link_token_create(
        &self,
        client_name: String,
        language: String,
        country_codes: Vec<CountryCode>,
        user: serde_json::Value,
        products: Vec<Products>,
        webhook: String,
        access_token: String,
        link_customization_name: String,
        redirect_uri: String,
        android_package_name: String,
        account_filters: serde_json::Value,
        eu_config: serde_json::Value,
        institution_id: String,
        payment_initiation: serde_json::Value,
        deposit_switch: serde_json::Value,
        income_verification: serde_json::Value,
        auth: serde_json::Value,
        transfer: serde_json::Value,
        update: serde_json::Value,
    ) -> request_model::LinkTokenCreateRequest {
        request_model::LinkTokenCreateRequest {
            client: &self,
            client_name,
            language,
            country_codes,
            user,
            products,
            webhook,
            access_token,
            link_customization_name,
            redirect_uri,
            android_package_name,
            account_filters,
            eu_config,
            institution_id,
            payment_initiation,
            deposit_switch,
            income_verification,
            auth,
            transfer,
            update,
        }
    }
    /**Get Link Token

The `/link/token/get` endpoint gets information about a previously-created `link_token` using the
`/link/token/create` endpoint. It can be useful for debugging purposes.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokenget>.*/
    pub fn link_token_get(
        &self,
        link_token: String,
    ) -> request_model::LinkTokenGetRequest {
        request_model::LinkTokenGetRequest {
            client: &self,
            link_token,
        }
    }
    /**Retrieve an Asset Report Audit Copy

`/asset_report/audit_copy/get` allows auditors to get a copy of an Asset Report that was previously shared via the `/asset_report/audit_copy/create` endpoint.  The caller of `/asset_report/audit_copy/create` must provide the `audit_copy_token` to the auditor.  This token can then be used to call `/asset_report/audit_copy/create`.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_audit_copy_get(
        &self,
        audit_copy_token: String,
    ) -> request_model::AssetReportAuditCopyGetRequest {
        request_model::AssetReportAuditCopyGetRequest {
            client: &self,
            audit_copy_token,
        }
    }
    /**Retrieve a deposit switch

This endpoint returns information related to how the user has configured their payroll allocation and the state of the switch. You can use this information to build logic related to the user's direct deposit allocation preferences.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchget>.*/
    pub fn deposit_switch_get(
        &self,
        deposit_switch_id: String,
    ) -> request_model::DepositSwitchGetRequest {
        request_model::DepositSwitchGetRequest {
            client: &self,
            deposit_switch_id,
        }
    }
    /**Retrieve a transfer

The `/transfer/get` fetches information about the transfer corresponding to the given `transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products#transferget>.*/
    pub fn transfer_get(
        &self,
        transfer_id: String,
    ) -> request_model::TransferGetRequest {
        request_model::TransferGetRequest {
            client: &self,
            transfer_id,
        }
    }
    /**Retrieve a bank transfer

The `/bank_transfer/get` fetches information about the bank transfer corresponding to the given `bank_transfer_id`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferget>.*/
    pub fn bank_transfer_get(
        &self,
        bank_transfer_id: String,
    ) -> request_model::BankTransferGetRequest {
        request_model::BankTransferGetRequest {
            client: &self,
            bank_transfer_id,
        }
    }
    /**Create a transfer authorization

Use the `/transfer/authorization/create` endpoint to determine transfer failure risk.

In Plaid's sandbox environment the decisions will be returned as follows:

  - To approve a transfer, make an authorization request with an `amount` less than the available balance in the account.

  - To decline a transfer with the rationale code `NSF`, the available balance on the account must be less than the authorization `amount`. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

  - To decline a transfer with the rationale code `RISK`, the available balance on the account must be exactly $0. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

  - To permit a transfer with the rationale code `MANUALLY_VERIFIED_ITEM`, create an Item in Link through the [Same Day Micro-deposits flow](https://plaid.com/docs/auth/coverage/testing/#testing-same-day-micro-deposits).

  - To permit a transfer with the rationale code `LOGIN_REQUIRED`, [reset the login for an Item](https://plaid.com/docs/sandbox/#item_login_required).

All username/password combinations other than the ones listed above will result in a decision of permitted and rationale code `ERROR`.

See endpoint docs at <https://plaid.com/docs/api/products#transferauthorizationcreate>.*/
    pub fn transfer_authorization_create(
        &self,
        access_token: String,
        account_id: String,
        type_: String,
        network: String,
        amount: String,
        ach_class: String,
        user: serde_json::Value,
        device: serde_json::Value,
        origination_account_id: String,
        iso_currency_code: String,
    ) -> request_model::TransferAuthorizationCreateRequest {
        request_model::TransferAuthorizationCreateRequest {
            client: &self,
            access_token,
            account_id,
            type_,
            network,
            amount,
            ach_class,
            user,
            device,
            origination_account_id,
            iso_currency_code,
        }
    }
    /**Create a transfer

Use the `/transfer/create` endpoint to initiate a new transfer.

See endpoint docs at <https://plaid.com/docs/api/products#transfercreate>.*/
    pub fn transfer_create(
        &self,
        idempotency_key: String,
        access_token: String,
        account_id: String,
        authorization_id: String,
        type_: String,
        network: String,
        amount: String,
        description: String,
        ach_class: String,
        user: serde_json::Value,
        metadata: Option<serde_json::Value>,
        origination_account_id: Option<String>,
        iso_currency_code: String,
    ) -> request_model::TransferCreateRequest {
        request_model::TransferCreateRequest {
            client: &self,
            idempotency_key,
            access_token,
            account_id,
            authorization_id,
            type_,
            network,
            amount,
            description,
            ach_class,
            user,
            metadata,
            origination_account_id,
            iso_currency_code,
        }
    }
    /**Create a bank transfer

Use the `/bank_transfer/create` endpoint to initiate a new bank transfer.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercreate>.*/
    pub fn bank_transfer_create(
        &self,
        idempotency_key: String,
        access_token: String,
        account_id: String,
        type_: String,
        network: String,
        amount: String,
        iso_currency_code: String,
        description: String,
        ach_class: String,
        user: serde_json::Value,
        custom_tag: Option<String>,
        metadata: Option<serde_json::Value>,
        origination_account_id: Option<String>,
    ) -> request_model::BankTransferCreateRequest {
        request_model::BankTransferCreateRequest {
            client: &self,
            idempotency_key,
            access_token,
            account_id,
            type_,
            network,
            amount,
            iso_currency_code,
            description,
            ach_class,
            user,
            custom_tag,
            metadata,
            origination_account_id,
        }
    }
    /**List transfers

Use the `/transfer/list` endpoint to see a list of all your transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired transfers.


See endpoint docs at <https://plaid.com/docs/api/products#transferlist>.*/
    pub fn transfer_list(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        count: i64,
        offset: i64,
        origination_account_id: Option<String>,
    ) -> request_model::TransferListRequest {
        request_model::TransferListRequest {
            client: &self,
            start_date,
            end_date,
            count,
            offset,
            origination_account_id,
        }
    }
    /**List bank transfers

Use the `/bank_transfer/list` endpoint to see a list of all your bank transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired bank transfers.


See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferlist>.*/
    pub fn bank_transfer_list(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        count: i64,
        offset: i64,
        origination_account_id: Option<String>,
        direction: Option<String>,
    ) -> request_model::BankTransferListRequest {
        request_model::BankTransferListRequest {
            client: &self,
            start_date,
            end_date,
            count,
            offset,
            origination_account_id,
            direction,
        }
    }
    /**Cancel a transfer

Use the `/transfer/cancel` endpoint to cancel a transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/api/products#transfercancel>.*/
    pub fn transfer_cancel(
        &self,
        transfer_id: String,
    ) -> request_model::TransferCancelRequest {
        request_model::TransferCancelRequest {
            client: &self,
            transfer_id,
        }
    }
    /**Cancel a bank transfer

Use the `/bank_transfer/cancel` endpoint to cancel a bank transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/bank_transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercancel>.*/
    pub fn bank_transfer_cancel(
        &self,
        bank_transfer_id: String,
    ) -> request_model::BankTransferCancelRequest {
        request_model::BankTransferCancelRequest {
            client: &self,
            bank_transfer_id,
        }
    }
    /**List transfer events

Use the `/transfer/event/list` endpoint to get a list of transfer events based on specified filter criteria.

See endpoint docs at <https://plaid.com/docs/api/products#transfereventlist>.*/
    pub fn transfer_event_list(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        transfer_id: Option<String>,
        account_id: Option<String>,
        transfer_type: Option<String>,
        event_types: Vec<TransferEventType>,
        sweep_id: String,
        count: Option<i64>,
        offset: Option<i64>,
        origination_account_id: Option<String>,
    ) -> request_model::TransferEventListRequest {
        request_model::TransferEventListRequest {
            client: &self,
            start_date,
            end_date,
            transfer_id,
            account_id,
            transfer_type,
            event_types,
            sweep_id,
            count,
            offset,
            origination_account_id,
        }
    }
    /**List bank transfer events

Use the `/bank_transfer/event/list` endpoint to get a list of bank transfer events based on specified filter criteria.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfereventlist>.*/
    pub fn bank_transfer_event_list(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        bank_transfer_id: Option<String>,
        account_id: Option<String>,
        bank_transfer_type: Option<String>,
        event_types: Vec<BankTransferEventType>,
        count: Option<i64>,
        offset: Option<i64>,
        origination_account_id: Option<String>,
        direction: Option<String>,
    ) -> request_model::BankTransferEventListRequest {
        request_model::BankTransferEventListRequest {
            client: &self,
            start_date,
            end_date,
            bank_transfer_id,
            account_id,
            bank_transfer_type,
            event_types,
            count,
            offset,
            origination_account_id,
            direction,
        }
    }
    /**Sync transfer events

`/transfer/event/sync` allows you to request up to the next 25 transfer events that happened after a specific `event_id`. Use the `/transfer/event/sync` endpoint to guarantee you have seen all transfer events.

See endpoint docs at <https://plaid.com/docs/api/products#transfereventsync>.*/
    pub fn transfer_event_sync(
        &self,
        after_id: i64,
        count: Option<i64>,
    ) -> request_model::TransferEventSyncRequest {
        request_model::TransferEventSyncRequest {
            client: &self,
            after_id,
            count,
        }
    }
    /**Sync bank transfer events

`/bank_transfer/event/sync` allows you to request up to the next 25 bank transfer events that happened after a specific `event_id`. Use the `/bank_transfer/event/sync` endpoint to guarantee you have seen all bank transfer events.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfereventsync>.*/
    pub fn bank_transfer_event_sync(
        &self,
        after_id: i64,
        count: Option<i64>,
    ) -> request_model::BankTransferEventSyncRequest {
        request_model::BankTransferEventSyncRequest {
            client: &self,
            after_id,
            count,
        }
    }
    /**Retrieve a sweep

The `/transfer/sweep/get` endpoint fetches a sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products#transfersweepget>.*/
    pub fn transfer_sweep_get(
        &self,
        sweep_id: String,
    ) -> request_model::TransferSweepGetRequest {
        request_model::TransferSweepGetRequest {
            client: &self,
            sweep_id,
        }
    }
    /**Retrieve a sweep

The `/bank_transfer/sweep/get` endpoint fetches information about the sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products#bank_transfersweepget>.*/
    pub fn bank_transfer_sweep_get(
        &self,
        sweep_id: String,
    ) -> request_model::BankTransferSweepGetRequest {
        request_model::BankTransferSweepGetRequest {
            client: &self,
            sweep_id,
        }
    }
    /**List sweeps

The `/transfer/sweep/list` endpoint fetches sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products#transfersweeplist>.*/
    pub fn transfer_sweep_list(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        count: Option<i64>,
        offset: i64,
    ) -> request_model::TransferSweepListRequest {
        request_model::TransferSweepListRequest {
            client: &self,
            start_date,
            end_date,
            count,
            offset,
        }
    }
    /**List sweeps

The `/bank_transfer/sweep/list` endpoint fetches information about the sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products#bank_transfersweeplist>.*/
    pub fn bank_transfer_sweep_list(
        &self,
        origination_account_id: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
        count: Option<i64>,
    ) -> request_model::BankTransferSweepListRequest {
        request_model::BankTransferSweepListRequest {
            client: &self,
            origination_account_id,
            start_time,
            end_time,
            count,
        }
    }
    /**Get balance of your Bank Transfer account

Use the `/bank_transfer/balance/get` endpoint to see the available balance in your bank transfer account. Debit transfers increase this balance once their status is posted. Credit transfers decrease this balance when they are created.

The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.

Note that this endpoint can only be used with FBO accounts, when using Bank Transfers in the Full Service configuration. It cannot be used on your own account when using Bank Transfers in the BTS Platform configuration.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferbalanceget>.*/
    pub fn bank_transfer_balance_get(
        &self,
        origination_account_id: Option<String>,
    ) -> request_model::BankTransferBalanceGetRequest {
        request_model::BankTransferBalanceGetRequest {
            client: &self,
            origination_account_id,
        }
    }
    /**Migrate account into Bank Transfers

As an alternative to adding Items via Link, you can also use the `/bank_transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items.  Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Bank Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/bank_transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfermigrate_account>.*/
    pub fn bank_transfer_migrate_account(
        &self,
        account_number: String,
        routing_number: String,
        account_type: String,
    ) -> request_model::BankTransferMigrateAccountRequest {
        request_model::BankTransferMigrateAccountRequest {
            client: &self,
            account_number,
            routing_number,
            account_type,
        }
    }
    /**Create a transfer intent object to invoke the Transfer UI

Use the `/transfer/intent/create` endpoint to generate a transfer intent object and invoke the Transfer UI.

See endpoint docs at <https://plaid.com/docs/api/products#transferintentcreate>.*/
    pub fn transfer_intent_create(
        &self,
        account_id: Option<String>,
        mode: String,
        amount: String,
        description: String,
        ach_class: String,
        origination_account_id: Option<String>,
        user: serde_json::Value,
        metadata: Option<serde_json::Value>,
        iso_currency_code: String,
    ) -> request_model::TransferIntentCreateRequest {
        request_model::TransferIntentCreateRequest {
            client: &self,
            account_id,
            mode,
            amount,
            description,
            ach_class,
            origination_account_id,
            user,
            metadata,
            iso_currency_code,
        }
    }
    /**Retrieve more information about a transfer intent

Use the `/transfer/intent/get` endpoint to retrieve more information about a transfer intent.

See endpoint docs at <https://plaid.com/docs/api/products#transferintentget>.*/
    pub fn transfer_intent_get(
        &self,
        transfer_intent_id: String,
    ) -> request_model::TransferIntentGetRequest {
        request_model::TransferIntentGetRequest {
            client: &self,
            transfer_intent_id,
        }
    }
    /**Lists historical repayments

The `/transfer/repayment/list` endpoint fetches repayments matching the given filters. Repayments are returned in reverse-chronological order (most recent first) starting at the given `start_time`.

See endpoint docs at <https://plaid.com/docs/api/products#transferrepaymentlist>.*/
    pub fn transfer_repayment_list(
        &self,
        start_date: Option<String>,
        end_date: Option<String>,
        count: Option<i64>,
        offset: i64,
    ) -> request_model::TransferRepaymentListRequest {
        request_model::TransferRepaymentListRequest {
            client: &self,
            start_date,
            end_date,
            count,
            offset,
        }
    }
    /**List the returns included in a repayment

The `/transfer/repayment/return/list` endpoint retrieves the set of returns that were batched together into the specified repayment. The sum of amounts of returns retrieved by this request equals the amount of the repayment.

See endpoint docs at <https://plaid.com/docs/api/products#transferrepaymentreturnlist>.*/
    pub fn transfer_repayment_return_list(
        &self,
        repayment_id: String,
        count: Option<i64>,
        offset: i64,
    ) -> request_model::TransferRepaymentReturnListRequest {
        request_model::TransferRepaymentReturnListRequest {
            client: &self,
            repayment_id,
            count,
            offset,
        }
    }
    /**Simulate a bank transfer event in Sandbox

Use the `/sandbox/bank_transfer/simulate` endpoint to simulate a bank transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/bank_transfer/event/sync` or `/bank_transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transfersimulate>.*/
    pub fn sandbox_bank_transfer_simulate(
        &self,
        bank_transfer_id: String,
        event_type: String,
        failure_reason: Option<serde_json::Value>,
    ) -> request_model::SandboxBankTransferSimulateRequest {
        request_model::SandboxBankTransferSimulateRequest {
            client: &self,
            bank_transfer_id,
            event_type,
            failure_reason,
        }
    }
    /**Simulate creating a sweep

Use the `/sandbox/transfer/sweep/simulate` endpoint to create a sweep and associated events in the Sandbox environment. Upon calling this endpoint, all `posted` or `pending` transfers with a sweep status of `unswept` will become `swept`, and all `reversed` transfers with a sweep status of `swept` will become `reverse_swept`.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersweepsimulate>.*/
    pub fn sandbox_transfer_sweep_simulate(
        &self,
    ) -> request_model::SandboxTransferSweepSimulateRequest {
        request_model::SandboxTransferSweepSimulateRequest {
            client: &self,
        }
    }
    /**Simulate a transfer event in Sandbox

Use the `/sandbox/transfer/simulate` endpoint to simulate a transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersimulate>.*/
    pub fn sandbox_transfer_simulate(
        &self,
        transfer_id: String,
        event_type: String,
        failure_reason: Option<serde_json::Value>,
    ) -> request_model::SandboxTransferSimulateRequest {
        request_model::SandboxTransferSimulateRequest {
            client: &self,
            transfer_id,
            event_type,
            failure_reason,
        }
    }
    /**Trigger the creation of a repayment

Use the `/sandbox/transfer/repayment/simulate` endpoint to trigger the creation of a repayment. As a side effect of calling this route, a repayment is created that includes all unreimbursed returns of guaranteed transfers. If there are no such returns, an 400 error is returned.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrepaymentsimulate>.*/
    pub fn sandbox_transfer_repayment_simulate(
        &self,
    ) -> request_model::SandboxTransferRepaymentSimulateRequest {
        request_model::SandboxTransferRepaymentSimulateRequest {
            client: &self,
        }
    }
    /**Search employer database

`/employers/search` allows you the ability to search Plaid’s database of known employers, for use with Deposit Switch. You can use this endpoint to look up a user's employer in order to confirm that they are supported. Users with non-supported employers can then be routed out of the Deposit Switch flow.

The data in the employer database is currently limited. As the Deposit Switch and Income products progress through their respective beta periods, more employers are being regularly added. Because the employer database is frequently updated, we recommend that you do not cache or store data from this endpoint for more than a day.

See endpoint docs at <https://plaid.com/docs/api/employers/#employerssearch>.*/
    pub fn employers_search(
        &self,
        query: String,
        products: Vec<String>,
    ) -> request_model::EmployersSearchRequest {
        request_model::EmployersSearchRequest {
            client: &self,
            query,
            products,
        }
    }
    /**(Deprecated) Create an income verification instance

`/income/verification/create` begins the income verification process by returning an `income_verification_id`. You can then provide the `income_verification_id` to `/link/token/create` under the `income_verification` parameter in order to create a Link instance that will prompt the user to go through the income verification flow. Plaid will fire an `INCOME` webhook once the user completes the Payroll Income flow, or when the uploaded documents in the Document Income flow have finished processing.

See endpoint docs at <https://plaid.com/docs/api/products/#incomeverificationcreate>.*/
    pub fn income_verification_create(
        &self,
        webhook: String,
        precheck_id: String,
        options: serde_json::Value,
    ) -> request_model::IncomeVerificationCreateRequest {
        request_model::IncomeVerificationCreateRequest {
            client: &self,
            webhook,
            precheck_id,
            options,
        }
    }
    /**(Deprecated) Retrieve a summary of information derived from income verification

`/income/verification/summary/get` returns a verification summary for the income that was verified for an end user. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

See endpoint docs at <https://plaid.com/docs/api/products/#incomeverificationsummaryget>.*/
    pub fn income_verification_summary_get(
        &self,
        income_verification_id: Option<String>,
        access_token: Option<String>,
    ) -> request_model::IncomeVerificationSummaryGetRequest {
        request_model::IncomeVerificationSummaryGetRequest {
            client: &self,
            income_verification_id,
            access_token,
        }
    }
    /**(Deprecated) Retrieve information from a single paystub used for income verification

/income/verification/paystub/get returns information from a single paystub used for income verification*/
    pub fn income_verification_paystub_get(
        &self,
        income_verification_id: Option<String>,
        access_token: Option<String>,
    ) -> request_model::IncomeVerificationPaystubGetRequest {
        request_model::IncomeVerificationPaystubGetRequest {
            client: &self,
            income_verification_id,
            access_token,
        }
    }
    /**Retrieve information from the paystubs used for income verification

`/income/verification/paystubs/get` returns the information collected from the paystubs that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

See endpoint docs at <https://plaid.com/docs/api/products/#incomeverificationpaystubsget>.*/
    pub fn income_verification_paystubs_get(
        &self,
        income_verification_id: Option<String>,
        access_token: Option<String>,
    ) -> request_model::IncomeVerificationPaystubsGetRequest {
        request_model::IncomeVerificationPaystubsGetRequest {
            client: &self,
            income_verification_id,
            access_token,
        }
    }
    /**Refresh an income verification

`/income/verification/refresh` refreshes a given income verification.

See endpoint docs at <https://plaid.com/docs/api/products/#incomeverificationrefresh>.*/
    pub fn income_verification_refresh(
        &self,
        income_verification_id: Option<String>,
        access_token: Option<String>,
    ) -> request_model::IncomeVerificationRefreshRequest {
        request_model::IncomeVerificationRefreshRequest {
            client: &self,
            income_verification_id,
            access_token,
        }
    }
    /**Retrieve information from the tax documents used for income verification

`/income/verification/taxforms/get` returns the information collected from forms that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

See endpoint docs at <https://plaid.com/docs/api/products/#incomeverificationtaxformsget>.*/
    pub fn income_verification_taxforms_get(
        &self,
        income_verification_id: Option<String>,
        access_token: Option<String>,
    ) -> request_model::IncomeVerificationTaxformsGetRequest {
        request_model::IncomeVerificationTaxformsGetRequest {
            client: &self,
            income_verification_id,
            access_token,
        }
    }
    /**Check digital income verification eligibility and optimize conversion

`/income/verification/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification and returns a `precheck_id` that can be provided to `/link/token/create`. If the user is eligible for digital verification, providing the `precheck_id` in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the `precheck_id` can still be provided to `/link/token/create` and the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing either `employer` or `transactions_access_tokens` data will increase the chance of receiving a useful result.

See endpoint docs at <https://plaid.com/docs/api/products/#incomeverificationprecheck>.*/
    pub fn income_verification_precheck(
        &self,
        user: Option<serde_json::Value>,
        employer: Option<serde_json::Value>,
        transactions_access_token: serde_json::Value,
        transactions_access_tokens: Vec<AccessToken>,
        us_military_info: Option<serde_json::Value>,
    ) -> request_model::IncomeVerificationPrecheckRequest {
        request_model::IncomeVerificationPrecheckRequest {
            client: &self,
            user,
            employer,
            transactions_access_token,
            transactions_access_tokens,
            us_military_info,
        }
    }
    /**Retrieve a summary of an individual's employment information

`/employment/verification/get` returns a list of employments through a user payroll that was verified by an end user.

See endpoint docs at <https://plaid.com/docs/api/products/#employmentverificationget>.*/
    pub fn employment_verification_get(
        &self,
        access_token: String,
    ) -> request_model::EmploymentVerificationGetRequest {
        request_model::EmploymentVerificationGetRequest {
            client: &self,
            access_token,
        }
    }
    /**Create a deposit switch without using Plaid Exchange

This endpoint provides an alternative to `/deposit_switch/create` for customers who have not yet fully integrated with Plaid Exchange. Like `/deposit_switch/create`, it creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchaltcreate>.*/
    pub fn deposit_switch_alt_create(
        &self,
        target_account: serde_json::Value,
        target_user: serde_json::Value,
        options: serde_json::Value,
        country_code: Option<String>,
    ) -> request_model::DepositSwitchAltCreateRequest {
        request_model::DepositSwitchAltCreateRequest {
            client: &self,
            target_account,
            target_user,
            options,
            country_code,
        }
    }
    /**Manually fire a Bank Transfer webhook

Use the `/sandbox/bank_transfer/fire_webhook` endpoint to manually trigger a Bank Transfers webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transferfire_webhook>.*/
    pub fn sandbox_bank_transfer_fire_webhook(
        &self,
        webhook: String,
    ) -> request_model::SandboxBankTransferFireWebhookRequest {
        request_model::SandboxBankTransferFireWebhookRequest {
            client: &self,
            webhook,
        }
    }
    /**Manually fire an Income webhook

Use the `/sandbox/income/fire_webhook` endpoint to manually trigger an Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxincomefire_webhook>.*/
    pub fn sandbox_income_fire_webhook(
        &self,
        income_verification_id: String,
        item_id: String,
        webhook: String,
        verification_status: String,
    ) -> request_model::SandboxIncomeFireWebhookRequest {
        request_model::SandboxIncomeFireWebhookRequest {
            client: &self,
            income_verification_id,
            item_id,
            webhook,
            verification_status,
        }
    }
    ///Save the selected accounts when connecting to the Platypus Oauth institution
    pub fn sandbox_oauth_select_accounts(
        &self,
        oauth_state_id: String,
        accounts: Vec<String>,
    ) -> request_model::SandboxOauthSelectAccountsRequest {
        request_model::SandboxOauthSelectAccountsRequest {
            client: &self,
            oauth_state_id,
            accounts,
        }
    }
    /**Evaluate a planned ACH transaction

Use `/signal/evaluate` to evaluate a planned ACH transaction to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned.

See endpoint docs at <https://plaid.com/docs/signal/reference#signalevaluate>.*/
    pub fn signal_evaluate(
        &self,
        access_token: String,
        account_id: String,
        client_transaction_id: String,
        amount: f64,
        user_present: Option<bool>,
        client_user_id: String,
        user: serde_json::Value,
        device: serde_json::Value,
    ) -> request_model::SignalEvaluateRequest {
        request_model::SignalEvaluateRequest {
            client: &self,
            access_token,
            account_id,
            client_transaction_id,
            amount,
            user_present,
            client_user_id,
            user,
            device,
        }
    }
    /**Report whether you initiated an ACH transaction

After calling `/signal/evaluate`, call `/signal/decision/report` to report whether the transaction was initiated. This endpoint will return an `INVALID_REQUEST` error if called a second time with a different value for `initiated`.

See endpoint docs at <https://plaid.com/docs/signal/reference#signaldecisionreport>.*/
    pub fn signal_decision_report(
        &self,
        client_transaction_id: String,
        initiated: bool,
        days_funds_on_hold: Option<i64>,
    ) -> request_model::SignalDecisionReportRequest {
        request_model::SignalDecisionReportRequest {
            client: &self,
            client_transaction_id,
            initiated,
            days_funds_on_hold,
        }
    }
    /**Report a return for an ACH transaction

Call the `/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/signal/evaluate` endpoint. Your feedback will be used by the foo to incorporate the latest risk trend in your portfolio.

See endpoint docs at <https://plaid.com/docs/signal/reference#signalreturnreport>.*/
    pub fn signal_return_report(
        &self,
        client_transaction_id: String,
        return_code: String,
    ) -> request_model::SignalReturnReportRequest {
        request_model::SignalReturnReportRequest {
            client: &self,
            client_transaction_id,
            return_code,
        }
    }
    /**Fetch an e-wallet

Fetch an e-wallet. The response includes the current balance.


See endpoint docs at <https://plaid.com/docs/api/products/#walletget>.*/
    pub fn wallet_get(&self, wallet_id: String) -> request_model::WalletGetRequest {
        request_model::WalletGetRequest {
            client: &self,
            wallet_id,
        }
    }
    /**Execute a transaction using an e-wallet

Execute a transaction using the specified e-wallet. Specify the e-wallet to debit from, the counterparty to credit to, the idempotency key to prevent duplicate payouts, the amount and reference for the payout. The payouts are executed over the Faster Payment rails, where settlement usually only takes a few seconds.


See endpoint docs at <https://plaid.com/docs/api/products/#wallettransactionexecute>.*/
    pub fn wallet_transaction_execute(
        &self,
        idempotency_key: String,
        wallet_id: String,
        counterparty: serde_json::Value,
        amount: serde_json::Value,
        reference: String,
    ) -> request_model::WalletTransactionExecuteRequest {
        request_model::WalletTransactionExecuteRequest {
            client: &self,
            idempotency_key,
            wallet_id,
            counterparty,
            amount,
            reference,
        }
    }
    /**List e-wallet transactions

This endpoint lists the latest transactions of the specified e-wallet. Transactions are returned in descending order by the `created_at` time.


See endpoint docs at <https://plaid.com/docs/api/products/#wallettransactionslist>.*/
    pub fn wallet_transactions_list(
        &self,
        wallet_id: String,
        cursor: String,
        count: i64,
    ) -> request_model::WalletTransactionsListRequest {
        request_model::WalletTransactionsListRequest {
            client: &self,
            wallet_id,
            cursor,
            count,
        }
    }
}
pub enum PlaidAuthentication {
    ClientId { client_id: String, secret: String, plaid_version: String },
}
impl PlaidAuthentication {
    pub fn from_env() -> Self {
        Self::ClientId {
            client_id: std::env::var("PLAID_CLIENT_ID")
                .expect("Environment variable PLAID_CLIENT_ID is not set."),
            secret: std::env::var("PLAID_SECRET")
                .expect("Environment variable PLAID_SECRET is not set."),
            plaid_version: std::env::var("PLAID_VERSION")
                .expect("Environment variable PLAID_VERSION is not set."),
        }
    }
}
