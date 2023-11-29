use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferCapabilitiesGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: Option<String>,
    pub account_id: Option<String>,
    pub payment_profile_token: Option<String>,
}
impl<'a> TransferCapabilitiesGetRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferCapabilitiesGetResponse> {
        let mut r = self.http_client.client.post("/transfer/capabilities/get");
        if let Some(ref unwrapped) = self.access_token {
            r = r.json(json!({ "access_token" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.account_id {
            r = r.json(json!({ "account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.payment_profile_token {
            r = r.json(json!({ "payment_profile_token" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn access_token(mut self, access_token: &str) -> Self {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn account_id(mut self, account_id: &str) -> Self {
        self.account_id = Some(account_id.to_owned());
        self
    }
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.payment_profile_token = Some(payment_profile_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferCapabilitiesGetRequest<'a> {
    type Output = crate::Result<TransferCapabilitiesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
