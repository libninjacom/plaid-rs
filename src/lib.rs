//! [`PlaidClient`](struct.PlaidClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;
pub use crate::request::error::*;
mod serde;
pub struct PlaidClient {
    pub client: httpclient::Client,
    authentication: PlaidAuthentication,
}
impl PlaidClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new()
                .base_url(
                    std::env::var("PLAID_ENV")
                        .expect("Missing environment variable PLAID_ENV")
                        .as_str(),
                ),
            authentication: PlaidAuthentication::from_env(),
        }
    }
}
impl PlaidClient {
    pub fn new(url: &str, authentication: PlaidAuthentication) -> Self {
        let client = httpclient::Client::new().base_url(url);
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: PlaidAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            PlaidAuthentication::ClientId { client_id, secret, plaid_version } => {
                r = r.header("PLAID-CLIENT-ID", client_id);
                r = r.header("PLAID-SECRET", secret);
                r = r.header("Plaid-Version", plaid_version);
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
    /**Create an Asset Report

The `/asset_report/create` endpoint initiates the process of creating an Asset Report, which can then be retrieved by passing the `asset_report_token` return value to the `/asset_report/get` or `/asset_report/pdf/get` endpoints.

The Asset Report takes some time to be created and is not available immediately after calling `/asset_report/create`. When the Asset Report is ready to be retrieved using `/asset_report/get` or `/asset_report/pdf/get`, Plaid will fire a `PRODUCT_READY` webhook. For full details of the webhook schema, see [Asset Report webhooks](https://plaid.com/docs/api/products/assets/#webhooks).

The `/asset_report/create` endpoint creates an Asset Report at a moment in time. Asset Reports are immutable. To get an updated Asset Report, use the `/asset_report/refresh` endpoint.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportcreate>.*/
    pub fn asset_report_create(
        &self,
        days_requested: i64,
    ) -> request::AssetReportCreateRequest {
        request::AssetReportCreateRequest {
            http_client: &self,
            access_tokens: None,
            days_requested,
            options: None,
            report_type: None,
            user_token: None,
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
            http_client: &self,
            asset_report_token: asset_report_token.to_owned(),
            fast_report: None,
            include_insights: None,
            options: None,
        }
    }
    /**Retrieve a PDF Asset Report

The `/asset_report/pdf/get` endpoint retrieves the Asset Report in PDF format. Before calling `/asset_report/pdf/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

The response to `/asset_report/pdf/get` is the PDF binary data. The `request_id`  is returned in the `Plaid-Request-ID` header.

[View a sample PDF Asset Report](https://plaid.com/documents/sample-asset-report.pdf).

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportpdfget>.*/
    pub fn asset_report_pdf_get(
        &self,
        asset_report_token: &str,
    ) -> request::AssetReportPdfGetRequest {
        request::AssetReportPdfGetRequest {
            http_client: &self,
            asset_report_token: asset_report_token.to_owned(),
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
            http_client: &self,
            asset_report_token: asset_report_token.to_owned(),
            days_requested: None,
            options: None,
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
        account_ids_to_exclude: &[&str],
        asset_report_token: &str,
    ) -> request::AssetReportFilterRequest {
        request::AssetReportFilterRequest {
            http_client: &self,
            account_ids_to_exclude: account_ids_to_exclude
                .iter()
                .map(|&x| x.to_owned())
                .collect(),
            asset_report_token: asset_report_token.to_owned(),
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
            http_client: &self,
            asset_report_token: asset_report_token.to_owned(),
        }
    }
    /**Create Asset Report Audit Copy

Plaid can provide an Audit Copy of any Asset Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy contains the same underlying data as the Asset Report.

To grant access to an Audit Copy, use the `/asset_report/audit_copy/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copycreate>.*/
    pub fn asset_report_audit_copy_create(
        &self,
        asset_report_token: &str,
    ) -> request::AssetReportAuditCopyCreateRequest {
        request::AssetReportAuditCopyCreateRequest {
            http_client: &self,
            asset_report_token: asset_report_token.to_owned(),
            auditor_id: None,
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
            http_client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
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
            http_client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Update an Audit Copy Token

The `/credit/audit_copy_token/update` endpoint updates the Audit Copy Token by adding the report tokens in the `report_tokens` field to the `audit_copy_token`. If the Audit Copy Token already contains a report of a certain type, it will be replaced with the token provided in the `report_tokens` field.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_audit_copy_token_update(
        &self,
        audit_copy_token: &str,
        report_tokens: &[&str],
    ) -> request::CreditAuditCopyTokenUpdateRequest {
        request::CreditAuditCopyTokenUpdateRequest {
            http_client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
            report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
        }
    }
    ///List a historical log of user consent events
    pub fn item_activity_list(&self) -> request::ItemActivityListRequest {
        request::ItemActivityListRequest {
            http_client: &self,
            access_token: None,
            count: None,
            cursor: None,
        }
    }
    ///List a user’s connected applications
    pub fn item_application_list(&self) -> request::ItemApplicationListRequest {
        request::ItemApplicationListRequest {
            http_client: &self,
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
            http_client: &self,
            access_token: args.access_token.to_owned(),
            application_id: args.application_id.to_owned(),
            context: args.context.to_owned(),
            scopes: args.scopes,
            state: None,
        }
    }
    /**Retrieve information about a Plaid application

Allows financial institutions to retrieve information about Plaid clients for the purpose of building control-tower experiences*/
    pub fn application_get(
        &self,
        application_id: &str,
    ) -> request::ApplicationGetRequest {
        request::ApplicationGetRequest {
            http_client: &self,
            application_id: application_id.to_owned(),
        }
    }
    /**Retrieve an Item

Returns information about the status of an Item.

See endpoint docs at <https://plaid.com/docs/api/items/#itemget>.*/
    pub fn item_get(&self, access_token: &str) -> request::ItemGetRequest {
        request::ItemGetRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Retrieve auth data

The `/auth/get` endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.

Note: This request may take some time to complete if `auth` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

Versioning note: In API version 2017-03-08, the schema of the `numbers` object returned by this endpoint is substantially different. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2018-05-22).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#authget>.*/
    pub fn auth_get(&self, access_token: &str) -> request::AuthGetRequest {
        request::AuthGetRequest {
            http_client: &self,
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
        end_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
    ) -> request::TransactionsGetRequest {
        request::TransactionsGetRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
            end_date,
            options: None,
            start_date,
        }
    }
    /**Refresh transaction data

`/transactions/refresh` is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled Item. If changes to transactions are discovered after calling `/transactions/refresh`, Plaid will fire a webhook: for `/transactions/sync` users, [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#sync_updates_available) will be fired if there are any transactions updated, added, or removed. For users of both `/transactions/sync` and `/transactions/get`, [`TRANSACTIONS_REMOVED`](https://plaid.com/docs/api/products/transactions/#transactions_removed) will be fired if any removed transactions are detected, and [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/transactions/#default_update) will be fired if any new transactions are detected. New transactions can be fetched by calling `/transactions/get` or `/transactions/sync`. Note that the `/transactions/refresh` endpoint is not supported for Capital One (`ins_128026`) and will result in a `PRODUCT_NOT_SUPPORTED` error if called on an Item from that institution.

`/transactions/refresh` is offered as an add-on to Transactions and has a separate [fee model](/docs/account/billing/#per-request-flat-fee). To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrefresh>.*/
    pub fn transactions_refresh(
        &self,
        access_token: &str,
    ) -> request::TransactionsRefreshRequest {
        request::TransactionsRefreshRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Fetch recurring transaction streams

The `/transactions/recurring/get` endpoint allows developers to receive a summary of the recurring outflow and inflow streams (expenses and deposits) from a user’s checking, savings or credit card accounts. Additionally, Plaid provides key insights about each recurring stream including the category, merchant, last amount, and more. Developers can use these insights to build tools and experiences that help their users better manage cash flow, monitor subscriptions, reduce spend, and stay on track with bill payments.

This endpoint is offered as an add-on to Transactions. To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

This endpoint can only be called on an Item that has already been initialized with Transactions (either during Link, by specifying it in `/link/token/create`; or after Link, by calling `/transactions/get` or `/transactions/sync`). Once all historical transactions have been fetched, call `/transactions/recurring/get` to receive the Recurring Transactions streams and subscribe to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook. To know when historical transactions have been fetched, if you are using `/transactions/sync` listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#SyncUpdatesAvailableWebhook-historical-update-complete) webhook and check that the `historical_update_complete` field in the payload is `true`. If using `/transactions/get`, listen for the [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhook.

After the initial call, you can call `/transactions/recurring/get` endpoint at any point in the future to retrieve the latest summary of recurring streams. Listen to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook to be notified when new updates are available.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrecurringget>.*/
    pub fn transactions_recurring_get(
        &self,
        access_token: &str,
        account_ids: &[&str],
    ) -> request::TransactionsRecurringGetRequest {
        request::TransactionsRecurringGetRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
            account_ids: account_ids.iter().map(|&x| x.to_owned()).collect(),
            options: None,
        }
    }
    /**Get incremental transaction updates on an Item

This endpoint replaces `/transactions/get` and its associated webhooks for most common use-cases.

The `/transactions/sync` endpoint allows developers to subscribe to all transactions associated with an Item and get updates synchronously in a stream-like manner, using a cursor to track which updates have already been seen. `/transactions/sync` provides the same functionality as `/transactions/get` and can be used instead of `/transactions/get` to simplify the process of tracking transactions updates.

This endpoint provides user-authorized transaction data for `credit`, `depository`, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from `investments` accounts, use `/investments/transactions/get` instead.

Returned transactions data is grouped into three types of update, indicating whether the transaction was added, removed, or modified since the last call to the API.

In the first call to `/transactions/sync` for an Item, the endpoint will return all historical transactions data associated with that Item up until the time of the API call (as "adds"), which then generates a `next_cursor` for that Item. In subsequent calls, send the `next_cursor` to receive only the changes that have occurred since the previous call.

Due to the potentially large number of transactions associated with an Item, results are paginated. The `has_more` field specifies if additional calls are necessary to fetch all available transaction updates. Call `/transactions/sync` with the new cursor, pulling all updates, until `has_more` is `false`.

When retrieving paginated updates, track both the `next_cursor` from the latest response and the original cursor from the first call in which `has_more` was `true`; if a call to `/transactions/sync` fails when retrieving a paginated update, which can occur as a result of the [`TRANSACTIONS_SYNC_MUTATION_DURING_PAGINATION`](https://plaid.com/docs/errors/transactions/#transactions_sync_mutation_during_pagination) error, the entire pagination request loop must be restarted beginning with the cursor for the first page of the update, rather than retrying only the single request that failed.

Whenever new or updated transaction data becomes available, `/transactions/sync` will provide these updates. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. An Item's `status.transactions.last_successful_update` field will show the timestamp of the most recent successful update. To force Plaid to check for new transactions, use the `/transactions/refresh` endpoint.

Note that for newly created Items, data may not be immediately available to `/transactions/sync`. Plaid begins preparing transactions data when the Item is created, but the process can take anywhere from a few seconds to several minutes to complete, depending on the number of transactions available.

To be alerted when new data is available, listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#sync_updates_available) webhook.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionssync>.*/
    pub fn transactions_sync(
        &self,
        access_token: &str,
    ) -> request::TransactionsSyncRequest {
        request::TransactionsSyncRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
            count: None,
            cursor: None,
            options: None,
        }
    }
    /**Enrich locally-held transaction data

The `/transactions/enrich` endpoint enriches raw transaction data generated by your own banking products or retrieved from other non-Plaid sources.

To request access to Enrich, reach out to your Plaid point of contact or send a note to enrich-feedback@plaid.com

See endpoint docs at <https://plaid.com/docs/api/products/enrich/#transactionsenrich>.*/
    pub fn transactions_enrich(
        &self,
        account_type: &str,
        transactions: Vec<ClientProvidedTransaction>,
    ) -> request::TransactionsEnrichRequest {
        request::TransactionsEnrichRequest {
            http_client: &self,
            account_type: account_type.to_owned(),
            options: None,
            transactions,
        }
    }
    /**Get details of all supported institutions

Returns a JSON response containing details on all financial institutions currently supported by Plaid. Because Plaid supports thousands of institutions, results are paginated.

If there is no overlap between an institution’s enabled products and a client’s enabled products, then the institution will be filtered out from the response. As a result, the number of institutions returned may not match the count specified in the call.

See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget>.*/
    pub fn institutions_get(
        &self,
        count: i64,
        country_codes: &[&str],
        offset: i64,
    ) -> request::InstitutionsGetRequest {
        request::InstitutionsGetRequest {
            http_client: &self,
            count,
            country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
            offset,
            options: None,
        }
    }
    /**Search institutions

Returns a JSON response containing details for institutions that match the query parameters, up to a maximum of ten institutions per query.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` parameters to authenticate to this endpoint. The `public_key` parameter has since been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionssearch>.*/
    pub fn institutions_search(
        &self,
        country_codes: &[&str],
        query: &str,
    ) -> request::InstitutionsSearchRequest {
        request::InstitutionsSearchRequest {
            http_client: &self,
            country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
            options: None,
            products: None,
            query: query.to_owned(),
        }
    }
    /**Get details of an institution

Returns a JSON response containing details on a specified financial institution currently supported by Plaid.

Versioning note: API versions 2019-05-29 and earlier allow use of the `public_key` parameter instead of the `client_id` and `secret` to authenticate to this endpoint. The `public_key` has been deprecated; all customers are encouraged to use `client_id` and `secret` instead.


See endpoint docs at <https://plaid.com/docs/api/institutions/#institutionsget_by_id>.*/
    pub fn institutions_get_by_id(
        &self,
        country_codes: &[&str],
        institution_id: &str,
    ) -> request::InstitutionsGetByIdRequest {
        request::InstitutionsGetByIdRequest {
            http_client: &self,
            country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
            institution_id: institution_id.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Get Categories

Send a request to the `/categories/get` endpoint to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#categoriesget>.*/
    pub fn categories_get(&self) -> request::CategoriesGetRequest {
        request::CategoriesGetRequest {
            http_client: &self,
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
            http_client: &self,
            institution_id: institution_id.to_owned(),
            options: None,
        }
    }
    /**Create a test Item

Use the `/sandbox/public_token/create` endpoint to create a valid `public_token`  for an arbitrary institution ID, initial products, and test credentials. The created `public_token` maps to a new Sandbox Item. You can then call `/item/public_token/exchange` to exchange the `public_token` for an `access_token` and perform all API actions. `/sandbox/public_token/create` can also be used with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data. `/sandbox/public_token/create` cannot be used with OAuth institutions.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate>.*/
    pub fn sandbox_public_token_create(
        &self,
        initial_products: &[&str],
        institution_id: &str,
    ) -> request::SandboxPublicTokenCreateRequest {
        request::SandboxPublicTokenCreateRequest {
            http_client: &self,
            initial_products: initial_products.iter().map(|&x| x.to_owned()).collect(),
            institution_id: institution_id.to_owned(),
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

`SYNC_UPDATES_AVAILABLE`: Transactions webhook to be fired for a given Sandbox Item.  If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

Note that this endpoint is provided for developer ease-of-use and is not required for testing webhooks; webhooks will also fire in Sandbox under the same conditions that they would in Production or Development.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemfire_webhook>.*/
    pub fn sandbox_item_fire_webhook(
        &self,
        access_token: &str,
        webhook_code: &str,
    ) -> request::SandboxItemFireWebhookRequest {
        request::SandboxItemFireWebhookRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
            webhook_code: webhook_code.to_owned(),
            webhook_type: None,
        }
    }
    /**Retrieve real-time balance data

The `/accounts/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/accounts/balance/get` forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, `balance` itself is not a product that can be used to initialize Link. As this endpoint triggers a synchronous request for fresh data, latency may be higher than for other Plaid endpoints; if you encounter errors, you may find it necessary to adjust your timeout period when making requests.

See endpoint docs at <https://plaid.com/docs/api/products/balance/#accountsbalanceget>.*/
    pub fn accounts_balance_get(
        &self,
        access_token: &str,
    ) -> request::AccountsBalanceGetRequest {
        request::AccountsBalanceGetRequest {
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            access_token: access_token.to_owned(),
            options: None,
            user: None,
        }
    }
    /**Retrieve a dashboard user

Retrieve information about a dashboard user.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#dashboard_userget>.*/
    pub fn dashboard_user_get(
        &self,
        dashboard_user_id: &str,
    ) -> request::DashboardUserGetRequest {
        request::DashboardUserGetRequest {
            http_client: &self,
            dashboard_user_id: dashboard_user_id.to_owned(),
        }
    }
    /**List dashboard users

List all dashboard users associated with your account.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#dashboard_userlist>.*/
    pub fn dashboard_user_list(&self) -> request::DashboardUserListRequest {
        request::DashboardUserListRequest {
            http_client: &self,
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
            http_client: &self,
            gave_consent: args.gave_consent,
            is_idempotent: None,
            is_shareable: args.is_shareable,
            template_id: args.template_id.to_owned(),
            user: args.user,
        }
    }
    /**Retrieve Identity Verification

Retrieve a previously created identity verification.

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationget>.*/
    pub fn identity_verification_get(
        &self,
        identity_verification_id: &str,
    ) -> request::IdentityVerificationGetRequest {
        request::IdentityVerificationGetRequest {
            http_client: &self,
            identity_verification_id: identity_verification_id.to_owned(),
        }
    }
    /**List Identity Verifications

Filter and list Identity Verifications created by your account

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationlist>.*/
    pub fn identity_verification_list(
        &self,
        client_user_id: &str,
        template_id: &str,
    ) -> request::IdentityVerificationListRequest {
        request::IdentityVerificationListRequest {
            http_client: &self,
            client_user_id: client_user_id.to_owned(),
            cursor: None,
            template_id: template_id.to_owned(),
        }
    }
    /**Retry an Identity Verification

Allow a customer to retry their identity verification

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationretry>.*/
    pub fn identity_verification_retry(
        &self,
        client_user_id: &str,
        strategy: &str,
        template_id: &str,
    ) -> request::IdentityVerificationRetryRequest {
        request::IdentityVerificationRetryRequest {
            http_client: &self,
            client_user_id: client_user_id.to_owned(),
            steps: None,
            strategy: strategy.to_owned(),
            template_id: template_id.to_owned(),
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
            http_client: &self,
            client_user_id: None,
            search_terms,
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
            http_client: &self,
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
            http_client: &self,
            cursor: None,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
        }
    }
    /**List hits for entity watchlist screenings

List all hits for the entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhitlist>.*/
    pub fn watchlist_screening_entity_hit_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningEntityHitListRequest {
        request::WatchlistScreeningEntityHitListRequest {
            http_client: &self,
            cursor: None,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
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
            http_client: &self,
            assignee: None,
            client_user_id: None,
            cursor: None,
            entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
            status: None,
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
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            comment: None,
            confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
            dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
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
            http_client: &self,
            cursor: None,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
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
            http_client: &self,
            assignee: None,
            client_user_id: None,
            entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            reset_fields: None,
            search_terms: None,
            status: None,
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
            http_client: &self,
            client_user_id: None,
            search_terms,
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
            http_client: &self,
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
            http_client: &self,
            cursor: None,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
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
            http_client: &self,
            cursor: None,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
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
            http_client: &self,
            assignee: None,
            client_user_id: None,
            cursor: None,
            status: None,
            watchlist_program_id: watchlist_program_id.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            comment: None,
            confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
            dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
            watchlist_screening_id: watchlist_screening_id.to_owned(),
        }
    }
    /**List reviews for individual watchlist screenings

List all reviews for the individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualreviewlist>.*/
    pub fn watchlist_screening_individual_review_list(
        &self,
        watchlist_screening_id: &str,
    ) -> request::WatchlistScreeningIndividualReviewListRequest {
        request::WatchlistScreeningIndividualReviewListRequest {
            http_client: &self,
            cursor: None,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
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
            http_client: &self,
            assignee: None,
            client_user_id: None,
            reset_fields: None,
            search_terms: None,
            status: None,
            watchlist_screening_id: watchlist_screening_id.to_owned(),
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
            http_client: &self,
            processor_token: processor_token.to_owned(),
        }
    }
    /**Evaluate a planned ACH transaction

Use `/processor/signal/evaluate` to evaluate a planned ACH transaction as a processor to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/processor/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned. For more information please refer to our error documentation on [item errors](/docs/errors/item/) and [Link in Update Mode](/docs/link/update-mode/).

Note: This request may take some time to complete if Signal is being added to an existing Item. This is because Plaid must communicate directly with the institution when retrieving the data for the first time.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorsignalevaluate>.*/
    pub fn processor_signal_evaluate(
        &self,
        amount: f64,
        client_transaction_id: &str,
        processor_token: &str,
    ) -> request::ProcessorSignalEvaluateRequest {
        request::ProcessorSignalEvaluateRequest {
            http_client: &self,
            amount,
            client_transaction_id: client_transaction_id.to_owned(),
            client_user_id: None,
            default_payment_method: None,
            device: None,
            is_recurring: None,
            processor_token: processor_token.to_owned(),
            user: None,
            user_present: None,
        }
    }
    /**Report whether you initiated an ACH transaction

After calling `/processor/signal/evaluate`, call `/processor/signal/decision/report` to report whether the transaction was initiated. This endpoint will return an [`INVALID_FIELD`](/docs/errors/invalid-request/#invalid_field) error if called a second time with a different value for `initiated`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorsignaldecisionreport>.*/
    pub fn processor_signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
        processor_token: &str,
    ) -> request::ProcessorSignalDecisionReportRequest {
        request::ProcessorSignalDecisionReportRequest {
            http_client: &self,
            amount_instantly_available: None,
            client_transaction_id: client_transaction_id.to_owned(),
            days_funds_on_hold: None,
            decision_outcome: None,
            initiated,
            payment_method: None,
            processor_token: processor_token.to_owned(),
        }
    }
    /**Report a return for an ACH transaction

Call the `/processor/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/processor/signal/evaluate` endpoint. Your feedback will be used by the model to incorporate the latest risk trend in your portfolio.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorsignalreturnreport>.*/
    pub fn processor_signal_return_report(
        &self,
        client_transaction_id: &str,
        processor_token: &str,
        return_code: &str,
    ) -> request::ProcessorSignalReturnReportRequest {
        request::ProcessorSignalReturnReportRequest {
            http_client: &self,
            client_transaction_id: client_transaction_id.to_owned(),
            processor_token: processor_token.to_owned(),
            return_code: return_code.to_owned(),
            returned_at: None,
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
            http_client: &self,
            ach_class: None,
            amount: args.amount.to_owned(),
            custom_tag: None,
            description: args.description.to_owned(),
            idempotency_key: args.idempotency_key.to_owned(),
            iso_currency_code: args.iso_currency_code.to_owned(),
            metadata: None,
            network: args.network.to_owned(),
            origination_account_id: None,
            processor_token: args.processor_token.to_owned(),
            type_: args.type_.to_owned(),
            user: args.user,
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
            http_client: &self,
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
            http_client: &self,
            options: None,
            processor_token: processor_token.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Get webhook verification key

Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the `Plaid-Verification` header.

The `/webhook_verification_key/get` endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.

See endpoint docs at <https://plaid.com/docs/api/webhooks/webhook-verification/#get-webhook-verification-key>.*/
    pub fn webhook_verification_key_get(
        &self,
        key_id: &str,
    ) -> request::WebhookVerificationKeyGetRequest {
        request::WebhookVerificationKeyGetRequest {
            http_client: &self,
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
            http_client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Create payment recipient

Create a payment recipient for payment initiation.  The recipient must be in Europe, within a country that is a member of the Single Euro Payment Area (SEPA) or a non-Eurozone country [supported](https://plaid.com/global) by Plaid. For a standing order (recurring) payment, the recipient must be in the UK.

It is recommended to use `bacs` in the UK and `iban` in EU.

The endpoint is idempotent: if a developer has already made a request with the same payment details, Plaid will return the same `recipient_id`.


See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientcreate>.*/
    pub fn payment_initiation_recipient_create(
        &self,
        name: &str,
    ) -> request::PaymentInitiationRecipientCreateRequest {
        request::PaymentInitiationRecipientCreateRequest {
            http_client: &self,
            address: None,
            bacs: None,
            iban: None,
            name: name.to_owned(),
        }
    }
    /**Reverse an existing payment

Reverse a settled payment from a Plaid virtual account.

The original payment must be in a settled state to be refunded.
To refund partially, specify the amount as part of the request.
If the amount is not specified, the refund amount will be equal to all
of the remaining payment amount that has not been refunded yet.

The refund will go back to the source account that initiated the payment.
The original payment must have been initiated to a Plaid virtual account
so that this account can be used to initiate the refund.


See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentreverse>.*/
    pub fn payment_initiation_payment_reverse(
        &self,
        idempotency_key: &str,
        payment_id: &str,
        reference: &str,
    ) -> request::PaymentInitiationPaymentReverseRequest {
        request::PaymentInitiationPaymentReverseRequest {
            http_client: &self,
            amount: None,
            idempotency_key: idempotency_key.to_owned(),
            payment_id: payment_id.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
        }
    }
    /**Create a payment

After creating a payment recipient, you can use the `/payment_initiation/payment/create` endpoint to create a payment to that recipient.  Payments can be one-time or standing order (recurring) and can be denominated in either EUR, GBP or other chosen [currency](https://plaid.com/docs/api/products/payment-initiation/#payment_initiation-payment-create-request-amount-currency).  If making domestic GBP-denominated payments, your recipient must have been created with BACS numbers. In general, EUR-denominated payments will be sent via SEPA Credit Transfer, GBP-denominated payments will be sent via the Faster Payments network and for non-Eurozone markets typically via the local payment scheme, but the payment network used will be determined by the institution. Payments sent via Faster Payments will typically arrive immediately, while payments sent via SEPA Credit Transfer or other local payment schemes will typically arrive in one business day.

Standing orders (recurring payments) must be denominated in GBP and can only be sent to recipients in the UK. Once created, standing order payments cannot be modified or canceled via the API. An end user can cancel or modify a standing order directly on their banking application or website, or by contacting the bank. Standing orders will follow the payment rules of the underlying rails (Faster Payments in UK). Payments can be sent Monday to Friday, excluding bank holidays. If the pre-arranged date falls on a weekend or bank holiday, the payment is made on the next working day. It is not possible to guarantee the exact time the payment will reach the recipient’s account, although at least 90% of standing order payments are sent by 6am.

In the Development environment, payments must be below 5 GBP or other chosen [currency](https://plaid.com/docs/api/products/payment-initiation/#payment_initiation-payment-create-request-amount-currency). For details on any payment limits in Production, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentcreate>.*/
    pub fn payment_initiation_payment_create(
        &self,
        amount: PaymentAmount,
        recipient_id: &str,
        reference: &str,
    ) -> request::PaymentInitiationPaymentCreateRequest {
        request::PaymentInitiationPaymentCreateRequest {
            http_client: &self,
            amount,
            options: None,
            recipient_id: recipient_id.to_owned(),
            reference: reference.to_owned(),
            schedule: None,
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
            http_client: &self,
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
            http_client: &self,
            constraints: args.constraints,
            options: None,
            recipient_id: args.recipient_id.to_owned(),
            reference: args.reference.to_owned(),
            scopes: args.scopes.iter().map(|&x| x.to_owned()).collect(),
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
            http_client: &self,
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
            http_client: &self,
            consent_id: consent_id.to_owned(),
        }
    }
    /**Execute a single payment using consent

The `/payment_initiation/consent/payment/execute` endpoint can be used to execute payments using payment consent.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentpaymentexecute>.*/
    pub fn payment_initiation_consent_payment_execute(
        &self,
        amount: PaymentAmount,
        consent_id: &str,
        idempotency_key: &str,
    ) -> request::PaymentInitiationConsentPaymentExecuteRequest {
        request::PaymentInitiationConsentPaymentExecuteRequest {
            http_client: &self,
            amount,
            consent_id: consent_id.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Create user

This endpoint should be called for each of your end users before they begin a Plaid income flow. This provides you a single token to access all income data associated with the user. You should only create one per end user.

If you call the endpoint multiple times with the same `client_user_id`, the first creation call will succeed and the rest will fail with an error message indicating that the user has been created for the given `client_user_id`.

Ensure that you store the `user_token` along with your user's identifier in your database, as it is not possible to retrieve a previously created `user_token`.

See endpoint docs at <https://plaid.com/docs/api/products/income/#usercreate>.*/
    pub fn user_create(&self, client_user_id: &str) -> request::UserCreateRequest {
        request::UserCreateRequest {
            http_client: &self,
            client_user_id: client_user_id.to_owned(),
        }
    }
    /**Retrieve Link sessions for your user

This endpoint can be used for your end users after they complete the Link flow. This endpoint returns a list of Link sessions that your user completed, where each session includes the results from the Link flow.

These results include details about the Item that was created and some product related metadata (showing, for example, whether the user finished the bank income verification step).

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditsessionsget>.*/
    pub fn credit_sessions_get(
        &self,
        user_token: &str,
    ) -> request::CreditSessionsGetRequest {
        request::CreditSessionsGetRequest {
            http_client: &self,
            user_token: user_token.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
            consent_id: None,
            count: None,
            cursor: None,
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
            http_client: &self,
            access_token: access_token.to_owned(),
            options: None,
        }
    }
    /**Get investment transactions

The `/investments/transactions/get` endpoint allows developers to retrieve up to 24 months of user-authorized transaction data for investment accounts.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.

Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the `total_investment_transactions` response body field to fetch all available investment transactions.

Note that Investments does not have a webhook to indicate when initial transaction data has loaded. Instead, if transactions data is not ready when `/investments/transactions/get` is first called, Plaid will wait for the data. For this reason, calling `/investments/transactions/get` immediately after Link may take up to one to two minutes to return.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentstransactionsget>.*/
    pub fn investments_transactions_get(
        &self,
        access_token: &str,
        end_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
    ) -> request::InvestmentsTransactionsGetRequest {
        request::InvestmentsTransactionsGetRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
            end_date,
            options: None,
            start_date,
        }
    }
    /**Create processor token

Used to create a token suitable for sending to one of Plaid's partners to enable integrations. Note that Stripe partnerships use bank account tokens instead; see `/processor/stripe/bank_account_token/create` for creating tokens for use with Stripe integrations. Once created, a processor token for a given Item cannot be modified or updated. If the account must be linked to a new or different partner resource, create a new Item by having the user go through the Link flow again; a new processor token can then be created from the new `access_token`. Processor tokens can also be revoked, using `/item/remove`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokencreate>.*/
    pub fn processor_token_create(
        &self,
        access_token: &str,
        account_id: &str,
        processor: &str,
    ) -> request::ProcessorTokenCreateRequest {
        request::ProcessorTokenCreateRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
            account_id: account_id.to_owned(),
            processor: processor.to_owned(),
        }
    }
    /**Create Stripe bank account token


Used to create a token suitable for sending to Stripe to enable Plaid-Stripe integrations. For a detailed guide on integrating Stripe, see [Add Stripe to your app](https://plaid.com/docs/auth/partnerships/stripe/).

Note that the Stripe bank account token is a one-time use token. To store bank account information for later use, you can use a Stripe customer object and create an associated bank account from the token, or you can use a Stripe Custom account and create an associated external bank account from the token. This bank account information should work indefinitely, unless the user's bank account information changes or they revoke Plaid's permissions to access their account. Stripe bank account information cannot be modified once the bank account token has been created. If you ever need to change the bank account details used by Stripe for a specific customer, have the user go through Link again and create a new bank account token from the new `access_token`.

Bank account tokens can also be revoked, using `/item/remove`.'

See endpoint docs at <https://plaid.com/docs/api/processors/#processorstripebank_account_tokencreate>.*/
    pub fn processor_stripe_bank_account_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> request::ProcessorStripeBankAccountTokenCreateRequest {
        request::ProcessorStripeBankAccountTokenCreateRequest {
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            country_code: None,
            options: None,
            target_access_token: target_access_token.to_owned(),
            target_account_id: target_account_id.to_owned(),
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
            http_client: &self,
            options: None,
            products: products.iter().map(|&x| x.to_owned()).collect(),
            user_auth,
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
            http_client: &self,
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
            http_client: &self,
            access_token: None,
            account_filters: None,
            additional_consented_products: None,
            android_package_name: None,
            auth: None,
            client_name: args.client_name.to_owned(),
            country_codes: args.country_codes.iter().map(|&x| x.to_owned()).collect(),
            deposit_switch: None,
            employment: None,
            eu_config: None,
            identity_verification: None,
            income_verification: None,
            institution_data: None,
            institution_id: None,
            investments: None,
            language: args.language.to_owned(),
            link_customization_name: None,
            payment_initiation: None,
            products: None,
            redirect_uri: None,
            transfer: None,
            update: None,
            user: args.user,
            user_token: None,
            webhook: None,
        }
    }
    /**Get Link Token

The `/link/token/get` endpoint gets information about a previously-created `link_token` using the
`/link/token/create` endpoint. It can be useful for debugging purposes.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokenget>.*/
    pub fn link_token_get(&self, link_token: &str) -> request::LinkTokenGetRequest {
        request::LinkTokenGetRequest {
            http_client: &self,
            link_token: link_token.to_owned(),
        }
    }
    /**Exchange the Link Correlation Id for a Link Token

Exchange an OAuth `link_correlation_id` for the corresponding `link_token`. The `link_correlation_id` is only available for 'payment_initiation' products and is provided to the client via the OAuth `redirect_uri` as a query parameter.
The `link_correlation_id` is ephemeral and expires in a brief period, after which it can no longer be exchanged for the 'link_token'.

See endpoint docs at <https://plaid.com/docs/api/oauth/#linkcorrelationid>.*/
    pub fn link_oauth_correlation_id_exchange(
        &self,
        link_correlation_id: &str,
    ) -> request::LinkOauthCorrelationIdExchangeRequest {
        request::LinkOauthCorrelationIdExchangeRequest {
            http_client: &self,
            link_correlation_id: link_correlation_id.to_owned(),
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
            http_client: &self,
            deposit_switch_id: deposit_switch_id.to_owned(),
        }
    }
    /**Retrieve a transfer

The `/transfer/get` endpoint fetches information about the transfer corresponding to the given `transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferget>.*/
    pub fn transfer_get(&self, transfer_id: &str) -> request::TransferGetRequest {
        request::TransferGetRequest {
            http_client: &self,
            originator_client_id: None,
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**Retrieve a recurring transfer

The `/transfer/recurring/get` fetches information about the recurring transfer corresponding to the given `recurring_transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringget>.*/
    pub fn transfer_recurring_get(
        &self,
        recurring_transfer_id: &str,
    ) -> request::TransferRecurringGetRequest {
        request::TransferRecurringGetRequest {
            http_client: &self,
            recurring_transfer_id: recurring_transfer_id.to_owned(),
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
            http_client: &self,
            bank_transfer_id: bank_transfer_id.to_owned(),
        }
    }
    /**Create a transfer authorization

Use the `/transfer/authorization/create` endpoint to determine transfer failure risk.

In Plaid's Sandbox environment the decisions will be returned as follows:

  - To approve a transfer with null rationale code, make an authorization request with an `amount` less than the available balance in the account.

  - To approve a transfer with the rationale code `MANUALLY_VERIFIED_ITEM`, create an Item in Link through the [Same Day Micro-deposits flow](https://plaid.com/docs/auth/coverage/testing/#testing-same-day-micro-deposits).

  - To approve a transfer with the rationale code `ITEM_LOGIN_REQUIRED`, [reset the login for an Item](https://plaid.com/docs/sandbox/#item_login_required).

  - To decline a transfer with the rationale code `NSF`, the available balance on the account must be less than the authorization `amount`. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

  - To decline a transfer with the rationale code `RISK`, the available balance on the account must be exactly $0. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

`device.ip_address`, `device.user_agent` are required fields.

For [Guarantee](https://www.plaid.com/docs//transfer/guarantee/), the following fields are required : `idempotency_key`, `user.phone_number` (optional if `email_address` provided), `user.email_address` (optional if `phone_number` provided), `device.ip_address`, `device.user_agent`, and `user_present`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferauthorizationcreate>.*/
    pub fn transfer_authorization_create(
        &self,
        args: request::TransferAuthorizationCreateRequired,
    ) -> request::TransferAuthorizationCreateRequest {
        request::TransferAuthorizationCreateRequest {
            http_client: &self,
            access_token: None,
            account_id: None,
            ach_class: None,
            amount: args.amount.to_owned(),
            beacon_session_id: None,
            device: None,
            funding_account_id: None,
            idempotency_key: None,
            iso_currency_code: None,
            network: args.network.to_owned(),
            origination_account_id: None,
            originator_client_id: None,
            payment_profile_token: None,
            type_: args.type_.to_owned(),
            user: args.user,
            user_present: None,
            with_guarantee: None,
        }
    }
    /**Get RTP eligibility information of a transfer

Use the `/transfer/capabilities/get` endpoint to determine the RTP eligibility information of a transfer.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercapabilitiesget>.*/
    pub fn transfer_capabilities_get(&self) -> request::TransferCapabilitiesGetRequest {
        request::TransferCapabilitiesGetRequest {
            http_client: &self,
            access_token: None,
            account_id: None,
            payment_profile_token: None,
        }
    }
    /**Get transfer product configuration

Use the `/transfer/configuration/get` endpoint to view your transfer product configurations.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferconfigurationget>.*/
    pub fn transfer_configuration_get(
        &self,
    ) -> request::TransferConfigurationGetRequest {
        request::TransferConfigurationGetRequest {
            http_client: &self,
            originator_client_id: None,
        }
    }
    /**Get transfer product usage metrics

Use the `/transfer/metrics/get` endpoint to view your transfer product usage metrics.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfermetricsget>.*/
    pub fn transfer_metrics_get(&self) -> request::TransferMetricsGetRequest {
        request::TransferMetricsGetRequest {
            http_client: &self,
            originator_client_id: None,
        }
    }
    /**Create a transfer

Use the `/transfer/create` endpoint to initiate a new transfer.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercreate>.*/
    pub fn transfer_create(
        &self,
        authorization_id: &str,
        description: &str,
    ) -> request::TransferCreateRequest {
        request::TransferCreateRequest {
            http_client: &self,
            access_token: None,
            account_id: None,
            ach_class: None,
            amount: None,
            authorization_id: authorization_id.to_owned(),
            description: description.to_owned(),
            idempotency_key: None,
            iso_currency_code: None,
            metadata: None,
            network: None,
            origination_account_id: None,
            payment_profile_token: None,
            type_: None,
            user: None,
        }
    }
    /**Create a recurring transfer

Use the `/transfer/recurring/create` endpoint to initiate a new recurring transfer.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringcreate>.*/
    pub fn transfer_recurring_create(
        &self,
        args: request::TransferRecurringCreateRequired,
    ) -> request::TransferRecurringCreateRequest {
        request::TransferRecurringCreateRequest {
            http_client: &self,
            access_token: args.access_token.to_owned(),
            account_id: args.account_id.to_owned(),
            ach_class: None,
            amount: args.amount.to_owned(),
            description: args.description.to_owned(),
            device: args.device,
            funding_account_id: None,
            idempotency_key: args.idempotency_key.to_owned(),
            iso_currency_code: None,
            network: args.network.to_owned(),
            schedule: args.schedule,
            test_clock_id: None,
            type_: args.type_.to_owned(),
            user: args.user,
            user_present: None,
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
            http_client: &self,
            access_token: args.access_token.to_owned(),
            account_id: args.account_id.to_owned(),
            ach_class: None,
            amount: args.amount.to_owned(),
            custom_tag: None,
            description: args.description.to_owned(),
            idempotency_key: args.idempotency_key.to_owned(),
            iso_currency_code: args.iso_currency_code.to_owned(),
            metadata: None,
            network: args.network.to_owned(),
            origination_account_id: None,
            type_: args.type_.to_owned(),
            user: args.user,
        }
    }
    /**List transfers

Use the `/transfer/list` endpoint to see a list of all your transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired transfers.


See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferlist>.*/
    pub fn transfer_list(&self) -> request::TransferListRequest {
        request::TransferListRequest {
            http_client: &self,
            count: None,
            end_date: None,
            funding_account_id: None,
            offset: None,
            origination_account_id: None,
            originator_client_id: None,
            start_date: None,
        }
    }
    /**List recurring transfers

Use the `/transfer/recurring/list` endpoint to see a list of all your recurring transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired recurring transfers.


See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringlist>.*/
    pub fn transfer_recurring_list(&self) -> request::TransferRecurringListRequest {
        request::TransferRecurringListRequest {
            http_client: &self,
            count: None,
            end_time: None,
            funding_account_id: None,
            offset: None,
            start_time: None,
        }
    }
    /**List bank transfers

Use the `/bank_transfer/list` endpoint to see a list of all your bank transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired bank transfers.


See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferlist>.*/
    pub fn bank_transfer_list(&self) -> request::BankTransferListRequest {
        request::BankTransferListRequest {
            http_client: &self,
            count: None,
            direction: None,
            end_date: None,
            offset: None,
            origination_account_id: None,
            start_date: None,
        }
    }
    /**Cancel a transfer

Use the `/transfer/cancel` endpoint to cancel a transfer.  A transfer is eligible for cancellation if the `cancellable` property returned by `/transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercancel>.*/
    pub fn transfer_cancel(&self, transfer_id: &str) -> request::TransferCancelRequest {
        request::TransferCancelRequest {
            http_client: &self,
            originator_client_id: None,
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**Cancel a recurring transfer.

Use the `/transfer/recurring/cancel` endpoint to cancel a recurring transfer.  Scheduled transfer that hasn't been submitted to bank will be cancelled.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringcancel>.*/
    pub fn transfer_recurring_cancel(
        &self,
        recurring_transfer_id: &str,
    ) -> request::TransferRecurringCancelRequest {
        request::TransferRecurringCancelRequest {
            http_client: &self,
            recurring_transfer_id: recurring_transfer_id.to_owned(),
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
            http_client: &self,
            bank_transfer_id: bank_transfer_id.to_owned(),
        }
    }
    /**List transfer events

Use the `/transfer/event/list` endpoint to get a list of transfer events based on specified filter criteria.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfereventlist>.*/
    pub fn transfer_event_list(&self) -> request::TransferEventListRequest {
        request::TransferEventListRequest {
            http_client: &self,
            account_id: None,
            count: None,
            end_date: None,
            event_types: None,
            funding_account_id: None,
            offset: None,
            origination_account_id: None,
            originator_client_id: None,
            start_date: None,
            sweep_id: None,
            transfer_id: None,
            transfer_type: None,
        }
    }
    /**List bank transfer events

Use the `/bank_transfer/event/list` endpoint to get a list of Plaid-initiated ACH or bank transfer events based on specified filter criteria. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/auth#bank_transfereventlist>.*/
    pub fn bank_transfer_event_list(&self) -> request::BankTransferEventListRequest {
        request::BankTransferEventListRequest {
            http_client: &self,
            account_id: None,
            bank_transfer_id: None,
            bank_transfer_type: None,
            count: None,
            direction: None,
            end_date: None,
            event_types: None,
            offset: None,
            origination_account_id: None,
            start_date: None,
        }
    }
    /**Sync transfer events

`/transfer/event/sync` allows you to request up to the next 25 transfer events that happened after a specific `event_id`. Use the `/transfer/event/sync` endpoint to guarantee you have seen all transfer events. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://www.plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfereventsync>.*/
    pub fn transfer_event_sync(
        &self,
        after_id: i64,
    ) -> request::TransferEventSyncRequest {
        request::TransferEventSyncRequest {
            http_client: &self,
            after_id,
            count: None,
        }
    }
    /**Sync bank transfer events

`/bank_transfer/event/sync` allows you to request up to the next 25 Plaid-initiated bank transfer events that happened after a specific `event_id`. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://www.plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#bank_transfereventsync>.*/
    pub fn bank_transfer_event_sync(
        &self,
        after_id: i64,
    ) -> request::BankTransferEventSyncRequest {
        request::BankTransferEventSyncRequest {
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            sweep_id: sweep_id.to_owned(),
        }
    }
    /**List sweeps

The `/transfer/sweep/list` endpoint fetches sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfersweeplist>.*/
    pub fn transfer_sweep_list(&self) -> request::TransferSweepListRequest {
        request::TransferSweepListRequest {
            http_client: &self,
            count: None,
            end_date: None,
            funding_account_id: None,
            offset: None,
            originator_client_id: None,
            start_date: None,
        }
    }
    /**List sweeps

The `/bank_transfer/sweep/list` endpoint fetches information about the sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweeplist>.*/
    pub fn bank_transfer_sweep_list(&self) -> request::BankTransferSweepListRequest {
        request::BankTransferSweepListRequest {
            http_client: &self,
            count: None,
            end_time: None,
            origination_account_id: None,
            start_time: None,
        }
    }
    /**Get balance of your Bank Transfer account

Use the `/bank_transfer/balance/get` endpoint to see the available balance in your bank transfer account. Debit transfers increase this balance once their status is posted. Credit transfers decrease this balance when they are created.

The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.

Note that this endpoint can only be used with FBO accounts, when using Bank Transfers in the Full Service configuration. It cannot be used on your own account when using Bank Transfers in the BTS Platform configuration.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferbalanceget>.*/
    pub fn bank_transfer_balance_get(&self) -> request::BankTransferBalanceGetRequest {
        request::BankTransferBalanceGetRequest {
            http_client: &self,
            origination_account_id: None,
        }
    }
    /**Migrate account into Bank Transfers

As an alternative to adding Items via Link, you can also use the `/bank_transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items.  Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Bank Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/bank_transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfermigrate_account>.*/
    pub fn bank_transfer_migrate_account(
        &self,
        account_number: &str,
        account_type: &str,
        routing_number: &str,
    ) -> request::BankTransferMigrateAccountRequest {
        request::BankTransferMigrateAccountRequest {
            http_client: &self,
            account_number: account_number.to_owned(),
            account_type: account_type.to_owned(),
            routing_number: routing_number.to_owned(),
            wire_routing_number: None,
        }
    }
    /**Migrate account into Transfers

As an alternative to adding Items via Link, you can also use the `/transfer/migrate_account` endpoint to migrate known account and routing numbers to Plaid Items.  Note that Items created in this way are not compatible with endpoints for other products, such as `/accounts/balance/get`, and can only be used with Transfer endpoints.  If you require access to other endpoints, create the Item through Link instead.  Access to `/transfer/migrate_account` is not enabled by default; to obtain access, contact your Plaid Account Manager.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfermigrate_account>.*/
    pub fn transfer_migrate_account(
        &self,
        account_number: &str,
        account_type: &str,
        routing_number: &str,
    ) -> request::TransferMigrateAccountRequest {
        request::TransferMigrateAccountRequest {
            http_client: &self,
            account_number: account_number.to_owned(),
            account_type: account_type.to_owned(),
            routing_number: routing_number.to_owned(),
            wire_routing_number: None,
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
            http_client: &self,
            account_id: None,
            ach_class: None,
            amount: args.amount.to_owned(),
            description: args.description.to_owned(),
            funding_account_id: None,
            iso_currency_code: None,
            metadata: None,
            mode: args.mode.to_owned(),
            network: None,
            origination_account_id: None,
            require_guarantee: None,
            user: args.user,
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
            http_client: &self,
            transfer_intent_id: transfer_intent_id.to_owned(),
        }
    }
    /**Lists historical repayments

The `/transfer/repayment/list` endpoint fetches repayments matching the given filters. Repayments are returned in reverse-chronological order (most recent first) starting at the given `start_time`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentlist>.*/
    pub fn transfer_repayment_list(&self) -> request::TransferRepaymentListRequest {
        request::TransferRepaymentListRequest {
            http_client: &self,
            count: None,
            end_date: None,
            offset: None,
            start_date: None,
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
            http_client: &self,
            count: None,
            offset: None,
            repayment_id: repayment_id.to_owned(),
        }
    }
    /**Create a new originator

Use the `/transfer/originator/create` endpoint to create a new originator and return an `originator_client_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorcreate>.*/
    pub fn transfer_originator_create(
        &self,
        company_name: &str,
    ) -> request::TransferOriginatorCreateRequest {
        request::TransferOriginatorCreateRequest {
            http_client: &self,
            company_name: company_name.to_owned(),
        }
    }
    /**Generate a Plaid-hosted onboarding UI URL.

The `/transfer/questionnaire/create` endpoint generates a Plaid-hosted onboarding UI URL. Redirect the originator to this URL to provide their due diligence information and agree to Plaid’s terms for ACH money movement.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferquestionnairecreate>.*/
    pub fn transfer_questionnaire_create(
        &self,
        originator_client_id: &str,
        redirect_uri: &str,
    ) -> request::TransferQuestionnaireCreateRequest {
        request::TransferQuestionnaireCreateRequest {
            http_client: &self,
            originator_client_id: originator_client_id.to_owned(),
            redirect_uri: redirect_uri.to_owned(),
        }
    }
    /**Get status of an originator's onboarding

The `/transfer/originator/get` endpoint gets status updates for an originator's onboarding process. This information is also available via the Transfer page on the Plaid dashboard.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorget>.*/
    pub fn transfer_originator_get(
        &self,
        originator_client_id: &str,
    ) -> request::TransferOriginatorGetRequest {
        request::TransferOriginatorGetRequest {
            http_client: &self,
            originator_client_id: originator_client_id.to_owned(),
        }
    }
    /**Get status of all originators' onboarding

The `/transfer/originator/list` endpoint gets status updates for all of your originators' onboarding. This information is also available via the Plaid dashboard.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorlist>.*/
    pub fn transfer_originator_list(&self) -> request::TransferOriginatorListRequest {
        request::TransferOriginatorListRequest {
            http_client: &self,
            count: None,
            offset: None,
        }
    }
    /**Create a refund

Use the `/transfer/refund/create` endpoint to create a refund for a transfer. A transfer can be refunded if the transfer was initiated in the past 180 days.

Processing of the refund will not occur until at least 3 business days following the transfer's settlement date, plus any hold/settlement delays. This 3-day window helps better protect your business from regular ACH returns. Consumer initiated returns (unauthorized returns) could still happen for about 60 days from the settlement date. If the original transfer is canceled, returned or failed, all pending refunds will automatically be canceled. Processed refunds cannot be revoked.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrefundcreate>.*/
    pub fn transfer_refund_create(
        &self,
        amount: &str,
        idempotency_key: &str,
        transfer_id: &str,
    ) -> request::TransferRefundCreateRequest {
        request::TransferRefundCreateRequest {
            http_client: &self,
            amount: amount.to_owned(),
            idempotency_key: idempotency_key.to_owned(),
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**Retrieve a refund

The `/transfer/refund/get` endpoint fetches information about the refund corresponding to the given `refund_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrefundget>.*/
    pub fn transfer_refund_get(
        &self,
        refund_id: &str,
    ) -> request::TransferRefundGetRequest {
        request::TransferRefundGetRequest {
            http_client: &self,
            refund_id: refund_id.to_owned(),
        }
    }
    /**Cancel a refund

Use the `/transfer/refund/cancel` endpoint to cancel a refund.  A refund is eligible for cancellation if it has not yet been submitted to the payment network.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrefundcancel>.*/
    pub fn transfer_refund_cancel(
        &self,
        refund_id: &str,
    ) -> request::TransferRefundCancelRequest {
        request::TransferRefundCancelRequest {
            http_client: &self,
            refund_id: refund_id.to_owned(),
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
            http_client: &self,
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
            http_client: &self,
        }
    }
    /**Simulate a transfer event in Sandbox

Use the `/sandbox/transfer/simulate` endpoint to simulate a transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersimulate>.*/
    pub fn sandbox_transfer_simulate(
        &self,
        event_type: &str,
        transfer_id: &str,
    ) -> request::SandboxTransferSimulateRequest {
        request::SandboxTransferSimulateRequest {
            http_client: &self,
            event_type: event_type.to_owned(),
            failure_reason: None,
            transfer_id: transfer_id.to_owned(),
        }
    }
    /**Trigger the creation of a repayment

Use the `/sandbox/transfer/repayment/simulate` endpoint to trigger the creation of a repayment. As a side effect of calling this route, a repayment is created that includes all unreimbursed returns of guaranteed transfers. If there are no such returns, an 400 error is returned.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrepaymentsimulate>.*/
    pub fn sandbox_transfer_repayment_simulate(
        &self,
    ) -> request::SandboxTransferRepaymentSimulateRequest {
        request::SandboxTransferRepaymentSimulateRequest {
            http_client: &self,
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
            http_client: &self,
            webhook: webhook.to_owned(),
        }
    }
    /**Create a test clock

Use the `/sandbox/transfer/test_clock/create` endpoint to create a `test_clock` in the Sandbox environment.

A test clock object represents an independent timeline and has a `virtual_time` field indicating the current timestamp of the timeline. Test clocks are used for testing recurring transfers in Sandbox.

A test clock can be associated with up to 5 recurring transfers.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockcreate>.*/
    pub fn sandbox_transfer_test_clock_create(
        &self,
        virtual_time: chrono::DateTime<chrono::Utc>,
    ) -> request::SandboxTransferTestClockCreateRequest {
        request::SandboxTransferTestClockCreateRequest {
            http_client: &self,
            virtual_time,
        }
    }
    /**Advance a test clock

Use the `/sandbox/transfer/test_clock/advance` endpoint to advance a `test_clock` in the Sandbox environment.

A test clock object represents an independent timeline and has a `virtual_time` field indicating the current timestamp of the timeline. A test clock can be advanced by incrementing `virtual_time`, but may never go back to a lower `virtual_time`.

If a test clock is advanced, we will simulate the changes that ought to occur during the time that elapsed.
For instance, a client creates a weekly recurring transfer with a test clock set at t. When the client advances the test clock by setting `virtual_time` = t + 15 days, 2 new originations should be created, along with the webhook events.

The advancement of the test clock from its current `virtual_time` should be limited such that there are no more than 20 originations resulting from the advance operation on each `recurring_transfer` associated with the `test_clock`.
For instance, if the recurring transfer associated with this test clock originates once every 4 weeks, you can advance the `virtual_time` up to 80 weeks on each API call.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockadvance>.*/
    pub fn sandbox_transfer_test_clock_advance(
        &self,
        new_virtual_time: chrono::DateTime<chrono::Utc>,
        test_clock_id: &str,
    ) -> request::SandboxTransferTestClockAdvanceRequest {
        request::SandboxTransferTestClockAdvanceRequest {
            http_client: &self,
            new_virtual_time,
            test_clock_id: test_clock_id.to_owned(),
        }
    }
    /**Get a test clock

Use the `/sandbox/transfer/test_clock/get` endpoint to get a `test_clock` in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockget>.*/
    pub fn sandbox_transfer_test_clock_get(
        &self,
        test_clock_id: &str,
    ) -> request::SandboxTransferTestClockGetRequest {
        request::SandboxTransferTestClockGetRequest {
            http_client: &self,
            test_clock_id: test_clock_id.to_owned(),
        }
    }
    /**List test clocks

Use the `/sandbox/transfer/test_clock/list` endpoint to see a list of all your test clocks in the Sandbox environment, by ascending `virtual_time`. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired test clocks.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clocklist>.*/
    pub fn sandbox_transfer_test_clock_list(
        &self,
    ) -> request::SandboxTransferTestClockListRequest {
        request::SandboxTransferTestClockListRequest {
            http_client: &self,
            count: None,
            end_virtual_time: None,
            offset: None,
            start_virtual_time: None,
        }
    }
    /**Reset the login of a Payment Profile

`/sandbox/payment_profile/reset_login/` forces a Payment Profile into a state where the login is no longer valid. This makes it easy to test update mode for Payment Profile in the Sandbox environment.

 After calling `/sandbox/payment_profile/reset_login`, calls to the `/transfer/authorization/create` with the Payment Profile will result in a `decision_rationale` `PAYMENT_PROFILE_LOGIN_REQUIRED`. You can then use update mode for Payment Profile to restore it into a good state.

 In order to invoke this endpoint, you must first [create a Payment Profile](https://plaid.com/docs/transfer/add-to-app/#create-a-payment-profile-optional) and [go through the Link flow](https://plaid.com/docs/transfer/add-to-app/#create-a-link-token).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpayment_profilereset_login>.*/
    pub fn sandbox_payment_profile_reset_login(
        &self,
        payment_profile_token: &str,
    ) -> request::SandboxPaymentProfileResetLoginRequest {
        request::SandboxPaymentProfileResetLoginRequest {
            http_client: &self,
            payment_profile_token: payment_profile_token.to_owned(),
        }
    }
    /**Search employer database

`/employers/search` allows you the ability to search Plaid’s database of known employers, for use with Deposit Switch. You can use this endpoint to look up a user's employer in order to confirm that they are supported. Users with non-supported employers can then be routed out of the Deposit Switch flow.

The data in the employer database is currently limited. As the Deposit Switch and Income products progress through their respective beta periods, more employers are being regularly added. Because the employer database is frequently updated, we recommend that you do not cache or store data from this endpoint for more than a day.

See endpoint docs at <https://plaid.com/docs/api/employers/#employerssearch>.*/
    pub fn employers_search(
        &self,
        products: &[&str],
        query: &str,
    ) -> request::EmployersSearchRequest {
        request::EmployersSearchRequest {
            http_client: &self,
            products: products.iter().map(|&x| x.to_owned()).collect(),
            query: query.to_owned(),
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
            http_client: &self,
            options: None,
            precheck_id: None,
            webhook: webhook.to_owned(),
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
            http_client: &self,
            access_token: None,
            income_verification_id: None,
        }
    }
    /**(Deprecated) Download the original documents used for income verification

`/income/verification/documents/download` provides the ability to download the source documents associated with the verification.

If Document Income was used, the documents will be those the user provided in Link. For Payroll Income, the most recent files available
for download from the payroll provider will be available from this endpoint.

The response to `/income/verification/documents/download` is a ZIP file in binary data. If a `document_id` is passed, a single document will be contained in this file.
If not, the response will contain all documents associated with the verification.

The `request_id` is returned in the `Plaid-Request-ID` header.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationdocumentsdownload>.*/
    pub fn income_verification_documents_download(
        &self,
    ) -> request::IncomeVerificationDocumentsDownloadRequest {
        request::IncomeVerificationDocumentsDownloadRequest {
            http_client: &self,
            access_token: None,
            document_id: None,
            income_verification_id: None,
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
            http_client: &self,
            access_token: None,
            income_verification_id: None,
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
            http_client: &self,
            employer: None,
            payroll_institution: None,
            transactions_access_token: None,
            transactions_access_tokens: None,
            us_military_info: None,
            user: None,
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
            http_client: &self,
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
            http_client: &self,
            country_code: None,
            options: None,
            target_account,
            target_user,
        }
    }
    /**Create Asset or Income Report Audit Copy Token

Plaid can create an Audit Copy token of an Asset Report and/or Income Report to share with participating Government Sponsored Entity (GSE). If you participate in the Day 1 Certainty™ program, Plaid can supply an Audit Copy token directly to Fannie Mae on your behalf. An Audit Copy token contains the same underlying data as the Asset Report and/or Income Report (result of /credit/payroll_income/get).

Use the `/credit/audit_copy_token/create` endpoint to create an `audit_copy_token` and then pass that token to the GSE who needs access.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditaudit_copy_tokencreate>.*/
    pub fn credit_audit_copy_token_create(
        &self,
        report_tokens: &[&str],
    ) -> request::CreditAuditCopyTokenCreateRequest {
        request::CreditAuditCopyTokenCreateRequest {
            http_client: &self,
            report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
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
            http_client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Retrieve an Asset Report with Freddie Mac format. Only Freddie Mac can use this endpoint.

The `credit/asset_report/freddie_mac/get` endpoint retrieves the Asset Report in Freddie Mac's JSON format.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_asset_report_freddie_mac_get(
        &self,
        audit_copy_token: &str,
    ) -> request::CreditAssetReportFreddieMacGetRequest {
        request::CreditAssetReportFreddieMacGetRequest {
            http_client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Retrieve an Asset Report with Freddie Mac format (aka VOA - Verification Of Assets), and a Verification Of Employment (VOE) report if this one is available. Only Freddie Mac can use this endpoint.

The `credit/asset_report/freddie_mac/get` endpoint retrieves the Verification of Assets and Verification of Employment reports.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_freddie_mac_reports_get(
        &self,
        audit_copy_token: &str,
    ) -> request::CreditFreddieMacReportsGetRequest {
        request::CreditFreddieMacReportsGetRequest {
            http_client: &self,
            audit_copy_token: audit_copy_token.to_owned(),
        }
    }
    /**Retrieve information from the bank accounts used for employment verification

`/credit/bank_employment/get` returns the employment report(s) derived from bank transaction data for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_employmentget>.*/
    pub fn credit_bank_employment_get(
        &self,
        user_token: &str,
    ) -> request::CreditBankEmploymentGetRequest {
        request::CreditBankEmploymentGetRequest {
            http_client: &self,
            user_token: user_token.to_owned(),
        }
    }
    /**Retrieve information from the bank accounts used for income verification

`/credit/bank_income/get` returns the bank income report(s) for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomeget>.*/
    pub fn credit_bank_income_get(&self) -> request::CreditBankIncomeGetRequest {
        request::CreditBankIncomeGetRequest {
            http_client: &self,
            options: None,
            user_token: None,
        }
    }
    /**Retrieve information from the bank accounts used for income verification in PDF format

`/credit/bank_income/pdf/get` returns the most recent bank income report for a specified user in PDF format.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomepdfget>.*/
    pub fn credit_bank_income_pdf_get(
        &self,
        user_token: &str,
    ) -> request::CreditBankIncomePdfGetRequest {
        request::CreditBankIncomePdfGetRequest {
            http_client: &self,
            user_token: user_token.to_owned(),
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
            http_client: &self,
            options: None,
            user_token: user_token.to_owned(),
        }
    }
    /**Retrieve a user's payroll information

This endpoint gets payroll income information for a specific user, either as a result of the user connecting to their payroll provider or uploading a pay related document.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeget>.*/
    pub fn credit_payroll_income_get(&self) -> request::CreditPayrollIncomeGetRequest {
        request::CreditPayrollIncomeGetRequest {
            http_client: &self,
            user_token: None,
        }
    }
    /**Check income verification eligibility and optimize conversion

`/credit/payroll_income/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification. If the user is eligible for digital verification, that information will be associated with the user token, and in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing `employer` data will increase the chance of receiving a useful result.

When testing in Sandbox, you can control the results by providing special test values in the `employer` and `access_tokens` fields. `employer_good` and `employer_bad` will result in `HIGH` and `LOW` confidence values, respectively. `employer_multi` will result in a `HIGH` confidence with multiple payroll options. Likewise, `access_good` and `access_bad` will result in `HIGH` and `LOW` confidence values, respectively. Any other value for `employer` and `access_tokens` in Sandbox will result in `UNKNOWN` confidence.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeprecheck>.*/
    pub fn credit_payroll_income_precheck(
        &self,
    ) -> request::CreditPayrollIncomePrecheckRequest {
        request::CreditPayrollIncomePrecheckRequest {
            http_client: &self,
            access_tokens: None,
            employer: None,
            payroll_institution: None,
            us_military_info: None,
            user_token: None,
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
            http_client: &self,
            user_token: user_token.to_owned(),
        }
    }
    /**Refresh a digital payroll income verification

`/credit/payroll_income/refresh` refreshes a given digital payroll income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomerefresh>.*/
    pub fn credit_payroll_income_refresh(
        &self,
        user_token: &str,
    ) -> request::CreditPayrollIncomeRefreshRequest {
        request::CreditPayrollIncomeRefreshRequest {
            http_client: &self,
            options: None,
            user_token: user_token.to_owned(),
        }
    }
    /**Create a relay token to share an Asset Report with a partner client (beta)

Plaid can share an Asset Report directly with a participating third party on your behalf. The shared Asset Report is the exact same Asset Report originally created in `/asset_report/create`.

To grant a third party access to an Asset Report, use the `/credit/relay/create` endpoint to create a `relay_token` and then pass that token to your third party. Each third party has its own `secondary_client_id`; for example, `ce5bd328dcd34123456`. You'll need to create a separate `relay_token` for each third party that needs access to the report on your behalf.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelaycreate>.*/
    pub fn credit_relay_create(
        &self,
        report_tokens: &[&str],
        secondary_client_id: &str,
    ) -> request::CreditRelayCreateRequest {
        request::CreditRelayCreateRequest {
            http_client: &self,
            report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
            secondary_client_id: secondary_client_id.to_owned(),
            webhook: None,
        }
    }
    /**Retrieve the reports associated with a relay token that was shared with you (beta)

`/credit/relay/get` allows third parties to receive a report that was shared with them, using a `relay_token` that was created by the report owner.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayget>.*/
    pub fn credit_relay_get(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> request::CreditRelayGetRequest {
        request::CreditRelayGetRequest {
            http_client: &self,
            relay_token: relay_token.to_owned(),
            report_type: report_type.to_owned(),
        }
    }
    /**Refresh a report of a relay token (beta)

The `/credit/relay/refresh` endpoint allows third parties to refresh a report that was relayed to them, using a `relay_token` that was created by the report owner. A new report will be created with the original report parameters, but with the most recent data available based on the `days_requested` value of the original report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayrefresh>.*/
    pub fn credit_relay_refresh(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> request::CreditRelayRefreshRequest {
        request::CreditRelayRefreshRequest {
            http_client: &self,
            relay_token: relay_token.to_owned(),
            report_type: report_type.to_owned(),
            webhook: None,
        }
    }
    /**Remove relay token (beta)

The `/credit/relay/remove` endpoint allows you to invalidate a `relay_token`. The third party holding the token will no longer be able to access or refresh the reports which the `relay_token` gives access to. The original report, associated Items, and other relay tokens that provide access to the same report are not affected and will remain accessible after removing the given `relay_token`.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayremove>.*/
    pub fn credit_relay_remove(
        &self,
        relay_token: &str,
    ) -> request::CreditRelayRemoveRequest {
        request::CreditRelayRemoveRequest {
            http_client: &self,
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
            http_client: &self,
            webhook: webhook.to_owned(),
        }
    }
    /**Manually fire an Income webhook

Use the `/sandbox/income/fire_webhook` endpoint to manually trigger an Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxincomefire_webhook>.*/
    pub fn sandbox_income_fire_webhook(
        &self,
        item_id: &str,
        verification_status: &str,
        webhook: &str,
    ) -> request::SandboxIncomeFireWebhookRequest {
        request::SandboxIncomeFireWebhookRequest {
            http_client: &self,
            item_id: item_id.to_owned(),
            user_id: None,
            verification_status: verification_status.to_owned(),
            webhook: webhook.to_owned(),
        }
    }
    ///Save the selected accounts when connecting to the Platypus Oauth institution
    pub fn sandbox_oauth_select_accounts(
        &self,
        accounts: &[&str],
        oauth_state_id: &str,
    ) -> request::SandboxOauthSelectAccountsRequest {
        request::SandboxOauthSelectAccountsRequest {
            http_client: &self,
            accounts: accounts.iter().map(|&x| x.to_owned()).collect(),
            oauth_state_id: oauth_state_id.to_owned(),
        }
    }
    /**Evaluate a planned ACH transaction

Use `/signal/evaluate` to evaluate a planned ACH transaction to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned. For more information please refer to the error documentation on [Item errors](/docs/errors/item/) and [Link in Update Mode](/docs/link/update-mode/).

Note: This request may take some time to complete if Signal is being added to an existing Item. This is because Plaid must communicate directly with the institution when retrieving the data for the first time.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalevaluate>.*/
    pub fn signal_evaluate(
        &self,
        args: request::SignalEvaluateRequired,
    ) -> request::SignalEvaluateRequest {
        request::SignalEvaluateRequest {
            http_client: &self,
            access_token: args.access_token.to_owned(),
            account_id: args.account_id.to_owned(),
            amount: args.amount,
            client_transaction_id: args.client_transaction_id.to_owned(),
            client_user_id: None,
            default_payment_method: None,
            device: None,
            is_recurring: None,
            user: None,
            user_present: None,
        }
    }
    /**Report whether you initiated an ACH transaction

After calling `/signal/evaluate`, call `/signal/decision/report` to report whether the transaction was initiated. This endpoint will return an [`INVALID_FIELD`](/docs/errors/invalid-request/#invalid_field) error if called a second time with a different value for `initiated`.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signaldecisionreport>.*/
    pub fn signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
    ) -> request::SignalDecisionReportRequest {
        request::SignalDecisionReportRequest {
            http_client: &self,
            amount_instantly_available: None,
            client_transaction_id: client_transaction_id.to_owned(),
            days_funds_on_hold: None,
            decision_outcome: None,
            initiated,
            payment_method: None,
        }
    }
    /**Report a return for an ACH transaction

Call the `/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/signal/evaluate` endpoint. Your feedback will be used by the model to incorporate the latest risk trend in your portfolio.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalreturnreport>.*/
    pub fn signal_return_report(
        &self,
        client_transaction_id: &str,
        return_code: &str,
    ) -> request::SignalReturnReportRequest {
        request::SignalReturnReportRequest {
            http_client: &self,
            client_transaction_id: client_transaction_id.to_owned(),
            return_code: return_code.to_owned(),
            returned_at: None,
        }
    }
    /**Opt-in an Item to Signal

When Link is not initialized with Signal, call `/signal/prepare` to opt-in that Item to the Signal data collection process, developing a Signal score.

If you are using other Plaid products after Link, e.g. Identity or Assets, call `/signal/prepare` after those product calls are complete.

Example flow: Link is initialized with Auth, call `/auth/get` for the account and routing number, call `/identity/get` to retrieve bank ownership details, then call `/signal/prepare` to begin Signal data collection. Later, once you have obtained details about the proposed transaction from the user, call `/signal/evaluate` for a Signal score. For more information please see [Recommendations for initializing Link with specific product combinations](https://www.plaid.com/docs/link/initializing-products/#recommendations-for-initializing-link-with-specific-product-combinations).

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalprepare>.*/
    pub fn signal_prepare(&self, access_token: &str) -> request::SignalPrepareRequest {
        request::SignalPrepareRequest {
            http_client: &self,
            access_token: access_token.to_owned(),
        }
    }
    /**Create an e-wallet

Create an e-wallet. The response is the newly created e-wallet object.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletcreate>.*/
    pub fn wallet_create(
        &self,
        iso_currency_code: &str,
    ) -> request::WalletCreateRequest {
        request::WalletCreateRequest {
            http_client: &self,
            iso_currency_code: iso_currency_code.to_owned(),
        }
    }
    /**Fetch an e-wallet

Fetch an e-wallet. The response includes the current balance.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletget>.*/
    pub fn wallet_get(&self, wallet_id: &str) -> request::WalletGetRequest {
        request::WalletGetRequest {
            http_client: &self,
            wallet_id: wallet_id.to_owned(),
        }
    }
    /**Fetch a list of e-wallets

This endpoint lists all e-wallets in descending order of creation.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletlist>.*/
    pub fn wallet_list(&self) -> request::WalletListRequest {
        request::WalletListRequest {
            http_client: &self,
            count: None,
            cursor: None,
            iso_currency_code: None,
        }
    }
    /**Execute a transaction using an e-wallet

Execute a transaction using the specified e-wallet.
Specify the e-wallet to debit from, the counterparty to credit to, the idempotency key to prevent duplicate transactions, the amount and reference for the transaction.
Transactions will settle in seconds to several days, depending on the underlying payment rail.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionexecute>.*/
    pub fn wallet_transaction_execute(
        &self,
        args: request::WalletTransactionExecuteRequired,
    ) -> request::WalletTransactionExecuteRequest {
        request::WalletTransactionExecuteRequest {
            http_client: &self,
            amount: args.amount,
            counterparty: args.counterparty,
            idempotency_key: args.idempotency_key.to_owned(),
            reference: args.reference.to_owned(),
            wallet_id: args.wallet_id.to_owned(),
        }
    }
    /**Fetch an e-wallet transaction

Fetch a specific e-wallet transaction

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionget>.*/
    pub fn wallet_transaction_get(
        &self,
        transaction_id: &str,
    ) -> request::WalletTransactionGetRequest {
        request::WalletTransactionGetRequest {
            http_client: &self,
            transaction_id: transaction_id.to_owned(),
        }
    }
    /**List e-wallet transactions

This endpoint lists the latest transactions of the specified e-wallet. Transactions are returned in descending order by the `created_at` time.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionlist>.*/
    pub fn wallet_transaction_list(
        &self,
        wallet_id: &str,
    ) -> request::WalletTransactionListRequest {
        request::WalletTransactionListRequest {
            http_client: &self,
            count: None,
            cursor: None,
            options: None,
            wallet_id: wallet_id.to_owned(),
        }
    }
    /**enhance locally-held transaction data

The `/beta/transactions/v1/enhance` endpoint enriches raw transaction data provided directly by clients.

The product is currently in beta.*/
    pub fn transactions_enhance(
        &self,
        account_type: &str,
        transactions: Vec<ClientProvidedRawTransaction>,
    ) -> request::TransactionsEnhanceRequest {
        request::TransactionsEnhanceRequest {
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
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
            http_client: &self,
            access_token: access_token.to_owned(),
            rule_id: rule_id.to_owned(),
        }
    }
    /**Create payment profile

Use `/payment_profile/create` endpoint to create a new payment profile.
To initiate the account linking experience, call `/link/token/create` and provide the `payment_profile_token` in the `transfer.payment_profile_token` field.
You can then use the `payment_profile_token` when creating transfers using `/transfer/authorization/create` and `/transfer/create`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profilecreate>.*/
    pub fn payment_profile_create(&self) -> request::PaymentProfileCreateRequest {
        request::PaymentProfileCreateRequest {
            http_client: &self,
        }
    }
    /**Get payment profile

Use `/payment_profile/get` endpoint to get the status of a given Payment Profile.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileget>.*/
    pub fn payment_profile_get(
        &self,
        payment_profile_token: &str,
    ) -> request::PaymentProfileGetRequest {
        request::PaymentProfileGetRequest {
            http_client: &self,
            payment_profile_token: payment_profile_token.to_owned(),
        }
    }
    /**Remove payment profile

Use the `/payment_profile/remove` endpoint to remove a given Payment Profile. Once it’s removed, it can no longer be used to create transfers.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileremove>.*/
    pub fn payment_profile_remove(
        &self,
        payment_profile_token: &str,
    ) -> request::PaymentProfileRemoveRequest {
        request::PaymentProfileRemoveRequest {
            http_client: &self,
            payment_profile_token: payment_profile_token.to_owned(),
        }
    }
    /**Creates a new end customer for a Plaid reseller.

The `/partner/customer/create` endpoint is used by reseller partners to create end customers.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomercreate>.*/
    pub fn partner_customer_create(
        &self,
        args: request::PartnerCustomerCreateRequired,
    ) -> request::PartnerCustomerCreateRequest {
        request::PartnerCustomerCreateRequest {
            http_client: &self,
            address: args.address,
            application_name: args.application_name.to_owned(),
            assets_under_management: None,
            billing_contact: None,
            client_id: None,
            company_name: args.company_name.to_owned(),
            create_link_customization: None,
            customer_support_info: None,
            is_bank_addendum_completed: args.is_bank_addendum_completed,
            is_diligence_attested: args.is_diligence_attested,
            legal_entity_name: args.legal_entity_name.to_owned(),
            logo: None,
            products: args.products.iter().map(|&x| x.to_owned()).collect(),
            redirect_uris: None,
            secret: None,
            technical_contact: None,
            website: args.website.to_owned(),
        }
    }
    /**Returns a Plaid reseller's end customer.

The `/partner/customer/get` endpoint is used by reseller partners to retrieve data about a single end customer.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerget>.*/
    pub fn partner_customer_get(
        &self,
        end_customer_client_id: &str,
    ) -> request::PartnerCustomerGetRequest {
        request::PartnerCustomerGetRequest {
            http_client: &self,
            client_id: None,
            end_customer_client_id: end_customer_client_id.to_owned(),
            secret: None,
        }
    }
    /**Enables a Plaid reseller's end customer in the Production environment.

The `/partner/customer/enable` endpoint is used by reseller partners to enable an end customer in the Production environment.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerenable>.*/
    pub fn partner_customer_enable(
        &self,
        end_customer_client_id: &str,
    ) -> request::PartnerCustomerEnableRequest {
        request::PartnerCustomerEnableRequest {
            http_client: &self,
            client_id: None,
            end_customer_client_id: end_customer_client_id.to_owned(),
            secret: None,
        }
    }
    /**Removes a Plaid reseller's end customer.

The `/partner/customer/remove` endpoint is used by reseller partners to remove an end customer. Removing an end customer will remove it from view in the Plaid Dashboard and deactivate its API keys. This endpoint can only be used to remove an end customer that has not yet been enabled in Production.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerremove>.*/
    pub fn partner_customer_remove(
        &self,
        end_customer_client_id: &str,
    ) -> request::PartnerCustomerRemoveRequest {
        request::PartnerCustomerRemoveRequest {
            http_client: &self,
            client_id: None,
            end_customer_client_id: end_customer_client_id.to_owned(),
            secret: None,
        }
    }
    /**Returns OAuth-institution registration information for a given end customer.

The `/partner/customer/oauth_institutions/get` endpoint is used by reseller partners to retrieve OAuth-institution registration information about a single end customer. To learn how to set up a webhook to listen to status update events, visit the [reseller documentation](https://plaid.com/docs/account/resellers/#enabling-end-customers).

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomeroauth_institutionsget>.*/
    pub fn partner_customer_oauth_institutions_get(
        &self,
        end_customer_client_id: &str,
    ) -> request::PartnerCustomerOauthInstitutionsGetRequest {
        request::PartnerCustomerOauthInstitutionsGetRequest {
            http_client: &self,
            client_id: None,
            end_customer_client_id: end_customer_client_id.to_owned(),
            secret: None,
        }
    }
    /**Create Link Delivery session

Use the `/link_delivery/create` endpoint to create a Link Delivery session.

See endpoint docs at <https://plaid.com/docs/docs/assets/waitlist/link-delivery/>.*/
    pub fn link_delivery_create(
        &self,
        link_token: &str,
    ) -> request::LinkDeliveryCreateRequest {
        request::LinkDeliveryCreateRequest {
            http_client: &self,
            link_token: link_token.to_owned(),
            options: None,
        }
    }
    /**Get Link Delivery session

Use the `/link_delivery/get` endpoint to get the status of a Link Delivery session.

See endpoint docs at <https://plaid.com/docs/docs/assets/waitlist/link-delivery/>.*/
    pub fn link_delivery_get(
        &self,
        link_delivery_session_id: &str,
    ) -> request::LinkDeliveryGetRequest {
        request::LinkDeliveryGetRequest {
            http_client: &self,
            link_delivery_session_id: link_delivery_session_id.to_owned(),
        }
    }
    /**Webhook receiver for fdx notifications

A generic webhook receiver endpoint for FDX Event Notifications

See endpoint docs at <https://plaid.com/docs/api/fdx/notifications/#post>.*/
    pub fn fdx_notifications(
        &self,
        args: request::FdxNotificationsRequired,
    ) -> request::FdxNotificationsRequest {
        request::FdxNotificationsRequest {
            http_client: &self,
            category: args.category.to_owned(),
            notification_id: args.notification_id.to_owned(),
            notification_payload: args.notification_payload,
            priority: None,
            publisher: args.publisher,
            sent_on: args.sent_on,
            severity: None,
            subscriber: None,
            type_: args.type_.to_owned(),
            url: None,
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