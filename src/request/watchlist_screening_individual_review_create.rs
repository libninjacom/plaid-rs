use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualReviewCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub comment: Option<String>,
    pub confirmed_hits: Vec<String>,
    pub dismissed_hits: Vec<String>,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualReviewCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningIndividualReviewCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/review/create");
        if let Some(ref unwrapped) = self.comment {
            r = r.json(json!({ "comment" : unwrapped }));
        }
        r = r.json(json!({ "confirmed_hits" : self.confirmed_hits }));
        r = r.json(json!({ "dismissed_hits" : self.dismissed_hits }));
        r = r.json(json!({ "watchlist_screening_id" : self.watchlist_screening_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualReviewCreateRequest<'a> {
    type Output = crate::Result<WatchlistScreeningIndividualReviewCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
