use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityProgramListRequest {
    pub cursor: Option<String>,
}
impl WatchlistScreeningEntityProgramListRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityProgramListRequest> {
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityProgramListRequest> {
    type Output = httpclient::InMemoryResult<
        WatchlistScreeningEntityProgramListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/watchlist_screening/entity/program/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}