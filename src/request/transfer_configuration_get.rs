use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_configuration_get`].

On request success, this will return a [`TransferConfigurationGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferConfigurationGetRequest {
    pub originator_client_id: Option<String>,
}
impl TransferConfigurationGetRequest {}
impl FluentRequest<'_, TransferConfigurationGetRequest> {
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.params.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferConfigurationGetRequest> {
    type Output = httpclient::InMemoryResult<TransferConfigurationGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/configuration/get";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.originator_client_id {
                r = r.json(json!({ "originator_client_id" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}