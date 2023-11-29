use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRecurringListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub count: Option<i64>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub funding_account_id: Option<String>,
    pub offset: Option<i64>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
}
impl<'a> TransferRecurringListRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferRecurringListResponse> {
        let mut r = self.http_client.client.post("/transfer/recurring/list");
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.end_time {
            r = r.json(json!({ "end_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.funding_account_id {
            r = r.json(json!({ "funding_account_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_time {
            r = r.json(json!({ "start_time" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn end_time(mut self, end_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_time = Some(end_time);
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
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_time = Some(start_time);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferRecurringListRequest<'a> {
    type Output = crate::Result<TransferRecurringListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
