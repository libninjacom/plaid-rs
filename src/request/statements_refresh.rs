use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::statements_refresh`].

On request success, this will return a [`StatementsRefreshResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementsRefreshRequest {
    pub access_token: String,
    pub end_date: chrono::NaiveDate,
    pub start_date: chrono::NaiveDate,
}
impl StatementsRefreshRequest {}
impl FluentRequest<'_, StatementsRefreshRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, StatementsRefreshRequest> {
    type Output = httpclient::InMemoryResult<StatementsRefreshResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/statements/refresh";
            let mut r = self.client.client.post(url);
            r = r.json(json!({ "access_token" : self.params.access_token }));
            r = r.json(json!({ "end_date" : self.params.end_date }));
            r = r.json(json!({ "start_date" : self.params.start_date }));
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}