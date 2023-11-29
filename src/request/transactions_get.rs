use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub end_date: chrono::NaiveDate,
    pub options: Option<TransactionsGetRequestOptions>,
    pub start_date: chrono::NaiveDate,
}
impl<'a> TransactionsGetRequest<'a> {
    pub async fn send(self) -> crate::Result<TransactionsGetResponse> {
        let mut r = self.http_client.client.post("/transactions/get");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "end_date" : self.end_date }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = r.json(json!({ "start_date" : self.start_date }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: TransactionsGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsGetRequest<'a> {
    type Output = crate::Result<TransactionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
