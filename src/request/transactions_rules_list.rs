use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_rules_list`].

On request success, this will return a [`TransactionsRulesListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesListRequest {
    pub access_token: String,
}
impl TransactionsRulesListRequest {}
impl FluentRequest<'_, TransactionsRulesListRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, TransactionsRulesListRequest> {
    type Output = httpclient::InMemoryResult<TransactionsRulesListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/rules/v1/list";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}