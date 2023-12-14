use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_refund_get`].

On request success, this will return a [`TransferRefundGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRefundGetRequest {
    pub refund_id: String,
}
impl TransferRefundGetRequest {}
impl FluentRequest<'_, TransferRefundGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferRefundGetRequest> {
    type Output = httpclient::InMemoryResult<TransferRefundGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/refund/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}