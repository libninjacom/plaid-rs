use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningIndividualListRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub status: Option<String>,
    pub watchlist_program_id: String,
}
impl WatchlistScreeningIndividualListRequest {}
impl FluentRequest<'_, WatchlistScreeningIndividualListRequest> {
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.params.assignee = Some(assignee.to_owned());
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.params.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.params.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningIndividualListRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningIndividualListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/watchlist_screening/individual/list";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}