use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::payment_initiation_recipient_list`].

On request success, this will return a [`PaymentInitiationRecipientListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListRequest {}
impl PaymentInitiationRecipientListRequest {}
impl FluentRequest<'_, PaymentInitiationRecipientListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PaymentInitiationRecipientListRequest> {
    type Output = httpclient::InMemoryResult<PaymentInitiationRecipientListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/payment_initiation/recipient/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}