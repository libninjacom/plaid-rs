use crate::model::*;
use crate::PlaidClient;
use serde_json::json;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FdxNotificationsRequest<'a> {
    pub(crate) http_client: &'a PlaidClient,
    pub category: String,
    pub notification_id: String,
    pub notification_payload: FdxNotificationPayload,
    pub priority: Option<String>,
    pub publisher: FdxParty,
    pub sent_on: chrono::DateTime<chrono::Utc>,
    pub severity: Option<String>,
    pub subscriber: Option<FdxParty>,
    pub type_: String,
    pub url: Option<FdxHateoasLink>,
}
impl<'a> FdxNotificationsRequest<'a> {
    pub async fn send(self) -> crate::Result<()> {
        let mut r = self.http_client.client.post("/fdx/notifications");
        r = r.json(json!({ "category" : self.category }));
        r = r.json(json!({ "notificationId" : self.notification_id }));
        r = r.json(json!({ "notificationPayload" : self.notification_payload }));
        if let Some(ref unwrapped) = self.priority {
            r = r.json(json!({ "priority" : unwrapped }));
        }
        r = r.json(json!({ "publisher" : self.publisher }));
        r = r.json(json!({ "sentOn" : self.sent_on }));
        if let Some(ref unwrapped) = self.severity {
            r = r.json(json!({ "severity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subscriber {
            r = r.json(json!({ "subscriber" : unwrapped }));
        }
        r = r.json(json!({ "type" : self.type_ }));
        if let Some(ref unwrapped) = self.url {
            r = r.json(json!({ "url" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        Ok(res.json()?)
    }
    pub fn priority(mut self, priority: &str) -> Self {
        self.priority = Some(priority.to_owned());
        self
    }
    pub fn severity(mut self, severity: &str) -> Self {
        self.severity = Some(severity.to_owned());
        self
    }
    pub fn subscriber(mut self, subscriber: FdxParty) -> Self {
        self.subscriber = Some(subscriber);
        self
    }
    pub fn url(mut self, url: FdxHateoasLink) -> Self {
        self.url = Some(url);
        self
    }
}
pub struct FdxNotificationsRequired<'a> {
    pub category: &'a str,
    pub notification_id: &'a str,
    pub notification_payload: FdxNotificationPayload,
    pub publisher: FdxParty,
    pub sent_on: chrono::DateTime<chrono::Utc>,
    pub type_: &'a str,
}
impl<'a> FdxNotificationsRequired<'a> {}
impl<'a> ::std::future::IntoFuture for FdxNotificationsRequest<'a> {
    type Output = crate::Result<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
