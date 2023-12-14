use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::bank_transfer_cancel`].

On request success, this will return a [`BankTransferCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferCancelRequest {
    pub bank_transfer_id: String,
}
impl BankTransferCancelRequest {}
impl FluentRequest<'_, BankTransferCancelRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, BankTransferCancelRequest> {
    type Output = httpclient::InMemoryResult<BankTransferCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/bank_transfer/cancel";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}