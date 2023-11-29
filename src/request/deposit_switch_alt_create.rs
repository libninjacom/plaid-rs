use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DepositSwitchAltCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub country_code: Option<String>,
    pub options: Option<DepositSwitchCreateRequestOptions>,
    pub target_account: DepositSwitchTargetAccount,
    pub target_user: DepositSwitchTargetUser,
}
impl<'a> DepositSwitchAltCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<DepositSwitchAltCreateResponse> {
        let mut r = self.http_client.client.post("/deposit_switch/alt/create");
        if let Some(ref unwrapped) = self.country_code {
            r = r.json(json!({ "country_code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "target_account" : self.target_account }));
        r = r.json(json!({ "target_user" : self.target_user }));
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
impl<'a> ::std::future::IntoFuture for DepositSwitchAltCreateRequest<'a> {
    type Output = crate::Result<DepositSwitchAltCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
