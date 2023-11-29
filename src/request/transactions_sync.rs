use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsSyncRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub options: Option<TransactionsSyncRequestOptions>,
}
impl<'a> TransactionsSyncRequest<'a> {
    pub async fn send(self) -> crate::Result<TransactionsSyncResponse> {
        let mut r = self.http_client.client.post("/transactions/sync");
        r = r.json(json!({ "access_token" : self.access_token }));
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.options {
            r = r.json(json!({ "options" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn options(mut self, options: TransactionsSyncRequestOptions) -> Self {
        self.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsSyncRequest<'a> {
    type Output = crate::Result<TransactionsSyncResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
