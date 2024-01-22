//! [`PlaidClient`](struct.PlaidClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
pub use httpclient::{Error, Result, InMemoryResponseExt};
use std::sync::{Arc, OnceLock};
use std::borrow::Cow;
use crate::model::*;
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
mod serde;
static SHARED_HTTPCLIENT: OnceLock<httpclient::Client> = OnceLock::new();
pub fn default_http_client() -> httpclient::Client {
    let plaid_env = std::env::var("PLAID_ENV")
        .expect("Missing environment variable PLAID_ENV");
    let url = format!("https://{plaid_env}.plaid.com");
    httpclient::Client::new()
        .base_url(&url)
}
/// Use this method if you want to add custom middleware to the httpclient.
/// It must be called before any requests are made, otherwise it will have no effect.
/// Example usage:
///
/// ```
/// init_http_client(default_http_client()
///     .with_middleware(..)
/// );
/// ```
pub fn init_http_client(init: httpclient::Client) {
    let _ = SHARED_HTTPCLIENT.set(init);
}
fn shared_http_client() -> Cow<'static, httpclient::Client> {
    Cow::Borrowed(SHARED_HTTPCLIENT.get_or_init(default_http_client))
}
#[derive(Clone)]
pub struct FluentRequest<'a, T> {
    pub(crate) client: &'a PlaidClient,
    pub params: T,
}
pub struct PlaidClient {
    client: Cow<'static, httpclient::Client>,
    authentication: PlaidAuth,
}
impl PlaidClient {
    pub fn from_env() -> Self {
        Self {
            client: shared_http_client(),
            authentication: PlaidAuth::from_env(),
        }
    }
    pub fn with_auth(authentication: PlaidAuth) -> Self {
        Self {
            client: shared_http_client(),
            authentication,
        }
    }
    pub fn new_with(client: httpclient::Client, authentication: PlaidAuth) -> Self {
        Self {
            client: Cow::Owned(client),
            authentication,
        }
    }
}
impl PlaidClient {
    pub(crate) fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            PlaidAuth::ClientId { client_id, secret, plaid_version } => {
                r = r.header("PLAID-CLIENT-ID", client_id);
                r = r.header("PLAID-SECRET", secret);
                r = r.header("Plaid-Version", plaid_version);
            }
        }
        r
    }
    /**Create an Asset Report

The `/asset_report/create` endpoint initiates the process of creating an Asset Report, which can then be retrieved by passing the `asset_report_token` return value to the `/asset_report/get` or `/asset_report/pdf/get` endpoints.

The Asset Report takes some time to be created and is not available immediately after calling `/asset_report/create`. The exact amount of time to create the report will vary depending on how many days of history are requested and will typically range from a few seconds to about one minute. When the Asset Report is ready to be retrieved using `/asset_report/get` or `/asset_report/pdf/get`, Plaid will fire a `PRODUCT_READY` webhook. For full details of the webhook schema, see [Asset Report webhooks](https://plaid.com/docs/api/products/assets/#webhooks).

The `/asset_report/create` endpoint creates an Asset Report at a moment in time. Asset Reports are immutable. To get an updated Asset Report, use the `/asset_report/refresh` endpoint.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportcreate>.*/
    pub fn asset_report_create(
        &self,
        days_requested: i64,
    ) -> FluentRequest<'_, request::AssetReportCreateRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportCreateRequest {
                access_tokens: None,
                days_requested,
                options: None,
            },
        }
    }
    /**Retrieve an Asset Report

The `/asset_report/get` endpoint retrieves the Asset Report in JSON format. Before calling `/asset_report/get`, you must first create the Asset Report using `/asset_report/create` (or filter an Asset Report using `/asset_report/filter`) and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

By default, an Asset Report includes transaction descriptions as returned by the bank, as opposed to parsed and categorized by Plaid. You can also receive cleaned and categorized transactions, as well as additional insights like merchant name or location information. We call this an Asset Report with Insights. An Asset Report with Insights provides transaction category, location, and merchant information in addition to the transaction strings provided in a standard Asset Report. To retrieve an Asset Report with Insights, call `/asset_report/get` endpoint with `include_insights` set to `true`.

For latency-sensitive applications, you can optionally call `/asset_report/create` with `options.add_ons` set to `["fast_assets"]`. This will cause Plaid to create two versions of the Asset Report: one with only current and available balance and identity information, and then later on the complete Asset Report. You will receive separate webhooks for each version of the Asset Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportget>.*/
    pub fn asset_report_get(&self) -> FluentRequest<'_, request::AssetReportGetRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportGetRequest {
                asset_report_token: None,
                fast_report: None,
                include_insights: None,
                options: None,
                user_token: None,
            },
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
    ) -> FluentRequest<'_, request::AssetReportPdfGetRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportPdfGetRequest {
                asset_report_token: asset_report_token.to_owned(),
                options: None,
            },
        }
    }
    /**Refresh an Asset Report

An Asset Report is an immutable snapshot of a user's assets. In order to "refresh" an Asset Report you created previously, you can use the `/asset_report/refresh` endpoint to create a new Asset Report based on the old one, but with the most recent data available.

The new Asset Report will contain the same Items as the original Report, as well as the same filters applied by any call to `/asset_report/filter`. By default, the new Asset Report will also use the same parameters you submitted with your original `/asset_report/create` request, but the original `days_requested` value and the values of any parameters in the `options` object can be overridden with new values. To change these arguments, simply supply new values for them in your request to `/asset_report/refresh`. Submit an empty string ("") for any previously-populated fields you would like set as empty.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportrefresh>.*/
    pub fn asset_report_refresh(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, request::AssetReportRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportRefreshRequest {
                asset_report_token: asset_report_token.to_owned(),
                days_requested: None,
                options: None,
            },
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
    ) -> FluentRequest<'_, request::AssetReportFilterRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportFilterRequest {
                account_ids_to_exclude: account_ids_to_exclude
                    .iter()
                    .map(|&x| x.to_owned())
                    .collect(),
                asset_report_token: asset_report_token.to_owned(),
            },
        }
    }
    /**Delete an Asset Report

The `/item/remove` endpoint allows you to invalidate an `access_token`, meaning you will not be able to create new Asset Reports with it. Removing an Item does not affect any Asset Reports or Audit Copies you have already created, which will remain accessible until you remove them specifically.

The `/asset_report/remove` endpoint allows you to remove an Asset Report. Removing an Asset Report invalidates its `asset_report_token`, meaning you will no longer be able to use it to access Report data or create new Audit Copies. Removing an Asset Report does not affect the underlying Items, but does invalidate any `audit_copy_tokens` associated with the Asset Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportremove>.*/
    pub fn asset_report_remove(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, request::AssetReportRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportRemoveRequest {
                asset_report_token: asset_report_token.to_owned(),
            },
        }
    }
    /**Create Asset Report Audit Copy

Plaid can provide an Audit Copy of any Asset Report directly to a participating third party on your behalf. For example, Plaid can supply an Audit Copy directly to Fannie Mae on your behalf if you participate in the Day 1 Certainty™ program. An Audit Copy contains the same underlying data as the Asset Report.

To grant access to an Audit Copy, use the `/asset_report/audit_copy/create` endpoint to create an `audit_copy_token` and then pass that token to the third party who needs access. Each third party has its own `auditor_id`, for example `fannie_mae`. You’ll need to create a separate Audit Copy for each third party to whom you want to grant access to the Report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copycreate>.*/
    pub fn asset_report_audit_copy_create(
        &self,
        asset_report_token: &str,
    ) -> FluentRequest<'_, request::AssetReportAuditCopyCreateRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportAuditCopyCreateRequest {
                asset_report_token: asset_report_token.to_owned(),
                auditor_id: None,
            },
        }
    }
    /**Retrieve an Asset Report Audit Copy

`/asset_report/audit_copy/get` allows auditors to get a copy of an Asset Report that was previously shared via the `/asset_report/audit_copy/create` endpoint.  The caller of `/asset_report/audit_copy/create` must provide the `audit_copy_token` to the auditor.  This token can then be used to call `/asset_report/audit_copy/create`.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn asset_report_audit_copy_get(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, request::AssetReportAuditCopyGetRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportAuditCopyGetRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
    /**Remove Asset Report Audit Copy

The `/asset_report/audit_copy/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Asset Report, the Asset Report itself and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#asset_reportaudit_copyremove>.*/
    pub fn asset_report_audit_copy_remove(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, request::AssetReportAuditCopyRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::AssetReportAuditCopyRemoveRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
    /**Retrieve a Base Report

This endpoint allows the customer to retrieve a Base Report. Customers should pass in the `user_token` created in `/link/token/create`.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn base_report_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::BaseReportGetRequest> {
        FluentRequest {
            client: self,
            params: request::BaseReportGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Update an Audit Copy Token

The `/credit/audit_copy_token/update` endpoint updates an existing  Audit Copy Token by adding the report tokens in the `report_tokens` field to the `audit_copy_token`. If the Audit Copy Token already contains a report of a certain type, it will be replaced with the token provided in the `report_tokens` field.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_audit_copy_token_update(
        &self,
        audit_copy_token: &str,
        report_tokens: &[&str],
    ) -> FluentRequest<'_, request::CreditAuditCopyTokenUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::CreditAuditCopyTokenUpdateRequest {
                audit_copy_token: audit_copy_token.to_owned(),
                report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    /**Retrieve information from the bank accounts used for income verification

`/cra/bank_income/get` returns the bank income report(s) for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#crabank_incomeget>.*/
    pub fn cra_bank_income_get(
        &self,
    ) -> FluentRequest<'_, request::CraBankIncomeGetRequest> {
        FluentRequest {
            client: self,
            params: request::CraBankIncomeGetRequest {
                user_token: None,
            },
        }
    }
    /**Retrieve a list of all statements associated with the provided item.

The `/statements/list` endpoint retrieves a list of all statements associated with the provided item.

See endpoint docs at <https://plaid.com/docs/api/products/statements#statementslist>.*/
    pub fn statements_list(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::StatementsListRequest> {
        FluentRequest {
            client: self,
            params: request::StatementsListRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Retrieve a single statement.

The `/statements/download` endpoint retrieves a single statement PDF in binary format.  The response will contain a `Plaid-Content-Hash` header containing a SHA 256 checksum of the statement. This can be used to verify that the file being sent by Plaid is the same file that was downloaded to your system.

See endpoint docs at <https://plaid.com/docs/api/products/statements#statementsdownload>.*/
    pub fn statements_download(
        &self,
        access_token: &str,
        statement_id: &str,
    ) -> FluentRequest<'_, request::StatementsDownloadRequest> {
        FluentRequest {
            client: self,
            params: request::StatementsDownloadRequest {
                access_token: access_token.to_owned(),
                statement_id: statement_id.to_owned(),
            },
        }
    }
    /**Refresh statements data.

`/statements/refresh` initiates an on-demand extraction to fetch the statements for the provided dates.

See endpoint docs at <https://plaid.com/docs/api/products/statements#statementsrefresh>.*/
    pub fn statements_refresh(
        &self,
        access_token: &str,
        end_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
    ) -> FluentRequest<'_, request::StatementsRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::StatementsRefreshRequest {
                access_token: access_token.to_owned(),
                end_date,
                start_date,
            },
        }
    }
    ///List a historical log of user consent events
    pub fn item_activity_list(
        &self,
    ) -> FluentRequest<'_, request::ItemActivityListRequest> {
        FluentRequest {
            client: self,
            params: request::ItemActivityListRequest {
                access_token: None,
                count: None,
                cursor: None,
            },
        }
    }
    ///List a user’s connected applications
    pub fn item_application_list(
        &self,
    ) -> FluentRequest<'_, request::ItemApplicationListRequest> {
        FluentRequest {
            client: self,
            params: request::ItemApplicationListRequest {
                access_token: None,
            },
        }
    }
    /**Unlink a user’s connected application

Unlink a user’s connected application. On an unlink request, Plaid will immediately revoke the Application’s access to the User’s data.  The User will have to redo the OAuth authentication process in order to restore functionality.

This endpoint only removes ongoing data access permissions, therefore the User will need to reach out to the Application itself in order to disable and delete their account and delete any data that the Application already received (if the Application does not do so by default).

This endpoint should be called in real time as the User is unlinking an Application, and should not be batched in order to ensure that the change is reflected as soon as possible.

See endpoint docs at <https://plaid.com/docsnone>.*/
    pub fn item_application_unlink(
        &self,
        access_token: &str,
        application_id: &str,
    ) -> FluentRequest<'_, request::ItemApplicationUnlinkRequest> {
        FluentRequest {
            client: self,
            params: request::ItemApplicationUnlinkRequest {
                access_token: access_token.to_owned(),
                application_id: application_id.to_owned(),
            },
        }
    }
    /**Update the scopes of access for a particular application

Enable consumers to update product access on selected accounts for an application.*/
    pub fn item_application_scopes_update(
        &self,
        args: request::ItemApplicationScopesUpdateRequired,
    ) -> FluentRequest<'_, request::ItemApplicationScopesUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::ItemApplicationScopesUpdateRequest {
                access_token: args.access_token.to_owned(),
                application_id: args.application_id.to_owned(),
                context: args.context.to_owned(),
                scopes: args.scopes,
                state: None,
            },
        }
    }
    /**Retrieve information about a Plaid application

Allows financial institutions to retrieve information about Plaid clients for the purpose of building control-tower experiences*/
    pub fn application_get(
        &self,
        application_id: &str,
    ) -> FluentRequest<'_, request::ApplicationGetRequest> {
        FluentRequest {
            client: self,
            params: request::ApplicationGetRequest {
                application_id: application_id.to_owned(),
            },
        }
    }
    /**Retrieve an Item

Returns information about the status of an Item.

See endpoint docs at <https://plaid.com/docs/api/items/#itemget>.*/
    pub fn item_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::ItemGetRequest> {
        FluentRequest {
            client: self,
            params: request::ItemGetRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Retrieve auth data

The `/auth/get` endpoint returns the bank account and bank identification numbers (such as routing numbers, for US accounts) associated with an Item's checking and savings accounts, along with high-level account data and balances when available.

Note: This request may take some time to complete if `auth` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

Versioning note: In API version 2017-03-08, the schema of the `numbers` object returned by this endpoint is substantially different. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2018-05-22).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#authget>.*/
    pub fn auth_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::AuthGetRequest> {
        FluentRequest {
            client: self,
            params: request::AuthGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
    /**Get transaction data

Note: All new implementations are encouraged to use `/transactions/sync` rather than `/transactions/get`. `/transactions/sync` provides the same functionality as `/transactions/get` and improves developer ease-of-use for handling transactions updates.

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
    ) -> FluentRequest<'_, request::TransactionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsGetRequest {
                access_token: access_token.to_owned(),
                end_date,
                options: None,
                start_date,
            },
        }
    }
    /**Refresh transaction data

`/transactions/refresh` is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled Item. If changes to transactions are discovered after calling `/transactions/refresh`, Plaid will fire a webhook: for `/transactions/sync` users, [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#sync_updates_available) will be fired if there are any transactions updated, added, or removed. For users of both `/transactions/sync` and `/transactions/get`, [`TRANSACTIONS_REMOVED`](https://plaid.com/docs/api/products/transactions/#transactions_removed) will be fired if any removed transactions are detected, and [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/transactions/#default_update) will be fired if any new transactions are detected. New transactions can be fetched by calling `/transactions/get` or `/transactions/sync`. Note that the `/transactions/refresh` endpoint is not supported for Capital One (`ins_128026`) and will result in a `PRODUCT_NOT_SUPPORTED` error if called on an Item from that institution.

`/transactions/refresh` is offered as an add-on to Transactions and has a separate [fee model](/docs/account/billing/#per-request-flat-fee). To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrefresh>.*/
    pub fn transactions_refresh(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::TransactionsRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsRefreshRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Fetch recurring transaction streams

The `/transactions/recurring/get` endpoint allows developers to receive a summary of the recurring outflow and inflow streams (expenses and deposits) from a user’s checking, savings or credit card accounts. Additionally, Plaid provides key insights about each recurring stream including the category, merchant, last amount, and more. Developers can use these insights to build tools and experiences that help their users better manage cash flow, monitor subscriptions, reduce spend, and stay on track with bill payments.

This endpoint is offered as an add-on to Transactions. To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

This endpoint can only be called on an Item that has already been initialized with Transactions (either during Link, by specifying it in `/link/token/create`; or after Link, by calling `/transactions/get` or `/transactions/sync`). For optimal results, we strongly recommend customers using Recurring Transactions to request at least 180 days of history when initializing items with Transactions (using the [`days_requested`](https://plaid.com/docs/api/tokens/#link-token-create-request-transactions-days-requested) option). Once all historical transactions have been fetched, call `/transactions/recurring/get` to receive the Recurring Transactions streams and subscribe to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook. To know when historical transactions have been fetched, if you are using `/transactions/sync` listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#SyncUpdatesAvailableWebhook-historical-update-complete) webhook and check that the `historical_update_complete` field in the payload is `true`. If using `/transactions/get`, listen for the [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhook.

After the initial call, you can call `/transactions/recurring/get` endpoint at any point in the future to retrieve the latest summary of recurring streams. Listen to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook to be notified when new updates are available.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionsrecurringget>.*/
    pub fn transactions_recurring_get(
        &self,
        access_token: &str,
        account_ids: &[&str],
    ) -> FluentRequest<'_, request::TransactionsRecurringGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsRecurringGetRequest {
                access_token: access_token.to_owned(),
                account_ids: account_ids.iter().map(|&x| x.to_owned()).collect(),
                options: None,
            },
        }
    }
    /**Get incremental transaction updates on an Item

The `/transactions/sync` endpoint allows developers to subscribe to all transactions associated with an Item and get updates synchronously in a stream-like manner, using a cursor to track which updates have already been seen.

`/transactions/sync` provides the same functionality as `/transactions/get` and can be used instead of `/transactions/get` to simplify the process of tracking transactions updates. To learn more about migrating from `/transactions/get`, see the [Transactions Sync migration guide](https://plaid.com/docs/transactions/sync-migration/).

This endpoint provides user-authorized transaction data for `credit`, `depository`, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from `investments` accounts, use `/investments/transactions/get` instead.

Returned transactions data is grouped into three types of update, indicating whether the transaction was added, removed, or modified since the last call to the API.

In the first call to `/transactions/sync` for an Item, the endpoint will return all historical transactions data associated with that Item up until the time of the API call (as "adds"), which then generates a `next_cursor` for that Item. In subsequent calls, send the `next_cursor` to receive only the changes that have occurred since the previous call.

Due to the potentially large number of transactions associated with an Item, results are paginated. The `has_more` field specifies if additional calls are necessary to fetch all available transaction updates. Call `/transactions/sync` with the new cursor, pulling all updates, until `has_more` is `false`.

When retrieving paginated updates, track both the `next_cursor` from the latest response and the original cursor from the first call in which `has_more` was `true`; if a call to `/transactions/sync` fails due to the [`TRANSACTIONS_SYNC_MUTATION_DURING_PAGINATION`](https://plaid.com/docs/errors/transactions/#transactions_sync_mutation_during_pagination) error, the entire pagination request loop must be restarted beginning with the cursor for the first page of the update, rather than retrying only the single request that failed.

Whenever new or updated transaction data becomes available, `/transactions/sync` will provide these updates. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. An Item's `status.transactions.last_successful_update` field will show the timestamp of the most recent successful update. To force Plaid to check for new transactions, use the `/transactions/refresh` endpoint.

For newly created Items, data may not be immediately available to `/transactions/sync`. Plaid begins preparing transactions data when the Item is created, but the process can take anywhere from a few seconds to several minutes to complete, depending on the number of transactions available.

To be alerted when new data is available, listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#sync_updates_available) webhook.

`/transactions/sync` does not directly return balance data. To get the balance for an account, call `/accounts/get`, which is a free-to-use endpoint that will return the cached balance as of the last successful transactions update.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#transactionssync>.*/
    pub fn transactions_sync(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::TransactionsSyncRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsSyncRequest {
                access_token: access_token.to_owned(),
                count: None,
                cursor: None,
                options: None,
            },
        }
    }
    /**Enrich locally-held transaction data

The `/transactions/enrich` endpoint enriches raw transaction data generated by your own banking products or retrieved from other non-Plaid sources.

See endpoint docs at <https://plaid.com/docs/api/products/enrich/#transactionsenrich>.*/
    pub fn transactions_enrich(
        &self,
        account_type: &str,
        transactions: Vec<ClientProvidedTransaction>,
    ) -> FluentRequest<'_, request::TransactionsEnrichRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsEnrichRequest {
                account_type: account_type.to_owned(),
                options: None,
                transactions,
            },
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
    ) -> FluentRequest<'_, request::InstitutionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::InstitutionsGetRequest {
                count,
                country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
                offset,
                options: None,
            },
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
    ) -> FluentRequest<'_, request::InstitutionsSearchRequest> {
        FluentRequest {
            client: self,
            params: request::InstitutionsSearchRequest {
                country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
                options: None,
                products: None,
                query: query.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::InstitutionsGetByIdRequest> {
        FluentRequest {
            client: self,
            params: request::InstitutionsGetByIdRequest {
                country_codes: country_codes.iter().map(|&x| x.to_owned()).collect(),
                institution_id: institution_id.to_owned(),
                options: None,
            },
        }
    }
    /**Remove an Item

The `/item/remove` endpoint allows you to remove an Item. Once removed, the `access_token`, as well as any processor tokens or bank account tokens associated with the Item, is no longer valid and cannot be used to access any data that was associated with the Item.

Note that in the Development environment, issuing an `/item/remove`  request will not decrement your live credential count. To increase your credential account in Development, contact Support.

Also note that for certain OAuth-based institutions, an Item removed via `/item/remove` may still show as an active connection in the institution's OAuth permission manager.

API versions 2019-05-29 and earlier return a `removed` boolean as part of the response.

See endpoint docs at <https://plaid.com/docs/api/items/#itemremove>.*/
    pub fn item_remove(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::ItemRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::ItemRemoveRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Retrieve accounts

The `/accounts/get` endpoint can be used to retrieve a list of accounts associated with any linked Item. Plaid will only return active bank accounts — that is, accounts that are not closed and are capable of carrying a balance.
For items that went through the updated account selection pane, this endpoint only returns accounts that were permissioned by the user when they initially created the Item. If a user creates a new account after the initial link, you can capture this event through the [`NEW_ACCOUNTS_AVAILABLE`](https://plaid.com/docs/api/items/#new_accounts_available) webhook and then use Link's [update mode](https://plaid.com/docs/link/update-mode/) to request that the user share this new account with you.

`/accounts/get` is free to use and retrieves cached information, rather than extracting fresh information from the institution. The balance returned will reflect the balance at the time of the last successful Item update. If the Item is enabled for a regularly updating product, such as Transactions, Investments, or Liabilities, the balance will typically update about once a day, as long as the Item is healthy. If the Item is enabled only for products that do not frequently update, such as Auth or Identity, balance data may be much older.

For realtime balance information, use the paid endpoint `/accounts/balance/get` instead.

See endpoint docs at <https://plaid.com/docs/api/accounts/#accountsget>.*/
    pub fn accounts_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::AccountsGetRequest> {
        FluentRequest {
            client: self,
            params: request::AccountsGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
    /**Get categories

Send a request to the `/categories/get` endpoint to get detailed information on categories returned by Plaid. This endpoint does not require authentication.

All implementations are recommended to use the newer `personal_finance_category` taxonomy instead of the older `category` taxonomy supported by this endpoint. The [`personal_finance_category taxonomy` CSV file](https://plaid.com/documents/transactions-personal-finance-category-taxonomy.csv) is available for download and is not accessible via API.

See endpoint docs at <https://plaid.com/docs/api/products/transactions/#categoriesget>.*/
    pub fn categories_get(&self) -> FluentRequest<'_, request::CategoriesGetRequest> {
        FluentRequest {
            client: self,
            params: request::CategoriesGetRequest {},
        }
    }
    /**Create a test Item and processor token

Use the `/sandbox/processor_token/create` endpoint to create a valid `processor_token` for an arbitrary institution ID and test credentials. The created `processor_token` corresponds to a new Sandbox Item. You can then use this `processor_token` with the `/processor/` API endpoints in Sandbox. You can also use `/sandbox/processor_token/create` with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxprocessor_tokencreate>.*/
    pub fn sandbox_processor_token_create(
        &self,
        institution_id: &str,
    ) -> FluentRequest<'_, request::SandboxProcessorTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxProcessorTokenCreateRequest {
                institution_id: institution_id.to_owned(),
                options: None,
            },
        }
    }
    /**Create a test Item

Use the `/sandbox/public_token/create` endpoint to create a valid `public_token`  for an arbitrary institution ID, initial products, and test credentials. The created `public_token` maps to a new Sandbox Item. You can then call `/item/public_token/exchange` to exchange the `public_token` for an `access_token` and perform all API actions. `/sandbox/public_token/create` can also be used with the [`user_custom` test username](https://plaid.com/docs/sandbox/user-custom) to generate a test account with custom data. `/sandbox/public_token/create` cannot be used with OAuth institutions.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxpublic_tokencreate>.*/
    pub fn sandbox_public_token_create(
        &self,
        initial_products: &[&str],
        institution_id: &str,
    ) -> FluentRequest<'_, request::SandboxPublicTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxPublicTokenCreateRequest {
                initial_products: initial_products
                    .iter()
                    .map(|&x| x.to_owned())
                    .collect(),
                institution_id: institution_id.to_owned(),
                options: None,
                user_token: None,
            },
        }
    }
    /**Fire a test webhook

The `/sandbox/item/fire_webhook` endpoint is used to test that code correctly handles webhooks. This endpoint can trigger the following webhooks:

`DEFAULT_UPDATE`: Transactions update webhook to be fired for a given Sandbox Item. If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`NEW_ACCOUNTS_AVAILABLE`: Webhook to be fired for a given Sandbox Item created with Account Select v2.

`AUTH_DATA_UPDATE`: Webhook to be fired for a given Sandbox Item created with Auth as an enabled product.

`LOGIN_REPAIRED`: Fired when an Item recovers from the `ITEM_LOGIN_REQUIRED` without the user going through update mode in your app.

`RECURRING_TRANSACTIONS_UPDATE`: Recurring Transactions webhook to be fired for a given Sandbox Item. If the Item does not support Recurring Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`SYNC_UPDATES_AVAILABLE`: Transactions webhook to be fired for a given Sandbox Item.  If the Item does not support Transactions, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`PRODUCT_READY`: Assets webhook to be fired when a given asset report has been successfully generated. If the Item does not support Assets, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

`ERROR`: Assets webhook to be fired when asset report generation has failed. If the Item does not support Assets, a `SANDBOX_PRODUCT_NOT_ENABLED` error will result.

Note that this endpoint is provided for developer ease-of-use and is not required for testing webhooks; webhooks will also fire in Sandbox under the same conditions that they would in Production or Development (except for webhooks of type `TRANSFER`).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemfire_webhook>.*/
    pub fn sandbox_item_fire_webhook(
        &self,
        access_token: &str,
        webhook_code: &str,
    ) -> FluentRequest<'_, request::SandboxItemFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxItemFireWebhookRequest {
                access_token: access_token.to_owned(),
                webhook_code: webhook_code.to_owned(),
                webhook_type: None,
            },
        }
    }
    /**Retrieve real-time balance data

The `/accounts/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints, such as `/accounts/get`, return a balance object, only `/accounts/balance/get` forces the available and current balance fields to be refreshed rather than cached. This endpoint can be used for existing Items that were added via any of Plaid’s other products. This endpoint can be used as long as Link has been initialized with any other product, `balance` itself is not a product that can be used to initialize Link. As this endpoint triggers a synchronous request for fresh data, latency may be higher than for other Plaid endpoints (typically less than 10 seconds, but occasionally up to 30 seconds or more); if you encounter errors, you may find it necessary to adjust your timeout period when making requests.

See endpoint docs at <https://plaid.com/docs/api/products/balance/#accountsbalanceget>.*/
    pub fn accounts_balance_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::AccountsBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: request::AccountsBalanceGetRequest {
                access_token: access_token.to_owned(),
                options: None,
                payment_details: None,
            },
        }
    }
    /**Retrieve identity data

The `/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses. Only name data is guaranteed to be returned; other fields will be empty arrays if not provided by the institution.

This request may take some time to complete if identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

Note: In API versions 2018-05-22 and earlier, the `owners` object is not returned, and instead identity information is returned in the top level `identity` object. For more details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2019-05-29).

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identityget>.*/
    pub fn identity_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::IdentityGetRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
    /**Retrieve identity match score

The `/identity/match` endpoint generates a match score, which indicates how well the provided identity data matches the identity information on file with the account holder's financial institution.

Fields within the `balances` object will always be null when retrieved by `/identity/match`. Instead, use the free `/accounts/get` endpoint to request balance cached data, or `/accounts/balance/get` for real-time data.

This request may take some time to complete if Identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identitymatch>.*/
    pub fn identity_match(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::IdentityMatchRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityMatchRequest {
                access_token: access_token.to_owned(),
                options: None,
                user: None,
            },
        }
    }
    /**Refresh identity data

`/identity/refresh` is an optional endpoint for users of the Identity product. It initiates an on-demand extraction to fetch the most up to date Identity information from the Financial Institution. This on-demand extraction takes place in addition to the periodic extractions that automatically occur any Identity-enabled Item. If changes to Identity are discovered after calling `/identity/refresh`, Plaid will fire a webhook [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/identity/#default_update).
`/identity/refresh` is offered as an add-on to Identity and has a separate [fee model](/docs/account/billing/#per-request-flat-fee). To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/identity/#identityrefresh>.*/
    pub fn identity_refresh(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::IdentityRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityRefreshRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Retrieve a dashboard user

Retrieve information about a dashboard user.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#dashboard_userget>.*/
    pub fn dashboard_user_get(
        &self,
        dashboard_user_id: &str,
    ) -> FluentRequest<'_, request::DashboardUserGetRequest> {
        FluentRequest {
            client: self,
            params: request::DashboardUserGetRequest {
                dashboard_user_id: dashboard_user_id.to_owned(),
            },
        }
    }
    /**List dashboard users

List all dashboard users associated with your account.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#dashboard_userlist>.*/
    pub fn dashboard_user_list(
        &self,
    ) -> FluentRequest<'_, request::DashboardUserListRequest> {
        FluentRequest {
            client: self,
            params: request::DashboardUserListRequest {
                cursor: None,
            },
        }
    }
    /**Create a new Identity Verification

Create a new Identity Verification for the user specified by the `client_user_id` field. The requirements and behavior of the verification are determined by the `template_id` provided.
If you don't know whether the associated user already has an active Identity Verification, you can specify `"is_idempotent": true` in the request body. With idempotency enabled, a new Identity Verification will only be created if one does not already exist for the associated `client_user_id` and `template_id`. If an Identity Verification is found, it will be returned unmodified with an `200 OK` HTTP status code.

You can also use this endpoint to supply information you already have collected about the user; if any of these fields are specified, the screens prompting the user to enter them will be skipped during the Link flow.


See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationcreate>.*/
    pub fn identity_verification_create(
        &self,
        gave_consent: bool,
        is_shareable: bool,
        template_id: &str,
    ) -> FluentRequest<'_, request::IdentityVerificationCreateRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityVerificationCreateRequest {
                client_user_id: None,
                gave_consent,
                is_idempotent: None,
                is_shareable,
                template_id: template_id.to_owned(),
                user: None,
            },
        }
    }
    /**Retrieve Identity Verification

Retrieve a previously created Identity Verification.

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationget>.*/
    pub fn identity_verification_get(
        &self,
        identity_verification_id: &str,
    ) -> FluentRequest<'_, request::IdentityVerificationGetRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityVerificationGetRequest {
                identity_verification_id: identity_verification_id.to_owned(),
            },
        }
    }
    /**List Identity Verifications

Filter and list Identity Verifications created by your account

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationlist>.*/
    pub fn identity_verification_list(
        &self,
        client_user_id: &str,
        template_id: &str,
    ) -> FluentRequest<'_, request::IdentityVerificationListRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityVerificationListRequest {
                client_user_id: client_user_id.to_owned(),
                cursor: None,
                template_id: template_id.to_owned(),
            },
        }
    }
    /**Retry an Identity Verification

Allow a customer to retry their Identity Verification

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationretry>.*/
    pub fn identity_verification_retry(
        &self,
        client_user_id: &str,
        strategy: &str,
        template_id: &str,
    ) -> FluentRequest<'_, request::IdentityVerificationRetryRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityVerificationRetryRequest {
                client_user_id: client_user_id.to_owned(),
                steps: None,
                strategy: strategy.to_owned(),
                template_id: template_id.to_owned(),
                user: None,
            },
        }
    }
    /**Create a watchlist screening for an entity

Create a new entity watchlist screening to check your customer against watchlists defined in the associated entity watchlist program. If your associated program has ongoing screening enabled, this is the profile information that will be used to monitor your customer over time.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentitycreate>.*/
    pub fn watchlist_screening_entity_create(
        &self,
        search_terms: EntityWatchlistSearchTerms,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityCreateRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityCreateRequest {
                client_user_id: None,
                search_terms,
            },
        }
    }
    /**Get an entity screening

Retrieve an entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityget>.*/
    pub fn watchlist_screening_entity_get(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityGetRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityGetRequest {
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List history for entity watchlist screenings

List all changes to the entity watchlist screening in reverse-chronological order. If the watchlist screening has not been edited, no history will be returned.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhistorylist>.*/
    pub fn watchlist_screening_entity_history_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityHistoryListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityHistoryListRequest {
                cursor: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List hits for entity watchlist screenings

List all hits for the entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityhitlist>.*/
    pub fn watchlist_screening_entity_hit_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityHitListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityHitListRequest {
                cursor: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List entity watchlist screenings

List all entity screenings.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentitylist>.*/
    pub fn watchlist_screening_entity_list(
        &self,
        entity_watchlist_program_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityListRequest {
                assignee: None,
                client_user_id: None,
                cursor: None,
                entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
                status: None,
            },
        }
    }
    /**Get entity watchlist screening program

Get an entity watchlist screening program

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityprogramget>.*/
    pub fn watchlist_screening_entity_program_get(
        &self,
        entity_watchlist_program_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityProgramGetRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityProgramGetRequest {
                entity_watchlist_program_id: entity_watchlist_program_id.to_owned(),
            },
        }
    }
    /**List entity watchlist screening programs

List all entity watchlist screening programs

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityprogramlist>.*/
    pub fn watchlist_screening_entity_program_list(
        &self,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityProgramListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityProgramListRequest {
                cursor: None,
            },
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
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityReviewCreateRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityReviewCreateRequest {
                comment: None,
                confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
                dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List reviews for entity watchlist screenings

List all reviews for a particular entity watchlist screening. Reviews are compliance reports created by users in your organization regarding the relevance of potential hits found by Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityreviewlist>.*/
    pub fn watchlist_screening_entity_review_list(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityReviewListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityReviewListRequest {
                cursor: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
            },
        }
    }
    /**Update an entity screening

Update an entity watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningentityupdate>.*/
    pub fn watchlist_screening_entity_update(
        &self,
        entity_watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningEntityUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningEntityUpdateRequest {
                assignee: None,
                client_user_id: None,
                entity_watchlist_screening_id: entity_watchlist_screening_id.to_owned(),
                reset_fields: None,
                search_terms: None,
                status: None,
            },
        }
    }
    /**Create a watchlist screening for a person

Create a new Watchlist Screening to check your customer against watchlists defined in the associated Watchlist Program. If your associated program has ongoing screening enabled, this is the profile information that will be used to monitor your customer over time.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualcreate>.*/
    pub fn watchlist_screening_individual_create(
        &self,
        search_terms: WatchlistScreeningRequestSearchTerms,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualCreateRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualCreateRequest {
                client_user_id: None,
                search_terms,
            },
        }
    }
    /**Retrieve an individual watchlist screening

Retrieve a previously created individual watchlist screening

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualget>.*/
    pub fn watchlist_screening_individual_get(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualGetRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualGetRequest {
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List history for individual watchlist screenings

List all changes to the individual watchlist screening in reverse-chronological order. If the watchlist screening has not been edited, no history will be returned.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualhistorylist>.*/
    pub fn watchlist_screening_individual_history_list(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualHistoryListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualHistoryListRequest {
                cursor: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List hits for individual watchlist screening

List all hits found by Plaid for a particular individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualhitlist>.*/
    pub fn watchlist_screening_individual_hit_list(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualHitListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualHitListRequest {
                cursor: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List Individual Watchlist Screenings

List previously created watchlist screenings for individuals

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividuallist>.*/
    pub fn watchlist_screening_individual_list(
        &self,
        watchlist_program_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualListRequest {
                assignee: None,
                client_user_id: None,
                cursor: None,
                status: None,
                watchlist_program_id: watchlist_program_id.to_owned(),
            },
        }
    }
    /**Get individual watchlist screening program

Get an individual watchlist screening program

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualprogramget>.*/
    pub fn watchlist_screening_individual_program_get(
        &self,
        watchlist_program_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualProgramGetRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualProgramGetRequest {
                watchlist_program_id: watchlist_program_id.to_owned(),
            },
        }
    }
    /**List individual watchlist screening programs

List all individual watchlist screening programs

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualprogramlist>.*/
    pub fn watchlist_screening_individual_program_list(
        &self,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualProgramListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualProgramListRequest {
                cursor: None,
            },
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
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualReviewCreateRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualReviewCreateRequest {
                comment: None,
                confirmed_hits: confirmed_hits.iter().map(|&x| x.to_owned()).collect(),
                dismissed_hits: dismissed_hits.iter().map(|&x| x.to_owned()).collect(),
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
    /**List reviews for individual watchlist screenings

List all reviews for the individual watchlist screening.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualreviewlist>.*/
    pub fn watchlist_screening_individual_review_list(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualReviewListRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualReviewListRequest {
                cursor: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
    /**Update individual watchlist screening

Update a specific individual watchlist screening. This endpoint can be used to add additional customer information, correct outdated information, add a reference id, assign the individual to a reviewer, and update which program it is associated with. Please note that you may not update `search_terms` and `status` at the same time since editing `search_terms` may trigger an automatic `status` change.

See endpoint docs at <https://plaid.com/docs/api/products/monitor/#watchlist_screeningindividualupdate>.*/
    pub fn watchlist_screening_individual_update(
        &self,
        watchlist_screening_id: &str,
    ) -> FluentRequest<'_, request::WatchlistScreeningIndividualUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::WatchlistScreeningIndividualUpdateRequest {
                assignee: None,
                client_user_id: None,
                reset_fields: None,
                search_terms: None,
                status: None,
                watchlist_screening_id: watchlist_screening_id.to_owned(),
            },
        }
    }
    /**Create a Beacon User

Create and scan a Beacon User against your Beacon Program, according to your program's settings.

When you submit a new user to `/beacon/user/create`, several checks are performed immediately:

  - The user's PII (provided within the `user` object) is searched against all other users within the Beacon Program you specified. If a match is found that violates your program's "Duplicate Information Filtering" settings, the user will be returned with a status of `pending_review`.

  - The user's PII is also searched against all fraud reports created by your organization across all of your Beacon Programs. If the user's data matches a fraud report that your team created, the user will be returned with a status of `rejected`.

  - Finally, the user's PII is searched against all fraud report shared with the Beacon Network by other companies. If a matching fraud report is found, the user will be returned with a `pending_review` status if your program has enabled automatic flagging based on network fraud.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconusercreate>.*/
    pub fn beacon_user_create(
        &self,
        client_user_id: &str,
        program_id: &str,
        user: BeaconUserRequestData,
    ) -> FluentRequest<'_, request::BeaconUserCreateRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconUserCreateRequest {
                client_user_id: client_user_id.to_owned(),
                program_id: program_id.to_owned(),
                user,
            },
        }
    }
    /**Get a Beacon User

Fetch a Beacon User.

The Beacon User is returned with all of their associated information and a `status` based on the Beacon Network duplicate record and fraud checks.


See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserget>.*/
    pub fn beacon_user_get(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, request::BeaconUserGetRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconUserGetRequest {
                beacon_user_id: beacon_user_id.to_owned(),
            },
        }
    }
    /**Review a Beacon User

Update the status of a Beacon User.

When updating a Beacon User's status via this endpoint, Plaid validates that the status change is consistent with the related state for this Beacon User. Specifically, we will check:

1. Whether there are any associated Beacon Reports connected to the Beacon User, and
2. Whether there are any confirmed Beacon Report Syndications connected to the Beacon User.

When updating a Beacon User's status to "rejected", we enforce that either a Beacon Report has been created for the Beacon User or a Beacon Report Syndication has been confirmed.
When updating a Beacon User's status to "cleared", we enforce that there are no active Beacon Reports or confirmed Beacon Report Syndications associated with the user. If you previously created a Beacon Report for this user, you must delete it before updating the Beacon User's status to "cleared".
There are no restrictions on updating a Beacon User's status to "pending_review".

If these conditions are not met, the request will be rejected with an error explaining the issue.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserreview>.*/
    pub fn beacon_user_review(
        &self,
        beacon_user_id: &str,
        status: &str,
    ) -> FluentRequest<'_, request::BeaconUserReviewRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconUserReviewRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                status: status.to_owned(),
            },
        }
    }
    /**Create a Beacon Report

Create a fraud report for a given Beacon User.

Note: If you are creating users with the express purpose of providing historical fraud data, you should use the `/beacon/user/create` endpoint instead and embed the fraud report in the request. This will ensure that the Beacon User you create will not be subject to any billing costs.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportcreate>.*/
    pub fn beacon_report_create(
        &self,
        beacon_user_id: &str,
        fraud_date: chrono::NaiveDate,
        type_: &str,
    ) -> FluentRequest<'_, request::BeaconReportCreateRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconReportCreateRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                fraud_amount: None,
                fraud_date,
                type_: type_.to_owned(),
            },
        }
    }
    /**List Beacon Reports for a Beacon User

Use the `/beacon/report/list` endpoint to view all Beacon Reports you created for a specific Beacon User. The reports returned by this endpoint are exclusively reports you created for a specific user. A Beacon User can only have one active report at a time, but a new report can be created if a previous report has been deleted. The results from this endpoint are paginated; the `next_cursor` field will be populated if there is another page of results that can be retrieved. To fetch the next page, pass the `next_cursor` value as the `cursor` parameter in the next request.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportlist>.*/
    pub fn beacon_report_list(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, request::BeaconReportListRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconReportListRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                cursor: None,
            },
        }
    }
    /**List Beacon Report Syndications for a Beacon User

Use the `/beacon/report_syndication/list` endpoint to view all Beacon Reports that have been syndicated to a specific Beacon User. This endpoint returns Beacon Report Syndications which are references to Beacon Reports created either by you, or another Beacon customer, that matched the specified Beacon User. A Beacon User can have multiple active Beacon Report Syndications at once. The results from this endpoint are paginated; the `next_cursor` field will be populated if there is another page of results that can be retrieved. To fetch the next page, pass the `next_cursor` value as the `cursor` parameter in the next request.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportsyndicationlist>.*/
    pub fn beacon_report_syndication_list(
        &self,
        beacon_user_id: &str,
    ) -> FluentRequest<'_, request::BeaconReportSyndicationListRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconReportSyndicationListRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                cursor: None,
            },
        }
    }
    /**Get a Beacon Report

Returns a Beacon report for a given Beacon report id.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportget>.*/
    pub fn beacon_report_get(
        &self,
        beacon_report_id: &str,
    ) -> FluentRequest<'_, request::BeaconReportGetRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconReportGetRequest {
                beacon_report_id: beacon_report_id.to_owned(),
            },
        }
    }
    /**Get a Beacon Report Syndication

Returns a Beacon Report Syndication for a given Beacon Report Syndication id.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconreportsyndicationget>.*/
    pub fn beacon_report_syndication_get(
        &self,
        beacon_report_syndication_id: &str,
    ) -> FluentRequest<'_, request::BeaconReportSyndicationGetRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconReportSyndicationGetRequest {
                beacon_report_syndication_id: beacon_report_syndication_id.to_owned(),
            },
        }
    }
    /**Update the identity data of a Beacon User

Update the identity data for a Beacon User in your Beacon Program.

Similar to `/beacon/user/create`, several checks are performed immediately when you submit a change to `/beacon/user/update`:

  - The user's updated PII is searched against all other users within the Beacon Program you specified. If a match is found that violates your program's "Duplicate Information Filtering" settings, the user will be returned with a status of `pending_review`.

  - The user's updated PII is also searched against all fraud reports created by your organization across all of your Beacon Programs. If the user's data matches a fraud report that your team created, the user will be returned with a status of `rejected`.

  - Finally, the user's PII is searched against all fraud report shared with the Beacon Network by other companies. If a matching fraud report is found, the user will be returned with a `pending_review` status if your program has enabled automatic flagging based on network fraud.

Plaid maintains a version history for each Beacon User, so the Beacon User's identity data before and after the update is retained as separate versions.

See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconuserupdate>.*/
    pub fn beacon_user_update(
        &self,
        beacon_user_id: &str,
        user: BeaconUserUpdateRequestData,
    ) -> FluentRequest<'_, request::BeaconUserUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconUserUpdateRequest {
                beacon_user_id: beacon_user_id.to_owned(),
                user,
            },
        }
    }
    /**Get a Beacon Duplicate

Returns a Beacon Duplicate for a given Beacon Duplicate id.

A Beacon Duplicate represents a pair of similar Beacon Users within your organization.

Two Beacon User revisions are returned for each Duplicate record in either the `beacon_user1` or `beacon_user2` response fields.

The `analysis` field in the response indicates which fields matched between `beacon_user1` and `beacon_user2`.


See endpoint docs at <https://plaid.com/docs/api/products/beacon/#beaconduplicateget>.*/
    pub fn beacon_duplicate_get(
        &self,
        beacon_duplicate_id: &str,
    ) -> FluentRequest<'_, request::BeaconDuplicateGetRequest> {
        FluentRequest {
            client: self,
            params: request::BeaconDuplicateGetRequest {
                beacon_duplicate_id: beacon_duplicate_id.to_owned(),
            },
        }
    }
    /**Create autofill for an Identity Verification

Try to autofill an Identity Verification based of the provided phone number, date of birth and country of residence.

See endpoint docs at <https://plaid.com/docs/api/products/identity-verification/#identity_verificationautofillcreate>.*/
    pub fn identity_verification_autofill_create(
        &self,
        identity_verification_id: &str,
    ) -> FluentRequest<'_, request::IdentityVerificationAutofillCreateRequest> {
        FluentRequest {
            client: self,
            params: request::IdentityVerificationAutofillCreateRequest {
                identity_verification_id: identity_verification_id.to_owned(),
            },
        }
    }
    /**Retrieve Auth data

The `/processor/auth/get` endpoint returns the bank account and bank identification number (such as the routing number, for US accounts), for a checking or savings account that''s associated with a given `processor_token`. The endpoint also returns high-level account data and balances when available.

Versioning note: API versions 2019-05-29 and earlier use a different schema for the `numbers` object returned by this endpoint. For details, see [Plaid API versioning](https://plaid.com/docs/api/versioning/#version-2020-09-14).


See endpoint docs at <https://plaid.com/docs/api/processors/#processorauthget>.*/
    pub fn processor_auth_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorAuthGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorAuthGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Retrieve the account associated with a processor token

