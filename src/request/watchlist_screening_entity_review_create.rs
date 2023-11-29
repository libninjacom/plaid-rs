use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningEntityReviewCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub comment: Option<String>,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub entity_watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningEntityReviewCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningEntityReviewCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/entity/review/create");
        if let Some(ref unwrapped) = self.comment {
            r = r.json(json!({ "comment" : unwrapped }));
        }
        r = r.json(json!({ "confirmed_hits" : self.confirmed_hits }));
        r = r.json(json!({ "dismissed_hits" : self.dismissed_hits }));
        r = r.json(json!(
            { "entity_watchlist_screening_id" : self
            .entity_watchlist_screening_id }
        ));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningEntityReviewCreateRequest<'a> {
    type Output = crate::Result<WatchlistScreeningEntityReviewCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
