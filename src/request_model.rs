use serde_json::json;
use crate::model;
use crate::model::*;
use crate::PlaidClient;
pub struct ItemApplicationListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: Option<String>,
}
impl<'a> ItemApplicationListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemApplicationListResponse> {
        let mut r = self.client.client.post("/item/application/list");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemApplicationScopesUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub application_id: String,
    pub scopes: serde_json::Value,
    pub state: String,
    pub context: String,
}
impl<'a> ItemApplicationScopesUpdateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::ItemApplicationScopesUpdateResponse> {
        let mut r = self.client.client.post("/item/application/scopes/update");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "application_id" : self
                    .application_id, "scopes" : self.scopes, "state" : self.state,
                    "context" : self.context }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ApplicationGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub application_id: String,
}
impl<'a> ApplicationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ApplicationGetResponse> {
        let mut r = self.client.client.post("/application/get");
        r = r.json(json!({ "application_id" : self.application_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemGetResponse> {
        let mut r = self.client.client.post("/item/get");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AuthGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> AuthGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AuthGetResponse> {
        let mut r = self.client.client.post("/auth/get");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "options" : self.options }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub options: serde_json::Value,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
}
impl<'a> TransactionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsGetResponse> {
        let mut r = self.client.client.post("/transactions/get");
        r = r
            .json(
                json!(
                    { "options" : self.options, "access_token" : self.access_token,
                    "start_date" : self.start_date, "end_date" : self.end_date }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> TransactionsRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRefreshResponse> {
        let mut r = self.client.client.post("/transactions/refresh");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRecurringGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
    pub account_ids: Vec<String>,
}
impl<'a> TransactionsRecurringGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRecurringGetResponse> {
        let mut r = self.client.client.post("/transactions/recurring/get");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "options" : self.options,
                    "account_ids" : self.account_ids }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsSyncRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub cursor: String,
    pub count: i64,
    pub options: serde_json::Value,
}
impl<'a> TransactionsSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsSyncResponse> {
        let mut r = self.client.client.post("/transactions/sync");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "cursor" : self.cursor, "count"
                    : self.count, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InstitutionsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub count: i64,
    pub offset: i64,
    pub country_codes: Vec<CountryCode>,
    pub options: serde_json::Value,
}
impl<'a> InstitutionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InstitutionsGetResponse> {
        let mut r = self.client.client.post("/institutions/get");
        r = r
            .json(
                json!(
                    { "count" : self.count, "offset" : self.offset, "country_codes" :
                    self.country_codes, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InstitutionsSearchRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub query: String,
    pub products: Option<Vec<Products>>,
    pub country_codes: Vec<CountryCode>,
    pub options: serde_json::Value,
}
impl<'a> InstitutionsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InstitutionsSearchResponse> {
        let mut r = self.client.client.post("/institutions/search");
        r = r
            .json(
                json!(
                    { "query" : self.query, "products" : self.products, "country_codes" :
                    self.country_codes, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InstitutionsGetByIdRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub institution_id: String,
    pub country_codes: Vec<CountryCode>,
    pub options: serde_json::Value,
}
impl<'a> InstitutionsGetByIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InstitutionsGetByIdResponse> {
        let mut r = self.client.client.post("/institutions/get_by_id");
        r = r
            .json(
                json!(
                    { "institution_id" : self.institution_id, "country_codes" : self
                    .country_codes, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemRemoveResponse> {
        let mut r = self.client.client.post("/item/remove");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AccountsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> AccountsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AccountsGetResponse> {
        let mut r = self.client.client.post("/accounts/get");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "options" : self.options }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CategoriesGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> CategoriesGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CategoriesGetResponse> {
        let mut r = self.client.client.post("/categories/get");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub institution_id: String,
    pub options: serde_json::Value,
}
impl<'a> SandboxProcessorTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxProcessorTokenCreateResponse> {
        let mut r = self.client.client.post("/sandbox/processor_token/create");
        r = r
            .json(
                json!(
                    { "institution_id" : self.institution_id, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxPublicTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub institution_id: String,
    pub initial_products: Vec<Products>,
    pub options: serde_json::Value,
    pub user_token: String,
}
impl<'a> SandboxPublicTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxPublicTokenCreateResponse> {
        let mut r = self.client.client.post("/sandbox/public_token/create");
        r = r
            .json(
                json!(
                    { "institution_id" : self.institution_id, "initial_products" : self
                    .initial_products, "options" : self.options, "user_token" : self
                    .user_token }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxItemFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub webhook_type: String,
    pub webhook_code: String,
}
impl<'a> SandboxItemFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxItemFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/item/fire_webhook");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "webhook_type" : self
                    .webhook_type, "webhook_code" : self.webhook_code }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AccountsBalanceGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> AccountsBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AccountsGetResponse> {
        let mut r = self.client.client.post("/accounts/balance/get");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "options" : self.options }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> IdentityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IdentityGetResponse> {
        let mut r = self.client.client.post("/identity/get");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "options" : self.options }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityMatchRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub user: serde_json::Value,
    pub options: serde_json::Value,
}
impl<'a> IdentityMatchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IdentityMatchResponse> {
        let mut r = self.client.client.post("/identity/match");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "user" : self.user, "options" :
                    self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DashobardUserGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub dashboard_user_id: String,
}
impl<'a> DashobardUserGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DashboardUserResponse> {
        let mut r = self.client.client.post("/dashboard_user/get");
        r = r.json(json!({ "dashboard_user_id" : self.dashboard_user_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DashboardUserListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> DashboardUserListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedDashboardUserListResponse> {
        let mut r = self.client.client.post("/dashboard_user/list");
        r = r.json(json!({ "cursor" : self.cursor }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityVerificationCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub is_shareable: bool,
    pub template_id: String,
    pub gave_consent: bool,
    pub user: serde_json::Value,
    pub is_idempotent: Option<bool>,
}
impl<'a> IdentityVerificationCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IdentityVerificationResponse> {
        let mut r = self.client.client.post("/identity_verification/create");
        r = r
            .json(
                json!(
                    { "is_shareable" : self.is_shareable, "template_id" : self
                    .template_id, "gave_consent" : self.gave_consent, "user" : self.user,
                    "is_idempotent" : self.is_idempotent }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityVerificationGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub identity_verification_id: String,
}
impl<'a> IdentityVerificationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IdentityVerificationResponse> {
        let mut r = self.client.client.post("/identity_verification/get");
        r = r
            .json(json!({ "identity_verification_id" : self.identity_verification_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityVerificationListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub template_id: String,
    pub client_user_id: String,
    pub cursor: Option<String>,
}
impl<'a> IdentityVerificationListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedIdentityVerificationListResponse> {
        let mut r = self.client.client.post("/identity_verification/list");
        r = r
            .json(
                json!(
                    { "template_id" : self.template_id, "client_user_id" : self
                    .client_user_id, "cursor" : self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IdentityVerificationRetryRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_user_id: String,
    pub template_id: String,
    pub strategy: String,
    pub steps: Option<serde_json::Value>,
}
impl<'a> IdentityVerificationRetryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IdentityVerificationResponse> {
        let mut r = self.client.client.post("/identity_verification/retry");
        r = r
            .json(
                json!(
                    { "client_user_id" : self.client_user_id, "template_id" : self
                    .template_id, "strategy" : self.strategy, "steps" : self.steps }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub search_terms: serde_json::Value,
    pub client_user_id: Option<serde_json::Value>,
}
impl<'a> WatchlistScreeningEntityCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EntityWatchlistScreeningResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/create");
        r = r
            .json(
                json!(
                    { "search_terms" : self.search_terms, "client_user_id" : self
                    .client_user_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningEntityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EntityWatchlistScreeningResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/get");
        r = r
            .json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityHistoryListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityHistoryListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedEntityWatchlistScreeningListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/history/list");
        r = r
            .json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id, "cursor" : self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityHitsListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityHitsListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedEntityWatchlistScreeningHitListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/hit/list");
        r = r
            .json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id, "cursor" : self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_program_id: String,
    pub client_user_id: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedEntityWatchlistScreeningListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/list");
        r = r
            .json(
                json!(
                    { "entity_watchlist_program_id" : self.entity_watchlist_program_id,
                    "client_user_id" : self.client_user_id, "status" : self.status,
                    "assignee" : self.assignee, "cursor" : self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityProgramGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_program_id: String,
}
impl<'a> WatchlistScreeningEntityProgramGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EntityWatchlistProgramResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/program/get");
        r = r
            .json(
                json!(
                    { "entity_watchlist_program_id" : self.entity_watchlist_program_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityProgramListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityProgramListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedEntityWatchlistProgramListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/program/list");
        r = r.json(json!({ "cursor" : self.cursor }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityReviewCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub confirmed_hits: Vec<EntityWatchlistScreeningHitId>,
    pub dismissed_hits: Vec<EntityWatchlistScreeningHitId>,
    pub comment: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningEntityReviewCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::EntityWatchlistScreeningReviewResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/review/create");
        r = r
            .json(
                json!(
                    { "confirmed_hits" : self.confirmed_hits, "dismissed_hits" : self
                    .dismissed_hits, "comment" : self.comment,
                    "entity_watchlist_screening_id" : self.entity_watchlist_screening_id
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityReviewListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityReviewListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedEntityWatchlistScreeningReviewListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/review/list");
        r = r
            .json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id, "cursor" : self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningEntityUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub search_terms: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub client_user_id: Option<serde_json::Value>,
    pub reset_fields: Option<Vec<UpdateEntityScreeningRequestResettableField>>,
}
impl<'a> WatchlistScreeningEntityUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EntityWatchlistScreeningResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/update");
        r = r
            .json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id, "search_terms" : self.search_terms,
                    "assignee" : self.assignee, "status" : self.status, "client_user_id"
                    : self.client_user_id, "reset_fields" : self.reset_fields }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub search_terms: serde_json::Value,
    pub client_user_id: Option<serde_json::Value>,
}
impl<'a> WatchlistScreeningIndividualCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::WatchlistScreeningIndividualResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/create");
        r = r
            .json(
                json!(
                    { "search_terms" : self.search_terms, "client_user_id" : self
                    .client_user_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::WatchlistScreeningIndividualResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/get");
        r = r.json(json!({ "watchlist_screening_id" : self.watchlist_screening_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualHistoryListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualHistoryListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedIndividualWatchlistScreeningListResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/history/list");
        r = r
            .json(
                json!(
                    { "watchlist_screening_id" : self.watchlist_screening_id, "cursor" :
                    self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualHitListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualHitListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedIndividualWatchlistScreeningHitListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/hit/list");
        r = r
            .json(
                json!(
                    { "watchlist_screening_id" : self.watchlist_screening_id, "cursor" :
                    self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_program_id: String,
    pub client_user_id: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedIndividualWatchlistScreeningListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/list");
        r = r
            .json(
                json!(
                    { "watchlist_program_id" : self.watchlist_program_id,
                    "client_user_id" : self.client_user_id, "status" : self.status,
                    "assignee" : self.assignee, "cursor" : self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualProgramGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_program_id: String,
}
impl<'a> WatchlistScreeningIndividualProgramGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IndividualWatchlistProgramResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/program/get");
        r = r.json(json!({ "watchlist_program_id" : self.watchlist_program_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualProgramListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualProgramListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedIndividualWatchlistProgramListResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/program/list");
        r = r.json(json!({ "cursor" : self.cursor }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualReviewCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub confirmed_hits: Vec<WatchlistScreeningHitId>,
    pub dismissed_hits: Vec<WatchlistScreeningHitId>,
    pub comment: Option<String>,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualReviewCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WatchlistScreeningReviewResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/review/create");
        r = r
            .json(
                json!(
                    { "confirmed_hits" : self.confirmed_hits, "dismissed_hits" : self
                    .dismissed_hits, "comment" : self.comment, "watchlist_screening_id" :
                    self.watchlist_screening_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualReviewsListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualReviewsListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaginatedIndividualWatchlistScreeningReviewListResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/review/list");
        r = r
            .json(
                json!(
                    { "watchlist_screening_id" : self.watchlist_screening_id, "cursor" :
                    self.cursor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WatchlistScreeningIndividualUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub search_terms: Option<serde_json::Value>,
    pub assignee: Option<serde_json::Value>,
    pub status: Option<serde_json::Value>,
    pub client_user_id: Option<serde_json::Value>,
    pub reset_fields: Option<Vec<UpdateIndividualScreeningRequestResettableField>>,
}
impl<'a> WatchlistScreeningIndividualUpdateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::WatchlistScreeningIndividualResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/update");
        r = r
            .json(
                json!(
                    { "watchlist_screening_id" : self.watchlist_screening_id,
                    "search_terms" : self.search_terms, "assignee" : self.assignee,
                    "status" : self.status, "client_user_id" : self.client_user_id,
                    "reset_fields" : self.reset_fields }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorAuthGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub processor_token: String,
}
impl<'a> ProcessorAuthGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorAuthGetResponse> {
        let mut r = self.client.client.post("/processor/auth/get");
        r = r.json(json!({ "processor_token" : self.processor_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorBankTransferCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: String,
    pub processor_token: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub iso_currency_code: String,
    pub description: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub custom_tag: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub origination_account_id: Option<String>,
}
impl<'a> ProcessorBankTransferCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::ProcessorBankTransferCreateResponse> {
        let mut r = self.client.client.post("/processor/bank_transfer/create");
        r = r
            .json(
                json!(
                    { "idempotency_key" : self.idempotency_key, "processor_token" : self
                    .processor_token, "type" : self.type_, "network" : self.network,
                    "amount" : self.amount, "iso_currency_code" : self.iso_currency_code,
                    "description" : self.description, "ach_class" : self.ach_class,
                    "user" : self.user, "custom_tag" : self.custom_tag, "metadata" : self
                    .metadata, "origination_account_id" : self.origination_account_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorIdentityGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub processor_token: String,
}
impl<'a> ProcessorIdentityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorIdentityGetResponse> {
        let mut r = self.client.client.post("/processor/identity/get");
        r = r.json(json!({ "processor_token" : self.processor_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorBalanceGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub processor_token: String,
    pub options: serde_json::Value,
}
impl<'a> ProcessorBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorBalanceGetResponse> {
        let mut r = self.client.client.post("/processor/balance/get");
        r = r
            .json(
                json!(
                    { "processor_token" : self.processor_token, "options" : self.options
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemWebhookUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub webhook: Option<String>,
}
impl<'a> ItemWebhookUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemWebhookUpdateResponse> {
        let mut r = self.client.client.post("/item/webhook/update");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "webhook" : self.webhook }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemAccessTokenInvalidateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemAccessTokenInvalidateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemAccessTokenInvalidateResponse> {
        let mut r = self.client.client.post("/item/access_token/invalidate");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WebhookVerificationKeyGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub key_id: String,
}
impl<'a> WebhookVerificationKeyGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WebhookVerificationKeyGetResponse> {
        let mut r = self.client.client.post("/webhook_verification_key/get");
        r = r.json(json!({ "key_id" : self.key_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct LiabilitiesGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> LiabilitiesGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::LiabilitiesGetResponse> {
        let mut r = self.client.client.post("/liabilities/get");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "options" : self.options }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationRecipientCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub name: String,
    pub iban: Option<String>,
    pub bacs: Option<serde_json::Value>,
    pub address: Option<serde_json::Value>,
}
impl<'a> PaymentInitiationRecipientCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationRecipientCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/recipient/create");
        r = r
            .json(
                json!(
                    { "name" : self.name, "iban" : self.iban, "bacs" : self.bacs,
                    "address" : self.address }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentReverseRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_id: String,
    pub idempotency_key: String,
    pub reference: String,
}
impl<'a> PaymentInitiationPaymentReverseRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentReverseResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/reverse");
        r = r
            .json(
                json!(
                    { "payment_id" : self.payment_id, "idempotency_key" : self
                    .idempotency_key, "reference" : self.reference }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationRecipientGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub recipient_id: String,
}
impl<'a> PaymentInitiationRecipientGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationRecipientGetResponse> {
        let mut r = self.client.client.post("/payment_initiation/recipient/get");
        r = r.json(json!({ "recipient_id" : self.recipient_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationRecipientListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> PaymentInitiationRecipientListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationRecipientListResponse> {
        let mut r = self.client.client.post("/payment_initiation/recipient/list");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub recipient_id: String,
    pub reference: String,
    pub amount: serde_json::Value,
    pub schedule: serde_json::Value,
    pub options: Option<serde_json::Value>,
}
impl<'a> PaymentInitiationPaymentCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/create");
        r = r
            .json(
                json!(
                    { "recipient_id" : self.recipient_id, "reference" : self.reference,
                    "amount" : self.amount, "schedule" : self.schedule, "options" : self
                    .options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreatePaymentTokenRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_id: String,
}
impl<'a> CreatePaymentTokenRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentTokenCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/token/create");
        r = r.json(json!({ "payment_id" : self.payment_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationConsentCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<PaymentInitiationConsentScope>,
    pub constraints: serde_json::Value,
    pub options: Option<serde_json::Value>,
}
impl<'a> PaymentInitiationConsentCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationConsentCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/consent/create");
        r = r
            .json(
                json!(
                    { "recipient_id" : self.recipient_id, "reference" : self.reference,
                    "scopes" : self.scopes, "constraints" : self.constraints, "options" :
                    self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationConsentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub consent_id: String,
}
impl<'a> PaymentInitiationConsentGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationConsentGetResponse> {
        let mut r = self.client.client.post("/payment_initiation/consent/get");
        r = r.json(json!({ "consent_id" : self.consent_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationConsentRevokeRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub consent_id: String,
}
impl<'a> PaymentInitiationConsentRevokeRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationConsentRevokeResponse> {
        let mut r = self.client.client.post("/payment_initiation/consent/revoke");
        r = r.json(json!({ "consent_id" : self.consent_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationConsentPaymentExecuteRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub consent_id: String,
    pub amount: serde_json::Value,
    pub idempotency_key: String,
}
impl<'a> PaymentInitiationConsentPaymentExecuteRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationConsentPaymentExecuteResponse> {
        let mut r = self
            .client
            .client
            .post("/payment_initiation/consent/payment/execute");
        r = r
            .json(
                json!(
                    { "consent_id" : self.consent_id, "amount" : self.amount,
                    "idempotency_key" : self.idempotency_key }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxItemResetLoginRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> SandboxItemResetLoginRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxItemResetLoginResponse> {
        let mut r = self.client.client.post("/sandbox/item/reset_login");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxItemSetVerificationStatusRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub verification_status: String,
}
impl<'a> SandboxItemSetVerificationStatusRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxItemSetVerificationStatusResponse> {
        let mut r = self.client.client.post("/sandbox/item/set_verification_status");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "account_id" : self.account_id,
                    "verification_status" : self.verification_status }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemPublicTokenExchangeRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub public_token: String,
}
impl<'a> ItemPublicTokenExchangeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemPublicTokenExchangeResponse> {
        let mut r = self.client.client.post("/item/public_token/exchange");
        r = r.json(json!({ "public_token" : self.public_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemCreatePublicTokenRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemCreatePublicTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemPublicTokenCreateResponse> {
        let mut r = self.client.client.post("/item/public_token/create");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct UserCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_user_id: String,
}
impl<'a> UserCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UserCreateResponse> {
        let mut r = self.client.client.post("/user/create");
        r = r.json(json!({ "client_user_id" : self.client_user_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_id: String,
}
impl<'a> PaymentInitiationPaymentGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentGetResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/get");
        r = r.json(json!({ "payment_id" : self.payment_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentInitiationPaymentListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub consent_id: Option<String>,
}
impl<'a> PaymentInitiationPaymentListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::PaymentInitiationPaymentListResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/list");
        r = r
            .json(
                json!(
                    { "count" : self.count, "cursor" : self.cursor, "consent_id" : self
                    .consent_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_tokens: Vec<AccessToken>,
    pub days_requested: i64,
    pub options: serde_json::Value,
}
impl<'a> AssetReportCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportCreateResponse> {
        let mut r = self.client.client.post("/asset_report/create");
        r = r
            .json(
                json!(
                    { "access_tokens" : self.access_tokens, "days_requested" : self
                    .days_requested, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: serde_json::Value,
}
impl<'a> AssetReportRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRefreshResponse> {
        let mut r = self.client.client.post("/asset_report/refresh");
        r = r
            .json(
                json!(
                    { "asset_report_token" : self.asset_report_token, "days_requested" :
                    self.days_requested, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRelayRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_relay_token: String,
    pub webhook: Option<String>,
}
impl<'a> AssetReportRelayRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRelayRefreshResponse> {
        let mut r = self.client.client.post("/asset_report/relay/refresh");
        r = r
            .json(
                json!(
                    { "asset_relay_token" : self.asset_relay_token, "webhook" : self
                    .webhook }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
}
impl<'a> AssetReportRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRemoveResponse> {
        let mut r = self.client.client.post("/asset_report/remove");
        r = r.json(json!({ "asset_report_token" : self.asset_report_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportFilterRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub account_ids_to_exclude: Vec<String>,
}
impl<'a> AssetReportFilterRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportFilterResponse> {
        let mut r = self.client.client.post("/asset_report/filter");
        r = r
            .json(
                json!(
                    { "asset_report_token" : self.asset_report_token,
                    "account_ids_to_exclude" : self.account_ids_to_exclude }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub include_insights: bool,
    pub fast_report: bool,
}
impl<'a> AssetReportGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportGetResponse> {
        let mut r = self.client.client.post("/asset_report/get");
        r = r
            .json(
                json!(
                    { "asset_report_token" : self.asset_report_token, "include_insights"
                    : self.include_insights, "fast_report" : self.fast_report }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportAuditCopyCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub auditor_id: String,
}
impl<'a> AssetReportAuditCopyCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::AssetReportAuditCopyCreateResponse> {
        let mut r = self.client.client.post("/asset_report/audit_copy/create");
        r = r
            .json(
                json!(
                    { "asset_report_token" : self.asset_report_token, "auditor_id" : self
                    .auditor_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportAuditCopyRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyRemoveRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::AssetReportAuditCopyRemoveResponse> {
        let mut r = self.client.client.post("/asset_report/audit_copy/remove");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRelayCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl<'a> AssetReportRelayCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRelayCreateResponse> {
        let mut r = self.client.client.post("/asset_report/relay/create");
        r = r
            .json(
                json!(
                    { "asset_report_token" : self.asset_report_token,
                    "secondary_client_id" : self.secondary_client_id, "webhook" : self
                    .webhook }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRelayGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_relay_token: String,
}
impl<'a> AssetReportRelayGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportGetResponse> {
        let mut r = self.client.client.post("/asset_report/relay/get");
        r = r.json(json!({ "asset_relay_token" : self.asset_relay_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportRelayRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_relay_token: String,
}
impl<'a> AssetReportRelayRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportRelayRemoveResponse> {
        let mut r = self.client.client.post("/asset_report/relay/remove");
        r = r.json(json!({ "asset_relay_token" : self.asset_relay_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InvestmentsHoldingsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: serde_json::Value,
}
impl<'a> InvestmentsHoldingsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvestmentsHoldingsGetResponse> {
        let mut r = self.client.client.post("/investments/holdings/get");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "options" : self.options }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct InvestmentsTransactionsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
    pub options: serde_json::Value,
}
impl<'a> InvestmentsTransactionsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::InvestmentsTransactionsGetResponse> {
        let mut r = self.client.client.post("/investments/transactions/get");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "start_date" : self.start_date,
                    "end_date" : self.end_date, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub processor: String,
}
impl<'a> ProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorTokenCreateResponse> {
        let mut r = self.client.client.post("/processor/token/create");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "account_id" : self.account_id,
                    "processor" : self.processor }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::ProcessorStripeBankAccountTokenCreateResponse> {
        let mut r = self
            .client
            .client
            .post("/processor/stripe/bank_account_token/create");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "account_id" : self.account_id
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProcessorApexProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorApexProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProcessorTokenCreateResponse> {
        let mut r = self.client.client.post("/processor/apex/processor_token/create");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "account_id" : self.account_id
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub target_access_token: String,
    pub target_account_id: String,
    pub country_code: Option<String>,
    pub options: serde_json::Value,
}
impl<'a> DepositSwitchCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchCreateResponse> {
        let mut r = self.client.client.post("/deposit_switch/create");
        r = r
            .json(
                json!(
                    { "target_access_token" : self.target_access_token,
                    "target_account_id" : self.target_account_id, "country_code" : self
                    .country_code, "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ItemImportRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub products: Vec<Products>,
    pub user_auth: serde_json::Value,
    pub options: serde_json::Value,
}
impl<'a> ItemImportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ItemImportResponse> {
        let mut r = self.client.client.post("/item/import");
        r = r
            .json(
                json!(
                    { "products" : self.products, "user_auth" : self.user_auth, "options"
                    : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchTokenCreateResponse> {
        let mut r = self.client.client.post("/deposit_switch/token/create");
        r = r.json(json!({ "deposit_switch_id" : self.deposit_switch_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct LinkTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<CountryCode>,
    pub user: serde_json::Value,
    pub products: Vec<Products>,
    pub additional_consented_products: Vec<Products>,
    pub webhook: String,
    pub access_token: String,
    pub link_customization_name: String,
    pub redirect_uri: String,
    pub android_package_name: String,
    pub institution_data: serde_json::Value,
    pub account_filters: serde_json::Value,
    pub eu_config: serde_json::Value,
    pub institution_id: String,
    pub payment_initiation: serde_json::Value,
    pub deposit_switch: serde_json::Value,
    pub income_verification: serde_json::Value,
    pub auth: serde_json::Value,
    pub transfer: serde_json::Value,
    pub update: serde_json::Value,
    pub identity_verification: serde_json::Value,
    pub user_token: String,
}
impl<'a> LinkTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::LinkTokenCreateResponse> {
        let mut r = self.client.client.post("/link/token/create");
        r = r
            .json(
                json!(
                    { "client_name" : self.client_name, "language" : self.language,
                    "country_codes" : self.country_codes, "user" : self.user, "products"
                    : self.products, "additional_consented_products" : self
                    .additional_consented_products, "webhook" : self.webhook,
                    "access_token" : self.access_token, "link_customization_name" : self
                    .link_customization_name, "redirect_uri" : self.redirect_uri,
                    "android_package_name" : self.android_package_name,
                    "institution_data" : self.institution_data, "account_filters" : self
                    .account_filters, "eu_config" : self.eu_config, "institution_id" :
                    self.institution_id, "payment_initiation" : self.payment_initiation,
                    "deposit_switch" : self.deposit_switch, "income_verification" : self
                    .income_verification, "auth" : self.auth, "transfer" : self.transfer,
                    "update" : self.update, "identity_verification" : self
                    .identity_verification, "user_token" : self.user_token }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct LinkTokenGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub link_token: String,
}
impl<'a> LinkTokenGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::LinkTokenGetResponse> {
        let mut r = self.client.client.post("/link/token/get");
        r = r.json(json!({ "link_token" : self.link_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct AssetReportAuditCopyGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportGetResponse> {
        let mut r = self.client.client.post("/asset_report/audit_copy/get");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchGetResponse> {
        let mut r = self.client.client.post("/deposit_switch/get");
        r = r.json(json!({ "deposit_switch_id" : self.deposit_switch_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_id: String,
}
impl<'a> TransferGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferGetResponse> {
        let mut r = self.client.client.post("/transfer/get");
        r = r.json(json!({ "transfer_id" : self.transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferGetResponse> {
        let mut r = self.client.client.post("/bank_transfer/get");
        r = r.json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferAuthorizationCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub device: serde_json::Value,
    pub origination_account_id: String,
    pub iso_currency_code: String,
    pub user_present: Option<bool>,
    pub payment_profile_id: String,
}
impl<'a> TransferAuthorizationCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::TransferAuthorizationCreateResponse> {
        let mut r = self.client.client.post("/transfer/authorization/create");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "account_id" : self.account_id,
                    "type" : self.type_, "network" : self.network, "amount" : self
                    .amount, "ach_class" : self.ach_class, "user" : self.user, "device" :
                    self.device, "origination_account_id" : self.origination_account_id,
                    "iso_currency_code" : self.iso_currency_code, "user_present" : self
                    .user_present, "payment_profile_id" : self.payment_profile_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: String,
    pub access_token: String,
    pub account_id: String,
    pub authorization_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub origination_account_id: Option<String>,
    pub iso_currency_code: String,
    pub payment_profile_id: String,
}
impl<'a> TransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferCreateResponse> {
        let mut r = self.client.client.post("/transfer/create");
        r = r
            .json(
                json!(
                    { "idempotency_key" : self.idempotency_key, "access_token" : self
                    .access_token, "account_id" : self.account_id, "authorization_id" :
                    self.authorization_id, "type" : self.type_, "network" : self.network,
                    "amount" : self.amount, "description" : self.description, "ach_class"
                    : self.ach_class, "user" : self.user, "metadata" : self.metadata,
                    "origination_account_id" : self.origination_account_id,
                    "iso_currency_code" : self.iso_currency_code, "payment_profile_id" :
                    self.payment_profile_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: String,
    pub access_token: String,
    pub account_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub iso_currency_code: String,
    pub description: String,
    pub ach_class: String,
    pub user: serde_json::Value,
    pub custom_tag: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferCreateResponse> {
        let mut r = self.client.client.post("/bank_transfer/create");
        r = r
            .json(
                json!(
                    { "idempotency_key" : self.idempotency_key, "access_token" : self
                    .access_token, "account_id" : self.account_id, "type" : self.type_,
                    "network" : self.network, "amount" : self.amount, "iso_currency_code"
                    : self.iso_currency_code, "description" : self.description,
                    "ach_class" : self.ach_class, "user" : self.user, "custom_tag" : self
                    .custom_tag, "metadata" : self.metadata, "origination_account_id" :
                    self.origination_account_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: i64,
    pub offset: i64,
    pub origination_account_id: Option<String>,
}
impl<'a> TransferListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferListResponse> {
        let mut r = self.client.client.post("/transfer/list");
        r = r
            .json(
                json!(
                    { "start_date" : self.start_date, "end_date" : self.end_date, "count"
                    : self.count, "offset" : self.offset, "origination_account_id" : self
                    .origination_account_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: i64,
    pub offset: i64,
    pub origination_account_id: Option<String>,
    pub direction: Option<String>,
}
impl<'a> BankTransferListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferListResponse> {
        let mut r = self.client.client.post("/bank_transfer/list");
        r = r
            .json(
                json!(
                    { "start_date" : self.start_date, "end_date" : self.end_date, "count"
                    : self.count, "offset" : self.offset, "origination_account_id" : self
                    .origination_account_id, "direction" : self.direction }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferCancelRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_id: String,
}
impl<'a> TransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferCancelResponse> {
        let mut r = self.client.client.post("/transfer/cancel");
        r = r.json(json!({ "transfer_id" : self.transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferCancelRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferCancelResponse> {
        let mut r = self.client.client.post("/bank_transfer/cancel");
        r = r.json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferEventListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub transfer_type: Option<String>,
    pub event_types: Vec<TransferEventType>,
    pub sweep_id: String,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
}
impl<'a> TransferEventListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferEventListResponse> {
        let mut r = self.client.client.post("/transfer/event/list");
        r = r
            .json(
                json!(
                    { "start_date" : self.start_date, "end_date" : self.end_date,
                    "transfer_id" : self.transfer_id, "account_id" : self.account_id,
                    "transfer_type" : self.transfer_type, "event_types" : self
                    .event_types, "sweep_id" : self.sweep_id, "count" : self.count,
                    "offset" : self.offset, "origination_account_id" : self
                    .origination_account_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferEventListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub bank_transfer_type: Option<String>,
    pub event_types: Vec<BankTransferEventType>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub direction: Option<String>,
}
impl<'a> BankTransferEventListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferEventListResponse> {
        let mut r = self.client.client.post("/bank_transfer/event/list");
        r = r
            .json(
                json!(
                    { "start_date" : self.start_date, "end_date" : self.end_date,
                    "bank_transfer_id" : self.bank_transfer_id, "account_id" : self
                    .account_id, "bank_transfer_type" : self.bank_transfer_type,
                    "event_types" : self.event_types, "count" : self.count, "offset" :
                    self.offset, "origination_account_id" : self.origination_account_id,
                    "direction" : self.direction }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferEventSyncRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> TransferEventSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferEventSyncResponse> {
        let mut r = self.client.client.post("/transfer/event/sync");
        r = r.json(json!({ "after_id" : self.after_id, "count" : self.count }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferEventSyncRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> BankTransferEventSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferEventSyncResponse> {
        let mut r = self.client.client.post("/bank_transfer/event/sync");
        r = r.json(json!({ "after_id" : self.after_id, "count" : self.count }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferSweepGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub sweep_id: String,
}
impl<'a> TransferSweepGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferSweepGetResponse> {
        let mut r = self.client.client.post("/transfer/sweep/get");
        r = r.json(json!({ "sweep_id" : self.sweep_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferSweepGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub sweep_id: String,
}
impl<'a> BankTransferSweepGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferSweepGetResponse> {
        let mut r = self.client.client.post("/bank_transfer/sweep/get");
        r = r.json(json!({ "sweep_id" : self.sweep_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferSweepListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: i64,
}
impl<'a> TransferSweepListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferSweepListResponse> {
        let mut r = self.client.client.post("/transfer/sweep/list");
        r = r
            .json(
                json!(
                    { "start_date" : self.start_date, "end_date" : self.end_date, "count"
                    : self.count, "offset" : self.offset }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferSweepListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub origination_account_id: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub count: Option<i64>,
}
impl<'a> BankTransferSweepListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferSweepListResponse> {
        let mut r = self.client.client.post("/bank_transfer/sweep/list");
        r = r
            .json(
                json!(
                    { "origination_account_id" : self.origination_account_id,
                    "start_time" : self.start_time, "end_time" : self.end_time, "count" :
                    self.count }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferBalanceGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BankTransferBalanceGetResponse> {
        let mut r = self.client.client.post("/bank_transfer/balance/get");
        r = r.json(json!({ "origination_account_id" : self.origination_account_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct BankTransferMigrateAccountRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_number: String,
    pub routing_number: String,
    pub wire_routing_number: String,
    pub account_type: String,
}
impl<'a> BankTransferMigrateAccountRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::BankTransferMigrateAccountResponse> {
        let mut r = self.client.client.post("/bank_transfer/migrate_account");
        r = r
            .json(
                json!(
                    { "account_number" : self.account_number, "routing_number" : self
                    .routing_number, "wire_routing_number" : self.wire_routing_number,
                    "account_type" : self.account_type }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferMigrateAccountRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_number: String,
    pub routing_number: String,
    pub wire_routing_number: String,
    pub account_type: String,
}
impl<'a> TransferMigrateAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferMigrateAccountResponse> {
        let mut r = self.client.client.post("/transfer/migrate_account");
        r = r
            .json(
                json!(
                    { "account_number" : self.account_number, "routing_number" : self
                    .routing_number, "wire_routing_number" : self.wire_routing_number,
                    "account_type" : self.account_type }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferIntentCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_id: Option<String>,
    pub mode: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub origination_account_id: Option<String>,
    pub user: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub iso_currency_code: String,
    pub require_guarantee: Option<bool>,
}
impl<'a> TransferIntentCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferIntentCreateResponse> {
        let mut r = self.client.client.post("/transfer/intent/create");
        r = r
            .json(
                json!(
                    { "account_id" : self.account_id, "mode" : self.mode, "amount" : self
                    .amount, "description" : self.description, "ach_class" : self
                    .ach_class, "origination_account_id" : self.origination_account_id,
                    "user" : self.user, "metadata" : self.metadata, "iso_currency_code" :
                    self.iso_currency_code, "require_guarantee" : self.require_guarantee
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferIntentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_intent_id: String,
}
impl<'a> TransferIntentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferIntentGetResponse> {
        let mut r = self.client.client.post("/transfer/intent/get");
        r = r.json(json!({ "transfer_intent_id" : self.transfer_intent_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferRepaymentListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: i64,
}
impl<'a> TransferRepaymentListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransferRepaymentListResponse> {
        let mut r = self.client.client.post("/transfer/repayment/list");
        r = r
            .json(
                json!(
                    { "start_date" : self.start_date, "end_date" : self.end_date, "count"
                    : self.count, "offset" : self.offset }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransferRepaymentReturnListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub repayment_id: String,
    pub count: Option<i64>,
    pub offset: i64,
}
impl<'a> TransferRepaymentReturnListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::TransferRepaymentReturnListResponse> {
        let mut r = self.client.client.post("/transfer/repayment/return/list");
        r = r
            .json(
                json!(
                    { "repayment_id" : self.repayment_id, "count" : self.count, "offset"
                    : self.offset }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxBankTransferSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<serde_json::Value>,
}
impl<'a> SandboxBankTransferSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxBankTransferSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/bank_transfer/simulate");
        r = r
            .json(
                json!(
                    { "bank_transfer_id" : self.bank_transfer_id, "event_type" : self
                    .event_type, "failure_reason" : self.failure_reason }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferSweepSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> SandboxTransferSweepSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxTransferSweepSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/sweep/simulate");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<serde_json::Value>,
}
impl<'a> SandboxTransferSimulateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxTransferSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/simulate");
        r = r
            .json(
                json!(
                    { "transfer_id" : self.transfer_id, "event_type" : self.event_type,
                    "failure_reason" : self.failure_reason }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferRepaymentSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> SandboxTransferRepaymentSimulateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxTransferRepaymentSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/repayment/simulate");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxTransferFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub webhook: String,
}
impl<'a> SandboxTransferFireWebhookRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxTransferFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/fire_webhook");
        r = r.json(json!({ "webhook" : self.webhook }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct EmployersSearchRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub query: String,
    pub products: Vec<String>,
}
impl<'a> EmployersSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EmployersSearchResponse> {
        let mut r = self.client.client.post("/employers/search");
        r = r.json(json!({ "query" : self.query, "products" : self.products }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub webhook: String,
    pub precheck_id: String,
    pub options: serde_json::Value,
}
impl<'a> IncomeVerificationCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IncomeVerificationCreateResponse> {
        let mut r = self.client.client.post("/income/verification/create");
        r = r
            .json(
                json!(
                    { "webhook" : self.webhook, "precheck_id" : self.precheck_id,
                    "options" : self.options }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationPaystubsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationPaystubsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationPaystubsGetResponse> {
        let mut r = self.client.client.post("/income/verification/paystubs/get");
        r = r
            .json(
                json!(
                    { "income_verification_id" : self.income_verification_id,
                    "access_token" : self.access_token }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::IncomeVerificationRefreshResponse> {
        let mut r = self.client.client.post("/income/verification/refresh");
        r = r
            .json(
                json!(
                    { "income_verification_id" : self.income_verification_id,
                    "access_token" : self.access_token }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationTaxformsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationTaxformsGetRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationTaxformsGetResponse> {
        let mut r = self.client.client.post("/income/verification/taxforms/get");
        r = r
            .json(
                json!(
                    { "income_verification_id" : self.income_verification_id,
                    "access_token" : self.access_token }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct IncomeVerificationPrecheckRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user: Option<serde_json::Value>,
    pub employer: Option<serde_json::Value>,
    pub transactions_access_token: serde_json::Value,
    pub transactions_access_tokens: Vec<AccessToken>,
    pub us_military_info: Option<serde_json::Value>,
}
impl<'a> IncomeVerificationPrecheckRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::IncomeVerificationPrecheckResponse> {
        let mut r = self.client.client.post("/income/verification/precheck");
        r = r
            .json(
                json!(
                    { "user" : self.user, "employer" : self.employer,
                    "transactions_access_token" : self.transactions_access_token,
                    "transactions_access_tokens" : self.transactions_access_tokens,
                    "us_military_info" : self.us_military_info }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct EmploymentVerificationGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> EmploymentVerificationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EmploymentVerificationGetResponse> {
        let mut r = self.client.client.post("/employment/verification/get");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct DepositSwitchAltCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub target_account: serde_json::Value,
    pub target_user: serde_json::Value,
    pub options: serde_json::Value,
    pub country_code: Option<String>,
}
impl<'a> DepositSwitchAltCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::DepositSwitchAltCreateResponse> {
        let mut r = self.client.client.post("/deposit_switch/alt/create");
        r = r
            .json(
                json!(
                    { "target_account" : self.target_account, "target_user" : self
                    .target_user, "options" : self.options, "country_code" : self
                    .country_code }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditAuditCopyTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub report_tokens: Vec<ReportToken>,
    pub auditor_id: String,
}
impl<'a> CreditAuditCopyTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::CreditAuditCopyTokenCreateResponse> {
        let mut r = self.client.client.post("/credit/audit_copy_token/create");
        r = r
            .json(
                json!(
                    { "report_tokens" : self.report_tokens, "auditor_id" : self
                    .auditor_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditReportAuditCopyRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> CreditReportAuditCopyRemoveRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::CreditAuditCopyTokenRemoveResponse> {
        let mut r = self.client.client.post("/credit/audit_copy_token/remove");
        r = r.json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditBankIncomeGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
    pub options: serde_json::Value,
}
impl<'a> CreditBankIncomeGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditBankIncomeGetResponse> {
        let mut r = self.client.client.post("/credit/bank_income/get");
        r = r.json(json!({ "user_token" : self.user_token, "options" : self.options }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditBankIncomeRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
    pub options: serde_json::Value,
}
impl<'a> CreditBankIncomeRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditBankIncomeRefreshResponse> {
        let mut r = self.client.client.post("/credit/bank_income/refresh");
        r = r.json(json!({ "user_token" : self.user_token, "options" : self.options }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditPayrollIncomeGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditPayrollIncomeGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditPayrollIncomeGetResponse> {
        let mut r = self.client.client.post("/credit/payroll_income/get");
        r = r.json(json!({ "user_token" : self.user_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditPayrollIncomePrecheckRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
    pub access_tokens: Vec<AccessToken>,
    pub employer: Option<serde_json::Value>,
    pub us_military_info: Option<serde_json::Value>,
}
impl<'a> CreditPayrollIncomePrecheckRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::CreditPayrollIncomePrecheckResponse> {
        let mut r = self.client.client.post("/credit/payroll_income/precheck");
        r = r
            .json(
                json!(
                    { "user_token" : self.user_token, "access_tokens" : self
                    .access_tokens, "employer" : self.employer, "us_military_info" : self
                    .us_military_info }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditEmploymentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditEmploymentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditEmploymentGetResponse> {
        let mut r = self.client.client.post("/credit/employment/get");
        r = r.json(json!({ "user_token" : self.user_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditPayrollIncomeRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditPayrollIncomeRefreshRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::CreditPayrollIncomeRefreshResponse> {
        let mut r = self.client.client.post("/credit/payroll_income/refresh");
        r = r.json(json!({ "user_token" : self.user_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditRelayCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub report_tokens: Vec<ReportToken>,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl<'a> CreditRelayCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditRelayCreateResponse> {
        let mut r = self.client.client.post("/credit/relay/create");
        r = r
            .json(
                json!(
                    { "report_tokens" : self.report_tokens, "secondary_client_id" : self
                    .secondary_client_id, "webhook" : self.webhook }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditRelayGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub relay_token: String,
    pub report_type: String,
}
impl<'a> CreditRelayGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::AssetReportGetResponse> {
        let mut r = self.client.client.post("/credit/relay/get");
        r = r
            .json(
                json!(
                    { "relay_token" : self.relay_token, "report_type" : self.report_type
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditRelayRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub relay_token: String,
    pub report_type: String,
    pub webhook: Option<String>,
}
impl<'a> CreditRelayRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditRelayRefreshResponse> {
        let mut r = self.client.client.post("/credit/relay/refresh");
        r = r
            .json(
                json!(
                    { "relay_token" : self.relay_token, "report_type" : self.report_type,
                    "webhook" : self.webhook }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreditRelayRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub relay_token: String,
}
impl<'a> CreditRelayRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CreditRelayRemoveResponse> {
        let mut r = self.client.client.post("/credit/relay/remove");
        r = r.json(json!({ "relay_token" : self.relay_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxBankTransferFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub webhook: String,
}
impl<'a> SandboxBankTransferFireWebhookRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxBankTransferFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/bank_transfer/fire_webhook");
        r = r.json(json!({ "webhook" : self.webhook }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxIncomeFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub item_id: String,
    pub user_id: String,
    pub webhook: String,
    pub verification_status: String,
}
impl<'a> SandboxIncomeFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SandboxIncomeFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/income/fire_webhook");
        r = r
            .json(
                json!(
                    { "item_id" : self.item_id, "user_id" : self.user_id, "webhook" :
                    self.webhook, "verification_status" : self.verification_status }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SandboxOauthSelectAccountsRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub oauth_state_id: String,
    pub accounts: Vec<String>,
}
impl<'a> SandboxOauthSelectAccountsRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<model::SandboxOauthSelectAccountsResponse> {
        let mut r = self.client.client.post("/sandbox/oauth/select_accounts");
        r = r
            .json(
                json!(
                    { "oauth_state_id" : self.oauth_state_id, "accounts" : self.accounts
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalEvaluateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub client_transaction_id: String,
    pub amount: f64,
    pub user_present: Option<bool>,
    pub client_user_id: String,
    pub user: serde_json::Value,
    pub device: serde_json::Value,
}
impl<'a> SignalEvaluateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalEvaluateResponse> {
        let mut r = self.client.client.post("/signal/evaluate");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "account_id" : self.account_id,
                    "client_transaction_id" : self.client_transaction_id, "amount" : self
                    .amount, "user_present" : self.user_present, "client_user_id" : self
                    .client_user_id, "user" : self.user, "device" : self.device }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalDecisionReportRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_transaction_id: String,
    pub initiated: bool,
    pub days_funds_on_hold: Option<i64>,
}
impl<'a> SignalDecisionReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalDecisionReportResponse> {
        let mut r = self.client.client.post("/signal/decision/report");
        r = r
            .json(
                json!(
                    { "client_transaction_id" : self.client_transaction_id, "initiated" :
                    self.initiated, "days_funds_on_hold" : self.days_funds_on_hold }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalReturnReportRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_transaction_id: String,
    pub return_code: String,
}
impl<'a> SignalReturnReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalReturnReportResponse> {
        let mut r = self.client.client.post("/signal/return/report");
        r = r
            .json(
                json!(
                    { "client_transaction_id" : self.client_transaction_id, "return_code"
                    : self.return_code }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct SignalPrepareRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> SignalPrepareRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::SignalPrepareResponse> {
        let mut r = self.client.client.post("/signal/prepare");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub iso_currency_code: String,
}
impl<'a> WalletCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletCreateResponse> {
        let mut r = self.client.client.post("/wallet/create");
        r = r.json(json!({ "iso_currency_code" : self.iso_currency_code }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub wallet_id: String,
}
impl<'a> WalletGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletGetResponse> {
        let mut r = self.client.client.post("/wallet/get");
        r = r.json(json!({ "wallet_id" : self.wallet_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub iso_currency_code: String,
    pub cursor: String,
    pub count: i64,
}
impl<'a> WalletListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletListResponse> {
        let mut r = self.client.client.post("/wallet/list");
        r = r
            .json(
                json!(
                    { "iso_currency_code" : self.iso_currency_code, "cursor" : self
                    .cursor, "count" : self.count }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletTransactionExecuteRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: String,
    pub wallet_id: String,
    pub counterparty: serde_json::Value,
    pub amount: serde_json::Value,
    pub reference: String,
}
impl<'a> WalletTransactionExecuteRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletTransactionExecuteResponse> {
        let mut r = self.client.client.post("/wallet/transaction/execute");
        r = r
            .json(
                json!(
                    { "idempotency_key" : self.idempotency_key, "wallet_id" : self
                    .wallet_id, "counterparty" : self.counterparty, "amount" : self
                    .amount, "reference" : self.reference }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletTransactionGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transaction_id: String,
}
impl<'a> WalletTransactionGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletTransactionGetResponse> {
        let mut r = self.client.client.post("/wallet/transaction/get");
        r = r.json(json!({ "transaction_id" : self.transaction_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletTransactionsListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub wallet_id: String,
    pub cursor: String,
    pub count: i64,
}
impl<'a> WalletTransactionsListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::WalletTransactionsListResponse> {
        let mut r = self.client.client.post("/wallet/transactions/list");
        r = r
            .json(
                json!(
                    { "wallet_id" : self.wallet_id, "cursor" : self.cursor, "count" :
                    self.count }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsEnhanceRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_type: String,
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl<'a> TransactionsEnhanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsEnhanceGetResponse> {
        let mut r = self.client.client.post("/beta/transactions/v1/enhance");
        r = r
            .json(
                json!(
                    { "account_type" : self.account_type, "transactions" : self
                    .transactions }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRulesCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub personal_finance_category: String,
    pub rule_details: serde_json::Value,
}
impl<'a> TransactionsRulesCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRulesCreateResponse> {
        let mut r = self.client.client.post("/beta/transactions/rules/v1/create");
        r = r
            .json(
                json!(
                    { "access_token" : self.access_token, "personal_finance_category" :
                    self.personal_finance_category, "rule_details" : self.rule_details }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRulesListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> TransactionsRulesListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRulesListResponse> {
        let mut r = self.client.client.post("/beta/transactions/rules/v1/list");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TransactionsRulesRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub rule_id: String,
}
impl<'a> TransactionsRulesRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TransactionsRulesRemoveResponse> {
        let mut r = self.client.client.post("/beta/transactions/rules/v1/remove");
        r = r
            .json(
                json!({ "access_token" : self.access_token, "rule_id" : self.rule_id }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentProfileCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> PaymentProfileCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::PaymentProfileCreateResponse> {
        let mut r = self.client.client.post("/payment_profile/create");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentProfileGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_profile_id: String,
}
impl<'a> PaymentProfileGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::PaymentProfileGetResponse> {
        let mut r = self.client.client.post("/payment_profile/get");
        r = r.json(json!({ "payment_profile_id" : self.payment_profile_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PaymentProfileRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_profile_id: String,
}
impl<'a> PaymentProfileRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::PaymentProfileRemoveResponse> {
        let mut r = self.client.client.post("/payment_profile/remove");
        r = r.json(json!({ "payment_profile_id" : self.payment_profile_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct PartnerCustomersCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> PartnerCustomersCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::PartnerCustomersCreateResponse> {
        let mut r = self.client.client.post("/beta/partner/v1/customers/create");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
