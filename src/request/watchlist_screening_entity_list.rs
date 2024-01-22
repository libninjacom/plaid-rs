use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_list`].

On request success, this will return a [`WatchlistScreeningEntityListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityListRequest {
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub cursor: Option<String>,
    pub entity_watchlist_program_id: String,
    pub status: Option<String>,
}
impl WatchlistScreeningEntityListRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityListRequest> {
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
for FluentRequest<'a, WatchlistScreeningEntityListRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningEntityListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.assignee {
                r = r.json(json!({ "assignee" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.client_user_id {
                r = r.json(json!({ "client_user_id" : unwrapped }));
            }
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            r = r
                .json(
                    json!(
                        { "entity_watchlist_program_id" : self.params
                        .entity_watchlist_program_id }
                    ),
                );
            if let Some(ref unwrapped) = self.params.status {
                r = r.json(json!({ "status" : unwrapped }));
            }
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}