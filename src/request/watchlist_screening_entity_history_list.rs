use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_history_list`].

On request success, this will return a [`WatchlistScreeningEntityHistoryListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityHistoryListRequest {
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl WatchlistScreeningEntityHistoryListRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityHistoryListRequest> {
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityHistoryListRequest> {
    type Output = httpclient::InMemoryResult<
        WatchlistScreeningEntityHistoryListResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/history/list";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.cursor {
                r = r.json(json!({ "cursor" : unwrapped }));
            }
            r = r
                .json(
                    json!(
                        { "entity_watchlist_screening_id" : self.params
                        .entity_watchlist_screening_id }
                    ),
                );
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}