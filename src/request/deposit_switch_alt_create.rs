use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::deposit_switch_alt_create`].

On request success, this will return a [`DepositSwitchAltCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_account: DepositSwitchTargetAccount,
    pub target_user: DepositSwitchTargetUser,
}
impl DepositSwitchAltCreateRequest {}
impl FluentRequest<'_, DepositSwitchAltCreateRequest> {
    pub fn country_code(mut self, country_code: &str) -> Self {
        self.params.country_code = Some(country_code.to_owned());
        self
    }
    pub fn options(mut self, options: DepositSwitchCreateRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DepositSwitchAltCreateRequest> {
    type Output = httpclient::InMemoryResult<DepositSwitchAltCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/deposit_switch/alt/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.country_code {
                r = r.json(json!({ "country_code" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "target_account" : self.params.target_account }));
            r = r.json(json!({ "target_user" : self.params.target_user }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}