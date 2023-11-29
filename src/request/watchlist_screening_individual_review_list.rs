use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualReviewListRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub cursor: Option<String>,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualReviewListRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningIndividualReviewListResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/review/list");
        if let Some(ref unwrapped) = self.cursor {
            r = r.json(json!({ "cursor" : unwrapped }));
        }
        r = r.json(json!({ "watchlist_screening_id" : self.watchlist_screening_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualReviewListRequest<'a> {
    type Output = crate::Result<WatchlistScreeningIndividualReviewListResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
