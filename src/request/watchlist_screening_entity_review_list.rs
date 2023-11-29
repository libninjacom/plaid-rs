use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningEntityReviewListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub cursor: Option<String>,
    pub entity_watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningEntityReviewListRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningEntityReviewListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/entity/review/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = r.json(json!(
            { "entity_watchlist_screening_id" : self
            .entity_watchlist_screening_id }
        ));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningEntityReviewListRequest<'a> {
    type Output = crate::Result<WatchlistScreeningEntityReviewListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
