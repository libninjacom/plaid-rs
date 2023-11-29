use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct TransferRepaymentReturnListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub repayment_id: String,
}
impl<'a> TransferRepaymentReturnListRequest<'a> {
    pub async fn send(self) -> crate::Result<TransferRepaymentReturnListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/transfer/repayment/return/list");
        if let Some(ref unwrapped) = self.count {
            r = r.json(json!({ "count" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.offset {
            r = r.json(json!({ "offset" : unwrapped }));
        }
        r = r.json(json!({ "repayment_id" : self.repayment_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}
impl<'a> ::std::future::IntoFuture for TransferRepaymentReturnListRequest<'a> {
    type Output = crate::Result<TransferRepaymentReturnListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
