use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_capabilities_get`].

On request success, this will return a [`TransferCapabilitiesGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferCapabilitiesGetRequest {
    pub access_token: String,
    pub account_id: String,
    pub payment_profile_token: Option<String>,
}
impl TransferCapabilitiesGetRequest {}
impl FluentRequest<'_, TransferCapabilitiesGetRequest> {
    pub fn payment_profile_token(mut self, payment_profile_token: &str) -> Self {
        self.params.payment_profile_token = Some(payment_profile_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferCapabilitiesGetRequest> {
    type Output = httpclient::InMemoryResult<TransferCapabilitiesGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/capabilities/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "account_id" : self.params.account_id }));
            if let Some(ref unwrapped) = self.params.payment_profile_token {
                r = r.json(json!({ "payment_profile_token" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}