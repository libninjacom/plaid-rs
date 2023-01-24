use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DepositSwitchAltCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub target_account: DepositSwitchTargetAccount,
    pub target_user: DepositSwitchTargetUser,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub country_code: Option<String>,
}
impl<'a> DepositSwitchAltCreateRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DepositSwitchAltCreateResponse> {
        let mut r = self.http_client.client.post("/deposit_switch/alt/create");
        r = r.json(json!({ "target_account" : self.target_account }));
        r = r.json(json!({ "target_user" : self.target_user }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.country_code {
            r = r.json(json!({ "country_code" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
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
impl<'a> ::std::future::IntoFuture for DepositSwitchAltCreateRequest<'a> {
    type Output = httpclient::InMemoryResult<DepositSwitchAltCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
