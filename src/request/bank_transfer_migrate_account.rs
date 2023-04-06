use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct BankTransferMigrateAccountRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub account_number: String,
    pub account_type: String,
    pub routing_number: String,
    pub wire_routing_number: Option<String>,
}
impl<'a> BankTransferMigrateAccountRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BankTransferMigrateAccountResponse> {
        let mut r = self.http_client.client.post("/bank_transfer/migrate_account");
        r = r.json(json!({ "account_number" : self.account_number }));
        r = r.json(json!({ "account_type" : self.account_type }));
        r = r.json(json!({ "routing_number" : self.routing_number }));
        if let Some(ref unwrapped) = self.wire_routing_number {
            r = r.json(json!({ "wire_routing_number" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn wire_routing_number(mut self, wire_routing_number: &str) -> Self {
        self.wire_routing_number = Some(wire_routing_number.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for BankTransferMigrateAccountRequest<'a> {
    type Output = httpclient::InMemoryResult<BankTransferMigrateAccountResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}