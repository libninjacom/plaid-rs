use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_originator_get`].

On request success, this will return a [`TransferOriginatorGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorGetRequest {
    pub originator_client_id: String,
}
impl TransferOriginatorGetRequest {}
impl FluentRequest<'_, TransferOriginatorGetRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferOriginatorGetRequest> {
    type Output = httpclient::InMemoryResult<TransferOriginatorGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/originator/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}