This endpoint returns the account associated with a given processor token.

This endpoint retrieves cached information, rather than extracting fresh information from the institution. As a result, the account balance returned may not be up-to-date; for realtime balance information, use `/processor/balance/get` instead. Note that some information is nullable.


See endpoint docs at <https://plaid.com/docs/api/processors/#processoraccountget>.*/
    pub fn processor_account_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorAccountGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorAccountGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Get transaction data

The `/processor/transactions/get` endpoint allows developers to receive user-authorized transaction data for credit, depository, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). Transaction data is standardized across financial institutions, and in many cases transactions are linked to a clean name, entity type, location, and category. Similarly, account data is standardized and returned with a clean name, number, balance, and other meta information where available.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.  Transactions are not immutable and can also be removed altogether by the institution; a removed transaction will no longer appear in `/processor/transactions/get`.  For more details, see [Pending and posted transactions](https://plaid.com/docs/transactions/transactions-data/#pending-and-posted-transactions).

Due to the potentially large number of transactions associated with a processor token, results are paginated. Manipulate the `count` and `offset` parameters in conjunction with the `total_transactions` response body field to fetch all available transactions.

Data returned by `/processor/transactions/get` will be the data available for the processor token as of the most recent successful check for new transactions. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. To force Plaid to check for new transactions, you can use the `/processor/transactions/refresh` endpoint.

Note that data may not be immediately available to `/processor/transactions/get`. Plaid will begin to prepare transactions data upon Item link, if Link was initialized with `transactions`, or upon the first call to `/processor/transactions/get`, if it wasn't. If no transaction history is ready when `/processor/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

To receive Transactions webhooks for a processor token, set its webhook URL via the [`/processor/token/webhook/update`](https://plaid.com/docs/api/processors/#processortokenwebhookupdate) endpoint.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortransactionsget>.*/
    pub fn processor_transactions_get(
        &self,
        end_date: chrono::NaiveDate,
        processor_token: &str,
        start_date: chrono::NaiveDate,
    ) -> FluentRequest<'_, request::ProcessorTransactionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTransactionsGetRequest {
                end_date,
                options: None,
                processor_token: processor_token.to_owned(),
                start_date,
            },
        }
    }
    /**Get incremental transaction updates on a processor token

This endpoint replaces `/processor/transactions/get` and its associated webhooks for most common use-cases.

The `/processor/transactions/sync` endpoint allows developers to subscribe to all transactions associated with a processor token and get updates synchronously in a stream-like manner, using a cursor to track which updates have already been seen. `/processor/transactions/sync` provides the same functionality as `/processor/transactions/get` and can be used instead of `/processor/transactions/get` to simplify the process of tracking transactions updates.

This endpoint provides user-authorized transaction data for `credit`, `depository`, and some loan-type accounts (only those with account subtype `student`; coverage may be limited). For transaction history from `investments` accounts, use `/investments/transactions/get` instead.

Returned transactions data is grouped into three types of update, indicating whether the transaction was added, removed, or modified since the last call to the API.

In the first call to `/processor/transactions/sync` for a processor token, the endpoint will return all historical transactions data associated with that processor token up until the time of the API call (as "adds"), which then generates a `next_cursor` for that processor token. In subsequent calls, send the `next_cursor` to receive only the changes that have occurred since the previous call.

Due to the potentially large number of transactions associated with a processor token, results are paginated. The `has_more` field specifies if additional calls are necessary to fetch all available transaction updates. Call `/processor/transactions/sync` with the new cursor, pulling all updates, until `has_more` is `false`.

When retrieving paginated updates, track both the `next_cursor` from the latest response and the original cursor from the first call in which `has_more` was `true`; if a call to `/processor/transactions/sync` fails when retrieving a paginated update, which can occur as a result of the [`TRANSACTIONS_SYNC_MUTATION_DURING_PAGINATION`](https://plaid.com/docs/errors/transactions/#transactions_sync_mutation_during_pagination) error, the entire pagination request loop must be restarted beginning with the cursor for the first page of the update, rather than retrying only the single request that failed.

Whenever new or updated transaction data becomes available, `/processor/transactions/sync` will provide these updates. Plaid typically checks for new data multiple times a day, but these checks may occur less frequently, such as once a day, depending on the institution. To force Plaid to check for new transactions, use the `/processor/transactions/refresh` endpoint.

Note that for newly created processor tokens, data may not be immediately available to `/processor/transactions/sync`. Plaid begins preparing transactions data when the corresponding Item is created, but the process can take anywhere from a few seconds to several minutes to complete, depending on the number of transactions available.

To receive Transactions webhooks for a processor token, set its webhook URL via the [`/processor/token/webhook/update`](https://plaid.com/docs/api/processors/#processortokenwebhookupdate) endpoint.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortransactionssync>.*/
    pub fn processor_transactions_sync(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorTransactionsSyncRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTransactionsSyncRequest {
                count: None,
                cursor: None,
                options: None,
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Refresh transaction data

`/processor/transactions/refresh` is an optional endpoint for users of the Transactions product. It initiates an on-demand extraction to fetch the newest transactions for a processor token. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Transactions-enabled processor token. If changes to transactions are discovered after calling `/processor/transactions/refresh`, Plaid will fire a webhook: for `/transactions/sync` users, [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#sync_updates_available) will be fired if there are any transactions updated, added, or removed. For users of both `/processor/transactions/sync` and `/processor/transactions/get`, [`TRANSACTIONS_REMOVED`](https://plaid.com/docs/api/products/transactions/#transactions_removed) will be fired if any removed transactions are detected, and [`DEFAULT_UPDATE`](https://plaid.com/docs/api/products/transactions/#default_update) will be fired if any new transactions are detected. New transactions can be fetched by calling `/processor/transactions/get` or `/processor/transactions/sync`. Note that the `/processor/transactions/refresh` endpoint is not supported for Capital One (`ins_128026`) and will result in a `PRODUCT_NOT_SUPPORTED` error if called on a processor token from that institution.

`/processor/transactions/refresh` is offered as an add-on to Transactions and has a separate [fee model](/docs/account/billing/#per-request-flat-fee). To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortransactionsrefresh>.*/
    pub fn processor_transactions_refresh(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorTransactionsRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTransactionsRefreshRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Fetch recurring transaction streams

The `/processor/transactions/recurring/get` endpoint allows developers to receive a summary of the recurring outflow and inflow streams (expenses and deposits) from a user’s checking, savings or credit card accounts. Additionally, Plaid provides key insights about each recurring stream including the category, merchant, last amount, and more. Developers can use these insights to build tools and experiences that help their users better manage cash flow, monitor subscriptions, reduce spend, and stay on track with bill payments.

This endpoint is offered as an add-on to Transactions. To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

This endpoint can only be called on a processor token that has already been initialized with Transactions (either during Link, by specifying it in `/link/token/create`; or after Link, by calling `/processor/transactions/get` or `/processor/transactions/sync`). Once all historical transactions have been fetched, call `/processor/transactions/recurring/get` to receive the Recurring Transactions streams and subscribe to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook. To know when historical transactions have been fetched, if you are using `/processor/transactions/sync` listen for the [`SYNC_UPDATES_AVAILABLE`](https://plaid.com/docs/api/products/transactions/#SyncUpdatesAvailableWebhook-historical-update-complete) webhook and check that the `historical_update_complete` field in the payload is `true`. If using `/processor/transactions/get`, listen for the [`HISTORICAL_UPDATE`](https://plaid.com/docs/api/products/transactions/#historical_update) webhook.

After the initial call, you can call `/processor/transactions/recurring/get` endpoint at any point in the future to retrieve the latest summary of recurring streams. Listen to the [`RECURRING_TRANSACTIONS_UPDATE`](https://plaid.com/docs/api/products/transactions/#recurring_transactions_update) webhook to be notified when new updates are available.

To receive Transactions webhooks for a processor token, set its webhook URL via the [`/processor/token/webhook/update`](https://plaid.com/docs/api/processors/#processortokenwebhookupdate) endpoint.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortransactionsrecurringget>.*/
    pub fn processor_transactions_recurring_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorTransactionsRecurringGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTransactionsRecurringGetRequest {
                options: None,
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Evaluate a planned ACH transaction

Use `/processor/signal/evaluate` to evaluate a planned ACH transaction as a processor to get a return risk assessment (such as a risk score and risk tier) and additional risk signals.

In order to obtain a valid score for an ACH transaction, Plaid must have an access token for the account, and the Item must be healthy (receiving product updates) or have recently been in a healthy state. If the transaction does not meet eligibility requirements, an error will be returned corresponding to the underlying cause. If `/processor/signal/evaluate` is called on the same transaction multiple times within a 24-hour period, cached results may be returned. For more information please refer to our error documentation on [item errors](/docs/errors/item/) and [Link in Update Mode](/docs/link/update-mode/).

Note: This request may take some time to complete if Signal is being added to an existing Item. This is because Plaid must communicate directly with the institution when retrieving the data for the first time. To reduce this latency, you can call `/signal/prepare` on the Item before you need to request Signal data.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorsignalevaluate>.*/
    pub fn processor_signal_evaluate(
        &self,
        amount: f64,
        client_transaction_id: &str,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorSignalEvaluateRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorSignalEvaluateRequest {
                amount,
                client_transaction_id: client_transaction_id.to_owned(),
                client_user_id: None,
                default_payment_method: None,
                device: None,
                is_recurring: None,
                processor_token: processor_token.to_owned(),
                user: None,
                user_present: None,
            },
        }
    }
    /**Report whether you initiated an ACH transaction

After calling `/processor/signal/evaluate`, call `/processor/signal/decision/report` to report whether the transaction was initiated.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorsignaldecisionreport>.*/
    pub fn processor_signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorSignalDecisionReportRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorSignalDecisionReportRequest {
                amount_instantly_available: None,
                client_transaction_id: client_transaction_id.to_owned(),
                days_funds_on_hold: None,
                decision_outcome: None,
                initiated,
                payment_method: None,
                processor_token: processor_token.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::ProcessorSignalReturnReportRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorSignalReturnReportRequest {
                client_transaction_id: client_transaction_id.to_owned(),
                processor_token: processor_token.to_owned(),
                return_code: return_code.to_owned(),
                returned_at: None,
            },
        }
    }
    /**Opt-in a processor token to Signal

When a processor token is not initialized with Signal, call `/processor/signal/prepare` to opt-in that processor token to the Signal data collection process, which will improve the accuracy of the Signal score.

If this endpoint is called with a processor token that is already initialized with Signal, it will return a 200 response and will not modify the processor token.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorsignalprepare>.*/
    pub fn processor_signal_prepare(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorSignalPrepareRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorSignalPrepareRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Create a bank transfer as a processor

Use the `/processor/bank_transfer/create` endpoint to initiate a new bank transfer as a processor

See endpoint docs at <https://plaid.com/docs/api/processors/#bank_transfercreate>.*/
    pub fn processor_bank_transfer_create(
        &self,
        args: request::ProcessorBankTransferCreateRequired,
    ) -> FluentRequest<'_, request::ProcessorBankTransferCreateRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorBankTransferCreateRequest {
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
            },
        }
    }
    /**Retrieve Liabilities data

The `/processor/liabilities/get` endpoint returns various details about a loan or credit account. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`.

The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/processor/liabilities/get`.

Note: This request may take some time to complete if `liabilities` was not specified as an initial product when creating the processor token. This is because Plaid must communicate directly with the institution to retrieve the additional data.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorliabilitiesget>.*/
    pub fn processor_liabilities_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorLiabilitiesGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorLiabilitiesGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Retrieve Identity data

The `/processor/identity/get` endpoint allows you to retrieve various account holder information on file with the financial institution, including names, emails, phone numbers, and addresses.

See endpoint docs at <https://plaid.com/docs/api/processors/#processoridentityget>.*/
    pub fn processor_identity_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorIdentityGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorIdentityGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Retrieve identity match score

The `/processor/identity/match` endpoint generates a match score, which indicates how well the provided identity data matches the identity information on file with the account holder's financial institution.

Fields within the `balances` object will always be null when retrieved by `/identity/match`. Instead, use the free `/accounts/get` endpoint to request balance cached data, or `/accounts/balance/get` for real-time data.

This request may take some time to complete if Identity was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the data.

See endpoint docs at <https://plaid.com/docs/api/processors/#processoridentitymatch>.*/
    pub fn processor_identity_match(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorIdentityMatchRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorIdentityMatchRequest {
                processor_token: processor_token.to_owned(),
                user: None,
            },
        }
    }
    /**Retrieve Balance data

The `/processor/balance/get` endpoint returns the real-time balance for each of an Item's accounts. While other endpoints may return a balance object, only `/processor/balance/get` forces the available and current balance fields to be refreshed rather than cached.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorbalanceget>.*/
    pub fn processor_balance_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorBalanceGetRequest {
                options: None,
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Update Webhook URL

The POST `/item/webhook/update` allows you to update the webhook URL associated with an Item. This request triggers a [`WEBHOOK_UPDATE_ACKNOWLEDGED`](https://plaid.com/docs/api/items/#webhook_update_acknowledged) webhook to the newly specified webhook URL.

See endpoint docs at <https://plaid.com/docs/api/items/#itemwebhookupdate>.*/
    pub fn item_webhook_update(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::ItemWebhookUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::ItemWebhookUpdateRequest {
                access_token: access_token.to_owned(),
                webhook: None,
            },
        }
    }
    /**Invalidate access_token

By default, the `access_token` associated with an Item does not expire and should be stored in a persistent, secure manner.

You can use the `/item/access_token/invalidate` endpoint to rotate the `access_token` associated with an Item. The endpoint returns a new `access_token` and immediately invalidates the previous `access_token`.


See endpoint docs at <https://plaid.com/docs/api/tokens/#itemaccess_tokeninvalidate>.*/
    pub fn item_access_token_invalidate(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::ItemAccessTokenInvalidateRequest> {
        FluentRequest {
            client: self,
            params: request::ItemAccessTokenInvalidateRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Get webhook verification key

Plaid signs all outgoing webhooks and provides JSON Web Tokens (JWTs) so that you can verify the authenticity of any incoming webhooks to your application. A message signature is included in the `Plaid-Verification` header.

The `/webhook_verification_key/get` endpoint provides a JSON Web Key (JWK) that can be used to verify a JWT.

See endpoint docs at <https://plaid.com/docs/api/webhooks/webhook-verification/#get-webhook-verification-key>.*/
    pub fn webhook_verification_key_get(
        &self,
        key_id: &str,
    ) -> FluentRequest<'_, request::WebhookVerificationKeyGetRequest> {
        FluentRequest {
            client: self,
            params: request::WebhookVerificationKeyGetRequest {
                key_id: key_id.to_owned(),
            },
        }
    }
    /**Retrieve Liabilities data

The `/liabilities/get` endpoint returns various details about an Item with loan or credit accounts. Liabilities data is available primarily for US financial institutions, with some limited coverage of Canadian institutions. Currently supported account types are account type `credit` with account subtype `credit card` or `paypal`, and account type `loan` with account subtype `student` or `mortgage`. To limit accounts listed in Link to types and subtypes supported by Liabilities, you can use the `account_filters` parameter when [creating a Link token](https://plaid.com/docs/api/tokens/#linktokencreate).

The types of information returned by Liabilities can include balances and due dates, loan terms, and account details such as original loan amount and guarantor. Data is refreshed approximately once per day; the latest data can be retrieved by calling `/liabilities/get`.

Note: This request may take some time to complete if `liabilities` was not specified as an initial product when creating the Item. This is because Plaid must communicate directly with the institution to retrieve the additional data.

See endpoint docs at <https://plaid.com/docs/api/products/liabilities/#liabilitiesget>.*/
    pub fn liabilities_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::LiabilitiesGetRequest> {
        FluentRequest {
            client: self,
            params: request::LiabilitiesGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
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
    ) -> FluentRequest<'_, request::PaymentInitiationRecipientCreateRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationRecipientCreateRequest {
                address: None,
                bacs: None,
                iban: None,
                name: name.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::PaymentInitiationPaymentReverseRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationPaymentReverseRequest {
                amount: None,
                idempotency_key: idempotency_key.to_owned(),
                payment_id: payment_id.to_owned(),
                reference: reference.to_owned(),
            },
        }
    }
    /**Get payment recipient

Get details about a payment recipient you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientget>.*/
    pub fn payment_initiation_recipient_get(
        &self,
        recipient_id: &str,
    ) -> FluentRequest<'_, request::PaymentInitiationRecipientGetRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationRecipientGetRequest {
                recipient_id: recipient_id.to_owned(),
            },
        }
    }
    /**List payment recipients

The `/payment_initiation/recipient/list` endpoint list the payment recipients that you have previously created.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationrecipientlist>.*/
    pub fn payment_initiation_recipient_list(
        &self,
    ) -> FluentRequest<'_, request::PaymentInitiationRecipientListRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationRecipientListRequest {
            },
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
    ) -> FluentRequest<'_, request::PaymentInitiationPaymentCreateRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationPaymentCreateRequest {
                amount,
                options: None,
                recipient_id: recipient_id.to_owned(),
                reference: reference.to_owned(),
                schedule: None,
            },
        }
    }
    /**Create payment token

The `/payment_initiation/payment/token/create` endpoint has been deprecated. New Plaid customers will be unable to use this endpoint, and existing customers are encouraged to migrate to the newer, `link_token`-based flow. The recommended flow is to provide the `payment_id` to `/link/token/create`, which returns a `link_token` used to initialize Link.

The `/payment_initiation/payment/token/create` is used to create a `payment_token`, which can then be used in Link initialization to enter a payment initiation flow. You can only use a `payment_token` once. If this attempt fails, the end user aborts the flow, or the token expires, you will need to create a new payment token. Creating a new payment token does not require end user input.

See endpoint docs at <https://plaid.com/docs/link/maintain-legacy-integration/#creating-a-payment-token>.*/
    pub fn create_payment_token(
        &self,
        payment_id: &str,
    ) -> FluentRequest<'_, request::CreatePaymentTokenRequest> {
        FluentRequest {
            client: self,
            params: request::CreatePaymentTokenRequest {
                payment_id: payment_id.to_owned(),
            },
        }
    }
    /**Create payment consent

The `/payment_initiation/consent/create` endpoint is used to create a payment consent, which can be used to initiate payments on behalf of the user. Payment consents are created with `UNAUTHORISED` status by default and must be authorised by the user before payments can be initiated.

Consents can be limited in time and scope, and have constraints that describe limitations for payments.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentcreate>.*/
    pub fn payment_initiation_consent_create(
        &self,
        args: request::PaymentInitiationConsentCreateRequired,
    ) -> FluentRequest<'_, request::PaymentInitiationConsentCreateRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationConsentCreateRequest {
                constraints: args.constraints,
                options: None,
                recipient_id: args.recipient_id.to_owned(),
                reference: args.reference.to_owned(),
                scopes: args.scopes.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    /**Get payment consent

The `/payment_initiation/consent/get` endpoint can be used to check the status of a payment consent, as well as to receive basic information such as recipient and constraints.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentget>.*/
    pub fn payment_initiation_consent_get(
        &self,
        consent_id: &str,
    ) -> FluentRequest<'_, request::PaymentInitiationConsentGetRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationConsentGetRequest {
                consent_id: consent_id.to_owned(),
            },
        }
    }
    /**Revoke payment consent

The `/payment_initiation/consent/revoke` endpoint can be used to revoke the payment consent. Once the consent is revoked, it is not possible to initiate payments using it.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationconsentrevoke>.*/
    pub fn payment_initiation_consent_revoke(
        &self,
        consent_id: &str,
    ) -> FluentRequest<'_, request::PaymentInitiationConsentRevokeRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationConsentRevokeRequest {
                consent_id: consent_id.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::PaymentInitiationConsentPaymentExecuteRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationConsentPaymentExecuteRequest {
                amount,
                consent_id: consent_id.to_owned(),
                idempotency_key: idempotency_key.to_owned(),
            },
        }
    }
    /**Force a Sandbox Item into an error state

`/sandbox/item/reset_login/` forces an Item into an `ITEM_LOGIN_REQUIRED` state in order to simulate an Item whose login is no longer valid. This makes it easy to test Link's [update mode](https://plaid.com/docs/link/update-mode) flow in the Sandbox environment.  After calling `/sandbox/item/reset_login`, You can then use Plaid Link update mode to restore the Item to a good state. An `ITEM_LOGIN_REQUIRED` webhook will also be fired after a call to this endpoint, if one is associated with the Item.


In the Sandbox, Items will transition to an `ITEM_LOGIN_REQUIRED` error state automatically after 30 days, even if this endpoint is not called.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemreset_login>.*/
    pub fn sandbox_item_reset_login(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::SandboxItemResetLoginRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxItemResetLoginRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Set verification status for Sandbox account

The `/sandbox/item/set_verification_status` endpoint can be used to change the verification status of an Item in in the Sandbox in order to simulate the Automated Micro-deposit flow.

For more information on testing Automated Micro-deposits in Sandbox, see [Auth full coverage testing](https://plaid.com/docs/auth/coverage/testing#).

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxitemset_verification_status>.*/
    pub fn sandbox_item_set_verification_status(
        &self,
        access_token: &str,
        account_id: &str,
        verification_status: &str,
    ) -> FluentRequest<'_, request::SandboxItemSetVerificationStatusRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxItemSetVerificationStatusRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
                verification_status: verification_status.to_owned(),
            },
        }
    }
    /**Exchange public token for an access token

Exchange a Link `public_token` for an API `access_token`. Link hands off the `public_token` client-side via the `onSuccess` callback once a user has successfully created an Item. The `public_token` is ephemeral and expires after 30 minutes. An `access_token` does not expire, but can be revoked by calling `/item/remove`.

The response also includes an `item_id` that should be stored with the `access_token`. The `item_id` is used to identify an Item in a webhook. The `item_id` can also be retrieved by making an `/item/get` request.

See endpoint docs at <https://plaid.com/docs/api/tokens/#itempublic_tokenexchange>.*/
    pub fn item_public_token_exchange(
        &self,
        public_token: &str,
    ) -> FluentRequest<'_, request::ItemPublicTokenExchangeRequest> {
        FluentRequest {
            client: self,
            params: request::ItemPublicTokenExchangeRequest {
                public_token: public_token.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::ItemCreatePublicTokenRequest> {
        FluentRequest {
            client: self,
            params: request::ItemCreatePublicTokenRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Create user

This endpoint should be called for each of your end users before they begin a Plaid income flow. This provides you a single token to access all income data associated with the user. You should only create one per end user.

If you call the endpoint multiple times with the same `client_user_id`, the first creation call will succeed and the rest will fail with an error message indicating that the user has been created for the given `client_user_id`.

Ensure that you store the `user_token` along with your user's identifier in your database, as it is not possible to retrieve a previously created `user_token`.

See endpoint docs at <https://plaid.com/docs/api/products/income/#usercreate>.*/
    pub fn user_create(
        &self,
        client_user_id: &str,
    ) -> FluentRequest<'_, request::UserCreateRequest> {
        FluentRequest {
            client: self,
            params: request::UserCreateRequest {
                client_user_id: client_user_id.to_owned(),
                consumer_report_user_identity: None,
            },
        }
    }
    /**Update user information

This endpoint is used to update user information associated with an existing `user_token`. The `user_token` should be in the response of `/user/create` call

If you call the endpoint with a non-exist `user_token`, the call will fail with an error message indicating that the user token is not found.

See endpoint docs at <https://plaid.com/docs/api/products/income/#userupdate>.*/
    pub fn user_update(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::UserUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::UserUpdateRequest {
                consumer_report_user_identity: None,
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Retrieve Link sessions for your user

This endpoint can be used for your end users after they complete the Link flow. This endpoint returns a list of Link sessions that your user completed, where each session includes the results from the Link flow.

These results include details about the Item that was created and some product related metadata (showing, for example, whether the user finished the bank income verification step).

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditsessionsget>.*/
    pub fn credit_sessions_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditSessionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditSessionsGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Get payment details

The `/payment_initiation/payment/get` endpoint can be used to check the status of a payment, as well as to receive basic information such as recipient and payment amount. In the case of standing orders, the `/payment_initiation/payment/get` endpoint will provide information about the status of the overall standing order itself; the API cannot be used to retrieve payment status for individual payments within a standing order.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentget>.*/
    pub fn payment_initiation_payment_get(
        &self,
        payment_id: &str,
    ) -> FluentRequest<'_, request::PaymentInitiationPaymentGetRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationPaymentGetRequest {
                payment_id: payment_id.to_owned(),
            },
        }
    }
    /**List payments

The `/payment_initiation/payment/list` endpoint can be used to retrieve all created payments. By default, the 10 most recent payments are returned. You can request more payments and paginate through the results using the optional `count` and `cursor` parameters.

See endpoint docs at <https://plaid.com/docs/api/products/payment-initiation/#payment_initiationpaymentlist>.*/
    pub fn payment_initiation_payment_list(
        &self,
    ) -> FluentRequest<'_, request::PaymentInitiationPaymentListRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentInitiationPaymentListRequest {
                consent_id: None,
                count: None,
                cursor: None,
            },
        }
    }
    /**Get Investment holdings

The `/investments/holdings/get` endpoint allows developers to receive user-authorized stock position data for `investment`-type accounts.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentsholdingsget>.*/
    pub fn investments_holdings_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::InvestmentsHoldingsGetRequest> {
        FluentRequest {
            client: self,
            params: request::InvestmentsHoldingsGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
        }
    }
    /**Get investment transactions

The `/investments/transactions/get` endpoint allows developers to retrieve up to 24 months of user-authorized transaction data for investment accounts.

Transactions are returned in reverse-chronological order, and the sequence of transaction ordering is stable and will not shift.

Due to the potentially large number of investment transactions associated with an Item, results are paginated. Manipulate the count and offset parameters in conjunction with the `total_investment_transactions` response body field to fetch all available investment transactions.

Note that Investments does not have a webhook to indicate when initial transaction data has loaded (unless you use the `async_update` option). Instead, if transactions data is not ready when `/investments/transactions/get` is first called, Plaid will wait for the data. For this reason, calling `/investments/transactions/get` immediately after Link may take up to one to two minutes to return.

Data returned by the asynchronous investments extraction flow (when `async_update` is set to true) may not be immediately available to `/investments/transactions/get`. To be alerted when the data is ready to be fetched, listen for the `HISTORICAL_UPDATE` webhook. If no investments history is ready when `/investments/transactions/get` is called, it will return a `PRODUCT_NOT_READY` error.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentstransactionsget>.*/
    pub fn investments_transactions_get(
        &self,
        access_token: &str,
        end_date: chrono::NaiveDate,
        start_date: chrono::NaiveDate,
    ) -> FluentRequest<'_, request::InvestmentsTransactionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::InvestmentsTransactionsGetRequest {
                access_token: access_token.to_owned(),
                end_date,
                options: None,
                start_date,
            },
        }
    }
    /**Refresh investment data

`/investments/refresh` is an optional endpoint for users of the Investments product. It initiates an on-demand extraction to fetch the newest investments, holdings and investment transactions for an Item. This on-demand extraction takes place in addition to the periodic extractions that automatically occur multiple times a day for any Investments-enabled Item. If changes to investments are discovered after calling `/investments/refresh`, Plaid will fire webhooks: [`HOLDINGS: DEFAULT_UPDATE`](https://plaid.com/docs/api/products/investments/#holdings-default_update) if any new holdings are detected, and [INVESTMENTS_TRANSACTIONS: DEFAULT_UPDATE](https://plaid.com/docs/api/products/investments/#investments_transactions-default_update) if any new investment transactions are detected. Updated holdings and investment transactions can be fetched by calling `/investments/holdings/get` and `/investments/transactions/get`. "Note that the `/investments/refresh` endpoint is not supported by all institutions. If called on an Item from an institution that does not support this functionality, it will return a `PRODUCT_NOT_SUPPORTED` error.
`/investments/refresh` is offered as an add-on to Investments and has a separate [fee model](/docs/account/billing/#per-request-flat-fee). To request access to this endpoint, submit a [product access request](https://dashboard.plaid.com/team/products) or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentsrefresh>.*/
    pub fn investments_refresh(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::InvestmentsRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::InvestmentsRefreshRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Get data needed to authorize an investments transfer

The `/investments/auth/get` endpoint allows developers to receive user-authorized data to facilitate the transfer of holdings

See endpoint docs at <https://plaid.com/docs/api/products/investments/#investmentsauth>.*/
    pub fn investments_auth_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::InvestmentsAuthGetRequest> {
        FluentRequest {
            client: self,
            params: request::InvestmentsAuthGetRequest {
                access_token: access_token.to_owned(),
                options: None,
            },
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
    ) -> FluentRequest<'_, request::ProcessorTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTokenCreateRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
                processor: processor.to_owned(),
            },
        }
    }
    /**Control a processor's access to products

Used to control a processor's access to products on the given processor token. By default, a processor will have access to all available products on the corresponding item. To restrict access to a particular set of products, call this endpoint with the desired products. To restore access to all available products, call this endpoint with an empty list. This endpoint can be called multiple times as your needs and your processor's needs change.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokenpermissionsset>.*/
    pub fn processor_token_permissions_set(
        &self,
        processor_token: &str,
        products: &[&str],
    ) -> FluentRequest<'_, request::ProcessorTokenPermissionsSetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTokenPermissionsSetRequest {
                processor_token: processor_token.to_owned(),
                products: products.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    /**Get a processor token's product permissions

Used to get a processor token's product permissions. The `products` field will be an empty list if the processor can access all available products.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokenpermissionsget>.*/
    pub fn processor_token_permissions_get(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorTokenPermissionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTokenPermissionsGetRequest {
                processor_token: processor_token.to_owned(),
            },
        }
    }
    /**Update a processor token's webhook URL

This endpoint allows you, the processor, to update the webhook URL associated with a processor token. This request triggers a `WEBHOOK_UPDATE_ACKNOWLEDGED` webhook to the newly specified webhook URL.

See endpoint docs at <https://plaid.com/docs/api/processors/#processortokenwebhookupdate>.*/
    pub fn processor_token_webhook_update(
        &self,
        processor_token: &str,
    ) -> FluentRequest<'_, request::ProcessorTokenWebhookUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorTokenWebhookUpdateRequest {
                processor_token: processor_token.to_owned(),
                webhook: None,
            },
        }
    }
    /**Create Stripe bank account token


Used to create a token suitable for sending to Stripe to enable Plaid-Stripe integrations. For a detailed guide on integrating Stripe, see [Add Stripe to your app](https://plaid.com/docs/auth/partnerships/stripe/).

Note that the Stripe bank account token is a one-time use token. To store bank account information for later use, you can use a Stripe customer object and create an associated bank account from the token, or you can use a Stripe Custom account and create an associated external bank account from the token. This bank account information should work indefinitely, unless the user's bank account information changes or they revoke Plaid's permissions to access their account. Stripe bank account information cannot be modified once the bank account token has been created. If you ever need to change the bank account details used by Stripe for a specific customer, have the user go through Link again and create a new bank account token from the new `access_token`.

Bank account tokens can also be revoked, using `/item/remove`.

See endpoint docs at <https://plaid.com/docs/api/processors/#processorstripebank_account_tokencreate>.*/
    pub fn processor_stripe_bank_account_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> FluentRequest<'_, request::ProcessorStripeBankAccountTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorStripeBankAccountTokenCreateRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
            },
        }
    }
    /**Create Apex bank account token

Used to create a token suitable for sending to Apex to enable Plaid-Apex integrations.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn processor_apex_processor_token_create(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> FluentRequest<'_, request::ProcessorApexProcessorTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::ProcessorApexProcessorTokenCreateRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
            },
        }
    }
    /**Create a deposit switch

This endpoint creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchcreate>.*/
    pub fn deposit_switch_create(
        &self,
        target_access_token: &str,
        target_account_id: &str,
    ) -> FluentRequest<'_, request::DepositSwitchCreateRequest> {
        FluentRequest {
            client: self,
            params: request::DepositSwitchCreateRequest {
                country_code: None,
                options: None,
                target_access_token: target_access_token.to_owned(),
                target_account_id: target_account_id.to_owned(),
            },
        }
    }
    /**Import Item

`/item/import` creates an Item via your Plaid Exchange Integration and returns an `access_token`. As part of an `/item/import` request, you will include a User ID (`user_auth.user_id`) and Authentication Token (`user_auth.auth_token`) that enable data aggregation through your Plaid Exchange API endpoints. These authentication principals are to be chosen by you.

Upon creating an Item via `/item/import`, Plaid will automatically begin an extraction of that Item through the Plaid Exchange infrastructure you have already integrated.*/
    pub fn item_import(
        &self,
        products: &[&str],
        user_auth: ItemImportRequestUserAuth,
    ) -> FluentRequest<'_, request::ItemImportRequest> {
        FluentRequest {
            client: self,
            params: request::ItemImportRequest {
                options: None,
                products: products.iter().map(|&x| x.to_owned()).collect(),
                user_auth,
            },
        }
    }
    /**Create a deposit switch token

In order for the end user to take action, you will need to create a public token representing the deposit switch. This token is used to initialize Link. It can be used one time and expires after 30 minutes.


See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchtokencreate>.*/
    pub fn deposit_switch_token_create(
        &self,
        deposit_switch_id: &str,
    ) -> FluentRequest<'_, request::DepositSwitchTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::DepositSwitchTokenCreateRequest {
                deposit_switch_id: deposit_switch_id.to_owned(),
            },
        }
    }
    /**Create Link Token

The `/link/token/create` endpoint creates a `link_token`, which is required as a parameter when initializing Link. Once Link has been initialized, it returns a `public_token`, which can then be exchanged for an `access_token` via `/item/public_token/exchange` as part of the main Link flow.

A `link_token` generated by `/link/token/create` is also used to initialize other Link flows, such as the update mode flow for tokens with expired credentials, or the Payment Initiation (Europe) flow.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokencreate>.*/
    pub fn link_token_create(
        &self,
        args: request::LinkTokenCreateRequired,
    ) -> FluentRequest<'_, request::LinkTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::LinkTokenCreateRequest {
                access_token: None,
                access_tokens: None,
                account_filters: None,
                additional_consented_products: None,
                android_package_name: None,
                auth: None,
                base_report: None,
                card_switch: None,
                client_name: args.client_name.to_owned(),
                consumer_report_permissible_purpose: None,
                country_codes: args
                    .country_codes
                    .iter()
                    .map(|&x| x.to_owned())
                    .collect(),
                cra_enabled: None,
                deposit_switch: None,
                employment: None,
                eu_config: None,
                hosted_link: None,
                identity_verification: None,
                income_verification: None,
                institution_data: None,
                institution_id: None,
                investments: None,
                investments_auth: None,
                language: args.language.to_owned(),
                link_customization_name: None,
                optional_products: None,
                payment_initiation: None,
                products: None,
                redirect_uri: None,
                required_if_supported_products: None,
                statements: None,
                transactions: None,
                transfer: None,
                update: None,
                user: args.user,
                user_token: None,
                webhook: None,
            },
        }
    }
    /**Get Link Token

The `/link/token/get` endpoint gets information about a previously-created `link_token` using the
`/link/token/create` endpoint. It can be useful for debugging purposes.

See endpoint docs at <https://plaid.com/docs/api/tokens/#linktokenget>.*/
    pub fn link_token_get(
        &self,
        link_token: &str,
    ) -> FluentRequest<'_, request::LinkTokenGetRequest> {
        FluentRequest {
            client: self,
            params: request::LinkTokenGetRequest {
                link_token: link_token.to_owned(),
            },
        }
    }
    /**Exchange the Link Correlation Id for a Link Token

Exchange an OAuth `link_correlation_id` for the corresponding `link_token`. The `link_correlation_id` is only available for 'payment_initiation' products and is provided to the client via the OAuth `redirect_uri` as a query parameter.
The `link_correlation_id` is ephemeral and expires in a brief period, after which it can no longer be exchanged for the 'link_token'.

See endpoint docs at <https://plaid.com/docs/api/oauth/#linkcorrelationid>.*/
    pub fn link_oauth_correlation_id_exchange(
        &self,
        link_correlation_id: &str,
    ) -> FluentRequest<'_, request::LinkOauthCorrelationIdExchangeRequest> {
        FluentRequest {
            client: self,
            params: request::LinkOauthCorrelationIdExchangeRequest {
                link_correlation_id: link_correlation_id.to_owned(),
            },
        }
    }
    /**Retrieve a deposit switch

This endpoint returns information related to how the user has configured their payroll allocation and the state of the switch. You can use this information to build logic related to the user's direct deposit allocation preferences.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchget>.*/
    pub fn deposit_switch_get(
        &self,
        deposit_switch_id: &str,
    ) -> FluentRequest<'_, request::DepositSwitchGetRequest> {
        FluentRequest {
            client: self,
            params: request::DepositSwitchGetRequest {
                deposit_switch_id: deposit_switch_id.to_owned(),
            },
        }
    }
    /**Retrieve a transfer

The `/transfer/get` endpoint fetches information about the transfer corresponding to the given `transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferget>.*/
    pub fn transfer_get(
        &self,
        transfer_id: &str,
    ) -> FluentRequest<'_, request::TransferGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferGetRequest {
                originator_client_id: None,
                transfer_id: transfer_id.to_owned(),
            },
        }
    }
    /**Retrieve a recurring transfer

The `/transfer/recurring/get` fetches information about the recurring transfer corresponding to the given `recurring_transfer_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringget>.*/
    pub fn transfer_recurring_get(
        &self,
        recurring_transfer_id: &str,
    ) -> FluentRequest<'_, request::TransferRecurringGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRecurringGetRequest {
                recurring_transfer_id: recurring_transfer_id.to_owned(),
            },
        }
    }
    /**Retrieve a bank transfer

The `/bank_transfer/get` fetches information about the bank transfer corresponding to the given `bank_transfer_id`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferget>.*/
    pub fn bank_transfer_get(
        &self,
        bank_transfer_id: &str,
    ) -> FluentRequest<'_, request::BankTransferGetRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferGetRequest {
                bank_transfer_id: bank_transfer_id.to_owned(),
            },
        }
    }
    /**Create a transfer authorization

Use the `/transfer/authorization/create` endpoint to authorize a transfer. This endpoint must be called prior to calling `/transfer/create`.

There are three possible outcomes to calling this endpoint: If the `authorization.decision` in the response is `declined`, the proposed transfer has failed the risk check and you cannot proceed with the transfer. If the `authorization.decision` is `approved`, and the `authorization.rationale_code` is `null`, the transfer has passed the risk check and you can proceed to call `/transfer/create`. If the `authorization.decision` is `approved` and the `authorization.rationale_code` is non-`null`, the risk check could not be run: you may proceed with the transfer, but should perform your own risk evaluation. For more details, see the response schema.

In Plaid's Sandbox environment the decisions will be returned as follows:

  - To approve a transfer with `null` rationale code, make an authorization request with an `amount` less than the available balance in the account.

  - To approve a transfer with the rationale code `MANUALLY_VERIFIED_ITEM`, create an Item in Link through the [Same Day Micro-deposits flow](https://plaid.com/docs/auth/coverage/testing/#testing-same-day-micro-deposits).

  - To approve a transfer with the rationale code `ITEM_LOGIN_REQUIRED`, [reset the login for an Item](https://plaid.com/docs/sandbox/#item_login_required).

  - To decline a transfer with the rationale code `NSF`, the available balance on the account must be less than the authorization `amount`. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

  - To decline a transfer with the rationale code `RISK`, the available balance on the account must be exactly $0. See [Create Sandbox test data](https://plaid.com/docs/sandbox/user-custom/) for details on how to customize data in Sandbox.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferauthorizationcreate>.*/
    pub fn transfer_authorization_create(
        &self,
        args: request::TransferAuthorizationCreateRequired,
    ) -> FluentRequest<'_, request::TransferAuthorizationCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferAuthorizationCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: args.amount.to_owned(),
                beacon_session_id: None,
                credit_funds_source: None,
                device: None,
                funding_account_id: None,
                idempotency_key: None,
                iso_currency_code: None,
                network: args.network.to_owned(),
                origination_account_id: None,
                originator_client_id: None,
                payment_profile_token: None,
                test_clock_id: None,
                type_: args.type_.to_owned(),
                user: args.user,
                user_present: None,
                with_guarantee: None,
            },
        }
    }
    /**Retrieve a balance held with Plaid

Use the `/transfer/balance/get` endpoint to view a balance held with Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferbalanceget>.*/
    pub fn transfer_balance_get(
        &self,
    ) -> FluentRequest<'_, request::TransferBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferBalanceGetRequest {
                originator_client_id: None,
                type_: None,
            },
        }
    }
    /**Get RTP eligibility information of a transfer

Use the `/transfer/capabilities/get` endpoint to determine the RTP eligibility information of a transfer. To simulate RTP eligibility in Sandbox, log in using the username `user_good` and password `pass_good` and use the first two checking and savings accounts in the "First Platypus Bank" institution (ending in 0000 or 1111), which will return `true`. Any other account will return `false`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercapabilitiesget>.*/
    pub fn transfer_capabilities_get(
        &self,
        access_token: &str,
        account_id: &str,
    ) -> FluentRequest<'_, request::TransferCapabilitiesGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferCapabilitiesGetRequest {
                access_token: access_token.to_owned(),
                account_id: account_id.to_owned(),
                payment_profile_token: None,
            },
        }
    }
    /**Get transfer product configuration

Use the `/transfer/configuration/get` endpoint to view your transfer product configurations.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferconfigurationget>.*/
    pub fn transfer_configuration_get(
        &self,
    ) -> FluentRequest<'_, request::TransferConfigurationGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferConfigurationGetRequest {
                originator_client_id: None,
            },
        }
    }
    /**Retrieve Plaid Ledger balance

Use the `/transfer/ledger/get` endpoint to view a balance on the ledger held with Plaid.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferledgerget>.*/
    pub fn transfer_ledger_get(
        &self,
    ) -> FluentRequest<'_, request::TransferLedgerGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferLedgerGetRequest {
                originator_client_id: None,
            },
        }
    }
    /**Move available balance between the ledgers of the platform and one of its originators

Use the `/transfer/ledger/distribute` endpoint to move available balance between the ledgers of the platform and one of its originators.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferledgerdistribute>.*/
    pub fn transfer_ledger_distribute(
        &self,
        args: request::TransferLedgerDistributeRequired,
    ) -> FluentRequest<'_, request::TransferLedgerDistributeRequest> {
        FluentRequest {
            client: self,
            params: request::TransferLedgerDistributeRequest {
                amount: args.amount.to_owned(),
                description: None,
                from_client_id: args.from_client_id.to_owned(),
                idempotency_key: args.idempotency_key.to_owned(),
                to_client_id: args.to_client_id.to_owned(),
            },
        }
    }
    /**Deposit funds into a Plaid Ledger balance

Use the `/transfer/ledger/deposit` endpoint to deposit funds into Plaid Ledger.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferledgerdeposit>.*/
    pub fn transfer_ledger_deposit(
        &self,
        amount: &str,
        idempotency_key: &str,
        network: &str,
    ) -> FluentRequest<'_, request::TransferLedgerDepositRequest> {
        FluentRequest {
            client: self,
            params: request::TransferLedgerDepositRequest {
                amount: amount.to_owned(),
                description: None,
                funding_account_id: None,
                idempotency_key: idempotency_key.to_owned(),
                network: network.to_owned(),
                originator_client_id: None,
            },
        }
    }
    /**Withdraw funds from a Plaid Ledger balance

Use the `/transfer/ledger/withdraw` endpoint to withdraw funds from a Plaid Ledger balance.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferledgerwithdraw>.*/
    pub fn transfer_ledger_withdraw(
        &self,
        amount: &str,
        idempotency_key: &str,
        network: &str,
    ) -> FluentRequest<'_, request::TransferLedgerWithdrawRequest> {
        FluentRequest {
            client: self,
            params: request::TransferLedgerWithdrawRequest {
                amount: amount.to_owned(),
                description: None,
                funding_account_id: None,
                idempotency_key: idempotency_key.to_owned(),
                network: network.to_owned(),
                originator_client_id: None,
            },
        }
    }
    /**Update the funding account associated with the originator

Use the `/transfer/originator/funding_account/update` endpoint to update the funding account associated with the originator.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorfunding_accountupdate>.*/
    pub fn transfer_originator_funding_account_update(
        &self,
        funding_account: TransferFundingAccount,
        originator_client_id: &str,
    ) -> FluentRequest<'_, request::TransferOriginatorFundingAccountUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferOriginatorFundingAccountUpdateRequest {
                funding_account,
                originator_client_id: originator_client_id.to_owned(),
            },
        }
    }
    /**Get transfer product usage metrics

Use the `/transfer/metrics/get` endpoint to view your transfer product usage metrics.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfermetricsget>.*/
    pub fn transfer_metrics_get(
        &self,
    ) -> FluentRequest<'_, request::TransferMetricsGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferMetricsGetRequest {
                originator_client_id: None,
            },
        }
    }
    /**Create a transfer

Use the `/transfer/create` endpoint to initiate a new transfer.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercreate>.*/
    pub fn transfer_create(
        &self,
        args: request::TransferCreateRequired,
    ) -> FluentRequest<'_, request::TransferCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: None,
                authorization_id: args.authorization_id.to_owned(),
                description: args.description.to_owned(),
                facilitator_fee: None,
                idempotency_key: None,
                iso_currency_code: None,
                metadata: None,
                network: None,
                origination_account_id: None,
                test_clock_id: None,
                type_: None,
                user: None,
            },
        }
    }
    /**Create a recurring transfer

Use the `/transfer/recurring/create` endpoint to initiate a new recurring transfer. This capability is not currently supported for Transfer UI or Platform Payments (beta) customers.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringcreate>.*/
    pub fn transfer_recurring_create(
        &self,
        args: request::TransferRecurringCreateRequired,
    ) -> FluentRequest<'_, request::TransferRecurringCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRecurringCreateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                ach_class: None,
                amount: args.amount.to_owned(),
                description: args.description.to_owned(),
                device: None,
                funding_account_id: None,
                idempotency_key: args.idempotency_key.to_owned(),
                iso_currency_code: None,
                network: args.network.to_owned(),
                schedule: args.schedule,
                test_clock_id: None,
                type_: args.type_.to_owned(),
                user: args.user,
                user_present: None,
            },
        }
    }
    /**Create a bank transfer

Use the `/bank_transfer/create` endpoint to initiate a new bank transfer.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercreate>.*/
    pub fn bank_transfer_create(
        &self,
        args: request::BankTransferCreateRequired,
    ) -> FluentRequest<'_, request::BankTransferCreateRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferCreateRequest {
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
            },
        }
    }
    /**List transfers

Use the `/transfer/list` endpoint to see a list of all your transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired transfers.


See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferlist>.*/
    pub fn transfer_list(&self) -> FluentRequest<'_, request::TransferListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferListRequest {
                count: None,
                end_date: None,
                funding_account_id: None,
                offset: None,
                origination_account_id: None,
                originator_client_id: None,
                start_date: None,
            },
        }
    }
    /**List recurring transfers

Use the `/transfer/recurring/list` endpoint to see a list of all your recurring transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired recurring transfers.


See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringlist>.*/
    pub fn transfer_recurring_list(
        &self,
    ) -> FluentRequest<'_, request::TransferRecurringListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRecurringListRequest {
                count: None,
                end_time: None,
                funding_account_id: None,
                offset: None,
                start_time: None,
            },
        }
    }
    /**List bank transfers

Use the `/bank_transfer/list` endpoint to see a list of all your bank transfers and their statuses. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired bank transfers.


See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferlist>.*/
    pub fn bank_transfer_list(
        &self,
    ) -> FluentRequest<'_, request::BankTransferListRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferListRequest {
                count: None,
                direction: None,
                end_date: None,
                offset: None,
                origination_account_id: None,
                start_date: None,
            },
        }
    }
    /**Cancel a transfer

Use the `/transfer/cancel` endpoint to cancel a transfer.  A transfer is eligible for cancellation if the `cancellable` property returned by `/transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfercancel>.*/
    pub fn transfer_cancel(
        &self,
        transfer_id: &str,
    ) -> FluentRequest<'_, request::TransferCancelRequest> {
        FluentRequest {
            client: self,
            params: request::TransferCancelRequest {
                originator_client_id: None,
                transfer_id: transfer_id.to_owned(),
            },
        }
    }
    /**Cancel a recurring transfer.

Use the `/transfer/recurring/cancel` endpoint to cancel a recurring transfer.  Scheduled transfer that hasn't been submitted to bank will be cancelled.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrecurringcancel>.*/
    pub fn transfer_recurring_cancel(
        &self,
        recurring_transfer_id: &str,
    ) -> FluentRequest<'_, request::TransferRecurringCancelRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRecurringCancelRequest {
                recurring_transfer_id: recurring_transfer_id.to_owned(),
            },
        }
    }
    /**Cancel a bank transfer

Use the `/bank_transfer/cancel` endpoint to cancel a bank transfer.  A transfer is eligible for cancelation if the `cancellable` property returned by `/bank_transfer/get` is `true`.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transfercancel>.*/
    pub fn bank_transfer_cancel(
        &self,
        bank_transfer_id: &str,
    ) -> FluentRequest<'_, request::BankTransferCancelRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferCancelRequest {
                bank_transfer_id: bank_transfer_id.to_owned(),
            },
        }
    }
    /**List transfer events

Use the `/transfer/event/list` endpoint to get a list of transfer events based on specified filter criteria.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfereventlist>.*/
    pub fn transfer_event_list(
        &self,
    ) -> FluentRequest<'_, request::TransferEventListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferEventListRequest {
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
            },
        }
    }
    /**List bank transfer events

Use the `/bank_transfer/event/list` endpoint to get a list of Plaid-initiated ACH or bank transfer events based on specified filter criteria. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/auth#bank_transfereventlist>.*/
    pub fn bank_transfer_event_list(
        &self,
    ) -> FluentRequest<'_, request::BankTransferEventListRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferEventListRequest {
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
            },
        }
    }
    /**Sync transfer events

`/transfer/event/sync` allows you to request up to the next 25 transfer events that happened after a specific `event_id`. Use the `/transfer/event/sync` endpoint to guarantee you have seen all transfer events.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfereventsync>.*/
    pub fn transfer_event_sync(
        &self,
        after_id: i64,
    ) -> FluentRequest<'_, request::TransferEventSyncRequest> {
        FluentRequest {
            client: self,
            params: request::TransferEventSyncRequest {
                after_id,
                count: None,
            },
        }
    }
    /**Sync bank transfer events

`/bank_transfer/event/sync` allows you to request up to the next 25 Plaid-initiated bank transfer events that happened after a specific `event_id`. When using Auth with micro-deposit verification enabled, this endpoint can be used to fetch status updates on ACH micro-deposits. For more details, see [micro-deposit events](https://www.plaid.com/docs/auth/coverage/microdeposit-events/).

See endpoint docs at <https://plaid.com/docs/api/products/auth/#bank_transfereventsync>.*/
    pub fn bank_transfer_event_sync(
        &self,
        after_id: i64,
    ) -> FluentRequest<'_, request::BankTransferEventSyncRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferEventSyncRequest {
                after_id,
                count: None,
            },
        }
    }
    /**Retrieve a sweep

The `/transfer/sweep/get` endpoint fetches a sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfersweepget>.*/
    pub fn transfer_sweep_get(
        &self,
        sweep_id: &str,
    ) -> FluentRequest<'_, request::TransferSweepGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferSweepGetRequest {
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
    /**Retrieve a sweep

The `/bank_transfer/sweep/get` endpoint fetches information about the sweep corresponding to the given `sweep_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweepget>.*/
    pub fn bank_transfer_sweep_get(
        &self,
        sweep_id: &str,
    ) -> FluentRequest<'_, request::BankTransferSweepGetRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferSweepGetRequest {
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
    /**List sweeps

The `/transfer/sweep/list` endpoint fetches sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transfersweeplist>.*/
    pub fn transfer_sweep_list(
        &self,
    ) -> FluentRequest<'_, request::TransferSweepListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferSweepListRequest {
                amount: None,
                count: None,
                end_date: None,
                funding_account_id: None,
                offset: None,
                originator_client_id: None,
                start_date: None,
                status: None,
                transfer_id: None,
                trigger: None,
            },
        }
    }
    /**List sweeps

The `/bank_transfer/sweep/list` endpoint fetches information about the sweeps matching the given filters.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#bank_transfersweeplist>.*/
    pub fn bank_transfer_sweep_list(
        &self,
    ) -> FluentRequest<'_, request::BankTransferSweepListRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferSweepListRequest {
                count: None,
                end_time: None,
                origination_account_id: None,
                start_time: None,
            },
        }
    }
    /**Get balance of your Bank Transfer account

Use the `/bank_transfer/balance/get` endpoint to see the available balance in your bank transfer account. Debit transfers increase this balance once their status is posted. Credit transfers decrease this balance when they are created.

The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.

Note that this endpoint can only be used with FBO accounts, when using Bank Transfers in the Full Service configuration. It cannot be used on your own account when using Bank Transfers in the BTS Platform configuration.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference#bank_transferbalanceget>.*/
    pub fn bank_transfer_balance_get(
        &self,
    ) -> FluentRequest<'_, request::BankTransferBalanceGetRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferBalanceGetRequest {
                origination_account_id: None,
            },
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
    ) -> FluentRequest<'_, request::BankTransferMigrateAccountRequest> {
        FluentRequest {
            client: self,
            params: request::BankTransferMigrateAccountRequest {
                account_number: account_number.to_owned(),
                account_type: account_type.to_owned(),
                routing_number: routing_number.to_owned(),
                wire_routing_number: None,
            },
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
    ) -> FluentRequest<'_, request::TransferMigrateAccountRequest> {
        FluentRequest {
            client: self,
            params: request::TransferMigrateAccountRequest {
                account_number: account_number.to_owned(),
                account_type: account_type.to_owned(),
                routing_number: routing_number.to_owned(),
                wire_routing_number: None,
            },
        }
    }
    /**Create a transfer intent object to invoke the Transfer UI

Use the `/transfer/intent/create` endpoint to generate a transfer intent object and invoke the Transfer UI.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferintentcreate>.*/
    pub fn transfer_intent_create(
        &self,
        args: request::TransferIntentCreateRequired,
    ) -> FluentRequest<'_, request::TransferIntentCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferIntentCreateRequest {
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
            },
        }
    }
    /**Retrieve more information about a transfer intent

Use the `/transfer/intent/get` endpoint to retrieve more information about a transfer intent.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferintentget>.*/
    pub fn transfer_intent_get(
        &self,
        transfer_intent_id: &str,
    ) -> FluentRequest<'_, request::TransferIntentGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferIntentGetRequest {
                transfer_intent_id: transfer_intent_id.to_owned(),
            },
        }
    }
    /**Lists historical repayments

The `/transfer/repayment/list` endpoint fetches repayments matching the given filters. Repayments are returned in reverse-chronological order (most recent first) starting at the given `start_time`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentlist>.*/
    pub fn transfer_repayment_list(
        &self,
    ) -> FluentRequest<'_, request::TransferRepaymentListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRepaymentListRequest {
                count: None,
                end_date: None,
                offset: None,
                start_date: None,
            },
        }
    }
    /**List the returns included in a repayment

The `/transfer/repayment/return/list` endpoint retrieves the set of returns that were batched together into the specified repayment. The sum of amounts of returns retrieved by this request equals the amount of the repayment.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrepaymentreturnlist>.*/
    pub fn transfer_repayment_return_list(
        &self,
        repayment_id: &str,
    ) -> FluentRequest<'_, request::TransferRepaymentReturnListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRepaymentReturnListRequest {
                count: None,
                offset: None,
                repayment_id: repayment_id.to_owned(),
            },
        }
    }
    /**Create a new originator

Use the `/transfer/originator/create` endpoint to create a new originator and return an `originator_client_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorcreate>.*/
    pub fn transfer_originator_create(
        &self,
        company_name: &str,
    ) -> FluentRequest<'_, request::TransferOriginatorCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferOriginatorCreateRequest {
                company_name: company_name.to_owned(),
            },
        }
    }
    /**Generate a Plaid-hosted onboarding UI URL.

The `/transfer/questionnaire/create` endpoint generates a Plaid-hosted onboarding UI URL. Redirect the originator to this URL to provide their due diligence information and agree to Plaid’s terms for ACH money movement.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferquestionnairecreate>.*/
    pub fn transfer_questionnaire_create(
        &self,
        originator_client_id: &str,
        redirect_uri: &str,
    ) -> FluentRequest<'_, request::TransferQuestionnaireCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferQuestionnaireCreateRequest {
                originator_client_id: originator_client_id.to_owned(),
                redirect_uri: redirect_uri.to_owned(),
            },
        }
    }
    /**Submit transfer diligence on behalf of the originator

Use the `/transfer/diligence/submit` endpoint to submit transfer diligence on behalf of the originator (i.e., the end customer).

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferdiligencesubmit>.*/
    pub fn transfer_diligence_submit(
        &self,
        originator_client_id: &str,
        originator_diligence: TransferOriginatorDiligence,
    ) -> FluentRequest<'_, request::TransferDiligenceSubmitRequest> {
        FluentRequest {
            client: self,
            params: request::TransferDiligenceSubmitRequest {
                originator_client_id: originator_client_id.to_owned(),
                originator_diligence,
            },
        }
    }
    /**Upload transfer diligence document on behalf of the originator

Third-party sender customers can use `/transfer/diligence/document/upload` endpoint to upload a document on behalf of its end customer (i.e. originator) to Plaid. You’ll need to send a request of type multipart/form-data.
You must provide the `client_id` in the `PLAID-CLIENT-ID` header and `secret` in the `PLAID-SECRET` header.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferdiligencedocumentupload>.*/
    pub fn transfer_diligence_document_upload(
        &self,
        file: &str,
        originator_client_id: &str,
        purpose: &str,
    ) -> FluentRequest<'_, request::TransferDiligenceDocumentUploadRequest> {
        FluentRequest {
            client: self,
            params: request::TransferDiligenceDocumentUploadRequest {
                file: file.to_owned(),
                originator_client_id: originator_client_id.to_owned(),
                purpose: purpose.to_owned(),
            },
        }
    }
    /**Get status of an originator's onboarding

The `/transfer/originator/get` endpoint gets status updates for an originator's onboarding process. This information is also available via the Transfer page on the Plaid dashboard.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorget>.*/
    pub fn transfer_originator_get(
        &self,
        originator_client_id: &str,
    ) -> FluentRequest<'_, request::TransferOriginatorGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferOriginatorGetRequest {
                originator_client_id: originator_client_id.to_owned(),
            },
        }
    }
    /**Get status of all originators' onboarding

The `/transfer/originator/list` endpoint gets status updates for all of your originators' onboarding. This information is also available via the Plaid dashboard.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferoriginatorlist>.*/
    pub fn transfer_originator_list(
        &self,
    ) -> FluentRequest<'_, request::TransferOriginatorListRequest> {
        FluentRequest {
            client: self,
            params: request::TransferOriginatorListRequest {
                count: None,
                offset: None,
            },
        }
    }
    /**Create a refund

Use the `/transfer/refund/create` endpoint to create a refund for a transfer. A transfer can be refunded if the transfer was initiated in the past 180 days.

Processing of the refund will not occur until at least 4 business days following the transfer's settlement date, plus any hold/settlement delays. This 3-day window helps better protect your business from regular ACH returns. Consumer initiated returns (unauthorized returns) could still happen for about 60 days from the settlement date. If the original transfer is canceled, returned or failed, all pending refunds will automatically be canceled. Processed refunds cannot be revoked.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrefundcreate>.*/
    pub fn transfer_refund_create(
        &self,
        amount: &str,
        idempotency_key: &str,
        transfer_id: &str,
    ) -> FluentRequest<'_, request::TransferRefundCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRefundCreateRequest {
                amount: amount.to_owned(),
                idempotency_key: idempotency_key.to_owned(),
                transfer_id: transfer_id.to_owned(),
            },
        }
    }
    /**Retrieve a refund

The `/transfer/refund/get` endpoint fetches information about the refund corresponding to the given `refund_id`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrefundget>.*/
    pub fn transfer_refund_get(
        &self,
        refund_id: &str,
    ) -> FluentRequest<'_, request::TransferRefundGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRefundGetRequest {
                refund_id: refund_id.to_owned(),
            },
        }
    }
    /**Cancel a refund

Use the `/transfer/refund/cancel` endpoint to cancel a refund.  A refund is eligible for cancellation if it has not yet been submitted to the payment network.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#transferrefundcancel>.*/
    pub fn transfer_refund_cancel(
        &self,
        refund_id: &str,
    ) -> FluentRequest<'_, request::TransferRefundCancelRequest> {
        FluentRequest {
            client: self,
            params: request::TransferRefundCancelRequest {
                refund_id: refund_id.to_owned(),
            },
        }
    }
    /**Simulate a bank transfer event in Sandbox

Use the `/sandbox/bank_transfer/simulate` endpoint to simulate a bank transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/bank_transfer/event/sync` or `/bank_transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transfersimulate>.*/
    pub fn sandbox_bank_transfer_simulate(
        &self,
        bank_transfer_id: &str,
        event_type: &str,
    ) -> FluentRequest<'_, request::SandboxBankTransferSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxBankTransferSimulateRequest {
                bank_transfer_id: bank_transfer_id.to_owned(),
                event_type: event_type.to_owned(),
                failure_reason: None,
            },
        }
    }
    /**Simulate creating a sweep

Use the `/sandbox/transfer/sweep/simulate` endpoint to create a sweep and associated events in the Sandbox environment. Upon calling this endpoint, all transfers with a sweep status of `swept` will become `swept_settled`, all `posted` or `pending` transfers with a sweep status of `unswept` will become `swept`, and all `returned` transfers with a sweep status of `swept` will become `return_swept`.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersweepsimulate>.*/
    pub fn sandbox_transfer_sweep_simulate(
        &self,
    ) -> FluentRequest<'_, request::SandboxTransferSweepSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferSweepSimulateRequest {
                test_clock_id: None,
            },
        }
    }
    /**Simulate a transfer event in Sandbox

Use the `/sandbox/transfer/simulate` endpoint to simulate a transfer event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfersimulate>.*/
    pub fn sandbox_transfer_simulate(
        &self,
        event_type: &str,
        transfer_id: &str,
    ) -> FluentRequest<'_, request::SandboxTransferSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferSimulateRequest {
                event_type: event_type.to_owned(),
                failure_reason: None,
                test_clock_id: None,
                transfer_id: transfer_id.to_owned(),
            },
        }
    }
    /**Simulate a refund event in Sandbox

Use the `/sandbox/transfer/refund/simulate` endpoint to simulate a refund event in the Sandbox environment.  Note that while an event will be simulated and will appear when using endpoints such as `/transfer/event/sync` or `/transfer/event/list`, no transactions will actually take place and funds will not move between accounts, even within the Sandbox.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrefundsimulate>.*/
    pub fn sandbox_transfer_refund_simulate(
        &self,
        event_type: &str,
        refund_id: &str,
    ) -> FluentRequest<'_, request::SandboxTransferRefundSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferRefundSimulateRequest {
                event_type: event_type.to_owned(),
                failure_reason: None,
                refund_id: refund_id.to_owned(),
                test_clock_id: None,
            },
        }
    }
    /**Simulate converting pending balance to available balance

Use the `/sandbox/transfer/ledger/simulate_available` endpoint to simulate converting pending balance to available balance for all originators in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferledgersimulate_available>.*/
    pub fn sandbox_transfer_ledger_simulate_available(
        &self,
    ) -> FluentRequest<'_, request::SandboxTransferLedgerSimulateAvailableRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferLedgerSimulateAvailableRequest {
                test_clock_id: None,
            },
        }
    }
    /**Simulate a ledger deposit event in Sandbox

Use the `/sandbox/transfer/ledger/deposit/simulate` endpoint to simulate a ledger deposit event in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferledgerdepositsimulate>.*/
    pub fn sandbox_transfer_ledger_deposit_simulate(
        &self,
        event_type: &str,
        sweep_id: &str,
    ) -> FluentRequest<'_, request::SandboxTransferLedgerDepositSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferLedgerDepositSimulateRequest {
                event_type: event_type.to_owned(),
                failure_reason: None,
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
    /**Simulate a ledger withdraw event in Sandbox

Use the `/sandbox/transfer/ledger/withdraw/simulate` endpoint to simulate a ledger withdraw event in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferledgerwithdrawsimulate>.*/
    pub fn sandbox_transfer_ledger_withdraw_simulate(
        &self,
        event_type: &str,
        sweep_id: &str,
    ) -> FluentRequest<'_, request::SandboxTransferLedgerWithdrawSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferLedgerWithdrawSimulateRequest {
                event_type: event_type.to_owned(),
                failure_reason: None,
                sweep_id: sweep_id.to_owned(),
            },
        }
    }
    /**Trigger the creation of a repayment

Use the `/sandbox/transfer/repayment/simulate` endpoint to trigger the creation of a repayment. As a side effect of calling this route, a repayment is created that includes all unreimbursed returns of guaranteed transfers. If there are no such returns, an 400 error is returned.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferrepaymentsimulate>.*/
    pub fn sandbox_transfer_repayment_simulate(
        &self,
    ) -> FluentRequest<'_, request::SandboxTransferRepaymentSimulateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferRepaymentSimulateRequest {
            },
        }
    }
    /**Manually fire a Transfer webhook

Use the `/sandbox/transfer/fire_webhook` endpoint to manually trigger a `TRANSFER_EVENTS_UPDATE` webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransferfire_webhook>.*/
    pub fn sandbox_transfer_fire_webhook(
        &self,
        webhook: &str,
    ) -> FluentRequest<'_, request::SandboxTransferFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferFireWebhookRequest {
                webhook: webhook.to_owned(),
            },
        }
    }
    /**Create a test clock

Use the `/sandbox/transfer/test_clock/create` endpoint to create a `test_clock` in the Sandbox environment.

A test clock object represents an independent timeline and has a `virtual_time` field indicating the current timestamp of the timeline. Test clocks are used for testing recurring transfers in Sandbox.

A test clock can be associated with up to 5 recurring transfers.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockcreate>.*/
    pub fn sandbox_transfer_test_clock_create(
        &self,
    ) -> FluentRequest<'_, request::SandboxTransferTestClockCreateRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferTestClockCreateRequest {
                virtual_time: None,
            },
        }
    }
    /**Advance a test clock

Use the `/sandbox/transfer/test_clock/advance` endpoint to advance a `test_clock` in the Sandbox environment.

A test clock object represents an independent timeline and has a `virtual_time` field indicating the current timestamp of the timeline. A test clock can be advanced by incrementing `virtual_time`, but may never go back to a lower `virtual_time`.

If a test clock is advanced, we will simulate the changes that ought to occur during the time that elapsed.

For example, a client creates a weekly recurring transfer with a test clock set at t. When the client advances the test clock by setting `virtual_time` = t + 15 days, 2 new originations should be created, along with the webhook events.

The advancement of the test clock from its current `virtual_time` should be limited such that there are no more than 20 originations resulting from the advance operation on each `recurring_transfer` associated with the `test_clock`.

For example, if the recurring transfer associated with this test clock originates once every 4 weeks, you can advance the `virtual_time` up to 80 weeks on each API call.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockadvance>.*/
    pub fn sandbox_transfer_test_clock_advance(
        &self,
        new_virtual_time: chrono::DateTime<chrono::Utc>,
        test_clock_id: &str,
    ) -> FluentRequest<'_, request::SandboxTransferTestClockAdvanceRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferTestClockAdvanceRequest {
                new_virtual_time,
                test_clock_id: test_clock_id.to_owned(),
            },
        }
    }
    /**Get a test clock

Use the `/sandbox/transfer/test_clock/get` endpoint to get a `test_clock` in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clockget>.*/
    pub fn sandbox_transfer_test_clock_get(
        &self,
        test_clock_id: &str,
    ) -> FluentRequest<'_, request::SandboxTransferTestClockGetRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferTestClockGetRequest {
                test_clock_id: test_clock_id.to_owned(),
            },
        }
    }
    /**List test clocks

Use the `/sandbox/transfer/test_clock/list` endpoint to see a list of all your test clocks in the Sandbox environment, by ascending `virtual_time`. Results are paginated; use the `count` and `offset` query parameters to retrieve the desired test clocks.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxtransfertest_clocklist>.*/
    pub fn sandbox_transfer_test_clock_list(
        &self,
    ) -> FluentRequest<'_, request::SandboxTransferTestClockListRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxTransferTestClockListRequest {
                count: None,
                end_virtual_time: None,
                offset: None,
                start_virtual_time: None,
            },
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
    ) -> FluentRequest<'_, request::SandboxPaymentProfileResetLoginRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxPaymentProfileResetLoginRequest {
                payment_profile_token: payment_profile_token.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::EmployersSearchRequest> {
        FluentRequest {
            client: self,
            params: request::EmployersSearchRequest {
                products: products.iter().map(|&x| x.to_owned()).collect(),
                query: query.to_owned(),
            },
        }
    }
    /**(Deprecated) Create an income verification instance

`/income/verification/create` begins the income verification process by returning an `income_verification_id`. You can then provide the `income_verification_id` to `/link/token/create` under the `income_verification` parameter in order to create a Link instance that will prompt the user to go through the income verification flow. Plaid will fire an `INCOME` webhook once the user completes the Payroll Income flow, or when the uploaded documents in the Document Income flow have finished processing.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationcreate>.*/
    pub fn income_verification_create(
        &self,
        webhook: &str,
    ) -> FluentRequest<'_, request::IncomeVerificationCreateRequest> {
        FluentRequest {
            client: self,
            params: request::IncomeVerificationCreateRequest {
                options: None,
                precheck_id: None,
                webhook: webhook.to_owned(),
            },
        }
    }
    /**(Deprecated) Retrieve information from the paystubs used for income verification

`/income/verification/paystubs/get` returns the information collected from the paystubs that were used to verify an end user's income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationpaystubsget>.*/
    pub fn income_verification_paystubs_get(
        &self,
    ) -> FluentRequest<'_, request::IncomeVerificationPaystubsGetRequest> {
        FluentRequest {
            client: self,
            params: request::IncomeVerificationPaystubsGetRequest {
                access_token: None,
                income_verification_id: None,
            },
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
    ) -> FluentRequest<'_, request::IncomeVerificationDocumentsDownloadRequest> {
        FluentRequest {
            client: self,
            params: request::IncomeVerificationDocumentsDownloadRequest {
                access_token: None,
                document_id: None,
                income_verification_id: None,
            },
        }
    }
    /**(Deprecated) Retrieve information from the tax documents used for income verification

`/income/verification/taxforms/get` returns the information collected from forms that were used to verify an end user''s income. It can be called once the status of the verification has been set to `VERIFICATION_STATUS_PROCESSING_COMPLETE`, as reported by the `INCOME: verification_status` webhook. Attempting to call the endpoint before verification has been completed will result in an error.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationtaxformsget>.*/
    pub fn income_verification_taxforms_get(
        &self,
    ) -> FluentRequest<'_, request::IncomeVerificationTaxformsGetRequest> {
        FluentRequest {
            client: self,
            params: request::IncomeVerificationTaxformsGetRequest {
                access_token: None,
                income_verification_id: None,
            },
        }
    }
    /**(Deprecated) Check digital income verification eligibility and optimize conversion

`/income/verification/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification and returns a `precheck_id` that can be provided to `/link/token/create`. If the user is eligible for digital verification, providing the `precheck_id` in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the `precheck_id` can still be provided to `/link/token/create` and the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing either `employer` or `transactions_access_tokens` data will increase the chance of receiving a useful result.

This endpoint has been deprecated; new integrations should use `/credit/payroll_income/precheck` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#incomeverificationprecheck>.*/
    pub fn income_verification_precheck(
        &self,
    ) -> FluentRequest<'_, request::IncomeVerificationPrecheckRequest> {
        FluentRequest {
            client: self,
            params: request::IncomeVerificationPrecheckRequest {
                employer: None,
                payroll_institution: None,
                transactions_access_token: None,
                transactions_access_tokens: None,
                us_military_info: None,
                user: None,
            },
        }
    }
    /**(Deprecated) Retrieve a summary of an individual's employment information

`/employment/verification/get` returns a list of employments through a user payroll that was verified by an end user.

This endpoint has been deprecated; new integrations should use `/credit/employment/get` instead.

See endpoint docs at <https://plaid.com/docs/api/products/income/#employmentverificationget>.*/
    pub fn employment_verification_get(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::EmploymentVerificationGetRequest> {
        FluentRequest {
            client: self,
            params: request::EmploymentVerificationGetRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Create a deposit switch without using Plaid Exchange

This endpoint provides an alternative to `/deposit_switch/create` for customers who have not yet fully integrated with Plaid Exchange. Like `/deposit_switch/create`, it creates a deposit switch entity that will be persisted throughout the lifecycle of the switch.

See endpoint docs at <https://plaid.com/docs/deposit-switch/reference#deposit_switchaltcreate>.*/
    pub fn deposit_switch_alt_create(
        &self,
        target_account: DepositSwitchTargetAccount,
        target_user: DepositSwitchTargetUser,
    ) -> FluentRequest<'_, request::DepositSwitchAltCreateRequest> {
        FluentRequest {
            client: self,
            params: request::DepositSwitchAltCreateRequest {
                country_code: None,
                options: None,
                target_account,
                target_user,
            },
        }
    }
    /**Create Asset or Income Report Audit Copy Token

Plaid can create an Audit Copy token of an Asset Report and/or Income Report to share with participating Government Sponsored Entity (GSE). If you participate in the Day 1 Certainty™ program, Plaid can supply an Audit Copy token directly to Fannie Mae on your behalf. An Audit Copy token contains the same underlying data as the Asset Report and/or Income Report (result of /credit/payroll_income/get).

Use the `/credit/audit_copy_token/create` endpoint to create an `audit_copy_token` and then pass that token to the GSE who needs access.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditaudit_copy_tokencreate>.*/
    pub fn credit_audit_copy_token_create(
        &self,
        report_tokens: &[&str],
    ) -> FluentRequest<'_, request::CreditAuditCopyTokenCreateRequest> {
        FluentRequest {
            client: self,
            params: request::CreditAuditCopyTokenCreateRequest {
                report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
            },
        }
    }
    /**Remove an Audit Copy token

The `/credit/audit_copy_token/remove` endpoint allows you to remove an Audit Copy. Removing an Audit Copy invalidates the `audit_copy_token` associated with it, meaning both you and any third parties holding the token will no longer be able to use it to access Report data. Items associated with the Report data and other Audit Copies of it are not affected and will remain accessible after removing the given Audit Copy.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditaudit_copy_tokenremove>.*/
    pub fn credit_report_audit_copy_remove(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, request::CreditReportAuditCopyRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::CreditReportAuditCopyRemoveRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
    /**Retrieve an Asset Report with Freddie Mac format. Only Freddie Mac can use this endpoint.

The `credit/asset_report/freddie_mac/get` endpoint retrieves the Asset Report in Freddie Mac's JSON format.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_asset_report_freddie_mac_get(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, request::CreditAssetReportFreddieMacGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditAssetReportFreddieMacGetRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
    /**Retrieve an Asset Report with Freddie Mac format (aka VOA - Verification Of Assets), and a Verification Of Employment (VOE) report if this one is available. Only Freddie Mac can use this endpoint.

The `credit/asset_report/freddie_mac/get` endpoint retrieves the Verification of Assets and Verification of Employment reports.

See endpoint docs at <https://plaid.com/docs/none/>.*/
    pub fn credit_freddie_mac_reports_get(
        &self,
        audit_copy_token: &str,
    ) -> FluentRequest<'_, request::CreditFreddieMacReportsGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditFreddieMacReportsGetRequest {
                audit_copy_token: audit_copy_token.to_owned(),
            },
        }
    }
    /**Retrieve information from the bank accounts used for employment verification

`/credit/bank_employment/get` returns the employment report(s) derived from bank transaction data for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_employmentget>.*/
    pub fn credit_bank_employment_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditBankEmploymentGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditBankEmploymentGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Retrieve information from the bank accounts used for income verification

