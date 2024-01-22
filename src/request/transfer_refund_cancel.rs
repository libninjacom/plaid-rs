use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_refund_cancel`].

On request success, this will return a [`TransferRefundCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundCancelRequest {
    pub refund_id: String,
}
impl TransferRefundCancelRequest {}
impl FluentRequest<'_, TransferRefundCancelRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRefundCancelRequest> {
    type Output = httpclient::InMemoryResult<TransferRefundCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/refund/cancel";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "refund_id" : self.params.refund_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}