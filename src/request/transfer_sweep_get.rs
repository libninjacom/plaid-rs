use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_sweep_get`].

On request success, this will return a [`TransferSweepGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferSweepGetRequest {
    pub sweep_id: String,
}
impl TransferSweepGetRequest {}
impl FluentRequest<'_, TransferSweepGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferSweepGetRequest> {
    type Output = httpclient::InMemoryResult<TransferSweepGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/sweep/get";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "sweep_id" : self.params.sweep_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}