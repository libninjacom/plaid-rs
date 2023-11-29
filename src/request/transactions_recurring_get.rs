use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsRecurringGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub account_ids: Vec<String>,
    pub options: Option<TransactionsRecurringGetRequestOptions>,
}
impl<'a> TransactionsRecurringGetRequest<'a> {
    pub async fn send(self) -> crate::Result<TransactionsRecurringGetResponse> {
        let mut r = self.http_client.client.post("/transactions/recurring/get");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "account_ids" : self.account_ids }));
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn options(mut self, options: TransactionsRecurringGetRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsRecurringGetRequest<'a> {
    type Output = crate::Result<TransactionsRecurringGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
