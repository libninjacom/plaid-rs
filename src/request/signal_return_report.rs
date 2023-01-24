use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct SignalReturnReportRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_transaction_id: String,
    pub return_code: String,
}
impl<'a> SignalReturnReportRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SignalReturnReportResponse> {
        let mut r = self.http_client.client.post("/signal/return/report");
        r = r.json(json!({ "client_transaction_id" : self.client_transaction_id }));
        r = r.json(json!({ "return_code" : self.return_code }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for SignalReturnReportRequest<'a> {
    type Output = httpclient::InMemoryResult<SignalReturnReportResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
