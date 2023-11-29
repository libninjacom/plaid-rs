use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BankTransferBalanceGetRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub origination_account_id: Option<String>,
}
impl<'a> BankTransferBalanceGetRequest<'a> {
    pub async fn send(self) -> crate::Result<BankTransferBalanceGetResponse> {
        let mut r = self.http_client.client.post("/bank_transfer/balance/get");
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for BankTransferBalanceGetRequest<'a> {
    type Output = crate::Result<BankTransferBalanceGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
