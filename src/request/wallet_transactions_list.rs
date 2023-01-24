use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletTransactionsListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub wallet_id: String,
    pub cursor: Option<String>,
    pub count: Option<i64>,
}
impl<'a> WalletTransactionsListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<WalletTransactionsListResponse> {
        let mut r = self.http_client.client.post("/wallet/transactions/list");
        r = r.json(json!({ "wallet_id" : self.wallet_id }));
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
}
impl<'a> ::std::future::IntoFuture for WalletTransactionsListRequest<'a> {
    type Output = httpclient::InMemoryResult<WalletTransactionsListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
