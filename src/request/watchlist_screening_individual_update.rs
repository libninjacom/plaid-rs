use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct WatchlistScreeningIndividualUpdateRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub assignee: Option<String>,
    pub client_user_id: Option<String>,
    pub reset_fields: Option<Vec<String>>,
    pub search_terms: Option<UpdateIndividualScreeningRequestSearchTerms>,
    pub status: Option<String>,
    pub watchlist_screening_id: String,
}
impl<'a> WatchlistScreeningIndividualUpdateRequest<'a> {
    pub async fn send(self) -> crate::Result<WatchlistScreeningIndividualUpdateResponse> {
        let mut r = self
            .http_client
            .client
            .post("/watchlist_screening/individual/update");
        if let Some(ref unwrapped) = self.assignee {
            r = r.json(json!({ "assignee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.client_user_id {
            r = r.json(json!({ "client_user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.reset_fields {
            r = r.json(json!({ "reset_fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.search_terms {
            r = r.json(json!({ "search_terms" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.status {
            r = r.json(json!({ "status" : unwrapped }));
        }
        r = r.json(json!({ "watchlist_screening_id" : self.watchlist_screening_id }));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn assignee(mut self, assignee: &str) -> Self {
        self.assignee = Some(assignee.to_owned());
        self
    }
    pub fn client_user_id(mut self, client_user_id: &str) -> Self {
        self.client_user_id = Some(client_user_id.to_owned());
        self
    }
    pub fn reset_fields(mut self, reset_fields: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.reset_fields = Some(
            reset_fields
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect(),
        );
        self
    }
    pub fn search_terms(
        mut self,
        search_terms: UpdateIndividualScreeningRequestSearchTerms,
    ) -> Self {
        self.search_terms = Some(search_terms);
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for WatchlistScreeningIndividualUpdateRequest<'a> {
    type Output = crate::Result<WatchlistScreeningIndividualUpdateResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
