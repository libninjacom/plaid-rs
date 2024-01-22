use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_recipient_get`].

On request success, this will return a [`PaymentInitiationRecipientGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetRequest {
    pub recipient_id: String,
}
impl PaymentInitiationRecipientGetRequest {}
impl FluentRequest<'_, PaymentInitiationRecipientGetRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationRecipientGetRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationRecipientGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/recipient/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "recipient_id" : self.params.recipient_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}