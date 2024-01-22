use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transactions_rules_remove`].

On request success, this will return a [`TransactionsRulesRemoveResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionsRulesRemoveRequest {
    pub access_token: String,
    pub rule_id: String,
}
impl TransactionsRulesRemoveRequest {}
impl FluentRequest<'_, TransactionsRulesRemoveRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransactionsRulesRemoveRequest> {
    type Output = httpclient::InMemoryResult<TransactionsRulesRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/beta/transactions/rules/v1/remove";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "rule_id" : self.params.rule_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}