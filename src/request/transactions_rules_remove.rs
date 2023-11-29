use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransactionsRulesRemoveRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub rule_id: String,
}
impl<'a> TransactionsRulesRemoveRequest<'a> {
    pub async fn send(self) -> crate::Result<TransactionsRulesRemoveResponse> {
        let mut r = self
            .http_client
            .client
            .post("/beta/transactions/rules/v1/remove");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "rule_id" : self.rule_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for TransactionsRulesRemoveRequest<'a> {
    type Output = crate::Result<TransactionsRulesRemoveResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
