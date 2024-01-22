use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::transfer_recurring_cancel`].

On request success, this will return a [`TransferRecurringCancelResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecurringCancelRequest {
    pub recurring_transfer_id: String,
}
impl TransferRecurringCancelRequest {}
impl FluentRequest<'_, TransferRecurringCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, TransferRecurringCancelRequest> {
    type Output = httpclient::InMemoryResult<TransferRecurringCancelResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/transfer/recurring/cancel";
            let mut r = self.client.client.post(url);
            r = r
                .json(
                    json!(
                        { "recurring_transfer_id" : self.params.recurring_transfer_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}