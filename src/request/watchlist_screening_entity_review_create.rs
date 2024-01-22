use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::watchlist_screening_entity_review_create`].

On request success, this will return a [`WatchlistScreeningEntityReviewCreateResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchlistScreeningEntityReviewCreateRequest {
    pub comment: Option<String>,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub entity_watchlist_screening_id: String,
}
impl WatchlistScreeningEntityReviewCreateRequest {}
impl FluentRequest<'_, WatchlistScreeningEntityReviewCreateRequest> {
    pub fn comment(mut self, comment: &str) -> Self {
        self.params.comment = Some(comment.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, WatchlistScreeningEntityReviewCreateRequest> {
    type Output = httpclient::InMemoryResult<
        WatchlistScreeningEntityReviewCreateResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/watchlist_screening/entity/review/create";
            let mut r = self.client.client.post(url);
            if let Some(ref unwrapped) = self.params.comment {
                r = r.json(json!({ "comment" : unwrapped }));
            }
            r = r.json(json!({ "confirmed_hits" : self.params.confirmed_hits }));
            r = r.json(json!({ "dismissed_hits" : self.params.dismissed_hits }));
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