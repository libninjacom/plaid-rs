use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DepositSwitchCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_access_token: String,
    pub target_account_id: String,
}
impl<'a> DepositSwitchCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<DepositSwitchCreateResponse> {
        let mut r = self.http_client.client.post("/deposit_switch/create");
        if let Some(ref unwrapped) = self.country_code {
            r = r.json(json!({ "country_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "target_access_token" : self.target_access_token }));
        r = r.json(json!({ "target_account_id" : self.target_account_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
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
impl<'a> ::std::future::IntoFuture for DepositSwitchCreateRequest<'a> {
    type Output = crate::Result<DepositSwitchCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
