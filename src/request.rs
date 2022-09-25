use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemApplicationListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: Option<String>,
}
impl<'a> ItemApplicationListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemApplicationListResponse> {
        let mut r = self.client.client.post("/item/application/list");
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemApplicationScopesUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub application_id: String,
    pub scopes: Scopes,
    pub state: Option<String>,
    pub context: String,
}
impl<'a> ItemApplicationScopesUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemApplicationScopesUpdateResponse> {
        let mut r = self.client.client.post("/item/application/scopes/update");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "application_id" : self.application_id }));
        r = r.push_json(json!({ "scopes" : self.scopes }));
        if let Some(ref unwrapped) = self.state {
            r = r.push_json(json!({ "state" : unwrapped }));
        }
        r = r.push_json(json!({ "context" : self.context }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
}
pub struct ItemApplicationScopesUpdateRequired<'a> {
    pub access_token: &'a str,
    pub application_id: &'a str,
    pub scopes: Scopes,
    pub context: &'a str,
}
impl<'a> ItemApplicationScopesUpdateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ApplicationGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub application_id: String,
}
impl<'a> ApplicationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ApplicationGetResponse> {
        let mut r = self.client.client.post("/application/get");
        r = r.push_json(json!({ "application_id" : self.application_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemGetResponse> {
        let mut r = self.client.client.post("/item/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AuthGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<AuthGetRequestOptions>,
}
impl<'a> AuthGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AuthGetResponse> {
        let mut r = self.client.client.post("/auth/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: AuthGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub options: Option<TransactionsGetRequestOptions>,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
}
impl<'a> TransactionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsGetResponse> {
        let mut r = self.client.client.post("/transactions/get");
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "start_date" : self.start_date }));
        r = r.push_json(json!({ "end_date" : self.end_date }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: TransactionsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> TransactionsRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsRefreshResponse> {
        let mut r = self.client.client.post("/transactions/refresh");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsRecurringGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<TransactionsRecurringGetRequestOptions>,
    pub account_ids: Vec<String>,
}
impl<'a> TransactionsRecurringGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsRecurringGetResponse> {
        let mut r = self.client.client.post("/transactions/recurring/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = r.push_json(json!({ "account_ids" : self.account_ids }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: TransactionsRecurringGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsSyncRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub cursor: Option<String>,
    pub count: Option<i64>,
    pub options: Option<TransactionsSyncRequestOptions>,
}
impl<'a> TransactionsSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsSyncResponse> {
        let mut r = self.client.client.post("/transactions/sync");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn options(mut self, options: TransactionsSyncRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct InstitutionsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub count: i64,
    pub offset: i64,
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsGetRequestOptions>,
}
impl<'a> InstitutionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InstitutionsGetResponse> {
        let mut r = self.client.client.post("/institutions/get");
        r = r.push_json(json!({ "count" : self.count }));
        r = r.push_json(json!({ "offset" : self.offset }));
        r = r.push_json(json!({ "country_codes" : self.country_codes }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: InstitutionsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct InstitutionsSearchRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub query: String,
    pub products: Option<Vec<String>>,
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsSearchRequestOptions>,
}
impl<'a> InstitutionsSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InstitutionsSearchResponse> {
        let mut r = self.client.client.post("/institutions/search");
        r = r.push_json(json!({ "query" : self.query }));
        if let Some(ref unwrapped) = self.products {
            r = r.push_json(json!({ "products" : unwrapped }));
        }
        r = r.push_json(json!({ "country_codes" : self.country_codes }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn products(
        mut self,
        products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .products = Some(
            products.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn options(mut self, options: InstitutionsSearchRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct InstitutionsGetByIdRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub institution_id: String,
    pub country_codes: Vec<String>,
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
impl<'a> InstitutionsGetByIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InstitutionsGetByIdResponse> {
        let mut r = self.client.client.post("/institutions/get_by_id");
        r = r.push_json(json!({ "institution_id" : self.institution_id }));
        r = r.push_json(json!({ "country_codes" : self.country_codes }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: InstitutionsGetByIdRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemRemoveResponse> {
        let mut r = self.client.client.post("/item/remove");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AccountsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<AccountsGetRequestOptions>,
}
impl<'a> AccountsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AccountsGetResponse> {
        let mut r = self.client.client.post("/accounts/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: AccountsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CategoriesGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> CategoriesGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CategoriesGetResponse> {
        let mut r = self.client.client.post("/categories/get");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub institution_id: String,
    pub options: Option<SandboxProcessorTokenCreateRequestOptions>,
}
impl<'a> SandboxProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxProcessorTokenCreateResponse> {
        let mut r = self.client.client.post("/sandbox/processor_token/create");
        r = r.push_json(json!({ "institution_id" : self.institution_id }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(
        mut self,
        options: SandboxProcessorTokenCreateRequestOptions,
    ) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxPublicTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub institution_id: String,
    pub initial_products: Vec<String>,
    pub options: Option<SandboxPublicTokenCreateRequestOptions>,
    pub user_token: Option<String>,
}
impl<'a> SandboxPublicTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxPublicTokenCreateResponse> {
        let mut r = self.client.client.post("/sandbox/public_token/create");
        r = r.push_json(json!({ "institution_id" : self.institution_id }));
        r = r.push_json(json!({ "initial_products" : self.initial_products }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_token {
            r = r.push_json(json!({ "user_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: SandboxPublicTokenCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxItemFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub webhook_type: Option<String>,
    pub webhook_code: String,
}
impl<'a> SandboxItemFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxItemFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/item/fire_webhook");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.webhook_type {
            r = r.push_json(json!({ "webhook_type" : unwrapped }));
        }
        r = r.push_json(json!({ "webhook_code" : self.webhook_code }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn webhook_type(mut self, webhook_type: &str) -> Self {
        self.webhook_type = Some(webhook_type.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AccountsBalanceGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<AccountsBalanceGetRequestOptions>,
}
impl<'a> AccountsBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AccountsGetResponse> {
        let mut r = self.client.client.post("/accounts/balance/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: AccountsBalanceGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IdentityGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<IdentityGetRequestOptions>,
}
impl<'a> IdentityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityGetResponse> {
        let mut r = self.client.client.post("/identity/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: IdentityGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IdentityMatchRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub user: Option<IdentityMatchUser>,
    pub options: Option<IdentityMatchRequestOptions>,
}
impl<'a> IdentityMatchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityMatchResponse> {
        let mut r = self.client.client.post("/identity/match");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user(mut self, user: IdentityMatchUser) -> Self {
        self.user = Some(user);
        self
    }
    pub fn options(mut self, options: IdentityMatchRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DashobardUserGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub dashboard_user_id: String,
}
impl<'a> DashobardUserGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DashboardUserResponse> {
        let mut r = self.client.client.post("/dashboard_user/get");
        r = r.push_json(json!({ "dashboard_user_id" : self.dashboard_user_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DashboardUserListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> DashboardUserListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaginatedDashboardUserListResponse> {
        let mut r = self.client.client.post("/dashboard_user/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IdentityVerificationCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub is_shareable: bool,
    pub template_id: String,
    pub gave_consent: bool,
    pub user: IdentityVerificationRequestUser,
    pub is_idempotent: Option<bool>,
}
impl<'a> IdentityVerificationCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationResponse> {
        let mut r = self.client.client.post("/identity_verification/create");
        r = r.push_json(json!({ "is_shareable" : self.is_shareable }));
        r = r.push_json(json!({ "template_id" : self.template_id }));
        r = r.push_json(json!({ "gave_consent" : self.gave_consent }));
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.is_idempotent {
            r = r.push_json(json!({ "is_idempotent" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn is_idempotent(mut self, is_idempotent: bool) -> Self {
        self.is_idempotent = Some(is_idempotent);
        self
    }
}
pub struct IdentityVerificationCreateRequired<'a> {
    pub is_shareable: bool,
    pub template_id: &'a str,
    pub gave_consent: bool,
    pub user: IdentityVerificationRequestUser,
}
impl<'a> IdentityVerificationCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IdentityVerificationGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub identity_verification_id: String,
}
impl<'a> IdentityVerificationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationResponse> {
        let mut r = self.client.client.post("/identity_verification/get");
        r = r
            .push_json(
                json!({ "identity_verification_id" : self.identity_verification_id }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IdentityVerificationListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub template_id: String,
    pub client_user_id: String,
    pub cursor: Option<String>,
}
impl<'a> IdentityVerificationListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedIdentityVerificationListResponse> {
        let mut r = self.client.client.post("/identity_verification/list");
        r = r.push_json(json!({ "template_id" : self.template_id }));
        r = r.push_json(json!({ "client_user_id" : self.client_user_id }));
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IdentityVerificationRetryRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_user_id: String,
    pub template_id: String,
    pub strategy: String,
    pub steps: Option<IdentityVerificationRetryRequestStepsObject>,
}
impl<'a> IdentityVerificationRetryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IdentityVerificationResponse> {
        let mut r = self.client.client.post("/identity_verification/retry");
        r = r.push_json(json!({ "client_user_id" : self.client_user_id }));
        r = r.push_json(json!({ "template_id" : self.template_id }));
        r = r.push_json(json!({ "strategy" : self.strategy }));
        if let Some(ref unwrapped) = self.steps {
            r = r.push_json(json!({ "steps" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn steps(mut self, steps: IdentityVerificationRetryRequestStepsObject) -> Self {
        self.steps = Some(steps);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub search_terms: EntityWatchlistSearchTerms,
    pub client_user_id: Option<String>,
}
impl<'a> WatchlistScreeningEntityCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntityWatchlistScreeningResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/create");
        r = r.push_json(json!({ "search_terms" : self.search_terms }));
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningEntityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntityWatchlistScreeningResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/get");
        r = r
            .push_json(
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
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityHistoryListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityHistoryListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedEntityWatchlistScreeningListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/history/list");
        r = r
            .push_json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id }
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityHitsListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityHitsListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedEntityWatchlistScreeningHitListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/hit/list");
        r = r
            .push_json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id }
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_program_id: String,
    pub client_user_id: Option<String>,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedEntityWatchlistScreeningListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/list");
        r = r
            .push_json(
                json!(
                    { "entity_watchlist_program_id" : self.entity_watchlist_program_id }
                ),
            );
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_json(json!({ "status" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.assignee {
            r = r.push_json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityProgramGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_program_id: String,
}
impl<'a> WatchlistScreeningEntityProgramGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntityWatchlistProgramResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/program/get");
        r = r
            .push_json(
                json!(
                    { "entity_watchlist_program_id" : self.entity_watchlist_program_id }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityProgramListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityProgramListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedEntityWatchlistProgramListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/program/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityReviewCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub comment: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningEntityReviewCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntityWatchlistScreeningReviewResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/review/create");
        r = r.push_json(json!({ "confirmed_hits" : self.confirmed_hits }));
        r = r.push_json(json!({ "dismissed_hits" : self.dismissed_hits }));
        if let Some(ref unwrapped) = self.comment {
            r = r.push_json(json!({ "comment" : unwrapped }));
        }
        r = r
            .push_json(
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
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityReviewListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityReviewListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedEntityWatchlistScreeningReviewListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/review/list");
        r = r
            .push_json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id }
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningEntityUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub search_terms: Option<UpdateEntityScreeningRequestSearchTerms>,
    pub assignee: Option<String>,
    pub status: Option<String>,
    pub client_user_id: Option<String>,
    pub reset_fields: Option<Vec<String>>,
}
impl<'a> WatchlistScreeningEntityUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntityWatchlistScreeningResponse> {
        let mut r = self.client.client.post("/watchlist_screening/entity/update");
        r = r
            .push_json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id }
                ),
            );
        if let Some(ref unwrapped) = self.search_terms {
            r = r.push_json(json!({ "search_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.assignee {
            r = r.push_json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_json(json!({ "status" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reset_fields {
            r = r.push_json(json!({ "reset_fields" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn search_terms(
        mut self,
        search_terms: UpdateEntityScreeningRequestSearchTerms,
    ) -> Self {
        self.search_terms = Some(search_terms);
        self
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn reset_fields(
        mut self,
        reset_fields: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .reset_fields = Some(
            reset_fields.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub search_terms: WatchlistScreeningRequestSearchTerms,
    pub client_user_id: Option<String>,
}
impl<'a> WatchlistScreeningIndividualCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WatchlistScreeningIndividualResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/create");
        r = r.push_json(json!({ "search_terms" : self.search_terms }));
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WatchlistScreeningIndividualResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/get");
        r = r
            .push_json(
                json!({ "watchlist_screening_id" : self.watchlist_screening_id }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualHistoryListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualHistoryListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedIndividualWatchlistScreeningListResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/history/list");
        r = r
            .push_json(
                json!({ "watchlist_screening_id" : self.watchlist_screening_id }),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualHitListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualHitListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedIndividualWatchlistScreeningHitListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/hit/list");
        r = r
            .push_json(
                json!({ "watchlist_screening_id" : self.watchlist_screening_id }),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_program_id: String,
    pub client_user_id: Option<String>,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedIndividualWatchlistScreeningListResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/list");
        r = r.push_json(json!({ "watchlist_program_id" : self.watchlist_program_id }));
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_json(json!({ "status" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.assignee {
            r = r.push_json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualProgramGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_program_id: String,
}
impl<'a> WatchlistScreeningIndividualProgramGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IndividualWatchlistProgramResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/program/get");
        r = r.push_json(json!({ "watchlist_program_id" : self.watchlist_program_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualProgramListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualProgramListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedIndividualWatchlistProgramListResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/program/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualReviewCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub comment: Option<String>,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualReviewCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WatchlistScreeningReviewResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/review/create");
        r = r.push_json(json!({ "confirmed_hits" : self.confirmed_hits }));
        r = r.push_json(json!({ "dismissed_hits" : self.dismissed_hits }));
        if let Some(ref unwrapped) = self.comment {
            r = r.push_json(json!({ "comment" : unwrapped }));
        }
        r = r
            .push_json(
                json!({ "watchlist_screening_id" : self.watchlist_screening_id }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualReviewsListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningIndividualReviewsListRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaginatedIndividualWatchlistScreeningReviewListResponse> {
        let mut r = self
            .client
            .client
            .post("/watchlist_screening/individual/review/list");
        r = r
            .push_json(
                json!({ "watchlist_screening_id" : self.watchlist_screening_id }),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WatchlistScreeningIndividualUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub watchlist_screening_id: String,
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    pub assignee: Option<String>,
    pub status: Option<String>,
    pub client_user_id: Option<String>,
    pub reset_fields: Option<Vec<String>>,
}
impl<'a> WatchlistScreeningIndividualUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WatchlistScreeningIndividualResponse> {
        let mut r = self.client.client.post("/watchlist_screening/individual/update");
        r = r
            .push_json(
                json!({ "watchlist_screening_id" : self.watchlist_screening_id }),
            );
        if let Some(ref unwrapped) = self.search_terms {
            r = r.push_json(json!({ "search_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.assignee {
            r = r.push_json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_json(json!({ "status" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reset_fields {
            r = r.push_json(json!({ "reset_fields" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn search_terms(
        mut self,
        search_terms: UpdateIndividualScreeningRequestSearchTerms,
    ) -> Self {
        self.search_terms = Some(search_terms);
        self
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn reset_fields(
        mut self,
        reset_fields: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .reset_fields = Some(
            reset_fields.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorAuthGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub processor_token: String,
}
impl<'a> ProcessorAuthGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProcessorAuthGetResponse> {
        let mut r = self.client.client.post("/processor/auth/get");
        r = r.push_json(json!({ "processor_token" : self.processor_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorBankTransferCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: String,
    pub processor_token: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub iso_currency_code: String,
    pub description: String,
    pub ach_class: Option<String>,
    pub user: BankTransferUser,
    pub custom_tag: Option<String>,
    pub metadata: Option<BankTransferMetadata>,
    pub origination_account_id: Option<String>,
}
impl<'a> ProcessorBankTransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProcessorBankTransferCreateResponse> {
        let mut r = self.client.client.post("/processor/bank_transfer/create");
        r = r.push_json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.push_json(json!({ "processor_token" : self.processor_token }));
        r = r.push_json(json!({ "type" : self.type_ }));
        r = r.push_json(json!({ "network" : self.network }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "iso_currency_code" : self.iso_currency_code }));
        r = r.push_json(json!({ "description" : self.description }));
        if let Some(ref unwrapped) = self.ach_class {
            r = r.push_json(json!({ "ach_class" : unwrapped }));
        }
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.custom_tag {
            r = r.push_json(json!({ "custom_tag" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn custom_tag(mut self, custom_tag: &str) -> Self {
        self.custom_tag = Some(custom_tag.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: BankTransferMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
pub struct ProcessorBankTransferCreateRequired<'a> {
    pub idempotency_key: &'a str,
    pub processor_token: &'a str,
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub iso_currency_code: &'a str,
    pub description: &'a str,
    pub user: BankTransferUser,
}
impl<'a> ProcessorBankTransferCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorIdentityGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub processor_token: String,
}
impl<'a> ProcessorIdentityGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProcessorIdentityGetResponse> {
        let mut r = self.client.client.post("/processor/identity/get");
        r = r.push_json(json!({ "processor_token" : self.processor_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorBalanceGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub processor_token: String,
    pub options: Option<ProcessorBalanceGetRequestOptions>,
}
impl<'a> ProcessorBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProcessorBalanceGetResponse> {
        let mut r = self.client.client.post("/processor/balance/get");
        r = r.push_json(json!({ "processor_token" : self.processor_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: ProcessorBalanceGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemWebhookUpdateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub webhook: Option<String>,
}
impl<'a> ItemWebhookUpdateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemWebhookUpdateResponse> {
        let mut r = self.client.client.post("/item/webhook/update");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemAccessTokenInvalidateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemAccessTokenInvalidateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemAccessTokenInvalidateResponse> {
        let mut r = self.client.client.post("/item/access_token/invalidate");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WebhookVerificationKeyGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub key_id: String,
}
impl<'a> WebhookVerificationKeyGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WebhookVerificationKeyGetResponse> {
        let mut r = self.client.client.post("/webhook_verification_key/get");
        r = r.push_json(json!({ "key_id" : self.key_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct LiabilitiesGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<LiabilitiesGetRequestOptions>,
}
impl<'a> LiabilitiesGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LiabilitiesGetResponse> {
        let mut r = self.client.client.post("/liabilities/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: LiabilitiesGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationRecipientCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub name: String,
    pub iban: Option<String>,
    pub bacs: Option<RecipientBacsNullable>,
    pub address: Option<PaymentInitiationAddress>,
}
impl<'a> PaymentInitiationRecipientCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationRecipientCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/recipient/create");
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.iban {
            r = r.push_json(json!({ "iban" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bacs {
            r = r.push_json(json!({ "bacs" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn iban(mut self, iban: &str) -> Self {
        self.iban = Some(iban.to_owned());
        self
    }
    pub fn bacs(mut self, bacs: RecipientBacsNullable) -> Self {
        self.bacs = Some(bacs);
        self
    }
    pub fn address(mut self, address: PaymentInitiationAddress) -> Self {
        self.address = Some(address);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationPaymentReverseRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_id: String,
    pub idempotency_key: String,
    pub reference: String,
}
impl<'a> PaymentInitiationPaymentReverseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationPaymentReverseResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/reverse");
        r = r.push_json(json!({ "payment_id" : self.payment_id }));
        r = r.push_json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.push_json(json!({ "reference" : self.reference }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationRecipientGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub recipient_id: String,
}
impl<'a> PaymentInitiationRecipientGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationRecipientGetResponse> {
        let mut r = self.client.client.post("/payment_initiation/recipient/get");
        r = r.push_json(json!({ "recipient_id" : self.recipient_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationRecipientListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> PaymentInitiationRecipientListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationRecipientListResponse> {
        let mut r = self.client.client.post("/payment_initiation/recipient/list");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationPaymentCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub recipient_id: String,
    pub reference: String,
    pub amount: PaymentAmount,
    pub schedule: Option<ExternalPaymentScheduleRequest>,
    pub options: Option<ExternalPaymentOptions>,
}
impl<'a> PaymentInitiationPaymentCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationPaymentCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/create");
        r = r.push_json(json!({ "recipient_id" : self.recipient_id }));
        r = r.push_json(json!({ "reference" : self.reference }));
        r = r.push_json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.schedule {
            r = r.push_json(json!({ "schedule" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn schedule(mut self, schedule: ExternalPaymentScheduleRequest) -> Self {
        self.schedule = Some(schedule);
        self
    }
    pub fn options(mut self, options: ExternalPaymentOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreatePaymentTokenRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_id: String,
}
impl<'a> CreatePaymentTokenRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaymentInitiationPaymentTokenCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/token/create");
        r = r.push_json(json!({ "payment_id" : self.payment_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationConsentCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub recipient_id: String,
    pub reference: String,
    pub scopes: Vec<String>,
    pub constraints: PaymentInitiationConsentConstraints,
    pub options: Option<ExternalPaymentInitiationConsentOptions>,
}
impl<'a> PaymentInitiationConsentCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationConsentCreateResponse> {
        let mut r = self.client.client.post("/payment_initiation/consent/create");
        r = r.push_json(json!({ "recipient_id" : self.recipient_id }));
        r = r.push_json(json!({ "reference" : self.reference }));
        r = r.push_json(json!({ "scopes" : self.scopes }));
        r = r.push_json(json!({ "constraints" : self.constraints }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: ExternalPaymentInitiationConsentOptions) -> Self {
        self.options = Some(options);
        self
    }
}
pub struct PaymentInitiationConsentCreateRequired<'a> {
    pub recipient_id: &'a str,
    pub reference: &'a str,
    pub scopes: &'a [&'a str],
    pub constraints: PaymentInitiationConsentConstraints,
}
impl<'a> PaymentInitiationConsentCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationConsentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub consent_id: String,
}
impl<'a> PaymentInitiationConsentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationConsentGetResponse> {
        let mut r = self.client.client.post("/payment_initiation/consent/get");
        r = r.push_json(json!({ "consent_id" : self.consent_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationConsentRevokeRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub consent_id: String,
}
impl<'a> PaymentInitiationConsentRevokeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationConsentRevokeResponse> {
        let mut r = self.client.client.post("/payment_initiation/consent/revoke");
        r = r.push_json(json!({ "consent_id" : self.consent_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationConsentPaymentExecuteRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub consent_id: String,
    pub amount: PaymentAmount,
    pub idempotency_key: String,
}
impl<'a> PaymentInitiationConsentPaymentExecuteRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<PaymentInitiationConsentPaymentExecuteResponse> {
        let mut r = self
            .client
            .client
            .post("/payment_initiation/consent/payment/execute");
        r = r.push_json(json!({ "consent_id" : self.consent_id }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "idempotency_key" : self.idempotency_key }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxItemResetLoginRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> SandboxItemResetLoginRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxItemResetLoginResponse> {
        let mut r = self.client.client.post("/sandbox/item/reset_login");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxItemSetVerificationStatusRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub verification_status: String,
}
impl<'a> SandboxItemSetVerificationStatusRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxItemSetVerificationStatusResponse> {
        let mut r = self.client.client.post("/sandbox/item/set_verification_status");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "account_id" : self.account_id }));
        r = r.push_json(json!({ "verification_status" : self.verification_status }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemPublicTokenExchangeRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub public_token: String,
}
impl<'a> ItemPublicTokenExchangeRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemPublicTokenExchangeResponse> {
        let mut r = self.client.client.post("/item/public_token/exchange");
        r = r.push_json(json!({ "public_token" : self.public_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemCreatePublicTokenRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> ItemCreatePublicTokenRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemPublicTokenCreateResponse> {
        let mut r = self.client.client.post("/item/public_token/create");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UserCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_user_id: String,
}
impl<'a> UserCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserCreateResponse> {
        let mut r = self.client.client.post("/user/create");
        r = r.push_json(json!({ "client_user_id" : self.client_user_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationPaymentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_id: String,
}
impl<'a> PaymentInitiationPaymentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationPaymentGetResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/get");
        r = r.push_json(json!({ "payment_id" : self.payment_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentInitiationPaymentListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub consent_id: Option<String>,
}
impl<'a> PaymentInitiationPaymentListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentInitiationPaymentListResponse> {
        let mut r = self.client.client.post("/payment_initiation/payment/list");
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.consent_id {
            r = r.push_json(json!({ "consent_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn consent_id(mut self, consent_id: &str) -> Self {
        self.consent_id = Some(consent_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_tokens: Vec<String>,
    pub days_requested: i64,
    pub options: Option<AssetReportCreateRequestOptions>,
}
impl<'a> AssetReportCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportCreateResponse> {
        let mut r = self.client.client.post("/asset_report/create");
        r = r.push_json(json!({ "access_tokens" : self.access_tokens }));
        r = r.push_json(json!({ "days_requested" : self.days_requested }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: AssetReportCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub days_requested: Option<i64>,
    pub options: Option<AssetReportRefreshRequestOptions>,
}
impl<'a> AssetReportRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportRefreshResponse> {
        let mut r = self.client.client.post("/asset_report/refresh");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        if let Some(ref unwrapped) = self.days_requested {
            r = r.push_json(json!({ "days_requested" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn days_requested(mut self, days_requested: i64) -> Self {
        self.days_requested = Some(days_requested);
        self
    }
    pub fn options(mut self, options: AssetReportRefreshRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportRelayRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_relay_token: String,
    pub webhook: Option<String>,
}
impl<'a> AssetReportRelayRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportRelayRefreshResponse> {
        let mut r = self.client.client.post("/asset_report/relay/refresh");
        r = r.push_json(json!({ "asset_relay_token" : self.asset_relay_token }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
}
impl<'a> AssetReportRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportRemoveResponse> {
        let mut r = self.client.client.post("/asset_report/remove");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportFilterRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub account_ids_to_exclude: Vec<String>,
}
impl<'a> AssetReportFilterRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportFilterResponse> {
        let mut r = self.client.client.post("/asset_report/filter");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        r = r
            .push_json(
                json!({ "account_ids_to_exclude" : self.account_ids_to_exclude }),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub include_insights: Option<bool>,
    pub fast_report: Option<bool>,
}
impl<'a> AssetReportGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportGetResponse> {
        let mut r = self.client.client.post("/asset_report/get");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        if let Some(ref unwrapped) = self.include_insights {
            r = r.push_json(json!({ "include_insights" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fast_report {
            r = r.push_json(json!({ "fast_report" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn include_insights(mut self, include_insights: bool) -> Self {
        self.include_insights = Some(include_insights);
        self
    }
    pub fn fast_report(mut self, fast_report: bool) -> Self {
        self.fast_report = Some(fast_report);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportPdfGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
}
impl<'a> AssetReportPdfGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/asset_report/pdf/get");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportAuditCopyCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub auditor_id: String,
}
impl<'a> AssetReportAuditCopyCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportAuditCopyCreateResponse> {
        let mut r = self.client.client.post("/asset_report/audit_copy/create");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        r = r.push_json(json!({ "auditor_id" : self.auditor_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportAuditCopyRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportAuditCopyRemoveResponse> {
        let mut r = self.client.client.post("/asset_report/audit_copy/remove");
        r = r.push_json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportRelayCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_report_token: String,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl<'a> AssetReportRelayCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportRelayCreateResponse> {
        let mut r = self.client.client.post("/asset_report/relay/create");
        r = r.push_json(json!({ "asset_report_token" : self.asset_report_token }));
        r = r.push_json(json!({ "secondary_client_id" : self.secondary_client_id }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportRelayGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_relay_token: String,
}
impl<'a> AssetReportRelayGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportGetResponse> {
        let mut r = self.client.client.post("/asset_report/relay/get");
        r = r.push_json(json!({ "asset_relay_token" : self.asset_relay_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportRelayRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub asset_relay_token: String,
}
impl<'a> AssetReportRelayRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportRelayRemoveResponse> {
        let mut r = self.client.client.post("/asset_report/relay/remove");
        r = r.push_json(json!({ "asset_relay_token" : self.asset_relay_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct InvestmentsHoldingsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub options: Option<InvestmentHoldingsGetRequestOptions>,
}
impl<'a> InvestmentsHoldingsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvestmentsHoldingsGetResponse> {
        let mut r = self.client.client.post("/investments/holdings/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: InvestmentHoldingsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct InvestmentsTransactionsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
    pub options: Option<InvestmentsTransactionsGetRequestOptions>,
}
impl<'a> InvestmentsTransactionsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvestmentsTransactionsGetResponse> {
        let mut r = self.client.client.post("/investments/transactions/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "start_date" : self.start_date }));
        r = r.push_json(json!({ "end_date" : self.end_date }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: InvestmentsTransactionsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub processor: String,
}
impl<'a> ProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProcessorTokenCreateResponse> {
        let mut r = self.client.client.post("/processor/token/create");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "account_id" : self.account_id }));
        r = r.push_json(json!({ "processor" : self.processor }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<ProcessorStripeBankAccountTokenCreateResponse> {
        let mut r = self
            .client
            .client
            .post("/processor/stripe/bank_account_token/create");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "account_id" : self.account_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ProcessorApexProcessorTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorApexProcessorTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProcessorTokenCreateResponse> {
        let mut r = self.client.client.post("/processor/apex/processor_token/create");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "account_id" : self.account_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DepositSwitchCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub target_access_token: String,
    pub target_account_id: String,
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
}
impl<'a> DepositSwitchCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DepositSwitchCreateResponse> {
        let mut r = self.client.client.post("/deposit_switch/create");
        r = r.push_json(json!({ "target_access_token" : self.target_access_token }));
        r = r.push_json(json!({ "target_account_id" : self.target_account_id }));
        if let Some(ref unwrapped) = self.country_code {
            r = r.push_json(json!({ "country_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn country_code(mut self, country_code: &str) -> Self {
        self.country_code = Some(country_code.to_owned());
        self
    }
    pub fn options(mut self, options: DepositSwitchCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ItemImportRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub products: Vec<String>,
    pub user_auth: ItemImportRequestUserAuth,
    pub options: Option<ItemImportRequestOptions>,
}
impl<'a> ItemImportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ItemImportResponse> {
        let mut r = self.client.client.post("/item/import");
        r = r.push_json(json!({ "products" : self.products }));
        r = r.push_json(json!({ "user_auth" : self.user_auth }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: ItemImportRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DepositSwitchTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DepositSwitchTokenCreateResponse> {
        let mut r = self.client.client.post("/deposit_switch/token/create");
        r = r.push_json(json!({ "deposit_switch_id" : self.deposit_switch_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct LinkTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_name: String,
    pub language: String,
    pub country_codes: Vec<String>,
    pub user: LinkTokenCreateRequestUser,
    pub products: Option<Vec<String>>,
    pub additional_consented_products: Option<Vec<String>>,
    pub webhook: Option<String>,
    pub access_token: Option<String>,
    pub link_customization_name: Option<String>,
    pub redirect_uri: Option<String>,
    pub android_package_name: Option<String>,
    pub institution_data: Option<LinkTokenCreateInstitutionData>,
    pub account_filters: Option<LinkTokenAccountFilters>,
    pub eu_config: Option<LinkTokenEuConfig>,
    pub institution_id: Option<String>,
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    pub auth: Option<LinkTokenCreateRequestAuth>,
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    pub update: Option<LinkTokenCreateRequestUpdate>,
    pub identity_verification: Option<LinkTokenCreateRequestIdentityVerification>,
    pub user_token: Option<String>,
}
impl<'a> LinkTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkTokenCreateResponse> {
        let mut r = self.client.client.post("/link/token/create");
        r = r.push_json(json!({ "client_name" : self.client_name }));
        r = r.push_json(json!({ "language" : self.language }));
        r = r.push_json(json!({ "country_codes" : self.country_codes }));
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.products {
            r = r.push_json(json!({ "products" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.additional_consented_products {
            r = r.push_json(json!({ "additional_consented_products" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.link_customization_name {
            r = r.push_json(json!({ "link_customization_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.redirect_uri {
            r = r.push_json(json!({ "redirect_uri" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.android_package_name {
            r = r.push_json(json!({ "android_package_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.institution_data {
            r = r.push_json(json!({ "institution_data" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_filters {
            r = r.push_json(json!({ "account_filters" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.eu_config {
            r = r.push_json(json!({ "eu_config" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.institution_id {
            r = r.push_json(json!({ "institution_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_initiation {
            r = r.push_json(json!({ "payment_initiation" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.deposit_switch {
            r = r.push_json(json!({ "deposit_switch" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.income_verification {
            r = r.push_json(json!({ "income_verification" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.auth {
            r = r.push_json(json!({ "auth" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transfer {
            r = r.push_json(json!({ "transfer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.update {
            r = r.push_json(json!({ "update" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.identity_verification {
            r = r.push_json(json!({ "identity_verification" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_token {
            r = r.push_json(json!({ "user_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn products(
        mut self,
        products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .products = Some(
            products.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn additional_consented_products(
        mut self,
        additional_consented_products: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .additional_consented_products = Some(
            additional_consented_products
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn link_customization_name(mut self, link_customization_name: &str) -> Self {
        self.link_customization_name = Some(link_customization_name.to_owned());
        self
    }
    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.redirect_uri = Some(redirect_uri.to_owned());
        self
    }
    pub fn android_package_name(mut self, android_package_name: &str) -> Self {
        self.android_package_name = Some(android_package_name.to_owned());
        self
    }
    pub fn institution_data(
        mut self,
        institution_data: LinkTokenCreateInstitutionData,
    ) -> Self {
        self.institution_data = Some(institution_data);
        self
    }
    pub fn account_filters(mut self, account_filters: LinkTokenAccountFilters) -> Self {
        self.account_filters = Some(account_filters);
        self
    }
    pub fn eu_config(mut self, eu_config: LinkTokenEuConfig) -> Self {
        self.eu_config = Some(eu_config);
        self
    }
    pub fn institution_id(mut self, institution_id: &str) -> Self {
        self.institution_id = Some(institution_id.to_owned());
        self
    }
    pub fn payment_initiation(
        mut self,
        payment_initiation: LinkTokenCreateRequestPaymentInitiation,
    ) -> Self {
        self.payment_initiation = Some(payment_initiation);
        self
    }
    pub fn deposit_switch(
        mut self,
        deposit_switch: LinkTokenCreateRequestDepositSwitch,
    ) -> Self {
        self.deposit_switch = Some(deposit_switch);
        self
    }
    pub fn income_verification(
        mut self,
        income_verification: LinkTokenCreateRequestIncomeVerification,
    ) -> Self {
        self.income_verification = Some(income_verification);
        self
    }
    pub fn auth(mut self, auth: LinkTokenCreateRequestAuth) -> Self {
        self.auth = Some(auth);
        self
    }
    pub fn transfer(mut self, transfer: LinkTokenCreateRequestTransfer) -> Self {
        self.transfer = Some(transfer);
        self
    }
    pub fn update(mut self, update: LinkTokenCreateRequestUpdate) -> Self {
        self.update = Some(update);
        self
    }
    pub fn identity_verification(
        mut self,
        identity_verification: LinkTokenCreateRequestIdentityVerification,
    ) -> Self {
        self.identity_verification = Some(identity_verification);
        self
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
pub struct LinkTokenCreateRequired<'a> {
    pub client_name: &'a str,
    pub language: &'a str,
    pub country_codes: &'a [&'a str],
    pub user: LinkTokenCreateRequestUser,
}
impl<'a> LinkTokenCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct LinkTokenGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub link_token: String,
}
impl<'a> LinkTokenGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<LinkTokenGetResponse> {
        let mut r = self.client.client.post("/link/token/get");
        r = r.push_json(json!({ "link_token" : self.link_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AssetReportAuditCopyGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> AssetReportAuditCopyGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportGetResponse> {
        let mut r = self.client.client.post("/asset_report/audit_copy/get");
        r = r.push_json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DepositSwitchGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub deposit_switch_id: String,
}
impl<'a> DepositSwitchGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DepositSwitchGetResponse> {
        let mut r = self.client.client.post("/deposit_switch/get");
        r = r.push_json(json!({ "deposit_switch_id" : self.deposit_switch_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_id: String,
}
impl<'a> TransferGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferGetResponse> {
        let mut r = self.client.client.post("/transfer/get");
        r = r.push_json(json!({ "transfer_id" : self.transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferGetResponse> {
        let mut r = self.client.client.post("/bank_transfer/get");
        r = r.push_json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferAuthorizationCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub ach_class: String,
    pub user: TransferAuthorizationUserInRequest,
    pub device: Option<TransferAuthorizationDevice>,
    pub origination_account_id: Option<String>,
    pub iso_currency_code: Option<String>,
    pub user_present: Option<bool>,
    pub payment_profile_id: Option<String>,
}
impl<'a> TransferAuthorizationCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferAuthorizationCreateResponse> {
        let mut r = self.client.client.post("/transfer/authorization/create");
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.push_json(json!({ "account_id" : unwrapped }));
        }
        r = r.push_json(json!({ "type" : self.type_ }));
        r = r.push_json(json!({ "network" : self.network }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "ach_class" : self.ach_class }));
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.device {
            r = r.push_json(json!({ "device" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.push_json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_present {
            r = r.push_json(json!({ "user_present" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_id {
            r = r.push_json(json!({ "payment_profile_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn device(mut self, device: TransferAuthorizationDevice) -> Self {
        self.device = Some(device);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
    pub fn payment_profile_id(mut self, payment_profile_id: &str) -> Self {
        self.payment_profile_id = Some(payment_profile_id.to_owned());
        self
    }
}
pub struct TransferAuthorizationCreateRequired<'a> {
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub ach_class: &'a str,
    pub user: TransferAuthorizationUserInRequest,
}
impl<'a> TransferAuthorizationCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: Option<String>,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub authorization_id: String,
    pub type_: String,
    pub network: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub user: TransferUserInRequest,
    pub metadata: Option<TransferMetadata>,
    pub origination_account_id: Option<String>,
    pub iso_currency_code: Option<String>,
    pub payment_profile_id: Option<String>,
}
impl<'a> TransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferCreateResponse> {
        let mut r = self.client.client.post("/transfer/create");
        if let Some(ref unwrapped) = self.idempotency_key {
            r = r.push_json(json!({ "idempotency_key" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.push_json(json!({ "account_id" : unwrapped }));
        }
        r = r.push_json(json!({ "authorization_id" : self.authorization_id }));
        r = r.push_json(json!({ "type" : self.type_ }));
        r = r.push_json(json!({ "network" : self.network }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "description" : self.description }));
        r = r.push_json(json!({ "ach_class" : self.ach_class }));
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.push_json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_id {
            r = r.push_json(json!({ "payment_profile_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn idempotency_key(mut self, idempotency_key: &str) -> Self {
        self.idempotency_key = Some(idempotency_key.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: TransferMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn payment_profile_id(mut self, payment_profile_id: &str) -> Self {
        self.payment_profile_id = Some(payment_profile_id.to_owned());
        self
    }
}
pub struct TransferCreateRequired<'a> {
    pub authorization_id: &'a str,
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub ach_class: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
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
    pub ach_class: Option<String>,
    pub user: BankTransferUser,
    pub custom_tag: Option<String>,
    pub metadata: Option<BankTransferMetadata>,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferCreateResponse> {
        let mut r = self.client.client.post("/bank_transfer/create");
        r = r.push_json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "account_id" : self.account_id }));
        r = r.push_json(json!({ "type" : self.type_ }));
        r = r.push_json(json!({ "network" : self.network }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "iso_currency_code" : self.iso_currency_code }));
        r = r.push_json(json!({ "description" : self.description }));
        if let Some(ref unwrapped) = self.ach_class {
            r = r.push_json(json!({ "ach_class" : unwrapped }));
        }
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.custom_tag {
            r = r.push_json(json!({ "custom_tag" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn ach_class(mut self, ach_class: &str) -> Self {
        self.ach_class = Some(ach_class.to_owned());
        self
    }
    pub fn custom_tag(mut self, custom_tag: &str) -> Self {
        self.custom_tag = Some(custom_tag.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: BankTransferMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
pub struct BankTransferCreateRequired<'a> {
    pub idempotency_key: &'a str,
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub type_: &'a str,
    pub network: &'a str,
    pub amount: &'a str,
    pub iso_currency_code: &'a str,
    pub description: &'a str,
    pub user: BankTransferUser,
}
impl<'a> BankTransferCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
}
impl<'a> TransferListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferListResponse> {
        let mut r = self.client.client.post("/transfer/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub direction: Option<String>,
}
impl<'a> BankTransferListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferListResponse> {
        let mut r = self.client.client.post("/bank_transfer/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.direction {
            r = r.push_json(json!({ "direction" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = Some(direction.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferCancelRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_id: String,
}
impl<'a> TransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferCancelResponse> {
        let mut r = self.client.client.post("/transfer/cancel");
        r = r.push_json(json!({ "transfer_id" : self.transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferCancelRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub bank_transfer_id: String,
}
impl<'a> BankTransferCancelRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferCancelResponse> {
        let mut r = self.client.client.post("/bank_transfer/cancel");
        r = r.push_json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferEventListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub transfer_type: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub sweep_id: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
}
impl<'a> TransferEventListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferEventListResponse> {
        let mut r = self.client.client.post("/transfer/event/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transfer_id {
            r = r.push_json(json!({ "transfer_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.push_json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transfer_type {
            r = r.push_json(json!({ "transfer_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.event_types {
            r = r.push_json(json!({ "event_types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.sweep_id {
            r = r.push_json(json!({ "sweep_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn transfer_id(mut self, transfer_id: &str) -> Self {
        self.transfer_id = Some(transfer_id.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn transfer_type(mut self, transfer_type: &str) -> Self {
        self.transfer_type = Some(transfer_type.to_owned());
        self
    }
    pub fn event_types(
        mut self,
        event_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .event_types = Some(
            event_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn sweep_id(mut self, sweep_id: &str) -> Self {
        self.sweep_id = Some(sweep_id.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferEventListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub bank_transfer_id: Option<String>,
    pub account_id: Option<String>,
    pub bank_transfer_type: Option<String>,
    pub event_types: Option<Vec<String>>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub direction: Option<String>,
}
impl<'a> BankTransferEventListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferEventListResponse> {
        let mut r = self.client.client.post("/bank_transfer/event/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bank_transfer_id {
            r = r.push_json(json!({ "bank_transfer_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.push_json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bank_transfer_type {
            r = r.push_json(json!({ "bank_transfer_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.event_types {
            r = r.push_json(json!({ "event_types" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.direction {
            r = r.push_json(json!({ "direction" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn bank_transfer_id(mut self, bank_transfer_id: &str) -> Self {
        self.bank_transfer_id = Some(bank_transfer_id.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn bank_transfer_type(mut self, bank_transfer_type: &str) -> Self {
        self.bank_transfer_type = Some(bank_transfer_type.to_owned());
        self
    }
    pub fn event_types(
        mut self,
        event_types: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .event_types = Some(
            event_types.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = Some(direction.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferEventSyncRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> TransferEventSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferEventSyncResponse> {
        let mut r = self.client.client.post("/transfer/event/sync");
        r = r.push_json(json!({ "after_id" : self.after_id }));
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferEventSyncRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub after_id: i64,
    pub count: Option<i64>,
}
impl<'a> BankTransferEventSyncRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferEventSyncResponse> {
        let mut r = self.client.client.post("/bank_transfer/event/sync");
        r = r.push_json(json!({ "after_id" : self.after_id }));
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferSweepGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub sweep_id: String,
}
impl<'a> TransferSweepGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferSweepGetResponse> {
        let mut r = self.client.client.post("/transfer/sweep/get");
        r = r.push_json(json!({ "sweep_id" : self.sweep_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferSweepGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub sweep_id: String,
}
impl<'a> BankTransferSweepGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferSweepGetResponse> {
        let mut r = self.client.client.post("/bank_transfer/sweep/get");
        r = r.push_json(json!({ "sweep_id" : self.sweep_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferSweepListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> TransferSweepListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferSweepListResponse> {
        let mut r = self.client.client.post("/transfer/sweep/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferSweepListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub origination_account_id: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub count: Option<i64>,
}
impl<'a> BankTransferSweepListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferSweepListResponse> {
        let mut r = self.client.client.post("/bank_transfer/sweep/list");
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_time {
            r = r.push_json(json!({ "start_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.push_json(json!({ "end_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn start_time(mut self, start_time: &str) -> Self {
        self.start_time = Some(start_time.to_owned());
        self
    }
    pub fn end_time(mut self, end_time: &str) -> Self {
        self.end_time = Some(end_time.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferBalanceGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferBalanceGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferBalanceGetResponse> {
        let mut r = self.client.client.post("/bank_transfer/balance/get");
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BankTransferMigrateAccountRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_number: String,
    pub routing_number: String,
    pub wire_routing_number: Option<String>,
    pub account_type: String,
}
impl<'a> BankTransferMigrateAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BankTransferMigrateAccountResponse> {
        let mut r = self.client.client.post("/bank_transfer/migrate_account");
        r = r.push_json(json!({ "account_number" : self.account_number }));
        r = r.push_json(json!({ "routing_number" : self.routing_number }));
        if let Some(ref unwrapped) = self.wire_routing_number {
            r = r.push_json(json!({ "wire_routing_number" : unwrapped }));
        }
        r = r.push_json(json!({ "account_type" : self.account_type }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn wire_routing_number(mut self, wire_routing_number: &str) -> Self {
        self.wire_routing_number = Some(wire_routing_number.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferMigrateAccountRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_number: String,
    pub routing_number: String,
    pub wire_routing_number: Option<String>,
    pub account_type: String,
}
impl<'a> TransferMigrateAccountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferMigrateAccountResponse> {
        let mut r = self.client.client.post("/transfer/migrate_account");
        r = r.push_json(json!({ "account_number" : self.account_number }));
        r = r.push_json(json!({ "routing_number" : self.routing_number }));
        if let Some(ref unwrapped) = self.wire_routing_number {
            r = r.push_json(json!({ "wire_routing_number" : unwrapped }));
        }
        r = r.push_json(json!({ "account_type" : self.account_type }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn wire_routing_number(mut self, wire_routing_number: &str) -> Self {
        self.wire_routing_number = Some(wire_routing_number.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferIntentCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_id: Option<String>,
    pub mode: String,
    pub amount: String,
    pub description: String,
    pub ach_class: String,
    pub origination_account_id: Option<String>,
    pub user: TransferUserInRequest,
    pub metadata: Option<TransferMetadata>,
    pub iso_currency_code: Option<String>,
    pub require_guarantee: Option<bool>,
}
impl<'a> TransferIntentCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferIntentCreateResponse> {
        let mut r = self.client.client.post("/transfer/intent/create");
        if let Some(ref unwrapped) = self.account_id {
            r = r.push_json(json!({ "account_id" : unwrapped }));
        }
        r = r.push_json(json!({ "mode" : self.mode }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "description" : self.description }));
        r = r.push_json(json!({ "ach_class" : self.ach_class }));
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.push_json(json!({ "origination_account_id" : unwrapped }));
        }
        r = r.push_json(json!({ "user" : self.user }));
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_json(json!({ "metadata" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.push_json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.require_guarantee {
            r = r.push_json(json!({ "require_guarantee" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: TransferMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn require_guarantee(mut self, require_guarantee: bool) -> Self {
        self.require_guarantee = Some(require_guarantee);
        self
    }
}
pub struct TransferIntentCreateRequired<'a> {
    pub mode: &'a str,
    pub amount: &'a str,
    pub description: &'a str,
    pub ach_class: &'a str,
    pub user: TransferUserInRequest,
}
impl<'a> TransferIntentCreateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferIntentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_intent_id: String,
}
impl<'a> TransferIntentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferIntentGetResponse> {
        let mut r = self.client.client.post("/transfer/intent/get");
        r = r.push_json(json!({ "transfer_intent_id" : self.transfer_intent_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferRepaymentListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> TransferRepaymentListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferRepaymentListResponse> {
        let mut r = self.client.client.post("/transfer/repayment/list");
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.push_json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: &str) -> Self {
        self.end_date = Some(end_date.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransferRepaymentReturnListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub repayment_id: String,
    pub count: Option<i64>,
    pub offset: Option<i64>,
}
impl<'a> TransferRepaymentReturnListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransferRepaymentReturnListResponse> {
        let mut r = self.client.client.post("/transfer/repayment/return/list");
        r = r.push_json(json!({ "repayment_id" : self.repayment_id }));
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.push_json(json!({ "offset" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxBankTransferSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub bank_transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<BankTransferFailure>,
}
impl<'a> SandboxBankTransferSimulateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxBankTransferSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/bank_transfer/simulate");
        r = r.push_json(json!({ "bank_transfer_id" : self.bank_transfer_id }));
        r = r.push_json(json!({ "event_type" : self.event_type }));
        if let Some(ref unwrapped) = self.failure_reason {
            r = r.push_json(json!({ "failure_reason" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn failure_reason(mut self, failure_reason: BankTransferFailure) -> Self {
        self.failure_reason = Some(failure_reason);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxTransferSweepSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> SandboxTransferSweepSimulateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxTransferSweepSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/sweep/simulate");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxTransferSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transfer_id: String,
    pub event_type: String,
    pub failure_reason: Option<TransferFailure>,
}
impl<'a> SandboxTransferSimulateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxTransferSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/simulate");
        r = r.push_json(json!({ "transfer_id" : self.transfer_id }));
        r = r.push_json(json!({ "event_type" : self.event_type }));
        if let Some(ref unwrapped) = self.failure_reason {
            r = r.push_json(json!({ "failure_reason" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn failure_reason(mut self, failure_reason: TransferFailure) -> Self {
        self.failure_reason = Some(failure_reason);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxTransferRepaymentSimulateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> SandboxTransferRepaymentSimulateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxTransferRepaymentSimulateResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/repayment/simulate");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxTransferFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub webhook: String,
}
impl<'a> SandboxTransferFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxTransferFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/transfer/fire_webhook");
        r = r.push_json(json!({ "webhook" : self.webhook }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct EmployersSearchRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub query: String,
    pub products: Vec<String>,
}
impl<'a> EmployersSearchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EmployersSearchResponse> {
        let mut r = self.client.client.post("/employers/search");
        r = r.push_json(json!({ "query" : self.query }));
        r = r.push_json(json!({ "products" : self.products }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IncomeVerificationCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub webhook: String,
    pub precheck_id: Option<String>,
    pub options: Option<IncomeVerificationCreateRequestOptions>,
}
impl<'a> IncomeVerificationCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IncomeVerificationCreateResponse> {
        let mut r = self.client.client.post("/income/verification/create");
        r = r.push_json(json!({ "webhook" : self.webhook }));
        if let Some(ref unwrapped) = self.precheck_id {
            r = r.push_json(json!({ "precheck_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn precheck_id(mut self, precheck_id: &str) -> Self {
        self.precheck_id = Some(precheck_id.to_owned());
        self
    }
    pub fn options(mut self, options: IncomeVerificationCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IncomeVerificationPaystubsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationPaystubsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IncomeVerificationPaystubsGetResponse> {
        let mut r = self.client.client.post("/income/verification/paystubs/get");
        if let Some(ref unwrapped) = self.income_verification_id {
            r = r.push_json(json!({ "income_verification_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IncomeVerificationDocumentsDownloadRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
    pub document_id: Option<String>,
}
impl<'a> IncomeVerificationDocumentsDownloadRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/income/verification/documents/download");
        if let Some(ref unwrapped) = self.income_verification_id {
            r = r.push_json(json!({ "income_verification_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.document_id {
            r = r.push_json(json!({ "document_id" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn document_id(mut self, document_id: &str) -> Self {
        self.document_id = Some(document_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IncomeVerificationRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IncomeVerificationRefreshResponse> {
        let mut r = self.client.client.post("/income/verification/refresh");
        if let Some(ref unwrapped) = self.income_verification_id {
            r = r.push_json(json!({ "income_verification_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IncomeVerificationTaxformsGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub income_verification_id: Option<String>,
    pub access_token: Option<String>,
}
impl<'a> IncomeVerificationTaxformsGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IncomeVerificationTaxformsGetResponse> {
        let mut r = self.client.client.post("/income/verification/taxforms/get");
        if let Some(ref unwrapped) = self.income_verification_id {
            r = r.push_json(json!({ "income_verification_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_token {
            r = r.push_json(json!({ "access_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn income_verification_id(mut self, income_verification_id: &str) -> Self {
        self.income_verification_id = Some(income_verification_id.to_owned());
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct IncomeVerificationPrecheckRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user: Option<IncomeVerificationPrecheckUser>,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub transactions_access_token: Option<String>,
    pub transactions_access_tokens: Option<Vec<String>>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
}
impl<'a> IncomeVerificationPrecheckRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IncomeVerificationPrecheckResponse> {
        let mut r = self.client.client.post("/income/verification/precheck");
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.employer {
            r = r.push_json(json!({ "employer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transactions_access_token {
            r = r.push_json(json!({ "transactions_access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.transactions_access_tokens {
            r = r.push_json(json!({ "transactions_access_tokens" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.us_military_info {
            r = r.push_json(json!({ "us_military_info" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user(mut self, user: IncomeVerificationPrecheckUser) -> Self {
        self.user = Some(user);
        self
    }
    pub fn employer(mut self, employer: IncomeVerificationPrecheckEmployer) -> Self {
        self.employer = Some(employer);
        self
    }
    pub fn transactions_access_token(mut self, transactions_access_token: &str) -> Self {
        self.transactions_access_token = Some(transactions_access_token.to_owned());
        self
    }
    pub fn transactions_access_tokens(
        mut self,
        transactions_access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .transactions_access_tokens = Some(
            transactions_access_tokens
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn us_military_info(
        mut self,
        us_military_info: IncomeVerificationPrecheckMilitaryInfo,
    ) -> Self {
        self.us_military_info = Some(us_military_info);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct EmploymentVerificationGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> EmploymentVerificationGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EmploymentVerificationGetResponse> {
        let mut r = self.client.client.post("/employment/verification/get");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DepositSwitchAltCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub target_account: DepositSwitchTargetAccount,
    pub target_user: DepositSwitchTargetUser,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub country_code: Option<String>,
}
impl<'a> DepositSwitchAltCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DepositSwitchAltCreateResponse> {
        let mut r = self.client.client.post("/deposit_switch/alt/create");
        r = r.push_json(json!({ "target_account" : self.target_account }));
        r = r.push_json(json!({ "target_user" : self.target_user }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country_code {
            r = r.push_json(json!({ "country_code" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: DepositSwitchCreateRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn country_code(mut self, country_code: &str) -> Self {
        self.country_code = Some(country_code.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditAuditCopyTokenCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub report_tokens: Vec<ReportToken>,
    pub auditor_id: String,
}
impl<'a> CreditAuditCopyTokenCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditAuditCopyTokenCreateResponse> {
        let mut r = self.client.client.post("/credit/audit_copy_token/create");
        r = r.push_json(json!({ "report_tokens" : self.report_tokens }));
        r = r.push_json(json!({ "auditor_id" : self.auditor_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditReportAuditCopyRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub audit_copy_token: String,
}
impl<'a> CreditReportAuditCopyRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditAuditCopyTokenRemoveResponse> {
        let mut r = self.client.client.post("/credit/audit_copy_token/remove");
        r = r.push_json(json!({ "audit_copy_token" : self.audit_copy_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditBankIncomeGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: Option<String>,
    pub options: Option<CreditBankIncomeGetRequestOptions>,
}
impl<'a> CreditBankIncomeGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditBankIncomeGetResponse> {
        let mut r = self.client.client.post("/credit/bank_income/get");
        if let Some(ref unwrapped) = self.user_token {
            r = r.push_json(json!({ "user_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
    pub fn options(mut self, options: CreditBankIncomeGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditBankIncomePdfGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditBankIncomePdfGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self.client.client.post("/credit/bank_income/pdf/get");
        r = r.push_json(json!({ "user_token" : self.user_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditBankIncomeRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
    pub options: Option<CreditBankIncomeRefreshRequestOptions>,
}
impl<'a> CreditBankIncomeRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditBankIncomeRefreshResponse> {
        let mut r = self.client.client.post("/credit/bank_income/refresh");
        r = r.push_json(json!({ "user_token" : self.user_token }));
        if let Some(ref unwrapped) = self.options {
            r = r.push_json(json!({ "options" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn options(mut self, options: CreditBankIncomeRefreshRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditPayrollIncomeGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: Option<String>,
}
impl<'a> CreditPayrollIncomeGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditPayrollIncomeGetResponse> {
        let mut r = self.client.client.post("/credit/payroll_income/get");
        if let Some(ref unwrapped) = self.user_token {
            r = r.push_json(json!({ "user_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditPayrollIncomePrecheckRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: Option<String>,
    pub access_tokens: Option<Vec<String>>,
    pub employer: Option<IncomeVerificationPrecheckEmployer>,
    pub us_military_info: Option<IncomeVerificationPrecheckMilitaryInfo>,
}
impl<'a> CreditPayrollIncomePrecheckRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditPayrollIncomePrecheckResponse> {
        let mut r = self.client.client.post("/credit/payroll_income/precheck");
        if let Some(ref unwrapped) = self.user_token {
            r = r.push_json(json!({ "user_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_tokens {
            r = r.push_json(json!({ "access_tokens" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.employer {
            r = r.push_json(json!({ "employer" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.us_military_info {
            r = r.push_json(json!({ "us_military_info" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
    pub fn access_tokens(
        mut self,
        access_tokens: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .access_tokens = Some(
            access_tokens.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn employer(mut self, employer: IncomeVerificationPrecheckEmployer) -> Self {
        self.employer = Some(employer);
        self
    }
    pub fn us_military_info(
        mut self,
        us_military_info: IncomeVerificationPrecheckMilitaryInfo,
    ) -> Self {
        self.us_military_info = Some(us_military_info);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditEmploymentGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: String,
}
impl<'a> CreditEmploymentGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditEmploymentGetResponse> {
        let mut r = self.client.client.post("/credit/employment/get");
        r = r.push_json(json!({ "user_token" : self.user_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditPayrollIncomeRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub user_token: Option<String>,
}
impl<'a> CreditPayrollIncomeRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditPayrollIncomeRefreshResponse> {
        let mut r = self.client.client.post("/credit/payroll_income/refresh");
        if let Some(ref unwrapped) = self.user_token {
            r = r.push_json(json!({ "user_token" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user_token(mut self, user_token: &str) -> Self {
        self.user_token = Some(user_token.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditRelayCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub report_tokens: Vec<ReportToken>,
    pub secondary_client_id: String,
    pub webhook: Option<String>,
}
impl<'a> CreditRelayCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditRelayCreateResponse> {
        let mut r = self.client.client.post("/credit/relay/create");
        r = r.push_json(json!({ "report_tokens" : self.report_tokens }));
        r = r.push_json(json!({ "secondary_client_id" : self.secondary_client_id }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditRelayGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub relay_token: String,
    pub report_type: String,
}
impl<'a> CreditRelayGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssetReportGetResponse> {
        let mut r = self.client.client.post("/credit/relay/get");
        r = r.push_json(json!({ "relay_token" : self.relay_token }));
        r = r.push_json(json!({ "report_type" : self.report_type }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditRelayRefreshRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub relay_token: String,
    pub report_type: String,
    pub webhook: Option<String>,
}
impl<'a> CreditRelayRefreshRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditRelayRefreshResponse> {
        let mut r = self.client.client.post("/credit/relay/refresh");
        r = r.push_json(json!({ "relay_token" : self.relay_token }));
        r = r.push_json(json!({ "report_type" : self.report_type }));
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn webhook(mut self, webhook: &str) -> Self {
        self.webhook = Some(webhook.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreditRelayRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub relay_token: String,
}
impl<'a> CreditRelayRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditRelayRemoveResponse> {
        let mut r = self.client.client.post("/credit/relay/remove");
        r = r.push_json(json!({ "relay_token" : self.relay_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxBankTransferFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub webhook: String,
}
impl<'a> SandboxBankTransferFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxBankTransferFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/bank_transfer/fire_webhook");
        r = r.push_json(json!({ "webhook" : self.webhook }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxIncomeFireWebhookRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub item_id: String,
    pub user_id: Option<String>,
    pub webhook: String,
    pub verification_status: String,
}
impl<'a> SandboxIncomeFireWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxIncomeFireWebhookResponse> {
        let mut r = self.client.client.post("/sandbox/income/fire_webhook");
        r = r.push_json(json!({ "item_id" : self.item_id }));
        if let Some(ref unwrapped) = self.user_id {
            r = r.push_json(json!({ "user_id" : unwrapped }));
        }
        r = r.push_json(json!({ "webhook" : self.webhook }));
        r = r.push_json(json!({ "verification_status" : self.verification_status }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SandboxOauthSelectAccountsRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub oauth_state_id: String,
    pub accounts: Vec<String>,
}
impl<'a> SandboxOauthSelectAccountsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SandboxOauthSelectAccountsResponse> {
        let mut r = self.client.client.post("/sandbox/oauth/select_accounts");
        r = r.push_json(json!({ "oauth_state_id" : self.oauth_state_id }));
        r = r.push_json(json!({ "accounts" : self.accounts }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SignalEvaluateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
    pub client_transaction_id: String,
    pub amount: f64,
    pub user_present: Option<bool>,
    pub client_user_id: Option<String>,
    pub user: Option<SignalUser>,
    pub device: Option<SignalDevice>,
}
impl<'a> SignalEvaluateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SignalEvaluateResponse> {
        let mut r = self.client.client.post("/signal/evaluate");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "account_id" : self.account_id }));
        r = r.push_json(json!({ "client_transaction_id" : self.client_transaction_id }));
        r = r.push_json(json!({ "amount" : self.amount }));
        if let Some(ref unwrapped) = self.user_present {
            r = r.push_json(json!({ "user_present" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.push_json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user {
            r = r.push_json(json!({ "user" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.device {
            r = r.push_json(json!({ "device" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn user_present(mut self, user_present: bool) -> Self {
        self.user_present = Some(user_present);
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn user(mut self, user: SignalUser) -> Self {
        self.user = Some(user);
        self
    }
    pub fn device(mut self, device: SignalDevice) -> Self {
        self.device = Some(device);
        self
    }
}
pub struct SignalEvaluateRequired<'a> {
    pub access_token: &'a str,
    pub account_id: &'a str,
    pub client_transaction_id: &'a str,
    pub amount: f64,
}
impl<'a> SignalEvaluateRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SignalDecisionReportRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_transaction_id: String,
    pub initiated: bool,
    pub days_funds_on_hold: Option<i64>,
}
impl<'a> SignalDecisionReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SignalDecisionReportResponse> {
        let mut r = self.client.client.post("/signal/decision/report");
        r = r.push_json(json!({ "client_transaction_id" : self.client_transaction_id }));
        r = r.push_json(json!({ "initiated" : self.initiated }));
        if let Some(ref unwrapped) = self.days_funds_on_hold {
            r = r.push_json(json!({ "days_funds_on_hold" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn days_funds_on_hold(mut self, days_funds_on_hold: i64) -> Self {
        self.days_funds_on_hold = Some(days_funds_on_hold);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SignalReturnReportRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub client_transaction_id: String,
    pub return_code: String,
}
impl<'a> SignalReturnReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SignalReturnReportResponse> {
        let mut r = self.client.client.post("/signal/return/report");
        r = r.push_json(json!({ "client_transaction_id" : self.client_transaction_id }));
        r = r.push_json(json!({ "return_code" : self.return_code }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SignalPrepareRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> SignalPrepareRequest<'a> {
    pub async fn send(self) -> anyhow::Result<SignalPrepareResponse> {
        let mut r = self.client.client.post("/signal/prepare");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WalletCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub iso_currency_code: String,
}
impl<'a> WalletCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WalletCreateResponse> {
        let mut r = self.client.client.post("/wallet/create");
        r = r.push_json(json!({ "iso_currency_code" : self.iso_currency_code }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WalletGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub wallet_id: String,
}
impl<'a> WalletGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WalletGetResponse> {
        let mut r = self.client.client.post("/wallet/get");
        r = r.push_json(json!({ "wallet_id" : self.wallet_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WalletListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub iso_currency_code: Option<String>,
    pub cursor: Option<String>,
    pub count: Option<i64>,
}
impl<'a> WalletListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WalletListResponse> {
        let mut r = self.client.client.post("/wallet/list");
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.push_json(json!({ "iso_currency_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WalletTransactionExecuteRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub idempotency_key: String,
    pub wallet_id: String,
    pub counterparty: WalletTransactionCounterparty,
    pub amount: WalletTransactionAmount,
    pub reference: String,
}
impl<'a> WalletTransactionExecuteRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WalletTransactionExecuteResponse> {
        let mut r = self.client.client.post("/wallet/transaction/execute");
        r = r.push_json(json!({ "idempotency_key" : self.idempotency_key }));
        r = r.push_json(json!({ "wallet_id" : self.wallet_id }));
        r = r.push_json(json!({ "counterparty" : self.counterparty }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "reference" : self.reference }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct WalletTransactionExecuteRequired<'a> {
    pub idempotency_key: &'a str,
    pub wallet_id: &'a str,
    pub counterparty: WalletTransactionCounterparty,
    pub amount: WalletTransactionAmount,
    pub reference: &'a str,
}
impl<'a> WalletTransactionExecuteRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WalletTransactionGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub transaction_id: String,
}
impl<'a> WalletTransactionGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WalletTransactionGetResponse> {
        let mut r = self.client.client.post("/wallet/transaction/get");
        r = r.push_json(json!({ "transaction_id" : self.transaction_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct WalletTransactionsListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub wallet_id: String,
    pub cursor: Option<String>,
    pub count: Option<i64>,
}
impl<'a> WalletTransactionsListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WalletTransactionsListResponse> {
        let mut r = self.client.client.post("/wallet/transactions/list");
        r = r.push_json(json!({ "wallet_id" : self.wallet_id }));
        if let Some(ref unwrapped) = self.cursor {
            r = r.push_json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_json(json!({ "count" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsEnhanceRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub account_type: String,
    pub transactions: Vec<ClientProvidedRawTransaction>,
}
impl<'a> TransactionsEnhanceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsEnhanceGetResponse> {
        let mut r = self.client.client.post("/beta/transactions/v1/enhance");
        r = r.push_json(json!({ "account_type" : self.account_type }));
        r = r.push_json(json!({ "transactions" : self.transactions }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsRulesCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub personal_finance_category: String,
    pub rule_details: TransactionsRuleDetails,
}
impl<'a> TransactionsRulesCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsRulesCreateResponse> {
        let mut r = self.client.client.post("/beta/transactions/rules/v1/create");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r
            .push_json(
                json!({ "personal_finance_category" : self.personal_finance_category }),
            );
        r = r.push_json(json!({ "rule_details" : self.rule_details }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsRulesListRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
}
impl<'a> TransactionsRulesListRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsRulesListResponse> {
        let mut r = self.client.client.post("/beta/transactions/rules/v1/list");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct TransactionsRulesRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub access_token: String,
    pub rule_id: String,
}
impl<'a> TransactionsRulesRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TransactionsRulesRemoveResponse> {
        let mut r = self.client.client.post("/beta/transactions/rules/v1/remove");
        r = r.push_json(json!({ "access_token" : self.access_token }));
        r = r.push_json(json!({ "rule_id" : self.rule_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentProfileCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
}
impl<'a> PaymentProfileCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentProfileCreateResponse> {
        let mut r = self.client.client.post("/payment_profile/create");
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentProfileGetRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_profile_id: String,
}
impl<'a> PaymentProfileGetRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentProfileGetResponse> {
        let mut r = self.client.client.post("/payment_profile/get");
        r = r.push_json(json!({ "payment_profile_id" : self.payment_profile_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PaymentProfileRemoveRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub payment_profile_id: String,
}
impl<'a> PaymentProfileRemoveRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PaymentProfileRemoveResponse> {
        let mut r = self.client.client.post("/payment_profile/remove");
        r = r.push_json(json!({ "payment_profile_id" : self.payment_profile_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PartnerCustomersCreateRequest<'a> {
    pub(crate) client: &'a PlaidClient,
    pub company_name: String,
    pub is_diligence_attested: bool,
    pub products: Vec<String>,
    pub create_link_customization: Option<bool>,
}
impl<'a> PartnerCustomersCreateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PartnerCustomersCreateResponse> {
        let mut r = self.client.client.post("/beta/partner/v1/customers/create");
        r = r.push_json(json!({ "company_name" : self.company_name }));
        r = r.push_json(json!({ "is_diligence_attested" : self.is_diligence_attested }));
        r = r.push_json(json!({ "products" : self.products }));
        if let Some(ref unwrapped) = self.create_link_customization {
            r = r.push_json(json!({ "create_link_customization" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn create_link_customization(mut self, create_link_customization: bool) -> Self {
        self.create_link_customization = Some(create_link_customization);
        self
    }
}
