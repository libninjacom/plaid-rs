use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_individual_list`].

On request success, this will return a [`WatchlistScreeningIndividualListResponse`].*/
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
        Box::pin(async move {
            let url = "/watchlist_screening/individual/list";
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
            if let Some(ref unwrapped) = self.params.status {
                r = r.json(json!({ "status" : unwrapped }));
            }
            r = r
                .json(
                    json!({ "watchlist_program_id" : self.params.watchlist_program_id }),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}