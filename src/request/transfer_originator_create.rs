use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_originator_create`].

On request success, this will return a [`TransferOriginatorCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOriginatorCreateRequest {
    pub company_name: String,
}
impl TransferOriginatorCreateRequest {}
impl FluentRequest<'_, TransferOriginatorCreateRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferOriginatorCreateRequest> {
    type Output = httpclient::InMemoryResult<TransferOriginatorCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/transfer/originator/create";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}