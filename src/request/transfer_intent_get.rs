use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_intent_get`].

On request success, this will return a [`TransferIntentGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferIntentGetRequest {
    pub transfer_intent_id: String,
}
impl TransferIntentGetRequest {}
impl FluentRequest<'_, TransferIntentGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferIntentGetRequest> {
    type Output = httpclient::InMemoryResult<TransferIntentGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/intent/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}