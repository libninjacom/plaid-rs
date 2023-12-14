use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::investments_transactions_get`].

On request success, this will return a [`InvestmentsTransactionsGetResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequest {
    pub access_token: String,
    pub end_date: chrono::NaiveDate,
    pub options: Option<InvestmentsTransactionsGetRequestOptions>,
    pub start_date: chrono::NaiveDate,
}
impl InvestmentsTransactionsGetRequest {}
impl FluentRequest<'_, InvestmentsTransactionsGetRequest> {
    pub fn options(mut self, options: InvestmentsTransactionsGetRequestOptions) -> Self {
        self.params.options = Some(options);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, InvestmentsTransactionsGetRequest> {
    type Output = httpclient::InMemoryResult<InvestmentsTransactionsGetResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/investments/transactions/get";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}