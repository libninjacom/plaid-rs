use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_refund_create`].

On request success, this will return a [`TransferRefundCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundCreateRequest {
    pub amount: String,
    pub idempotency_key: String,
    pub transfer_id: String,
}
impl TransferRefundCreateRequest {}
impl FluentRequest<'_, TransferRefundCreateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRefundCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferRefundCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/refund/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}