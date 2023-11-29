use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WalletListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub count: Option<i64>,
    pub cursor: Option<String>,
    pub iso_currency_code: Option<String>,
}
impl<'a> WalletListRequest<'a> {
    pub async fn send(self) -> crate::Result<WalletListResponse> {
        let mut r = self.http_client.client.post("/wallet/list");
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.iso_currency_code {
            r = r.json(json!({ "iso_currency_code" : unwrapped }));
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
    pub fn iso_currency_code(mut self, iso_currency_code: &str) -> Self {
        self.iso_currency_code = Some(iso_currency_code.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WalletListRequest<'a> {
    type Output = crate::Result<WalletListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