`/credit/bank_income/get` returns the bank income report(s) for a specified user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomeget>.*/
    pub fn credit_bank_income_get(
        &self,
    ) -> FluentRequest<'_, request::CreditBankIncomeGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditBankIncomeGetRequest {
                options: None,
                user_token: None,
            },
        }
    }
    /**Retrieve information from the bank accounts used for income verification in PDF format

`/credit/bank_income/pdf/get` returns the most recent bank income report for a specified user in PDF format.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomepdfget>.*/
    pub fn credit_bank_income_pdf_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditBankIncomePdfGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditBankIncomePdfGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Refresh a user's bank income information

`/credit/bank_income/refresh` refreshes the bank income report data for a specific user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomerefresh>.*/
    pub fn credit_bank_income_refresh(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditBankIncomeRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::CreditBankIncomeRefreshRequest {
                options: None,
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Subscribe and unsubscribe to proactive notifications for a user's income profile

`/credit/bank_income/webhook/update` allows you to subscribe or unsubscribe a user for income webhook notifications. By default, all users start out unsubscribed.

If a user is subscribed, on significant changes to the user's income profile, you will receive a `BANK_INCOME_REFRESH_UPDATE` webhook, prompting you to refresh bank income data for the user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_incomewebhookupdate>.*/
    pub fn credit_bank_income_webhook_update(
        &self,
        enable_webhooks: bool,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditBankIncomeWebhookUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::CreditBankIncomeWebhookUpdateRequest {
                enable_webhooks,
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Update the parsing configuration for a document income verification

`/credit/payroll_income/parsing_config/update` updates the parsing configuration for a document income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeparsing_configupdate>.*/
    pub fn credit_payroll_income_parsing_config_update(
        &self,
        parsing_config: &[&str],
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditPayrollIncomeParsingConfigUpdateRequest> {
        FluentRequest {
            client: self,
            params: request::CreditPayrollIncomeParsingConfigUpdateRequest {
                item_id: None,
                parsing_config: parsing_config.iter().map(|&x| x.to_owned()).collect(),
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Retrieve data for a user's uploaded bank statements

`/credit/bank_statements/uploads/get` returns parsed data from bank statements uploaded by users as part of the Document Income flow. If your account is not enabled for Document Parsing, contact your account manager to request access.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditbank_statementsuploadsget>.*/
    pub fn credit_bank_statements_uploads_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditBankStatementsUploadsGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditBankStatementsUploadsGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Retrieve a user's payroll information

This endpoint gets payroll income information for a specific user, either as a result of the user connecting to their payroll provider or uploading a pay related document.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeget>.*/
    pub fn credit_payroll_income_get(
        &self,
    ) -> FluentRequest<'_, request::CreditPayrollIncomeGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditPayrollIncomeGetRequest {
                user_token: None,
            },
        }
    }
    /**Retrieve fraud insights for a user's manually uploaded document(s).

`/credit/payroll_income/risk_signals/get` can be used as part of the Document Income flow to assess a user-uploaded document for signs of potential fraud or tampering. It returns a risk score for each uploaded document that indicates the likelihood of the document being fraudulent, in addition to details on the individual risk signals contributing to the score.

To trigger risk signal generation for an Item, call `/link/token/create` with `parsing_config` set to include `fraud_risk`, or call `/credit/payroll_income/parsing_config/update`. Once risk signal generation has been triggered, `/credit/payroll_income/risk_signals/get` can be called at any time after the `INCOME_VERIFICATION_RISK_SIGNALS` webhook has been fired.

`/credit/payroll_income/risk_signals/get` is offered as an add-on to Document Income and is billed separately. To request access to this endpoint, submit a product access request or contact your Plaid account manager.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomerisk_signalsget>.*/
    pub fn credit_payroll_income_risk_signals_get(
        &self,
    ) -> FluentRequest<'_, request::CreditPayrollIncomeRiskSignalsGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditPayrollIncomeRiskSignalsGetRequest {
                user_token: None,
            },
        }
    }
    /**Check income verification eligibility and optimize conversion

`/credit/payroll_income/precheck` is an optional endpoint that can be called before initializing a Link session for income verification. It evaluates whether a given user is supportable by digital income verification. If the user is eligible for digital verification, that information will be associated with the user token, and in this way will generate a Link UI optimized for the end user and their specific employer. If the user cannot be confirmed as eligible, the user can still use the income verification flow, but they may be required to manually upload a paystub to verify their income.

While all request fields are optional, providing `employer` data will increase the chance of receiving a useful result.

When testing in Sandbox, you can control the results by providing special test values in the `employer` and `access_tokens` fields. `employer_good` and `employer_bad` will result in `HIGH` and `LOW` confidence values, respectively. `employer_multi` will result in a `HIGH` confidence with multiple payroll options. Likewise, `access_good` and `access_bad` will result in `HIGH` and `LOW` confidence values, respectively. Any other value for `employer` and `access_tokens` in Sandbox will result in `UNKNOWN` confidence.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomeprecheck>.*/
    pub fn credit_payroll_income_precheck(
        &self,
    ) -> FluentRequest<'_, request::CreditPayrollIncomePrecheckRequest> {
        FluentRequest {
            client: self,
            params: request::CreditPayrollIncomePrecheckRequest {
                access_tokens: None,
                employer: None,
                payroll_institution: None,
                us_military_info: None,
                user_token: None,
            },
        }
    }
    /**Retrieve a summary of an individual's employment information

`/credit/employment/get` returns a list of items with employment information from a user's payroll provider that was verified by an end user.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditemploymentget>.*/
    pub fn credit_employment_get(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditEmploymentGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditEmploymentGetRequest {
                user_token: user_token.to_owned(),
            },
        }
    }
    /**Refresh a digital payroll income verification

`/credit/payroll_income/refresh` refreshes a given digital payroll income verification.

See endpoint docs at <https://plaid.com/docs/api/products/income/#creditpayroll_incomerefresh>.*/
    pub fn credit_payroll_income_refresh(
        &self,
        user_token: &str,
    ) -> FluentRequest<'_, request::CreditPayrollIncomeRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::CreditPayrollIncomeRefreshRequest {
                options: None,
                user_token: user_token.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::CreditRelayCreateRequest> {
        FluentRequest {
            client: self,
            params: request::CreditRelayCreateRequest {
                report_tokens: report_tokens.iter().map(|&x| x.to_owned()).collect(),
                secondary_client_id: secondary_client_id.to_owned(),
                webhook: None,
            },
        }
    }
    /**Retrieve the reports associated with a relay token that was shared with you (beta)

`/credit/relay/get` allows third parties to receive a report that was shared with them, using a `relay_token` that was created by the report owner.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayget>.*/
    pub fn credit_relay_get(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> FluentRequest<'_, request::CreditRelayGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditRelayGetRequest {
                relay_token: relay_token.to_owned(),
                report_type: report_type.to_owned(),
            },
        }
    }
    /**Retrieve the pdf reports associated with a relay token that was shared with you (beta)

`/credit/relay/pdf/get` allows third parties to receive a pdf report that was shared with them, using a `relay_token` that was created by the report owner.

The `/credit/relay/pdf/get` endpoint retrieves the Asset Report in PDF format. Before calling `/credit/relay/pdf/get`, you must first create the Asset Report using `/credit/relay/create` and then wait for the [`PRODUCT_READY`](https://plaid.com/docs/api/products/assets/#product_ready) webhook to fire, indicating that the Report is ready to be retrieved.

The response to `/credit/relay/pdf/get` is the PDF binary data. The `request_id` is returned in the `Plaid-Request-ID` header.

[View a sample PDF Asset Report](https://plaid.com/documents/sample-asset-report.pdf).

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelaypdfget>.*/
    pub fn credit_relay_pdf_get(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> FluentRequest<'_, request::CreditRelayPdfGetRequest> {
        FluentRequest {
            client: self,
            params: request::CreditRelayPdfGetRequest {
                relay_token: relay_token.to_owned(),
                report_type: report_type.to_owned(),
            },
        }
    }
    /**Refresh a report of a relay token (beta)

The `/credit/relay/refresh` endpoint allows third parties to refresh a report that was relayed to them, using a `relay_token` that was created by the report owner. A new report will be created with the original report parameters, but with the most recent data available based on the `days_requested` value of the original report.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayrefresh>.*/
    pub fn credit_relay_refresh(
        &self,
        relay_token: &str,
        report_type: &str,
    ) -> FluentRequest<'_, request::CreditRelayRefreshRequest> {
        FluentRequest {
            client: self,
            params: request::CreditRelayRefreshRequest {
                relay_token: relay_token.to_owned(),
                report_type: report_type.to_owned(),
                webhook: None,
            },
        }
    }
    /**Remove relay token (beta)

The `/credit/relay/remove` endpoint allows you to invalidate a `relay_token`. The third party holding the token will no longer be able to access or refresh the reports which the `relay_token` gives access to. The original report, associated Items, and other relay tokens that provide access to the same report are not affected and will remain accessible after removing the given `relay_token`.

See endpoint docs at <https://plaid.com/docs/api/products/assets/#creditrelayremove>.*/
    pub fn credit_relay_remove(
        &self,
        relay_token: &str,
    ) -> FluentRequest<'_, request::CreditRelayRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::CreditRelayRemoveRequest {
                relay_token: relay_token.to_owned(),
            },
        }
    }
    /**Manually fire a Bank Transfer webhook

Use the `/sandbox/bank_transfer/fire_webhook` endpoint to manually trigger a Bank Transfers webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/bank-transfers/reference/#sandboxbank_transferfire_webhook>.*/
    pub fn sandbox_bank_transfer_fire_webhook(
        &self,
        webhook: &str,
    ) -> FluentRequest<'_, request::SandboxBankTransferFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxBankTransferFireWebhookRequest {
                webhook: webhook.to_owned(),
            },
        }
    }
    /**Manually fire an Income webhook

Use the `/sandbox/income/fire_webhook` endpoint to manually trigger a Payroll or Document Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxincomefire_webhook>.*/
    pub fn sandbox_income_fire_webhook(
        &self,
        item_id: &str,
        webhook: &str,
        webhook_code: &str,
    ) -> FluentRequest<'_, request::SandboxIncomeFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxIncomeFireWebhookRequest {
                item_id: item_id.to_owned(),
                user_id: None,
                verification_status: None,
                webhook: webhook.to_owned(),
                webhook_code: webhook_code.to_owned(),
            },
        }
    }
    /**Manually fire a bank income webhook in sandbox

Use the `/sandbox/bank_income/fire_webhook` endpoint to manually trigger a Bank Income webhook in the Sandbox environment.

See endpoint docs at <https://plaid.com/docs/api/sandbox/#sandboxbankincomefire_webhook>.*/
    pub fn sandbox_bank_income_fire_webhook(
        &self,
        webhook_code: &str,
        webhook_fields: SandboxBankIncomeWebhookFireRequestWebhookFields,
    ) -> FluentRequest<'_, request::SandboxBankIncomeFireWebhookRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxBankIncomeFireWebhookRequest {
                webhook_code: webhook_code.to_owned(),
                webhook_fields,
                webhook_override: None,
            },
        }
    }
    ///Save the selected accounts when connecting to the Platypus Oauth institution
    pub fn sandbox_oauth_select_accounts(
        &self,
        accounts: &[&str],
        oauth_state_id: &str,
    ) -> FluentRequest<'_, request::SandboxOauthSelectAccountsRequest> {
        FluentRequest {
            client: self,
            params: request::SandboxOauthSelectAccountsRequest {
                accounts: accounts.iter().map(|&x| x.to_owned()).collect(),
                oauth_state_id: oauth_state_id.to_owned(),
            },
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
    ) -> FluentRequest<'_, request::SignalEvaluateRequest> {
        FluentRequest {
            client: self,
            params: request::SignalEvaluateRequest {
                access_token: args.access_token.to_owned(),
                account_id: args.account_id.to_owned(),
                amount: args.amount,
                client_transaction_id: args.client_transaction_id.to_owned(),
                client_user_id: None,
                default_payment_method: None,
                device: None,
                is_recurring: None,
                risk_profile_key: None,
                user: None,
                user_present: None,
            },
        }
    }
    /**Report whether you initiated an ACH transaction

After calling `/signal/evaluate`, call `/signal/decision/report` to report whether the transaction was initiated.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signaldecisionreport>.*/
    pub fn signal_decision_report(
        &self,
        client_transaction_id: &str,
        initiated: bool,
    ) -> FluentRequest<'_, request::SignalDecisionReportRequest> {
        FluentRequest {
            client: self,
            params: request::SignalDecisionReportRequest {
                amount_instantly_available: None,
                client_transaction_id: client_transaction_id.to_owned(),
                days_funds_on_hold: None,
                decision_outcome: None,
                initiated,
                payment_method: None,
            },
        }
    }
    /**Report a return for an ACH transaction

Call the `/signal/return/report` endpoint to report a returned transaction that was previously sent to the `/signal/evaluate` endpoint. Your feedback will be used by the model to incorporate the latest risk trend in your portfolio.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalreturnreport>.*/
    pub fn signal_return_report(
        &self,
        client_transaction_id: &str,
        return_code: &str,
    ) -> FluentRequest<'_, request::SignalReturnReportRequest> {
        FluentRequest {
            client: self,
            params: request::SignalReturnReportRequest {
                client_transaction_id: client_transaction_id.to_owned(),
                return_code: return_code.to_owned(),
                returned_at: None,
            },
        }
    }
    /**Opt-in an Item to Signal

When Link is not initialized with Signal, call `/signal/prepare` to opt-in that Item to the Signal data collection process, developing a Signal score.

If you are using other Plaid products after Link, e.g. Identity or Assets, call `/signal/prepare` after those product calls are complete.

Example flow: Link is initialized with Auth, call `/auth/get` for the account and routing number, call `/identity/get` to retrieve bank ownership details, then call `/signal/prepare` to begin Signal data collection. Later, once you have obtained details about the proposed transaction from the user, call `/signal/evaluate` for a Signal score. For more information please see [Recommendations for initializing Link with specific product combinations](https://www.plaid.com/docs/link/initializing-products/#recommendations-for-initializing-link-with-specific-product-combinations).

If run on an Item that is already initialized with Signal, this endpoint will return a 200 response and will not modify the Item.

See endpoint docs at <https://plaid.com/docs/api/products/signal#signalprepare>.*/
    pub fn signal_prepare(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::SignalPrepareRequest> {
        FluentRequest {
            client: self,
            params: request::SignalPrepareRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Create an e-wallet

Create an e-wallet. The response is the newly created e-wallet object.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletcreate>.*/
    pub fn wallet_create(
        &self,
        iso_currency_code: &str,
    ) -> FluentRequest<'_, request::WalletCreateRequest> {
        FluentRequest {
            client: self,
            params: request::WalletCreateRequest {
                iso_currency_code: iso_currency_code.to_owned(),
            },
        }
    }
    /**Fetch an e-wallet

Fetch an e-wallet. The response includes the current balance.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletget>.*/
    pub fn wallet_get(
        &self,
        wallet_id: &str,
    ) -> FluentRequest<'_, request::WalletGetRequest> {
        FluentRequest {
            client: self,
            params: request::WalletGetRequest {
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
    /**Fetch a list of e-wallets

This endpoint lists all e-wallets in descending order of creation.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#walletlist>.*/
    pub fn wallet_list(&self) -> FluentRequest<'_, request::WalletListRequest> {
        FluentRequest {
            client: self,
            params: request::WalletListRequest {
                count: None,
                cursor: None,
                iso_currency_code: None,
            },
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
    ) -> FluentRequest<'_, request::WalletTransactionExecuteRequest> {
        FluentRequest {
            client: self,
            params: request::WalletTransactionExecuteRequest {
                amount: args.amount,
                counterparty: args.counterparty,
                idempotency_key: args.idempotency_key.to_owned(),
                reference: args.reference.to_owned(),
                wallet_id: args.wallet_id.to_owned(),
            },
        }
    }
    /**Fetch an e-wallet transaction

Fetch a specific e-wallet transaction

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionget>.*/
    pub fn wallet_transaction_get(
        &self,
        transaction_id: &str,
    ) -> FluentRequest<'_, request::WalletTransactionGetRequest> {
        FluentRequest {
            client: self,
            params: request::WalletTransactionGetRequest {
                transaction_id: transaction_id.to_owned(),
            },
        }
    }
    /**List e-wallet transactions

This endpoint lists the latest transactions of the specified e-wallet. Transactions are returned in descending order by the `created_at` time.

See endpoint docs at <https://plaid.com/docs/api/products/virtual-accounts/#wallettransactionlist>.*/
    pub fn wallet_transaction_list(
        &self,
        wallet_id: &str,
    ) -> FluentRequest<'_, request::WalletTransactionListRequest> {
        FluentRequest {
            client: self,
            params: request::WalletTransactionListRequest {
                count: None,
                cursor: None,
                options: None,
                wallet_id: wallet_id.to_owned(),
            },
        }
    }
    /**enhance locally-held transaction data

The `/beta/transactions/v1/enhance` endpoint enriches raw transaction data provided directly by clients.

The product is currently in beta.*/
    pub fn transactions_enhance(
        &self,
        account_type: &str,
        transactions: Vec<ClientProvidedRawTransaction>,
    ) -> FluentRequest<'_, request::TransactionsEnhanceRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsEnhanceRequest {
                account_type: account_type.to_owned(),
                transactions,
            },
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
    ) -> FluentRequest<'_, request::TransactionsRulesCreateRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsRulesCreateRequest {
                access_token: access_token.to_owned(),
                personal_finance_category: personal_finance_category.to_owned(),
                rule_details,
            },
        }
    }
    /**Return a list of rules created for the Item associated with the access token.

The `/transactions/rules/v1/list` returns a list of transaction rules created for the Item associated with the access token.*/
    pub fn transactions_rules_list(
        &self,
        access_token: &str,
    ) -> FluentRequest<'_, request::TransactionsRulesListRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsRulesListRequest {
                access_token: access_token.to_owned(),
            },
        }
    }
    /**Remove transaction rule

The `/transactions/rules/v1/remove` endpoint is used to remove a transaction rule.*/
    pub fn transactions_rules_remove(
        &self,
        access_token: &str,
        rule_id: &str,
    ) -> FluentRequest<'_, request::TransactionsRulesRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsRulesRemoveRequest {
                access_token: access_token.to_owned(),
                rule_id: rule_id.to_owned(),
            },
        }
    }
    /**Obtain user insights based on transactions sent through /transactions/enrich

The `/beta/transactions/user_insights/v1/get` gets user insights for clients who have enriched data with `/transactions/enrich`.

The product is currently in beta.

See endpoint docs at <https://plaid.com/docs/api/products/enrich/#userinsightsget>.*/
    pub fn transactions_user_insights_get(
        &self,
        client_user_id: &str,
    ) -> FluentRequest<'_, request::TransactionsUserInsightsGetRequest> {
        FluentRequest {
            client: self,
            params: request::TransactionsUserInsightsGetRequest {
                client_user_id: client_user_id.to_owned(),
            },
        }
    }
    /**Create payment profile

Use `/payment_profile/create` endpoint to create a new payment profile.
To initiate the account linking experience, call `/link/token/create` and provide the `payment_profile_token` in the `transfer.payment_profile_token` field.
You can then use the `payment_profile_token` when creating transfers using `/transfer/authorization/create` and `/transfer/create`.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profilecreate>.*/
    pub fn payment_profile_create(
        &self,
    ) -> FluentRequest<'_, request::PaymentProfileCreateRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentProfileCreateRequest {
            },
        }
    }
    /**Get payment profile

Use `/payment_profile/get` endpoint to get the status of a given Payment Profile.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileget>.*/
    pub fn payment_profile_get(
        &self,
        payment_profile_token: &str,
    ) -> FluentRequest<'_, request::PaymentProfileGetRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentProfileGetRequest {
                payment_profile_token: payment_profile_token.to_owned(),
            },
        }
    }
    /**Remove payment profile

Use the `/payment_profile/remove` endpoint to remove a given Payment Profile. Once it’s removed, it can no longer be used to create transfers.

See endpoint docs at <https://plaid.com/docs/api/products/transfer/#payment_profileremove>.*/
    pub fn payment_profile_remove(
        &self,
        payment_profile_token: &str,
    ) -> FluentRequest<'_, request::PaymentProfileRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::PaymentProfileRemoveRequest {
                payment_profile_token: payment_profile_token.to_owned(),
            },
        }
    }
    /**Creates a new end customer for a Plaid reseller.

The `/partner/customer/create` endpoint is used by reseller partners to create end customers.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomercreate>.*/
    pub fn partner_customer_create(
        &self,
        args: request::PartnerCustomerCreateRequired,
    ) -> FluentRequest<'_, request::PartnerCustomerCreateRequest> {
        FluentRequest {
            client: self,
            params: request::PartnerCustomerCreateRequest {
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
                products: None,
                redirect_uris: None,
                registration_number: None,
                secret: None,
                technical_contact: None,
                website: args.website.to_owned(),
            },
        }
    }
    /**Returns a Plaid reseller's end customer.

The `/partner/customer/get` endpoint is used by reseller partners to retrieve data about a single end customer.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerget>.*/
    pub fn partner_customer_get(
        &self,
        end_customer_client_id: &str,
    ) -> FluentRequest<'_, request::PartnerCustomerGetRequest> {
        FluentRequest {
            client: self,
            params: request::PartnerCustomerGetRequest {
                client_id: None,
                end_customer_client_id: end_customer_client_id.to_owned(),
                secret: None,
            },
        }
    }
    /**Enables a Plaid reseller's end customer in the Production environment.

The `/partner/customer/enable` endpoint is used by reseller partners to enable an end customer in the Production environment.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerenable>.*/
    pub fn partner_customer_enable(
        &self,
        end_customer_client_id: &str,
    ) -> FluentRequest<'_, request::PartnerCustomerEnableRequest> {
        FluentRequest {
            client: self,
            params: request::PartnerCustomerEnableRequest {
                client_id: None,
                end_customer_client_id: end_customer_client_id.to_owned(),
                secret: None,
            },
        }
    }
    /**Removes a Plaid reseller's end customer.

The `/partner/customer/remove` endpoint is used by reseller partners to remove an end customer. Removing an end customer will remove it from view in the Plaid Dashboard and deactivate its API keys. This endpoint can only be used to remove an end customer that has not yet been enabled in Production.

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomerremove>.*/
    pub fn partner_customer_remove(
        &self,
        end_customer_client_id: &str,
    ) -> FluentRequest<'_, request::PartnerCustomerRemoveRequest> {
        FluentRequest {
            client: self,
            params: request::PartnerCustomerRemoveRequest {
                client_id: None,
                end_customer_client_id: end_customer_client_id.to_owned(),
                secret: None,
            },
        }
    }
    /**Returns OAuth-institution registration information for a given end customer.

The `/partner/customer/oauth_institutions/get` endpoint is used by reseller partners to retrieve OAuth-institution registration information about a single end customer. To learn how to set up a webhook to listen to status update events, visit the [reseller documentation](https://plaid.com/docs/account/resellers/#enabling-end-customers).

See endpoint docs at <https://plaid.com/docs/api/partner/#partnercustomeroauth_institutionsget>.*/
    pub fn partner_customer_oauth_institutions_get(
        &self,
        end_customer_client_id: &str,
    ) -> FluentRequest<'_, request::PartnerCustomerOauthInstitutionsGetRequest> {
        FluentRequest {
            client: self,
            params: request::PartnerCustomerOauthInstitutionsGetRequest {
                client_id: None,
                end_customer_client_id: end_customer_client_id.to_owned(),
                secret: None,
            },
        }
    }
    /**Create Hosted Link session

Use the `/link_delivery/create` endpoint to create a Hosted Link session.

See endpoint docs at <https://plaid.com/docs/assets/waitlist/hosted-link/>.*/
    pub fn link_delivery_create(
        &self,
        link_token: &str,
    ) -> FluentRequest<'_, request::LinkDeliveryCreateRequest> {
        FluentRequest {
            client: self,
            params: request::LinkDeliveryCreateRequest {
                link_token: link_token.to_owned(),
                options: None,
            },
        }
    }
    /**Get Hosted Link session

Use the `/link_delivery/get` endpoint to get the status of a Hosted Link session.

See endpoint docs at <https://plaid.com/docs/assets/waitlist/hosted-link/>.*/
    pub fn link_delivery_get(
        &self,
        link_delivery_session_id: &str,
    ) -> FluentRequest<'_, request::LinkDeliveryGetRequest> {
        FluentRequest {
            client: self,
            params: request::LinkDeliveryGetRequest {
                link_delivery_session_id: link_delivery_session_id.to_owned(),
            },
        }
    }
    /**Webhook receiver for fdx notifications

A generic webhook receiver endpoint for FDX Event Notifications

See endpoint docs at <https://plaid.com/docs/api/fdx/notifications/#post>.*/
    pub fn fdx_notifications(
        &self,
        args: request::FdxNotificationsRequired,
    ) -> FluentRequest<'_, request::FdxNotificationsRequest> {
        FluentRequest {
            client: self,
            params: request::FdxNotificationsRequest {
                category: args.category.to_owned(),
                notification_id: args.notification_id.to_owned(),
                notification_payload: args.notification_payload,
                priority: None,
                publisher: None,
                sent_on: args.sent_on,
                severity: None,
                subscriber: None,
                type_: args.type_.to_owned(),
                url: None,
            },
        }
    }
}
pub enum PlaidAuth {
    ClientId { client_id: String, secret: String, plaid_version: String },
}
impl PlaidAuth {
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
