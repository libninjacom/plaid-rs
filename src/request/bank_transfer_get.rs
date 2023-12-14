use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::bank_transfer_get`].

On request success, this will return a [`BankTransferGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferGetRequest {
    pub bank_transfer_id: String,
}
impl BankTransferGetRequest {}
impl FluentRequest<'_, BankTransferGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferGetRequest> {
    type Output = httpclient::InMemoryResult<BankTransferGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/bank_transfer/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}