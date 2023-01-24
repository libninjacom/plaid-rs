use serde_json::json;
use crate::model::*;
use crate::PlaidClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningEntityHistoryListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub entity_watchlist_screening_id: String,
    pub cursor: Option<String>,
}
impl<'a> WatchlistScreeningEntityHistoryListRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaginatedEntityWatchlistScreeningListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/entity/history/list");
        r = r
            .json(
                json!(
                    { "entity_watchlist_screening_id" : self
                    .entity_watchlist_screening_id }
                ),
            );
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningEntityHistoryListRequest<'a> {
    type Output = httpclient::InMemoryResult<
        PaginatedEntityWatchlistScreeningListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
