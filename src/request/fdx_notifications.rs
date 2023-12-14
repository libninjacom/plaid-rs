use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::PlaidClient;
/**You should use this struct via [`PlaidClient::fdx_notifications`].

On request success, this will return a [`()`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdxNotificationsRequest {
    pub category: String,
    pub notification_id: String,
    pub notification_payload: FdxNotificationPayload,
    pub priority: Option<String>,
    pub publisher: Option<FdxParty>,
    pub sent_on: chrono::DateTime<chrono::Utc>,
    pub severity: Option<String>,
    pub subscriber: Option<FdxParty>,
    pub type_: String,
    pub url: Option<FdxHateoasLink>,
}
impl FdxNotificationsRequest {}
pub struct FdxNotificationsRequired<'a> {
    pub category: &'a str,
    pub notification_id: &'a str,
    pub notification_payload: FdxNotificationPayload,
    pub sent_on: chrono::DateTime<chrono::Utc>,
    pub type_: &'a str,
}
impl<'a> FdxNotificationsRequired<'a> {}
impl FluentRequest<'_, FdxNotificationsRequest> {
    pub fn priority(mut self, priority: &str) -> Self {
        self.params.priority = Some(priority.to_owned());
        self
    }
    pub fn publisher(mut self, publisher: FdxParty) -> Self {
        self.params.publisher = Some(publisher);
        self
    }
    pub fn severity(mut self, severity: &str) -> Self {
        self.params.severity = Some(severity.to_owned());
        self
    }
    pub fn subscriber(mut self, subscriber: FdxParty) -> Self {
        self.params.subscriber = Some(subscriber);
        self
    }
    pub fn url(mut self, url: FdxHateoasLink) -> Self {
        self.params.url = Some(url);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, FdxNotificationsRequest> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/fdx/notifications";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}