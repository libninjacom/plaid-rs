use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_repayment_return_list`].

On request success, this will return a [`TransferRepaymentReturnListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListRequest {
    pub count: Option<i64>,
    pub offset: Option<i64>,
    pub repayment_id: String,
}
impl TransferRepaymentReturnListRequest {}
impl FluentRequest<'_, TransferRepaymentReturnListRequest> {
    pub fn count(mut self, count: i64) -> Self {
        self.params.count = Some(count);
        self
    }
    pub fn offset(mut self, offset: i64) -> Self {
        self.params.offset = Some(offset);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferRepaymentReturnListRequest> {
    type Output = httpclient::InMemoryResult<TransferRepaymentReturnListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/repayment/return/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.count {
                r = r.json(json!({ "count" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.offset {
                r = r.json(json!({ "offset" : unwrapped }));
            }
            r = r.json(json!({ "repayment_id" : self.params.repayment_id }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}