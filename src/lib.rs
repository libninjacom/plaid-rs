//! The [`PlaidClient`] is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]

use serde_json::json;
pub mod model;
pub mod request;
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
        if let Some(ref authentication) = self.authentication {
            match authentication {
                PlaidAuthentication::ClientId { client_id, secret, plaid_version } => {
                    r = r.header("PLAID-CLIENT-ID", client_id);
                    r = r.header("PLAID-SECRET", secret);
                    r = r.header("Plaid-Version", plaid_version);
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
    pub fn item_application_list(&self) -> request::ItemApplicationListRequest {
        request::ItemApplicationListRequest {
            client: &self,
            access_token: None,
        }
    }
    /**Update the scopes of access for a particular application

Enable consumers to update product access on selected accounts for an application.*/
    pub fn item_application_scopes_update(
        &self,
        args: request::ItemApplicationScopesUpdateRequired,
    ) -> request::ItemApplicationScopesUpdateRequest {
        request::ItemApplicationScopesUpdateRequest {
            client: &self,
            access_token: args.access_token.to_owned(),
            application_id: args.application_id.to_owned(),
            scopes: args.scopes,
            state: None,
            context: args.context.to_owned(),
        }
    }
    /**Retrieve information about a Plaid application

Allows financial institutions to retrieve information about Plaid clients for the purpose of building control-tower experiences*/
    pub fn application_get(
        &self,
        application_id: &str,
    ) -> request::ApplicationGetRequest {
        request::ApplicationGetRequest {
            client: &self,
            application_id: application_id.to_owned(),
        }
    }
    /**Retrieve an Item

Returns information about the status of an Item.

See endpoint docs at <https://plaid.com/docs/api/items/#itemget>.*/
    pub fn item_get(&self, access_token: &str) -> request::ItemGetRequest {
        request::ItemGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Retrieve auth data

The `/auth/get` endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.

Note: This request may take some time to complete if `auth` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

Also note that `/auth/get` will not return data for any new accounts opened after the Item was created. To obtain data for new accounts, create a new Item.

Versioning note: In API version 2017-03-08, the schema of the `numbers` object returned by this endpoint is substantially different. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2018-05-22).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#authget>.*/
    pub fn auth_get(&self, access_token: &str) -> request::AuthGetRequest {
        request::AuthGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Get transaction data

The `/transactions/get` endpoint allows developers to receive user-authorized transaction data for credit, depository, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from investments accounts, use the [Investments endpoint](https://plaid.com/docs/api/products/investments/) instead. Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.  Transactions are not immutable and can also be removed altogether by the institution; a removed transaction will no longer appear in `/transactions/get`.  For more details, see [Pending and posted transactions](https://plaid.com/docs/transactions/transactions-data/#pending-and-posted-transactions).

Due to the potentially large number of transactions associated with an Item, results are paginated. Manipulate the `count` and `offset` parameters in conjunction with the `total_transactions` response body field to fetch all available transactions.

Data returned by `/transactions/get` will be the data available for the Item as of the most recent successful check for new transactions. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. An Item's `status.transactions.last_successful_update` field will show the timestamp of the most recent successful update. To force Plaid to check for new transactions, you can use the `/transactions/refresh` endpoint.

Note that data may not be immediately available to `/transactions/get`. Plaid will begin to prepare transactions data upon Item link, if Link was initialized with `transactions`, or upon the first call to `/transactions/get`, if it wasn't. To be alerted when transaction data is ready to be fetched, listen for the [`INITIAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#initial_update) and [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhooks. If no transaction history is ready when `/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsget>.*/
    pub fn transactions_get(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
    ) -> request::TransactionsGetRequest {
        request::TransactionsGetRequest {
            client: &self,
            options: None,
            access_token: access_token.to_owned(),
            start_date: start_date.to_owned(),
            end_date: end_date.to_owned(),
        }
    }
    /**Refresh transaction data

`/transactions/refresh` is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled Item. If changes to transactions are discovered after calling `/transactions/refresh`, Plaid will fire a webhook: [`TRANSACTIONS_REMOVED`](https://plaid.com/docs/api/products/transactions/#transactions_removed) will be fired if any removed transactions are detected, and [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/transactions/#default_update) will be fired if any new transactions are detected. New transactions can be fetched by calling `/transactions/get`.

Access to `/transactions/refresh` in Production is specific to certain pricing plans. If you cannot access `/transactions/refresh` in Production, [contact Sales](https://www.plaid.com/contact) for assistance.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrefresh>.*/
    pub fn transactions_refresh(
        &self,
        access_token: &str,
    ) -> request::TransactionsRefreshRequest {
        request::TransactionsRefreshRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Fetch recurring transaction streams

The `/transactions/recurring/get` endpoint allows developers to receive a summary of the recurring outflow and inflow streams (expenses and deposits) from a user’s checking, savings or credit card accounts. Additionally, Plaid provides key insights about each recurring stream including the category, merchant, last amount, and more. Developers can use these insights to build tools and experiences that help their users better manage cash flow, monitor subscriptions, reduce spend, and stay on track with bill payments.

This endpoint is not included by default as part of Transactions. To request access to this endpoint and learn more about pricing, contact your Plaid account manager.

Note that unlike `/transactions/get`, `/transactions/recurring/get` will not initialize an Item with Transactions. The Item must already have been initialized with Transactions (either during Link, by specifying it in `/link/token/create`, or after Link, by calling `/transactions/get`) before calling this endpoint. Data is available to `/transactions/recurring/get` approximately 5 seconds after the [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhook has fired (about 1-2 minutes after initialization).

After the initial call, you can call `/transactions/recurring/get` endpoint at any point in the future to retrieve the latest summary of recurring streams. Since recurring streams do not change often, it will typically not be necessary to call this endpoint more than once per day.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrecurringget>.*/
    pub fn transactions_recurring_get(
        &self,
        access_token: &str,
        account_ids: &[&str],
    ) -> request::TransactionsRecurringGetRequest {
        request::TransactionsRecurringGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
            account_ids: account_ids.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Get incremental transaction updates on an Item

This endpoint replaces `/transactions/get` and its associated webhooks for most common use-cases.

The `/transactions/sync` endpoint allows developers to subscribe to all transactions associated with an Item and get updates synchronously in a stream-like manner, using a cursor to track which updates have already been seen. `/transactions/sync` provides the same functionality as `/transactions/get` and can be used instead of `/transactions/get` to simplify the process of tracking transactions updates.

This endpoint provides user-authorized transaction data for `credit`, `depository`, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from `investments` accounts, use `/investments/transactions/get` instead.

Returned transactions data is grouped into three types of update, indicating whether the transaction was added, removed, or modified since the last call to the API.

In the first call to `/transactions/sync` for an Item, the endpoint will return all historical transactions data associated with that Item up until the time of the API call (as "adds"), which then generates a `next_cursor` for that Item. In subsequent calls, send the `next_cursor` to receive only the changes that have occurred since the previous call.

Due to the potentially large number of transactions associated with an Item, results are paginated. The `has_more` field specifies if additional calls are necessary to fetch all available transaction updates.

Whenever new or updated transaction data becomes available, `/transactions/sync` will provide these updates. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. An Item's `status.transactions.last_successful_update` field will show the timestamp of the most recent successful update. To force Plaid to check for new transactions, use the `/transactions/refresh` endpoint.

Note that for newly created Items, data may not be immediately available to `/transactions/sync`. Plaid begins preparing transactions data when the Item is created, but the process can take anywhere from a few seconds to several minutes to complete, depending on the number of transactions available.

To be alerted when new data is available, listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#sync_updates_available) webhook.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionssync>.*/
    pub fn transactions_sync(
        &self,
        access_token: &str,
        cursor: &Option<String>,
        count: &Option<i64>,
    ) -> request::TransactionsSyncRequest {
        request::TransactionsSyncRequest {
            client: &self,
            access_token: access_token.to_owned(),
            cursor: cursor.clone(),
            count: count.clone(),
            options: None,
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
        country_codes: &[&str],
    ) -> request::InstitutionsGetRequest {
        request::InstitutionsGetRequest {
            client: &self,
            count,
            offset,
            country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
            options: None,
        }
    }
    /**Search institutions

Returns a JSON response containing details for institutions that match the query parameters, up to a maximum of ten institutions per query.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` parameters to authenticate to this endpoint. The `public_key` parameter has since been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionssearch>.*/
    pub fn institutions_search(
        &self,
        query: &str,
        country_codes: &[&str],
    ) -> request::InstitutionsSearchRequest {
        request::InstitutionsSearchRequest {
            client: &self,
            query: query.to_owned(),
            products: None,
            country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
            options: None,
        }
    }
    /**Get details of an institution

Returns a JSON response containing details on a specified financial institution currently supported by Plaid.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` to authenticate to this endpoint. The `public_key` has been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget_by_id>.*/
    pub fn institutions_get_by_id(
        &self,
        institution_id: &str,
        country_codes: &[&str],
    ) -> request::InstitutionsGetByIdRequest {
        request::InstitutionsGetByIdRequest {
            client: &self,
            institution_id: institution_id.to_owned(),
            country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
            options: None,
        }
    }
    /**Remove an Item

The `/item/remove` endpoint allows you to remove an Item. Once removed, the `access_token`, as well as any processor tokens or bank account tokens associated with the Item, is no longer valid and cannot be used to access any data that was associated with the Item.

Note that in the Development environment, issuing an `/item/remove`  request will not decrement your live credential count. To increase your credential account in Development, contact Support.

Also note that for certain OAuth-based institutions, an Item removed via `/item/remove` may still show as an active connection in the institution's OAuth permission manager.

API versions 2019-05-29 and earlier return a `removed` boolean as part of the response.

See endpoint docs at <https://plaid.com/docs/api/items/#itemremove>.*/
    pub fn item_remove(&self, access_token: &str) -> request::ItemRemoveRequest {
        request::ItemRemoveRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Retrieve accounts

The `/accounts/get` endpoint can be used to retrieve a list of accounts associated with any linked Item. Plaid will only return active bank accounts — that is, accounts that are not closed and are capable of carrying a balance.
For items that went through the updated account selection pane, this endpoint only returns accounts that were permissioned by the user when they initially created the Item. If a user creates a new account after the initial link, you can capture this event through the [`NEW_ACCOUNTS_AVAILABLE`](https://plaid.com/docs/api/items/#new_accounts_available) webhook and then use Link's [update mode](https://plaid.com/docs/link/update-mode/) to request that the user share this new account with you.

This endpoint retrieves cached information, rather than extracting fresh information from the institution. As a result, balances returned may not be up-to-date; for realtime balance information, use `/accounts/balance/get` instead. Note that some information is nullable.

See endpoint docs at <https://plaid.com/docs/api/accounts/#accountsget>.*/
    pub fn accounts_get(&self, access_token: &str) -> request::AccountsGetRequest {
        request::AccountsGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Get Categories

Send a request to the `/categories/get` endpoint to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#categoriesget>.*/
    pub fn categories_get(&self) -> request::CategoriesGetRequest {
        request::CategoriesGetRequest {
            client: &self,
        }
    }
    /**Create a test Item and processor token

Use the `/sandbox/processor_token/create` endpoint to create a valid `processor_token` for an arbitrary institution ID and test credentials. The created `processor_token` corresponds to a new Sandbox Item. You can then use this `processor_token` with the `/processor/` API endpoints in Sandbox. You can also use `/sandbox/processor_token/create` with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxprocessor_tokencreate>.*/
    pub fn sandbox_processor_token_create(
        &self,
        institution_id: &str,
    ) -> request::SandboxProcessorTokenCreateRequest {
        request::SandboxProcessorTokenCreateRequest {
            client: &self,
            institution_id: institution_id.to_owned(),
            options: None,
        }
    }
    /**Create a test Item

Use the `/sandbox/public_token/create` endpoint to create a valid `public_token`  for an arbitrary institution ID, initial products, and test credentials. The created `public_token` maps to a new Sandbox Item. You can then call `/item/public_token/exchange` to exchange the `public_token` for an `access_token` and perform all API actions. `/sandbox/public_token/create` can also be used with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data. `/sandbox/public_token/create` cannot be used with OAuth institutions.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate>.*/
    pub fn sandbox_public_token_create(
        &self,
        institution_id: &str,
        initial_products: &[&str],
    ) -> request::SandboxPublicTokenCreateRequest {
        request::SandboxPublicTokenCreateRequest {
            client: &self,
            institution_id: institution_id.to_owned(),
            initial_products: initial_products.iter().map(|&x| x.to_owned()).collect(),
            options: None,
            user_token: None,
        }
    }
    /**Fire a test webhook

The `/sandbox/item/fire_webhook` endpoint is used to test that code correctly handles webhooks. This endpoint can trigger the following webhooks:

`DEFAULT_UPDATE`: Transactions update webhook to be fired for a given Sandbox Item. If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`NEW_ACCOUNTS_AVAILABLE`: Webhook to be fired for a given Sandbox Item created with Account Select v2.

`AUTH_DATA_UPDATE`: Webhook to be fired for a given Sandbox Item created with Auth as an enabled product.

`RECURRING_TRANSACTIONS_UPDATE`: Recurring Transactions webhook to be fired for a given Sandbox Item. If the Item does not support Recurring Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

Note that this endpoint is provided for developer ease-of-use and is not required for testing webhooks; webhooks will also fire in Sandbox under the same conditions that they would in Production or Development.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemfire_webhook>.*/
    pub fn sandbox_item_fire_webhook(
        &self,
        access_token: &str,
        webhook_code: &str,
    ) -> request::SandboxItemFireWebhookRequest {
        request::SandboxItemFireWebhookRequest {
            client: &self,
            access_token: access_token.to_owned(),
            webhook_type: None,
            webhook_code: webhook_code.to_owned(),
        }
    }
    /**Retrieve real-time balance data

The `/accounts/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/accounts/balance/get` forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, `balance` itself is not a product that can be used to initialize Link.

See endpoint docs at <https://plaid.com/docs/api/products/balance/#accountsbalanceget>.*/
    pub fn accounts_balance_get(
        &self,
        access_token: &str,
    ) -> request::AccountsBalanceGetRequest {
        request::AccountsBalanceGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Retrieve identity data

The `/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses. Only name data is guaranteed to be returned; other fields will be empty arrays if not provided by the institution.

This request may take some time to complete if identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

Note: In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29).

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identityget>.*/
    pub fn identity_get(&self, access_token: &str) -> request::IdentityGetRequest {
        request::IdentityGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Retrieve identity match score

The `/identity/match` endpoint generates a match score, which indicates how well the provided identity data matches the identity information on file with the account holder's financial institution.

This request may take some time to complete if Identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identitymatch>.*/
    pub fn identity_match(&self, access_token: &str) -> request::IdentityMatchRequest {
        request::IdentityMatchRequest {
            client: &self,
            access_token: access_token.to_owned(),
            user: None,
            options: None,
        }
    }
    /**Retrieve a dashboard user

Retrieve information about a dashboard user.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#dashboard_userget>.*/
    pub fn dashobard_user_get(
        &self,
        dashboard_user_id: &str,
    ) -> request::DashobardUserGetRequest {
        request::DashobardUserGetRequest {
            client: &self,
            dashboard_user_id: dashboard_user_id.to_owned(),
        }
    }
    /**List dashboard users

List all dashboard users associated with your account.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#dashboard_userlist>.*/
    pub fn dashboard_user_list(&self) -> request::DashboardUserListRequest {
        request::DashboardUserListRequest {
            client: &self,
            cursor: None,
        }
    }
    /**Create a new identity verification

Create a new Identity Verification for the user specified by the `client_user_id` field. The requirements and behavior of the verification are determined by the `template_id` provided.
If you don't know whether the associated user already has an active Identity Verification, you can specify `"is_idempotent": true` in the request body. With idempotency enabled, a new Identity Verification will only be created if one does not already exist for the associated `client_user_id` and `template_id`. If an Identity Verification is found, it will be returned unmodified with an `200 OK` HTTP status code.


See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationcreate>.*/
    pub fn identity_verification_create(
        &self,
        args: request::IdentityVerificationCreateRequired,
    ) -> request::IdentityVerificationCreateRequest {
        request::IdentityVerificationCreateRequest {
            client: &self,
            is_shareable: args.is_shareable,
            template_id: args.template_id.to_owned(),
            gave_consent: args.gave_consent,
            user: args.user,
            is_idempotent: None,
        }
    }
    /**Retrieve Identity Verification

Retrieve a previously created identity verification

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationget>.*/
    pub fn identity_verification_get(
        &self,
        identity_verification_id: &str,
    ) -> request::IdentityVerificationGetRequest {
        request::IdentityVerificationGetRequest {
            client: &self,
            identity_verification_id: identity_verification_id.to_owned(),
        }
    }
    /**List Identity Verifications

Filter and list Identity Verifications created by your account

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationlist>.*/
    pub fn identity_verification_list(
        &self,
        template_id: &str,
        client_user_id: &str,
    ) -> request::IdentityVerificationListRequest {
        request::IdentityVerificationListRequest {
            client: &self,
            template_id: template_id.to_owned(),
            client_user_id: client_user_id.to_owned(),
            cursor: None,
        }
    }
    /**Retry an Identity Verification

Allow a customer to retry their identity verification

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationretry>.*/
    pub fn identity_verification_retry(
        &self,
        client_user_id: &str,
        template_id: &str,
        strategy: &str,
    ) -> request::IdentityVerificationRetryRequest {
        request::IdentityVerificationRetryRequest {
            client: &self,
            client_user_id: client_user_id.to_owned(),
            template_id: template_id.to_owned(),
            strategy: strategy.to_owned(),
            steps: None,
        }
    }
    /**Create a watchlist screening for an entity

Create a new entity watchlist screening to check your customer against watchlists defined in the associated entity watchlist program. If your associated program has ongoing screening enabled, this is the profile information that will be used to monitor your customer over time.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentitycreate>.*/
    pub fn watchlist_screening_entity_create(
        &self,
        search_terms: EntityWatchlistSearchTerms,
    ) -> request::WatchlistScreeningEntityCreateRequest {
        request::WatchlistScreeningEntityCreateRequest {
            client: &self,
            search_terms,
            client_user_id: None,
        }
    }
    /**Get an entity screening

Retrieve an entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityget>.*/
    pub fn watchlist_screening_entity_get(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityGetRequest {
        request::WatchlistScreeningEntityGetRequest {
            client: &self,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
        }
    }
    /**List history for entity watchlist screenings

List all changes to the entity watchlist screening in reverse-chronological order. If the watchlist screening has not been edited, no history will be returned.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhistorylist>.*/
    pub fn watchlist_screening_entity_history_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityHistoryListRequest {
        request::WatchlistScreeningEntityHistoryListRequest {
            client: &self,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            cursor: None,
        }
    }
    /**List hits for entity watchlist screenings

List all hits for the entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhitlist>.*/
    pub fn watchlist_screening_entity_hits_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityHitsListRequest {
        request::WatchlistScreeningEntityHitsListRequest {
            client: &self,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            cursor: None,
        }
    }
    /**List entity watchlist screenings

List all entity screenings.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentitylist>.*/
    pub fn watchlist_screening_entity_list(
        &self,
        entity_watchlist_program_id: &str,
    ) -> request::WatchlistScreeningEntityListRequest {
        request::WatchlistScreeningEntityListRequest {
            client: &self,
            entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
            client_user_id: None,
            status: None,
            assignee: None,
            cursor: None,
        }
    }
    /**Get entity watchlist screening program

Get an entity watchlist screening program

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityprogramget>.*/
    pub fn watchlist_screening_entity_program_get(
        &self,
        entity_watchlist_program_id: &str,
    ) -> request::WatchlistScreeningEntityProgramGetRequest {
        request::WatchlistScreeningEntityProgramGetRequest {
            client: &self,
            entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
        }
    }
    /**List entity watchlist screening programs

List all entity watchlist screening programs

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityprogramlist>.*/
    pub fn watchlist_screening_entity_program_list(
        &self,
    ) -> request::WatchlistScreeningEntityProgramListRequest {
        request::WatchlistScreeningEntityProgramListRequest {
            client: &self,
            cursor: None,
        }
    }
    /**Create a review for an entity watchlist screening

Create a review for an entity watchlist screening. Reviews are compliance reports created by users in your organization regarding the relevance of potential hits found by Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityreviewcreate>.*/
    pub fn watchlist_screening_entity_review_create(
        &self,
        confirmed_hits: &[&str],
        dismissed_hits: &[&str],
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityReviewCreateRequest {
        request::WatchlistScreeningEntityReviewCreateRequest {
            client: &self,
            confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
            dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
            comment: None,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
        }
    }
    /**List reviews for entity watchlist screenings

List all reviews for a particular entity watchlist screening. Reviews are compliance reports created by users in your organization regarding the relevance of potential hits found by Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityreviewlist>.*/
    pub fn watchlist_screening_entity_review_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityReviewListRequest {
        request::WatchlistScreeningEntityReviewListRequest {
            client: &self,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            cursor: None,
        }
    }
    /**Update an entity screening

Update an entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityupdate>.*/
    pub fn watchlist_screening_entity_update(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityUpdateRequest {
        request::WatchlistScreeningEntityUpdateRequest {
            client: &self,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            search_terms: None,
            assignee: None,
            status: None,
            client_user_id: None,
            reset_fields: None,
        }
    }
    /**Create a watchlist screening for a person

Create a new Watchlist Screening to check your customer against watchlists defined in the associated Watchlist Program. If your associated program has ongoing screening enabled, this is the profile information that will be used to monitor your customer over time.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualcreate>.*/
    pub fn watchlist_screening_individual_create(
        &self,
        search_terms: WatchlistScreeningRequestSearchTerms,
    ) -> request::WatchlistScreeningIndividualCreateRequest {
        request::WatchlistScreeningIndividualCreateRequest {
            client: &self,
            search_terms,
            client_user_id: None,
        }
    }
    /**Retrieve an individual watchlist screening

Retrieve a previously created individual watchlist screening

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualget>.*/
    pub fn watchlist_screening_individual_get(
        &self,
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualGetRequest {
        request::WatchlistScreeningIndividualGetRequest {
            client: &self,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
        }
    }
    /**List history for individual watchlist screenings

List all changes to the individual watchlist screening in reverse-chronological order. If the watchlist screening has not been edited, no history will be returned.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualhistorylist>.*/
    pub fn watchlist_screening_individual_history_list(
        &self,
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualHistoryListRequest {
        request::WatchlistScreeningIndividualHistoryListRequest {
            client: &self,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
            cursor: None,
        }
    }
    /**List hits for individual watchlist screening

List all hits found by Plaid for a particular individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualhitlist>.*/
    pub fn watchlist_screening_individual_hit_list(
        &self,
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualHitListRequest {
        request::WatchlistScreeningIndividualHitListRequest {
            client: &self,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
            cursor: None,
        }
    }
    /**List Individual Watchlist Screenings

List previously created watchlist screenings for individuals

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividuallist>.*/
    pub fn watchlist_screening_individual_list(
        &self,
        watchlist_program_id: &str,
    ) -> request::WatchlistScreeningIndividualListRequest {
        request::WatchlistScreeningIndividualListRequest {
            client: &self,
            watchlist_program_id: watchlist_program_id.to_owned(),
            client_user_id: None,
            status: None,
            assignee: None,
            cursor: None,
        }
    }
    /**Get individual watchlist screening program

Get an individual watchlist screening program

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualprogramget>.*/
    pub fn watchlist_screening_individual_program_get(
        &self,
        watchlist_program_id: &str,
    ) -> request::WatchlistScreeningIndividualProgramGetRequest {
        request::WatchlistScreeningIndividualProgramGetRequest {
            client: &self,
            watchlist_program_id: watchlist_program_id.to_owned(),
        }
    }
    /**List individual watchlist screening programs

List all individual watchlist screening programs

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualprogramlist>.*/
    pub fn watchlist_screening_individual_program_list(
        &self,
    ) -> request::WatchlistScreeningIndividualProgramListRequest {
        request::WatchlistScreeningIndividualProgramListRequest {
            client: &self,
            cursor: None,
        }
    }
    /**Create a review for an individual watchlist screening

Create a review for the individual watchlist screening. Reviews are compliance reports created by users in your organization regarding the relevance of potential hits found by Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualreviewcreate>.*/
    pub fn watchlist_screening_individual_review_create(
        &self,
        confirmed_hits: &[&str],
        dismissed_hits: &[&str],
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualReviewCreateRequest {
        request::WatchlistScreeningIndividualReviewCreateRequest {
            client: &self,
            confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
            dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
            comment: None,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
        }
    }
    /**List reviews for individual watchlist screenings

List all reviews for the individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualreviewlist>.*/
    pub fn watchlist_screening_individual_reviews_list(
        &self,
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualReviewsListRequest {
        request::WatchlistScreeningIndividualReviewsListRequest {
            client: &self,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
            cursor: None,
        }
    }
    /**Update individual watchlist screening

Update a specific individual watchlist screening. This endpoint can be used to add additional customer information, correct outdated information, add a reference id, assign the individual to a reviewer, and update which program it is associated with. Please note that you may not update `search_terms` and `status` at the same time since editing `search_terms` may trigger an automatic `status` change.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualupdate>.*/
    pub fn watchlist_screening_individual_update(
        &self,
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualUpdateRequest {
        request::WatchlistScreeningIndividualUpdateRequest {
            client: &self,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
            search_terms: None,
            assignee: None,
            status: None,
            client_user_id: None,
            reset_fields: None,
        }
    }
    /**Retrieve Auth data

The `/processor/auth/get` endpoint returns the bank account and bank identification number (such as the routing number, for US accounts), for a checking or savings account that''s associated with a given `processor_token`. The endpoint also returns high-level account data and balances when available.

Versioning note: API versions 2019-05-29 and earlier use a different schema for the `numbers` object returned by this endpoint. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2020-09-14).


See endpoint docs at <https://plaid.com/docs/api/processors/#processorauthget>.*/
    pub fn processor_auth_get(
        &self,
        processor_token: &str,
    ) -> request::ProcessorAuthGetRequest {
        request::ProcessorAuthGetRequest {
            client: &self,
            processor_token: processor_token.to_owned(),
        }
    }
    /**Create a bank transfer as a processor

Use the `/processor/bank_transfer/create` endpoint to initiate a new bank transfer as a processor

See endpoint docs at <https://plaid.com/docs/api/processors/#bank_transfercreate>.*/
    pub fn processor_bank_transfer_create(
        &self,
        args: request::ProcessorBankTransferCreateRequired,
    ) -> request::ProcessorBankTransferCreateRequest {
        request::ProcessorBankTransferCreateRequest {
            client: &self,
            idempotency_key: args.idempotency_key.to_owned(),
            processor_token: args.processor_token.to_owned(),
            type_: args.type_.to_owned(),
            network: args.network.to_owned(),
            amount: args.amount.to_owned(),
            iso_currency_code: args.iso_currency_code.to_owned(),
            description: args.description.to_owned(),
            ach_class: None,
            user: args.user,
            custom_tag: None,
            metadata: None,
            origination_account_id: None,
        }
    }
    /**Retrieve Identity data

The `/processor/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses.

See endpoint docs at <https://plaid.com/docs/api/processors/#processoridentityget>.*/
    pub fn processor_identity_get(
        &self,
        processor_token: &str,
    ) -> request::ProcessorIdentityGetRequest {
        request::ProcessorIdentityGetRequest {
            client: &self,
            processor_token: processor_token.to_owned(),
        }
    }
    /**Retrieve Balance data

The `/processor/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/processor/balance/get` forces the available and current balance fields to be refreshed rather than cached.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorbalanceget>.*/
    pub fn processor_balance_get(
        &self,
        processor_token: &str,
    ) -> request::ProcessorBalanceGetRequest {
        request::ProcessorBalanceGetRequest {
            client: &self,
            processor_token: processor_token.to_owned(),
            options: None,
        }
    }
    /**Update Webhook URL

The POST `/item/webhook/update` allows you to update the webhook URL associated with an Item. This request triggers a [`WEBHOOK_UPDATE_ACKNOWLEDGED`](https://plaid.com/docs/api/items/#webhook_update_acknowledged) webhook to the newly specified webhook URL.

See endpoint docs at <https://plaid.com/docs/api/items/#itemwebhookupdate>.*/
    pub fn item_webhook_update(
        &self,
        access_token: &str,
    ) -> request::ItemWebhookUpdateRequest {
        request::ItemWebhookUpdateRequest {
            client: &self,
            access_token: access_token.to_owned(),
            webhook: None,
        }
    }
    /**Invalidate access_token

By default, the `access_token` associated with an Item does not expire and should be stored in a persistent, secure manner.

You can use the `/item/access_token/invalidate` endpoint to rotate the `access_token` associated with an Item. The endpoint returns a new `access_token` and immediately invalidates the previous `access_token`.


See endpoint docs at <https://plaid.com/docs/api/tokens/#itemaccess_tokeninvalidate>.*/
    pub fn item_access_token_invalidate(
        &self,
        access_token: &str,
    ) -> request::ItemAccessTokenInvalidateRequest {
        request::ItemAccessTokenInvalidateRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Get webhook verification key

Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the `Plaid-Verification` header.

The `/webhook_verification_key/get` endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.

See endpoint docs at <https://plaid.com/docs/api/webhooks/webhook-verification/#webhook_verification_keyget>.*/
    pub fn webhook_verification_key_get(
        &self,
        key_id: &str,
    ) -> request::WebhookVerificationKeyGetRequest {
        request::WebhookVerificationKeyGetRequest {
            client: &self,
            key_id: key_id.to_owned(),
        }
    }
    /**Retrieve Liabilities data

The `/liabilities/get` endpoint returns various details about an Item with loan or credit accounts. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`. To limit accounts listed in Link to types and subtypes supported by Liabilities, you can use the `account_filters` parameter when [creating a Link token](https://plaid.com/docs/api/tokens/#linktokencreate).

The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/liabilities/get`.

Note: This request may take some time to complete if `liabilities` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the additional data.

See endpoint docs at <https://plaid.com/docs/api/products/liabilities/#liabilitiesget>.*/
    pub fn liabilities_get(&self, access_token: &str) -> request::LiabilitiesGetRequest {
        request::LiabilitiesGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Create payment recipient

Create a payment recipient for payment initiation.  The recipient must be in Europe, within a country that is a member of the Single Euro Payment Area (SEPA).  For a standing order (recurring) payment, the recipient must be in the UK.

The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same `recipient_id`.


See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientcreate>.*/
    pub fn payment_initiation_recipient_create(
        &self,
        name: &str,
    ) -> request::PaymentInitiationRecipientCreateRequest {
        request::PaymentInitiationRecipientCreateRequest {
            client: &self,
            name: name.to_owned(),
            iban: None,
            bacs: None,
            address: None,
        }
    }
    /**Reverse an existing payment

Reverse a previously initiated payment.

A payment can only be reversed once and will be refunded to the original sender's account.


See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentreverse>.*/
    pub fn payment_initiation_payment_reverse(
        &self,
        payment_id: &str,
        idempotency_key: &str,
        reference: &str,
    ) -> request::PaymentInitiationPaymentReverseRequest {
        request::PaymentInitiationPaymentReverseRequest {
            client: &self,
            payment_id: payment_id.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
            reference: reference.to_owned(),
        }
    }
    /**Get payment recipient

Get details about a payment recipient you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientget>.*/
    pub fn payment_initiation_recipient_get(
        &self,
        recipient_id: &str,
    ) -> request::PaymentInitiationRecipientGetRequest {
        request::PaymentInitiationRecipientGetRequest {
            client: &self,
            recipient_id: recipient_id.to_owned(),
        }
    }
    /**List payment recipients

The `/payment_initiation/recipient/list` endpoint list the payment recipients that you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientlist>.*/
    pub fn payment_initiation_recipient_list(
        &self,
    ) -> request::PaymentInitiationRecipientListRequest {
        request::PaymentInitiationRecipientListRequest {
            client: &self,
        }
    }
    /**Create a payment

After creating a payment recipient, you can use the `/payment_initiation/payment/create` endpoint to create a payment to that recipient.  Payments can be one-time or standing order (recurring) and can be denominated in either EUR or GBP.  If making domestic GBP-denominated payments, your recipient must have been created with BACS numbers. In general, EUR-denominated payments will be sent via SEPA Credit Transfer and GBP-denominated payments will be sent via the Faster Payments network, but the payment network used will be determined by the institution. Payments sent via Faster Payments will typically arrive immediately, while payments sent via SEPA Credit Transfer will typically arrive in one business day.

Standing orders (recurring payments) must be denominated in GBP and can only be sent to recipients in the UK. Once created, standing order payments cannot be modified or canceled via the API. An end user can cancel or modify a standing order directly on their banking application or website, or by contacting the bank. Standing orders will follow the payment rules of the underlying rails (Faster Payments in UK). Payments can be sent Monday to Friday, excluding bank holidays. If the pre-arranged date falls on a weekend or bank holiday, the payment is made on the next working day. It is not possible to guarantee the exact time the payment will reach the recipient’s account, although at least 90% of standing order payments are sent by 6am.

In the Development environment, payments must be below 5 GBP / EUR. For details on any payment limits in Production, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentcreate>.*/
    pub fn payment_initiation_payment_create(
        &self,
        recipient_id: &str,
        reference: &str,
        amount: PaymentAmount,
    ) -> request::PaymentInitiationPaymentCreateRequest {
        request::PaymentInitiationPaymentCreateRequest {
            client: &self,
            recipient_id: recipient_id.to_owned(),
            reference: reference.to_owned(),
            amount,
            schedule: None,
            options: None,
        }
    }
    /**Create payment token

The `/payment_initiation/payment/token/create` endpoint has been deprecated. New Plaid customers will be unable to use this endpoint, and existing customers are encouraged to migrate to the newer, `link_token`-based flow. The recommended flow is to provide the `payment_id` to `/link/token/create`, which returns a `link_token` used to initialize Link.

The `/payment_initiation/payment/token/create` is used to create a `payment_token`, which can then be used in Link initialization to enter a payment initiation flow. You can only use a `payment_token` once. If this attempt fails, the end user aborts the flow, or the token expires, you will need to create a new payment token. Creating a new payment token does not require end user input.

See endpoint docs at <https://plaid.com/docs/link/maintain-legacy-integration/#creating-a-payment-token>.*/
    pub fn create_payment_token(
        &self,
        payment_id: &str,
    ) -> request::CreatePaymentTokenRequest {
        request::CreatePaymentTokenRequest {
            client: &self,
            payment_id: payment_id.to_owned(),
        }
    }
    /**Create payment consent

The `/payment_initiation/consent/create` endpoint is used to create a payment consent, which can be used to initiate payments on behalf of the user. Payment consents are created with `UNAUTHORISED` status by default and must be authorised by the user before payments can be initiated.

Consents can be limited in time and scope, and have constraints that describe limitations for payments.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentcreate>.*/
    pub fn payment_initiation_consent_create(
        &self,
        args: request::PaymentInitiationConsentCreateRequired,
    ) -> request::PaymentInitiationConsentCreateRequest {
        request::PaymentInitiationConsentCreateRequest {
            client: &self,
            recipient_id: args.recipient_id.to_owned(),
            reference: args.reference.to_owned(),
            scopes: args.scopes.iter().map(|&x| x.to_owned()).collect(),
            constraints: args.constraints,
            options: None,
        }
    }
    /**Get payment consent

The `/payment_initiation/consent/get` endpoint can be used to check the status of a payment consent, as well as to receive basic information such as recipient and constraints.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentget>.*/
    pub fn payment_initiation_consent_get(
        &self,
        consent_id: &str,
    ) -> request::PaymentInitiationConsentGetRequest {
        request::PaymentInitiationConsentGetRequest {
            client: &self,
            consent_id: consent_id.to_owned(),
        }
    }
    /**Revoke payment consent

The `/payment_initiation/consent/revoke` endpoint can be used to revoke the payment consent. Once the consent is revoked, it is not possible to initiate payments using it.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentrevoke>.*/
    pub fn payment_initiation_consent_revoke(
        &self,
        consent_id: &str,
    ) -> request::PaymentInitiationConsentRevokeRequest {
        request::PaymentInitiationConsentRevokeRequest {
            client: &self,
            consent_id: consent_id.to_owned(),
        }
    }
    /**Execute a single payment using consent

The `/payment_initiation/consent/payment/execute` endpoint can be used to execute payments using payment consent.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentpaymentexecute>.*/
    pub fn payment_initiation_consent_payment_execute(
        &self,
        consent_id: &str,
        amount: PaymentAmount,
        idempotency_key: &str,
    ) -> request::PaymentInitiationConsentPaymentExecuteRequest {
        request::PaymentInitiationConsentPaymentExecuteRequest {
            client: &self,
            consent_id: consent_id.to_owned(),
            amount,
            idempotency_key: idempotency_key.to_owned(),
        }
    }
    /**Force a Sandbox Item into an error state

`/sandbox/item/reset_login/` forces an Item into an `ITEM_LOGIN_REQUIRED` state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's [update mode](https://plaid.com/docs/link/update-mode) flow in the Sandbox environment.  After calling `/sandbox/item/reset_login`, You can then use Plaid Link update mode to restore the Item to a good state. An `ITEM_LOGIN_REQUIRED` webhook will also be fired after a call to this endpoint, if one is associated with the Item.


In the Sandbox, Items will transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days, even if this endpoint is not called.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemreset_login>.*/
    pub fn sandbox_item_reset_login(
        &self,
        access_token: &str,
    ) -> request::SandboxItemResetLoginRequest {
        request::SandboxItemResetLoginRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Set verification status for Sandbox account

The `/sandbox/item/set_verification_status` endpoint can be used to change the verification status of an Item in in the Sandbox in order to simulate the Automated Micro-deposit flow.

Note that not all Plaid developer accounts are enabled for micro-deposit based verification by default. Your account must be enabled for this feature in order to test it in Sandbox. To enable this features or check your status, contact your account manager or [submit a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access).

For more information on testing Automated Micro-deposits in Sandbox, see [Auth full coverage testing](https://plaid.com/docs/auth/coverage/testing#).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemset_verification_status>.*/
    pub fn sandbox_item_set_verification_status(
        &self,
        access_token: &str,
        account_id: &str,
        verification_status: &str,
    ) -> request::SandboxItemSetVerificationStatusRequest {
        request::SandboxItemSetVerificationStatusRequest {
            client: &self,
            access_token: access_token.to_owned(),
            account_id: account_id.to_owned(),
            verification_status: verification_status.to_owned(),
        }
    }
    /**Exchange public token for an access token

Exchange a Link `public_token` for an API `access_token`. Link hands off the `public_token` client-side via the `onSuccess` callback once a user has successfully created an Item. The `public_token` is ephemeral and expires after 30 minutes. An `access_token` does not expire, but can be revoked by calling `/item/remove`.

The response also includes an `item_id` that should be stored with the `access_token`. The `item_id` is used to identify an Item in a webhook. The `item_id` can also be retrieved by making an `/item/get` request.

See endpoint docs at <https://plaid.com/docs/api/tokens/#itempublic_tokenexchange>.*/
    pub fn item_public_token_exchange(
        &self,
        public_token: &str,
    ) -> request::ItemPublicTokenExchangeRequest {
        request::ItemPublicTokenExchangeRequest {
            client: &self,
            public_token: public_token.to_owned(),
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
        access_token: &str,
    ) -> request::ItemCreatePublicTokenRequest {
        request::ItemCreatePublicTokenRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Create user

This endpoint should be called for each of your end users before they begin a Plaid income flow. This provides you a single token to access all income data associated with the user. You should only create one per end user.

If you call the endpoint multiple times with the same `client_user_id`, the first creation call will succeed and the rest will fail with an error message indicating that the user has been created for the given `client_user_id`.

See endpoint docs at <https://plaid.com/docs/api/products/income/#usercreate>.*/
    pub fn user_create(&self, client_user_id: &str) -> request::UserCreateRequest {
        request::UserCreateRequest {
            client: &self,
            client_user_id: client_user_id.to_owned(),
        }
    }
    /**Get payment details

The `/payment_initiation/payment/get` endpoint can be used to check the status of a payment, as well as to receive basic information such as recipient and payment amount. In the case of standing orders, the `/payment_initiation/payment/get` endpoint will provide information about the status of the overall standing order itself; the API cannot be used to retrieve payment status for individual payments within a standing order.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentget>.*/
    pub fn payment_initiation_payment_get(
        &self,
        payment_id: &str,
    ) -> request::PaymentInitiationPaymentGetRequest {
        request::PaymentInitiationPaymentGetRequest {
            client: &self,
            payment_id: payment_id.to_owned(),
        }
    }
    /**List payments

The `/payment_initiation/payment/list` endpoint can be used to retrieve all created payments. By default, the 10 most recent payments are returned. You can request more payments and paginate through the results using the optional `count` and `cursor` parameters.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentlist>.*/
    pub fn payment_initiation_payment_list(
        &self,
    ) -> request::PaymentInitiationPaymentListRequest {
        request::PaymentInitiationPaymentListRequest {
            client: &self,
            count: None,
            cursor: None,
            consent_id: None,
        }
    }
    /**Create an Asset Report

The `/asset_report/create` endpoint initiates the process of creating an Asset Report, which can then be retrieved by passing the `asset_report_token` return value to the `/asset_report/get` or `/asset_report/pdf/get` endpoints.

The Asset Report takes some time to be created and is not available immediately after calling `/asset_report/create`. When the Asset Report is ready to be retrieved using `/asset_report/get` or `/asset_report/pdf/get`, Plaid will fire a `PRODUCT_READY` webhook. For full details of the webhook schema, see [Asset Report webhooks](https://plaid.com/docs/api/products/assets/#webhooks).

The `/asset_report/create` endpoint creates an Asset Report at a moment in time. Asset Reports are immutable. To get an updated Asset Report, use the `/asset_report/refresh` endpoint.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportcreate>.*/
    pub fn asset_report_create(
        &self,
        access_tokens: &[&str],
        days_requested: i64,
    ) -> request::AssetReportCreateRequest {
        request::AssetReportCreateRequest {
            client: &self,
            access_tokens: access_tokens.iter().map(|&x| x.to_owned()).collect(),
            days_requested,
            options: None,
        }
    }
    /**Refresh an Asset Report

An Asset Report is an immutable snapshot of a user's assets. In order to "refresh" an Asset Report you created previously, you can use the `/asset_report/refresh` endpoint to create a new Asset Report based on the old one, but with the most recent data available.

The new Asset Report will contain the same Items as the original Report, as well as the same filters applied by any call to `/asset_report/filter`. By default, the new Asset Report will also use the same parameters you submitted with your original `/asset_report/create` request, but the original `days_requested` value and the values of any parameters in the `options` object can be overridden with new values. To change these arguments, simply supply new values for them in your request to `/asset_report/refresh`. Submit an empty string ("") for any previously-populated fields you would like set as empty.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportrefresh>.*/
    pub fn asset_report_refresh(
        &self,
        asset_report_token: &str,
    ) -> request::AssetReportRefreshRequest {
        request::AssetReportRefreshRequest {
            client: &self,
            asset_report_token: asset_report_token.to_owned(),
            days_requested: None,
            options: None,
        }
    }
    /**Refresh a Relay Token's Asset Report

The `/asset_report/relay/refresh` endpoint allows third parties to refresh an Asset Report that was relayed to them, using an `asset_relay_token` that was created by the report owner. A new Asset Report will be created based on the old one, but with the most recent data available.

See endpoint docs at <https://plaid.com/docs/api/products/#asset_reportrelayrefresh>.*/
    pub fn asset_report_relay_refresh(
        &self,
        asset_relay_token: &str,
    ) -> request::AssetReportRelayRefreshRequest {
        request::AssetReportRelayRefreshRequest {
            client: &self,
            asset_relay_token: asset_relay_token.to_owned(),
            webhook: None,
        }
    }
    /**Delete an Asset Report

The `/item/remove` endpoint allows you to invalidate an `access_token`, meaning you will not be able to create new Asset Reports with it. Removing an Item does not affect any Asset Reports or Audit Copies you have already created, which will remain accessible until you remove them specifically.

The `/asset_report/remove` endpoint allows you to remove an Asset Report. Removing an Asset Report invalidates its `asset_report_token`, meaning you will no longer be able to use it to access Report data or create new Audit Copies. Removing an Asset Report does not affect the underlying Items, but does invalidate any `audit_copy_tokens` associated with the Asset Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportremove>.*/
    pub fn asset_report_remove(
        &self,
        asset_report_token: &str,
    ) -> request::AssetReportRemoveRequest {
        request::AssetReportRemoveRequest {
            client: &self,
            asset_report_token: asset_report_token.to_owned(),
        }
    }
    /**Filter Asset Report

By default, an Asset Report will contain all of the accounts on a given Item. In some cases, you may not want the Asset Report to contain all accounts. For example, you might have the end user choose which accounts are relevant in Link using the Account Select view, which you can enable in the dashboard. Or, you might always exclude certain account types or subtypes, which you can identify by using the `/accounts/get` endpoint. To narrow an Asset Report to only a subset of accounts, use the `/asset_report/filter` endpoint.

To exclude certain Accounts from an Asset Report, first use the `/asset_report/create` endpoint to create the report, then send the `asset_report_token` along with a list of `account_ids` to exclude to the `/asset_report/filter` endpoint, to create a new Asset Report which contains only a subset of the original Asset Report's data.

Because Asset Reports are immutable, calling `/asset_report/filter` does not alter the original Asset Report in any way; rather, `/asset_report/filter` creates a new Asset Report with a new token and id. Asset Reports created via `/asset_report/filter` do not contain new Asset data, and are not billed.

Plaid will fire a [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook once generation of the filtered Asset Report has completed.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportfilter>.*/
    pub fn asset_report_filter(
        &self,
        asset_report_token: &str,
        account_ids_to_exclude: &[&str],
    ) -> request::AssetReportFilterRequest {
        request::AssetReportFilterRequest {
            client: &self,
            asset_report_token: asset_report_token.to_owned(),
            account_ids_to_exclude: account_ids_to_exclude
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
        }
    }
    /**Retrieve an Asset Report

The `/asset_report/get` endpoint retrieves the Asset Report in JSON format. Before calling `/asset_report/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

By default, an Asset Report includes transaction descriptions as returned by the bank, as opposed to parsed and categorized by Plaid. You can also receive cleaned and categorized transactions, as well as additional insights like merchant name or location information. We call this an Asset Report with Insights. An Asset Report with Insights provides transaction category, location, and merchant information in addition to the transaction strings provided in a standard Asset Report.

To retrieve an Asset Report with Insights, call the `/asset_report/get` endpoint with `include_insights` set to `true`.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportget>.*/
    pub fn asset_report_get(
        &self,
        asset_report_token: &str,
    ) -> request::AssetReportGetRequest {
        request::AssetReportGetRequest {
            client: &self,
            asset_report_token: asset_report_token.to_owned(),
            include_insights: None,
            fast_report: None,
        }
    }
    /**Create Asset Report Audit Copy

Plaid can provide an Audit Copy of any Asset Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy contains the same underlying data as the Asset Report.

To grant access to an Audit Copy, use the `/asset_report/audit_copy/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copycreate>.*/
    pub fn asset_report_audit_copy_create(
        &self,
        asset_report_token: &str,
        auditor_id: &str,
    ) -> request::AssetReportAuditCopyCreateRequest {
        request::AssetReportAuditCopyCreateRequest {
            client: &self,
            asset_report_token: asset_report_token.to_owned(),
            auditor_id: auditor_id.to_owned(),
        }
    }
    /**Remove Asset Report Audit Copy

The `/asset_report/audit_copy/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Asset Report, the Asset Report itself and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copyremove>.*/
    pub fn asset_report_audit_copy_remove(
        &self,
        audit_copy_token: &str,
    ) -> request::AssetReportAuditCopyRemoveRequest {
        request::AssetReportAuditCopyRemoveRequest {
            client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Create an `asset_relay_token` to share an Asset Report with a partner client

Plaid can share an Asset Report directly with a participating third party on your behalf. The shared Asset Report is the exact same Asset Report originally created in `/asset_report/create`.

To grant access to an Asset Report to a third party, use the `/asset_report/relay/create` endpoint to create an `asset_relay_token` and then pass that token to the third party who needs access. Each third party has its own `secondary_client_id`, for example `ce5bd328dcd34123456`. You'll need to create a separate `asset_relay_token` for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_relay_create(
        &self,
        asset_report_token: &str,
        secondary_client_id: &str,
    ) -> request::AssetReportRelayCreateRequest {
        request::AssetReportRelayCreateRequest {
            client: &self,
            asset_report_token: asset_report_token.to_owned(),
            secondary_client_id: secondary_client_id.to_owned(),
            webhook: None,
        }
    }
    /**Retrieve an Asset Report that was shared with you

`/asset_report/relay/get` allows third parties to get an Asset Report that was shared with them, using an `asset_relay_token` that was created by the report owner.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_relay_get(
        &self,
        asset_relay_token: &str,
    ) -> request::AssetReportRelayGetRequest {
        request::AssetReportRelayGetRequest {
            client: &self,
            asset_relay_token: asset_relay_token.to_owned(),
        }
    }
    /**Remove Asset Report Relay Token

The `/asset_report/relay/remove` endpoint allows you to invalidate an `asset_relay_token`, meaning the third party holding the token will no longer be able to use it to access the Asset Report to which the `asset_relay_token` gives access to. The Asset Report, Items associated with it, and other Asset Relay Tokens that provide access to the same Asset Report are not affected and will remain accessible after removing the given `asset_relay_token.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_relay_remove(
        &self,
        asset_relay_token: &str,
    ) -> request::AssetReportRelayRemoveRequest {
        request::AssetReportRelayRemoveRequest {
            client: &self,
            asset_relay_token: asset_relay_token.to_owned(),
        }
    }
    /**Get Investment holdings

The `/investments/holdings/get` endpoint allows developers to receive user-authorized stock position data for `investment`-type accounts.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentsholdingsget>.*/
    pub fn investments_holdings_get(
        &self,
        access_token: &str,
    ) -> request::InvestmentsHoldingsGetRequest {
        request::InvestmentsHoldingsGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Get investment transactions

The `/investments/transactions/get` endpoint allows developers to retrieve up to 24 months of user-authorized transaction data for investment accounts.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.

Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the `total_investment_transactions` response body field to fetch all available investment transactions.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentstransactionsget>.*/
    pub fn investments_transactions_get(
        &self,
        access_token: &str,
        start_date: &str,
        end_date: &str,
    ) -> request::InvestmentsTransactionsGetRequest {
        request::InvestmentsTransactionsGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
            start_date: start_date.to_owned(),
            end_date: end_date.to_owned(),
            options: None,
        }
    }
    /**Create processor token

Used to create a token suitable for sending to one of Plaid's partners to enable integrations. Note that Stripe partnerships use bank account tokens instead; see `/processor/stripe/bank_account_token/create` for creating tokens for use with Stripe integrations. Processor tokens can also be revoked, using `/item/remove`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokencreate>.*/
    pub fn processor_token_create(
        &self,
        access_token: &str,
        account_id: &str,
        processor: &str,
    ) -> request::ProcessorTokenCreateRequest {
        request::ProcessorTokenCreateRequest {
            client: &self,
            access_token: access_token.to_owned(),
            account_id: account_id.to_owned(),
            processor: processor.to_owned(),
        }
    }
    /**Create Stripe bank account token

Used to create a token suitable for sending to Stripe to enable Plaid-Stripe integrations. For a detailed guide on integrating Stripe, see [Add Stripe to your app](https://plaid.com/docs/auth/partnerships/stripe/). Bank account tokens can also be revoked, using `/item/remove`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorstripebank_account_tokencreate>.*/
    pub fn processor_stripe_bank_account_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> request::ProcessorStripeBankAccountTokenCreateRequest {
        request::ProcessorStripeBankAccountTokenCreateRequest {
            client: &self,
            access_token: access_token.to_owned(),
            account_id: account_id.to_owned(),
        }
    }
    /**Create Apex bank account token

Used to create a token suitable for sending to Apex to enable Plaid-Apex integrations.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn processor_apex_processor_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> request::ProcessorApexProcessorTokenCreateRequest {
        request::ProcessorApexProcessorTokenCreateRequest {
            client: &self,
            access_token: access_token.to_owned(),
            account_id: account_id.to_owned(),
        }
    }
    /**Create a deposit switch

This endpoint creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchcreate>.*/
    pub fn deposit_switch_create(
        &self,
        target_access_token: &str,
        target_account_id: &str,
    ) -> request::DepositSwitchCreateRequest {
        request::DepositSwitchCreateRequest {
            client: &self,
            target_access_token: target_access_token.to_owned(),
            target_account_id: target_account_id.to_owned(),
            country_code: None,
            options: None,
        }
    }
    /**Import Item

`/item/import` creates an Item via your Plaid Exchange Integration and returns an `access_token`. As part of an `/item/import` request, you will include a User ID (`user_auth.user_id`) and Authentication Token (`user_auth.auth_token`) that enable data aggregation through your Plaid Exchange API endpoints. These authentication principals are to be chosen by you.

Upon creating an Item via `/item/import`, Plaid will automatically begin an extraction of that Item through the Plaid Exchange infrastructure you have already integrated. This will automatically generate the Plaid native account ID for the account the user will switch their direct deposit to (`target_account_id`).*/
    pub fn item_import(
        &self,
        products: &[&str],
        user_auth: ItemImportRequestUserAuth,
    ) -> request::ItemImportRequest {
        request::ItemImportRequest {
            client: &self,
            products: products.iter().map(|&x| x.to_owned()).collect(),
            user_auth,
            options: None,
        }
    }
    /**Create a deposit switch token

In order for the end user to take action, you will need to create a public token representing the deposit switch. This token is used to initialize Link. It can be used one time and expires after 30 minutes.


See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchtokencreate>.*/
    pub fn deposit_switch_token_create(
        &self,
        deposit_switch_id: &str,
    ) -> request::DepositSwitchTokenCreateRequest {
        request::DepositSwitchTokenCreateRequest {
            client: &self,
            deposit_switch_id: deposit_switch_id.to_owned(),
        }
    }
    /**Create Link Token

The `/link/token/create` endpoint creates a `link_token`, which is required as a parameter when initializing Link. Once Link has been initialized, it returns a `public_token`, which can then be exchanged for an `access_token` via `/item/public_token/exchange` as part of the main Link flow.

A `link_token` generated by `/link/token/create` is also used to initialize other Link flows, such as the update mode flow for tokens with expired credentials, or the Payment Initiation (Europe) flow.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokencreate>.*/
    pub fn link_token_create(
        &self,
        args: request::LinkTokenCreateRequired,
    ) -> request::LinkTokenCreateRequest {
        request::LinkTokenCreateRequest {
            client: &self,
            client_name: args.client_name.to_owned(),
            language: args.language.to_owned(),
            country_codes: args.country_codes.iter().map(|&x| x.to_owned()).collect(),
            user: args.user,
            products: None,
            additional_consented_products: None,
            webhook: None,
            access_token: None,
            link_customization_name: None,
            redirect_uri: None,
            android_package_name: None,
            institution_data: None,
            account_filters: None,
            eu_config: None,
            institution_id: None,
            payment_initiation: None,
            deposit_switch: None,
            income_verification: None,
            auth: None,
            transfer: None,
            update: None,
            identity_verification: None,
            user_token: None,
        }
    }
    /**Get Link Token

The `/link/token/get` endpoint gets information about a previously-created `link_token` using the
`/link/token/create` endpoint. It can be useful for debugging purposes.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokenget>.*/
    pub fn link_token_get(&self, link_token: &str) -> request::LinkTokenGetRequest {
        request::LinkTokenGetRequest {
            client: &self,
            link_token: link_token.to_owned(),
        }
    }
    /**Retrieve an Asset Report Audit Copy

`/asset_report/audit_copy/get` allows auditors to get a copy of an Asset Report that was previously shared via the `/asset_report/audit_copy/create` endpoint.  The caller of `/asset_report/audit_copy/create` must provide the `audit_copy_token` to the auditor.  This token can then be used to call `/asset_report/audit_copy/create`.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_audit_copy_get(
        &self,
        audit_copy_token: &str,
    ) -> request::AssetReportAuditCopyGetRequest {
        request::AssetReportAuditCopyGetRequest {
            client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Retrieve a deposit switch

This endpoint returns information related to how the user has configured their payroll allocation and the state of the switch. You can use this information to build logic related to the user's direct deposit allocation preferences.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchget>.*/
    pub fn deposit_switch_get(
        &self,
        deposit_switch_id: &str,
    ) -> request::DepositSwitchGetRequest {
        request::DepositSwitchGetRequest {
            client: &self,
            deposit_switch_id: deposit_switch_id.to_owned(),
        }
    }
    /**Retrieve a transfer

The `/transfer/get` fetches information about the transfer corresponding to the given `transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferget>.*/
    pub fn transfer_get(&self, transfer_id: &str) -> request::TransferGetRequest {
        request::TransferGetRequest {
            client: &self,
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**Retrieve a bank transfer

The `/bank_transfer/get` fetches information about the bank transfer corresponding to the given `bank_transfer_id`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferget>.*/
    pub fn bank_transfer_get(
        &self,
        bank_transfer_id: &str,
    ) -> request::BankTransferGetRequest {
        request::BankTransferGetRequest {
            client: &self,
            bank_transfer_id: bank_transfer_id.to_owned(),
        }
    }
    /**Create a transfer authorization

Use the `/transfer/authorization/create` endpoint to determine transfer failure risk.

In Plaid's sandbox environment the decisions will be returned as follows:

  - To approve a transfer with null rationale code, make an authorization request with an `amount` less than the available balance in the account.

  - To approve a transfer with the rationale code `MANUALLY_VERIFIED_ITEM`, create an Item in Link through the [Same Day Micro-deposits flow](https://plaid.com/docs/auth/coverage/testing/#testing-same-day-micro-deposits).

  - To approve a transfer with the rationale code `LOGIN_REQUIRED`, [reset the login for an Item](https://plaid.com/docs/sandbox/#item_login_required).

  - To decline a transfer with the rationale code `NSF`, the available balance on the account must be less than the authorization `amount`. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

  - To decline a transfer with the rationale code `RISK`, the available balance on the account must be exactly $0. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

For guaranteed ACH customers, the following fields are required : `user.phone_number` (optional if `email_address` provided), `user.email_address` (optional if `phone_number` provided), `device.ip_address`, `device.user_agent`, and `user_present`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferauthorizationcreate>.*/
    pub fn transfer_authorization_create(
        &self,
        args: request::TransferAuthorizationCreateRequired,
    ) -> request::TransferAuthorizationCreateRequest {
        request::TransferAuthorizationCreateRequest {
            client: &self,
            access_token: None,
            account_id: None,
            type_: args.type_.to_owned(),
            network: args.network.to_owned(),
            amount: args.amount.to_owned(),
            ach_class: args.ach_class.to_owned(),
            user: args.user,
            device: None,
            origination_account_id: None,
            iso_currency_code: None,
            user_present: None,
            payment_profile_id: None,
        }
    }
    /**Create a transfer

Use the `/transfer/create` endpoint to initiate a new transfer.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercreate>.*/
    pub fn transfer_create(
        &self,
        args: request::TransferCreateRequired,
    ) -> request::TransferCreateRequest {
        request::TransferCreateRequest {
            client: &self,
            idempotency_key: None,
            access_token: None,
            account_id: None,
            authorization_id: args.authorization_id.to_owned(),
            type_: args.type_.to_owned(),
            network: args.network.to_owned(),
            amount: args.amount.to_owned(),
            description: args.description.to_owned(),
            ach_class: args.ach_class.to_owned(),
            user: args.user,
            metadata: None,
            origination_account_id: None,
            iso_currency_code: None,
            payment_profile_id: None,
        }
    }
    /**Create a bank transfer

Use the `/bank_transfer/create` endpoint to initiate a new bank transfer.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercreate>.*/
    pub fn bank_transfer_create(
        &self,
        args: request::BankTransferCreateRequired,
    ) -> request::BankTransferCreateRequest {
        request::BankTransferCreateRequest {
            client: &self,
            idempotency_key: args.idempotency_key.to_owned(),
            access_token: args.access_token.to_owned(),
            account_id: args.account_id.to_owned(),
            type_: args.type_.to_owned(),
            network: args.network.to_owned(),
            amount: args.amount.to_owned(),
            iso_currency_code: args.iso_currency_code.to_owned(),
            description: args.description.to_owned(),
            ach_class: None,
            user: args.user,
            custom_tag: None,
            metadata: None,
            origination_account_id: None,
        }
    }
    /**List transfers

Use the `/transfer/list` endpoint to see a list of all your transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired transfers.


See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferlist>.*/
    pub fn transfer_list(&self) -> request::TransferListRequest {
        request::TransferListRequest {
            client: &self,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
            origination_account_id: None,
        }
    }
    /**List bank transfers

Use the `/bank_transfer/list` endpoint to see a list of all your bank transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired bank transfers.


See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferlist>.*/
    pub fn bank_transfer_list(&self) -> request::BankTransferListRequest {
        request::BankTransferListRequest {
            client: &self,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
            origination_account_id: None,
            direction: None,
        }
    }
    /**Cancel a transfer

Use the `/transfer/cancel` endpoint to cancel a transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercancel>.*/
    pub fn transfer_cancel(&self, transfer_id: &str) -> request::TransferCancelRequest {
        request::TransferCancelRequest {
            client: &self,
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**Cancel a bank transfer

Use the `/bank_transfer/cancel` endpoint to cancel a bank transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/bank_transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercancel>.*/
    pub fn bank_transfer_cancel(
        &self,
        bank_transfer_id: &str,
    ) -> request::BankTransferCancelRequest {
        request::BankTransferCancelRequest {
            client: &self,
            bank_transfer_id: bank_transfer_id.to_owned(),
        }
    }
    /**List transfer events

Use the `/transfer/event/list` endpoint to get a list of transfer events based on specified filter criteria.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfereventlist>.*/
    pub fn transfer_event_list(&self) -> request::TransferEventListRequest {
        request::TransferEventListRequest {
            client: &self,
            start_date: None,
            end_date: None,
            transfer_id: None,
            account_id: None,
            transfer_type: None,
            event_types: None,
            sweep_id: None,
            count: None,
            offset: None,
            origination_account_id: None,
        }
    }
    /**List bank transfer events

Use the `/bank_transfer/event/list` endpoint to get a list of bank transfer events based on specified filter criteria.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfereventlist>.*/
    pub fn bank_transfer_event_list(&self) -> request::BankTransferEventListRequest {
        request::BankTransferEventListRequest {
            client: &self,
            start_date: None,
            end_date: None,
            bank_transfer_id: None,
            account_id: None,
            bank_transfer_type: None,
            event_types: None,
            count: None,
            offset: None,
            origination_account_id: None,
            direction: None,
        }
    }
    /**Sync transfer events

`/transfer/event/sync` allows you to request up to the next 25 transfer events that happened after a specific `event_id`. Use the `/transfer/event/sync` endpoint to guarantee you have seen all transfer events.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfereventsync>.*/
    pub fn transfer_event_sync(
        &self,
        after_id: i64,
    ) -> request::TransferEventSyncRequest {
        request::TransferEventSyncRequest {
            client: &self,
            after_id,
            count: None,
        }
    }
    /**Sync bank transfer events

`/bank_transfer/event/sync` allows you to request up to the next 25 bank transfer events that happened after a specific `event_id`. Use the `/bank_transfer/event/sync` endpoint to guarantee you have seen all bank transfer events.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfereventsync>.*/
    pub fn bank_transfer_event_sync(
        &self,
        after_id: i64,
    ) -> request::BankTransferEventSyncRequest {
        request::BankTransferEventSyncRequest {
            client: &self,
            after_id,
            count: None,
        }
    }
    /**Retrieve a sweep

The `/transfer/sweep/get` endpoint fetches a sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfersweepget>.*/
    pub fn transfer_sweep_get(
        &self,
        sweep_id: &str,
    ) -> request::TransferSweepGetRequest {
        request::TransferSweepGetRequest {
            client: &self,
            sweep_id: sweep_id.to_owned(),
        }
    }
    /**Retrieve a sweep

The `/bank_transfer/sweep/get` endpoint fetches information about the sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweepget>.*/
    pub fn bank_transfer_sweep_get(
        &self,
        sweep_id: &str,
    ) -> request::BankTransferSweepGetRequest {
        request::BankTransferSweepGetRequest {
            client: &self,
            sweep_id: sweep_id.to_owned(),
        }
    }
    /**List sweeps

The `/transfer/sweep/list` endpoint fetches sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfersweeplist>.*/
    pub fn transfer_sweep_list(&self) -> request::TransferSweepListRequest {
        request::TransferSweepListRequest {
            client: &self,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
        }
    }
    /**List sweeps

The `/bank_transfer/sweep/list` endpoint fetches information about the sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweeplist>.*/
    pub fn bank_transfer_sweep_list(&self) -> request::BankTransferSweepListRequest {
        request::BankTransferSweepListRequest {
            client: &self,
            origination_account_id: None,
            start_time: None,
            end_time: None,
            count: None,
        }
    }
    /**Get balance of your Bank Transfer account

Use the `/bank_transfer/balance/get` endpoint to see the available balance in your bank transfer account. Debit transfers increase this balance once their status is posted. Credit transfers decrease this balance when they are created.

The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.

Note that this endpoint can only be used with FBO accounts, when using Bank Transfers in the Full Service configuration. It cannot be used on your own account when using Bank Transfers in the BTS Platform configuration.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferbalanceget>.*/
    pub fn bank_transfer_balance_get(&self) -> request::BankTransferBalanceGetRequest {
        request::BankTransferBalanceGetRequest {
            client: &self,
            origination_account_id: None,
        }
    }
    /**Migrate account into Bank Transfers

As an alternative to adding Items via Link, you can also use the `/bank_transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items.  Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Bank Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/bank_transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfermigrate_account>.*/
    pub fn bank_transfer_migrate_account(
        &self,
        account_number: &str,
        routing_number: &str,
        account_type: &str,
    ) -> request::BankTransferMigrateAccountRequest {
        request::BankTransferMigrateAccountRequest {
            client: &self,
            account_number: account_number.to_owned(),
            routing_number: routing_number.to_owned(),
            wire_routing_number: None,
            account_type: account_type.to_owned(),
        }
    }
    /**Migrate account into Transfers

As an alternative to adding Items via Link, you can also use the `/transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items.  Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfermigrate_account>.*/
    pub fn transfer_migrate_account(
        &self,
        account_number: &str,
        routing_number: &str,
        account_type: &str,
    ) -> request::TransferMigrateAccountRequest {
        request::TransferMigrateAccountRequest {
            client: &self,
            account_number: account_number.to_owned(),
            routing_number: routing_number.to_owned(),
            wire_routing_number: None,
            account_type: account_type.to_owned(),
        }
    }
    /**Create a transfer intent object to invoke the Transfer UI

Use the `/transfer/intent/create` endpoint to generate a transfer intent object and invoke the Transfer UI.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferintentcreate>.*/
    pub fn transfer_intent_create(
        &self,
        args: request::TransferIntentCreateRequired,
    ) -> request::TransferIntentCreateRequest {
        request::TransferIntentCreateRequest {
            client: &self,
            account_id: None,
            mode: args.mode.to_owned(),
            amount: args.amount.to_owned(),
            description: args.description.to_owned(),
            ach_class: args.ach_class.to_owned(),
            origination_account_id: None,
            user: args.user,
            metadata: None,
            iso_currency_code: None,
            require_guarantee: None,
        }
    }
    /**Retrieve more information about a transfer intent

Use the `/transfer/intent/get` endpoint to retrieve more information about a transfer intent.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferintentget>.*/
    pub fn transfer_intent_get(
        &self,
        transfer_intent_id: &str,
    ) -> request::TransferIntentGetRequest {
        request::TransferIntentGetRequest {
            client: &self,
            transfer_intent_id: transfer_intent_id.to_owned(),
        }
    }
    /**Lists historical repayments

The `/transfer/repayment/list` endpoint fetches repayments matching the given filters. Repayments are returned in reverse-chronological order (most recent first) starting at the given `start_time`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentlist>.*/
    pub fn transfer_repayment_list(&self) -> request::TransferRepaymentListRequest {
        request::TransferRepaymentListRequest {
            client: &self,
            start_date: None,
            end_date: None,
            count: None,
            offset: None,
        }
    }
    /**List the returns included in a repayment

The `/transfer/repayment/return/list` endpoint retrieves the set of returns that were batched together into the specified repayment. The sum of amounts of returns retrieved by this request equals the amount of the repayment.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentreturnlist>.*/
    pub fn transfer_repayment_return_list(
        &self,
        repayment_id: &str,
    ) -> request::TransferRepaymentReturnListRequest {
        request::TransferRepaymentReturnListRequest {
            client: &self,
            repayment_id: repayment_id.to_owned(),
            count: None,
            offset: None,
        }
    }
    /**Simulate a bank transfer event in Sandbox

Use the `/sandbox/bank_transfer/simulate` endpoint to simulate a bank transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/bank_transfer/event/sync` or `/bank_transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transfersimulate>.*/
    pub fn sandbox_bank_transfer_simulate(
        &self,
        bank_transfer_id: &str,
        event_type: &str,
    ) -> request::SandboxBankTransferSimulateRequest {
        request::SandboxBankTransferSimulateRequest {
            client: &self,
            bank_transfer_id: bank_transfer_id.to_owned(),
            event_type: event_type.to_owned(),
            failure_reason: None,
        }
    }
    /**Simulate creating a sweep

Use the `/sandbox/transfer/sweep/simulate` endpoint to create a sweep and associated events in the Sandbox environment. Upon calling this endpoint, all `posted` or `pending` transfers with a sweep status of `unswept` will become `swept`, and all `returned` transfers with a sweep status of `swept` will become `return_swept`.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersweepsimulate>.*/
    pub fn sandbox_transfer_sweep_simulate(
        &self,
    ) -> request::SandboxTransferSweepSimulateRequest {
        request::SandboxTransferSweepSimulateRequest {
            client: &self,
        }
    }
    /**Simulate a transfer event in Sandbox

Use the `/sandbox/transfer/simulate` endpoint to simulate a transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersimulate>.*/
    pub fn sandbox_transfer_simulate(
        &self,
        transfer_id: &str,
        event_type: &str,
    ) -> request::SandboxTransferSimulateRequest {
        request::SandboxTransferSimulateRequest {
            client: &self,
            transfer_id: transfer_id.to_owned(),
            event_type: event_type.to_owned(),
            failure_reason: None,
        }
    }
    /**Trigger the creation of a repayment

Use the `/sandbox/transfer/repayment/simulate` endpoint to trigger the creation of a repayment. As a side effect of calling this route, a repayment is created that includes all unreimbursed returns of guaranteed transfers. If there are no such returns, an 400 error is returned.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrepaymentsimulate>.*/
    pub fn sandbox_transfer_repayment_simulate(
        &self,
    ) -> request::SandboxTransferRepaymentSimulateRequest {
        request::SandboxTransferRepaymentSimulateRequest {
            client: &self,
        }
    }
    /**Manually fire a Transfer webhook

Use the `/sandbox/transfer/fire_webhook` endpoint to manually trigger a Transfer webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferfire_webhook>.*/
    pub fn sandbox_transfer_fire_webhook(
        &self,
        webhook: &str,
    ) -> request::SandboxTransferFireWebhookRequest {
        request::SandboxTransferFireWebhookRequest {
            client: &self,
            webhook: webhook.to_owned(),
        }
    }
    /**Search employer database

`/employers/search` allows you the ability to search Plaid’s database of known employers, for use with Deposit Switch. You can use this endpoint to look up a user's employer in order to confirm that they are supported. Users with non-supported employers can then be routed out of the Deposit Switch flow.

The data in the employer database is currently limited. As the Deposit Switch and Income products progress through their respective beta periods, more employers are being regularly added. Because the employer database is frequently updated, we recommend that you do not cache or store data from this endpoint for more than a day.

See endpoint docs at <https://plaid.com/docs/api/employers/#employerssearch>.*/
    pub fn employers_search(
        &self,
        query: &str,
        products: &[&str],
    ) -> request::EmployersSearchRequest {
        request::EmployersSearchRequest {
            client: &self,
            query: query.to_owned(),
            products: products.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**(Deprecated) Create an income verification instance

`/income/verification/create` begins the income verification process by returning an `income_verification_id`. You can then provide the `income_verification_id` to `/link/token/create` under the `income_verification` parameter in order to create a Link instance that will prompt the user to go through the income verification flow. Plaid will fire an `INCOME` webhook once the user completes the Payroll Income flow, or when the uploaded documents in the Document Income flow have finished processing.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationcreate>.*/
    pub fn income_verification_create(
        &self,
        webhook: &str,
    ) -> request::IncomeVerificationCreateRequest {
        request::IncomeVerificationCreateRequest {
            client: &self,
            webhook: webhook.to_owned(),
            precheck_id: None,
            options: None,
        }
    }
    /**(Deprecated) Retrieve information from the paystubs used for income verification

`/income/verification/paystubs/get` returns the information collected from the paystubs that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationpaystubsget>.*/
    pub fn income_verification_paystubs_get(
        &self,
    ) -> request::IncomeVerificationPaystubsGetRequest {
        request::IncomeVerificationPaystubsGetRequest {
            client: &self,
            income_verification_id: None,
            access_token: None,
        }
    }
    /**(Deprecated) Refresh an income verification

`/income/verification/refresh` refreshes a given income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationrefresh>.*/
    pub fn income_verification_refresh(
        &self,
    ) -> request::IncomeVerificationRefreshRequest {
        request::IncomeVerificationRefreshRequest {
            client: &self,
            income_verification_id: None,
            access_token: None,
        }
    }
    /**(Deprecated) Retrieve information from the tax documents used for income verification

`/income/verification/taxforms/get` returns the information collected from forms that were used to verify an end user''s income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationtaxformsget>.*/
    pub fn income_verification_taxforms_get(
        &self,
    ) -> request::IncomeVerificationTaxformsGetRequest {
        request::IncomeVerificationTaxformsGetRequest {
            client: &self,
            income_verification_id: None,
            access_token: None,
        }
    }
    /**(Deprecated) Check digital income verification eligibility and optimize conversion

`/income/verification/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification and returns a `precheck_id` that can be provided to `/link/token/create`. If the user is eligible for digital verification, providing the `precheck_id` in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the `precheck_id` can still be provided to `/link/token/create` and the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing either `employer` or `transactions_access_tokens` data will increase the chance of receiving a useful result.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/precheck` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationprecheck>.*/
    pub fn income_verification_precheck(
        &self,
    ) -> request::IncomeVerificationPrecheckRequest {
        request::IncomeVerificationPrecheckRequest {
            client: &self,
            user: None,
            employer: None,
            transactions_access_token: None,
            transactions_access_tokens: None,
            us_military_info: None,
        }
    }
    /**(Deprecated) Retrieve a summary of an individual's employment information

`/employment/verification/get` returns a list of employments through a user payroll that was verified by an end user.

This endpoint has been deprecated; new integrations should use `/credit/employment/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#employmentverificationget>.*/
    pub fn employment_verification_get(
        &self,
        access_token: &str,
    ) -> request::EmploymentVerificationGetRequest {
        request::EmploymentVerificationGetRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Create a deposit switch without using Plaid Exchange

This endpoint provides an alternative to `/deposit_switch/create` for customers who have not yet fully integrated with Plaid Exchange. Like `/deposit_switch/create`, it creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchaltcreate>.*/
    pub fn deposit_switch_alt_create(
        &self,
        target_account: DepositSwitchTargetAccount,
        target_user: DepositSwitchTargetUser,
    ) -> request::DepositSwitchAltCreateRequest {
        request::DepositSwitchAltCreateRequest {
            client: &self,
            target_account,
            target_user,
            options: None,
            country_code: None,
        }
    }
    /**Create Asset or Income Report Audit Copy Token

Plaid can provide an Audit Copy token of an Asset Report and/or Income Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy token directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy token contains the same underlying data as the Asset Report and/or Income Report (result of /credit/payroll_income/get).

To grant access to an Audit Copy token, use the `/credit/audit_copy_token/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditaudit_copy_tokencreate>.*/
    pub fn credit_audit_copy_token_create(
        &self,
        report_tokens: Vec<ReportToken>,
        auditor_id: &str,
    ) -> request::CreditAuditCopyTokenCreateRequest {
        request::CreditAuditCopyTokenCreateRequest {
            client: &self,
            report_tokens,
            auditor_id: auditor_id.to_owned(),
        }
    }
    /**Remove an Audit Copy token

The `/credit/audit_copy_token/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Report data and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditaudit_copy_tokenremove>.*/
    pub fn credit_report_audit_copy_remove(
        &self,
        audit_copy_token: &str,
    ) -> request::CreditReportAuditCopyRemoveRequest {
        request::CreditReportAuditCopyRemoveRequest {
            client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Retrieve information from the bank accounts used for income verification

`/credit/bank_income/get` returns the bank income report(s) for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomeget>.*/
    pub fn credit_bank_income_get(&self) -> request::CreditBankIncomeGetRequest {
        request::CreditBankIncomeGetRequest {
            client: &self,
            user_token: None,
            options: None,
        }
    }
    /**Refresh a user's bank income information

`/credit/bank_income/refresh` refreshes the bank income report data for a specific user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomerefresh>.*/
    pub fn credit_bank_income_refresh(
        &self,
        user_token: &str,
    ) -> request::CreditBankIncomeRefreshRequest {
        request::CreditBankIncomeRefreshRequest {
            client: &self,
            user_token: user_token.to_owned(),
            options: None,
        }
    }
    /**Retrieve a user's payroll information

This endpoint gets payroll income information for a specific user, either as a result of the user connecting to their payroll provider or uploading a pay related document.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeget>.*/
    pub fn credit_payroll_income_get(&self) -> request::CreditPayrollIncomeGetRequest {
        request::CreditPayrollIncomeGetRequest {
            client: &self,
            user_token: None,
        }
    }
    /**Check income verification eligibility and optimize conversion

`/credit/payroll_income/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification. If the user is eligible for digital verification, that information will be associated with the user token, and in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing `employer` data will increase the chance of receiving a useful result.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeprecheck>.*/
    pub fn credit_payroll_income_precheck(
        &self,
    ) -> request::CreditPayrollIncomePrecheckRequest {
        request::CreditPayrollIncomePrecheckRequest {
            client: &self,
            user_token: None,
            access_tokens: None,
            employer: None,
            us_military_info: None,
        }
    }
    /**Retrieve a summary of an individual's employment information

`/credit/employment/get` returns a list of items with employment information from a user's payroll provider that was verified by an end user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditemploymentget>.*/
    pub fn credit_employment_get(
        &self,
        user_token: &str,
    ) -> request::CreditEmploymentGetRequest {
        request::CreditEmploymentGetRequest {
            client: &self,
            user_token: user_token.to_owned(),
        }
    }
    /**Refresh a digital payroll income verification

`/credit/payroll_income/refresh` refreshes a given digital payroll income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomerefresh>.*/
    pub fn credit_payroll_income_refresh(
        &self,
    ) -> request::CreditPayrollIncomeRefreshRequest {
        request::CreditPayrollIncomeRefreshRequest {
            client: &self,
            user_token: None,
        }
    }
    /**Create a `relay_token` to share an Asset Report with a partner client

Plaid can share an Asset Report directly with a participating third party on your behalf. The shared Asset Report is the exact same Asset Report originally created in `/asset_report/create`.

To grant access to an Asset Report to a third party, use the `/credit/relay/create` endpoint to create a `relay_token` and then pass that token to the third party who needs access. Each third party has its own `secondary_client_id`, for example `ce5bd328dcd34123456`. You'll need to create a separate `relay_token` for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_relay_create(
        &self,
        report_tokens: Vec<ReportToken>,
        secondary_client_id: &str,
    ) -> request::CreditRelayCreateRequest {
        request::CreditRelayCreateRequest {
            client: &self,
            report_tokens,
            secondary_client_id: secondary_client_id.to_owned(),
            webhook: None,
        }
    }
    /**Retrieve the reports associated with a Relay token that was shared with you

`/credit/relay/get` allows third parties to get a report that was shared with them, using an `relay_token` that was created by the report owner.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_relay_get(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> request::CreditRelayGetRequest {
        request::CreditRelayGetRequest {
            client: &self,
            relay_token: relay_token.to_owned(),
            report_type: report_type.to_owned(),
        }
    }
    /**Refresh a report of a Relay Token

The `/credit/relay/refresh` endpoint allows third parties to refresh an report that was relayed to them, using a `relay_token` that was created by the report owner. A new report will be created based on the old one, but with the most recent data available.

See endpoint docs at <https://plaid.com/docs/api/products/#creditrelayrefresh>.*/
    pub fn credit_relay_refresh(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> request::CreditRelayRefreshRequest {
        request::CreditRelayRefreshRequest {
            client: &self,
            relay_token: relay_token.to_owned(),
            report_type: report_type.to_owned(),
            webhook: None,
        }
    }
    /**Remove Credit Relay Token

The `/credit/relay/remove` endpoint allows you to invalidate a `relay_token`, meaning the third party holding the token will no longer be able to use it to access the reports to which the `relay_token` gives access to. The report, items associated with it, and other Relay tokens that provide access to the same report are not affected and will remain accessible after removing the given `relay_token.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_relay_remove(
        &self,
        relay_token: &str,
    ) -> request::CreditRelayRemoveRequest {
        request::CreditRelayRemoveRequest {
            client: &self,
            relay_token: relay_token.to_owned(),
        }
    }
    /**Manually fire a Bank Transfer webhook

Use the `/sandbox/bank_transfer/fire_webhook` endpoint to manually trigger a Bank Transfers webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transferfire_webhook>.*/
    pub fn sandbox_bank_transfer_fire_webhook(
        &self,
        webhook: &str,
    ) -> request::SandboxBankTransferFireWebhookRequest {
        request::SandboxBankTransferFireWebhookRequest {
            client: &self,
            webhook: webhook.to_owned(),
        }
    }
    /**Manually fire an Income webhook

Use the `/sandbox/income/fire_webhook` endpoint to manually trigger an Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxincomefire_webhook>.*/
    pub fn sandbox_income_fire_webhook(
        &self,
        item_id: &str,
        webhook: &str,
        verification_status: &str,
    ) -> request::SandboxIncomeFireWebhookRequest {
        request::SandboxIncomeFireWebhookRequest {
            client: &self,
            item_id: item_id.to_owned(),
            user_id: None,
            webhook: webhook.to_owned(),
            verification_status: verification_status.to_owned(),
        }
    }
    ///Save the selected accounts when connecting to the Platypus Oauth institution
    pub fn sandbox_oauth_select_accounts(
        &self,
        oauth_state_id: &str,
        accounts: &[&str],
    ) -> request::SandboxOauthSelectAccountsRequest {
        request::SandboxOauthSelectAccountsRequest {
            client: &self,
            oauth_state_id: oauth_state_id.to_owned(),
            accounts: accounts.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    /**Evaluate a planned ACH transaction

Use `/signal/evaluate` to evaluate a planned ACH transaction to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned.

See endpoint docs at <https://plaid.com/docs/signal/reference#signalevaluate>.*/
    pub fn signal_evaluate(
        &self,
        args: request::SignalEvaluateRequired,
    ) -> request::SignalEvaluateRequest {
        request::SignalEvaluateRequest {
            client: &self,
            access_token: args.access_token.to_owned(),
            account_id: args.account_id.to_owned(),
            client_transaction_id: args.client_transaction_id.to_owned(),
            amount: args.amount,
            user_present: None,
            client_user_id: None,
            user: None,
            device: None,
        }
    }
    /**Report whether you initiated an ACH transaction

After calling `/signal/evaluate`, call `/signal/decision/report` to report whether the transaction was initiated. This endpoint will return an `INVALID_REQUEST` error if called a second time with a different value for `initiated`.

See endpoint docs at <https://plaid.com/docs/signal/reference#signaldecisionreport>.*/
    pub fn signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
    ) -> request::SignalDecisionReportRequest {
        request::SignalDecisionReportRequest {
            client: &self,
            client_transaction_id: client_transaction_id.to_owned(),
            initiated,
            days_funds_on_hold: None,
        }
    }
    /**Report a return for an ACH transaction

Call the `/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/signal/evaluate` endpoint. Your feedback will be used by the model to incorporate the latest risk trend in your portfolio.

See endpoint docs at <https://plaid.com/docs/signal/reference#signalreturnreport>.*/
    pub fn signal_return_report(
        &self,
        client_transaction_id: &str,
        return_code: &str,
    ) -> request::SignalReturnReportRequest {
        request::SignalReturnReportRequest {
            client: &self,
            client_transaction_id: client_transaction_id.to_owned(),
            return_code: return_code.to_owned(),
        }
    }
    /**Prepare the Signal product before calling `/signal/evaluate`

Call `/signal/prepare` with Plaid-linked bank account information at least 10 seconds before calling `/signal/evaluate` or as soon as an end-user enters the ACH deposit flow in your application.

See endpoint docs at <https://plaid.com/docs/signal/reference#signalprepare>.*/
    pub fn signal_prepare(&self, access_token: &str) -> request::SignalPrepareRequest {
        request::SignalPrepareRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Create an e-wallet

Create an e-wallet. The response is the newly created e-wallet object.

See endpoint docs at <https://plaid.com/docs/api/products/#walletcreate>.*/
    pub fn wallet_create(
        &self,
        iso_currency_code: &str,
    ) -> request::WalletCreateRequest {
        request::WalletCreateRequest {
            client: &self,
            iso_currency_code: iso_currency_code.to_owned(),
        }
    }
    /**Fetch an e-wallet

Fetch an e-wallet. The response includes the current balance.

See endpoint docs at <https://plaid.com/docs/api/products/#walletget>.*/
    pub fn wallet_get(&self, wallet_id: &str) -> request::WalletGetRequest {
        request::WalletGetRequest {
            client: &self,
            wallet_id: wallet_id.to_owned(),
        }
    }
    /**Fetch a list of e-wallets

This endpoint lists all e-wallets in descending order of creation.

See endpoint docs at <https://plaid.com/docs/api/products/#walletlist>.*/
    pub fn wallet_list(&self) -> request::WalletListRequest {
        request::WalletListRequest {
            client: &self,
            iso_currency_code: None,
            cursor: None,
            count: None,
        }
    }
    /**Execute a transaction using an e-wallet

Execute a transaction using the specified e-wallet.
Specify the e-wallet to debit from, the counterparty to credit to, the idempotency key to prevent duplicate payouts, the amount and reference for the payout.
The payouts are executed over the Faster Payment rails, where settlement usually only takes a few seconds.

See endpoint docs at <https://plaid.com/docs/api/products/#wallettransactionexecute>.*/
    pub fn wallet_transaction_execute(
        &self,
        args: request::WalletTransactionExecuteRequired,
    ) -> request::WalletTransactionExecuteRequest {
        request::WalletTransactionExecuteRequest {
            client: &self,
            idempotency_key: args.idempotency_key.to_owned(),
            wallet_id: args.wallet_id.to_owned(),
            counterparty: args.counterparty,
            amount: args.amount,
            reference: args.reference.to_owned(),
        }
    }
    /**Fetch a specific e-wallet transaction

See endpoint docs at <https://plaid.com/docs/api/products/#wallettransactionget>.*/
    pub fn wallet_transaction_get(
        &self,
        transaction_id: &str,
    ) -> request::WalletTransactionGetRequest {
        request::WalletTransactionGetRequest {
            client: &self,
            transaction_id: transaction_id.to_owned(),
        }
    }
    /**List e-wallet transactions

This endpoint lists the latest transactions of the specified e-wallet. Transactions are returned in descending order by the `created_at` time.

See endpoint docs at <https://plaid.com/docs/api/products/#wallettransactionslist>.*/
    pub fn wallet_transactions_list(
        &self,
        wallet_id: &str,
    ) -> request::WalletTransactionsListRequest {
        request::WalletTransactionsListRequest {
            client: &self,
            wallet_id: wallet_id.to_owned(),
            cursor: None,
            count: None,
        }
    }
    /**enhance locally-held transaction data

The '/beta/transactions/v1/enhance' endpoint enriches raw transaction data provided directly by clients.

The product is currently in beta.*/
    pub fn transactions_enhance(
        &self,
        account_type: &str,
        transactions: Vec<ClientProvidedRawTransaction>,
    ) -> request::TransactionsEnhanceRequest {
        request::TransactionsEnhanceRequest {
            client: &self,
            account_type: account_type.to_owned(),
            transactions,
        }
    }
    /**Create transaction category rule

The `/transactions/rules/v1/create` endpoint creates transaction categorization rules.

Rules will be applied on the Item's transactions returned in `/transactions/get` response.

The product is currently in beta. To request access, contact transactions-feedback@plaid.com.*/
    pub fn transactions_rules_create(
        &self,
        access_token: &str,
        personal_finance_category: &str,
        rule_details: TransactionsRuleDetails,
    ) -> request::TransactionsRulesCreateRequest {
        request::TransactionsRulesCreateRequest {
            client: &self,
            access_token: access_token.to_owned(),
            personal_finance_category: personal_finance_category.to_owned(),
            rule_details,
        }
    }
    /**Return a list of rules created for the Item associated with the access token.

The `/transactions/rules/v1/list` returns a list of transaction rules created for the Item associated with the access token.*/
    pub fn transactions_rules_list(
        &self,
        access_token: &str,
    ) -> request::TransactionsRulesListRequest {
        request::TransactionsRulesListRequest {
            client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Remove transaction rule

The `/transactions/rules/v1/remove` endpoint is used to remove a transaction rule.*/
    pub fn transactions_rules_remove(
        &self,
        access_token: &str,
        rule_id: &str,
    ) -> request::TransactionsRulesRemoveRequest {
        request::TransactionsRulesRemoveRequest {
            client: &self,
            access_token: access_token.to_owned(),
            rule_id: rule_id.to_owned(),
        }
    }
    /**Create payment profile

Use `/payment_profile/create` endpoint to create a new payment profile, the return value is a Payment Profile ID. Attach it to the link token create request and the link workflow will then "activate" this Payment Profile if the linkage is successful. It can then be used to create Transfers using `/transfer/authorization/create` and /transfer/create`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profilecreate>.*/
    pub fn payment_profile_create(&self) -> request::PaymentProfileCreateRequest {
        request::PaymentProfileCreateRequest {
            client: &self,
        }
    }
    /**Get payment profile

Use the `/payment_profile/get` endpoint to get the status of a given Payment Profile.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileget>.*/
    pub fn payment_profile_get(
        &self,
        payment_profile_id: &str,
    ) -> request::PaymentProfileGetRequest {
        request::PaymentProfileGetRequest {
            client: &self,
            payment_profile_id: payment_profile_id.to_owned(),
        }
    }
    /**Remove payment profile

Use the `/payment_profile/remove` endpoint to remove a given Payment Profile. Once it’s removed, it can no longer be used to create transfers.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileremove>.*/
    pub fn payment_profile_remove(
        &self,
        payment_profile_id: &str,
    ) -> request::PaymentProfileRemoveRequest {
        request::PaymentProfileRemoveRequest {
            client: &self,
            payment_profile_id: payment_profile_id.to_owned(),
        }
    }
    /**Creates a new client for a reseller partner end customer.

The `/partner/v1/customers/create` endpoint is used by reseller partners to create an end customer client.*/
    pub fn partner_customers_create(
        &self,
        company_name: &str,
        is_diligence_attested: bool,
        products: &[&str],
    ) -> request::PartnerCustomersCreateRequest {
        request::PartnerCustomersCreateRequest {
            client: &self,
            company_name: company_name.to_owned(),
            is_diligence_attested,
            products: products.iter().map(|&x| x.to_owned()).collect(),
            create_link_customization: None,
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
