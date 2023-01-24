use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningEntityListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub entity_watchlist_program_id: String,
    pub client_user_id: Option<String>,
    pub status: Option<String>,
    pub assignee: Option<String>,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaginatedEntityWatchlistScreeningListResponse> {
        let mut r = self.http_client.client.post("/watchlist_screening/entity/list");
        r = r
            .json(
                json!(
                    { "entity_watchlist_program_id" : self.entity_watchlist_program_id }
                ),
            );
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.json(json!({ "status" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.assignee {
            r = r.json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningEntityListRequest<'a> {
    type Output = httpclient::InMemoryResult<
        PaginatedEntityWatchlistScreeningListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
