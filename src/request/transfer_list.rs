use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub count: Option<i64>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub funding_account_id: Option<String>,
    pub offset: Option<i64>,
    pub origination_account_id: Option<String>,
    pub originator_client_id: Option<String>,
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
impl<'a> TransferListRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferListResponse> {
        let mut r = self.http_client.client.post("/transfer/list");
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.json(json!({ "end_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.funding_account_id {
            r = r.json(json!({ "funding_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.origination_account_id {
            r = r.json(json!({ "origination_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.originator_client_id {
            r = r.json(json!({ "originator_client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.json(json!({ "start_date" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn end_date(mut self, end_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }
    pub fn funding_account_id(mut self, funding_account_id: &str) -> Self {
        self.funding_account_id = Some(funding_account_id.to_owned());
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn origination_account_id(mut self, origination_account_id: &str) -> Self {
        self.origination_account_id = Some(origination_account_id.to_owned());
        self
    }
    pub fn originator_client_id(mut self, originator_client_id: &str) -> Self {
        self.originator_client_id = Some(originator_client_id.to_owned());
        self
    }
    pub fn start_date(mut self, start_date: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferListRequest<'a> {
    type Output = crate::Result<TransferListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
