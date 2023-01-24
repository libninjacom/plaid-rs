use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub options: Option<TransactionsGetRequestOptions>,
    pub access_token: String,
    pub start_date: String,
    pub end_date: String,
}
impl<'a> TransactionsGetRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TransactionsGetResponse> {
        let mut r = self.http_client.client.post("/transactions/get");
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "start_date" : self.start_date }));
        r = r.json(json!({ "end_date" : self.end_date }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn options(mut self, options: TransactionsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsGetRequest<'a> {
    type Output = httpclient::InMemoryResult<TransactionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
