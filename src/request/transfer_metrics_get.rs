use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferMetricsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub originator_client_id: Option<String>,
}
impl<'a> TransferMetricsGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TransferMetricsGetResponse> {
        let mut r = self.http_client.client.post("/transfer/metrics/get");
        if let Some(ref unwrapped) = self.originator_client_id {
            r = r.json(json!({ "originator_client_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferMetricsGetRequest<'a> {
    type Output = httpclient::InMemoryResult<TransferMetricsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}