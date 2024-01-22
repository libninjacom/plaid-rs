use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::wallet_transaction_list`].

On request success, this will return a [`WalletTransactionListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionListRequest {
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub options: Option<WalletTransactionListRequestOptions>,
    pub wallet_id: String,
}
impl WalletTransactionListRequest {}
impl FluentRequest<'_, WalletTransactionListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    pub fn options(mut self, options: WalletTransactionListRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, WalletTransactionListRequest> {
    type Output = httpclient::InMemoryResult<WalletTransactionListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/wallet/transaction/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.options {
                r = r.json(json!({ "options" : unwrapped }));
            }
            r = r.json(json!({ "wallet_id" : self.params.wallet_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}