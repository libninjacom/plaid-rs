use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub access_token: String,
    pub account_id: String,
}
impl<'a> ProcessorStripeBankAccountTokenCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<ProcessorStripeBankAccountTokenCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/processor/stripe/bank_account_token/create");
        r = r.json(json!({ "access_token" : self.access_token }));
        r = r.json(json!({ "account_id" : self.account_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
}
impl<'a> ::std::future::IntoFuture for ProcessorStripeBankAccountTokenCreateRequest<'a> {
    type Output = crate::Result<ProcessorStripeBankAccountTokenCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
