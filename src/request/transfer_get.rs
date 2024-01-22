use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_get`].

On request success, this will return a [`TransferGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferGetRequest {
    pub originator_client_id: Option<String>,
    pub transfer_id: String,
}
impl TransferGetRequest {}
impl FluentRequest<'_, TransferGetRequest> {
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransferGetRequest> {
    type Output = httpclient::InMemoryResult<TransferGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(json!({ "originator_client_id" : unwrapped }));
            }
            r = r.json(json!({ "transfer_id" : self.params.transfer_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}