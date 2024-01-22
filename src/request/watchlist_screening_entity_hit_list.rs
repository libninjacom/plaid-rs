use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_hit_list`].

On request success, this will return a [`WatchlistScreeningEntityHitListResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityHitListRequest {
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl WatchlistScreeningEntityHitListRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityHitListRequest> {
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.params.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityHitListRequest> {
    type Output = httpclient::InMemoryResult<WatchlistScreeningEntityHitListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/hit/list";
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