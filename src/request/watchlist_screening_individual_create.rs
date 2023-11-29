use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualCreateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub client_user_id: Option<String>,
    pub search_terms: WatchlistScreeningRequestSearchTerms,
}
impl<'a> WatchlistScreeningIndividualCreateRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningIndividualCreateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/create");
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        r = r.json(json!({ "search_terms" : self.search_terms }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualCreateRequest<'a> {
    type Output = crate::Result<WatchlistScreeningIndividualCreateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
