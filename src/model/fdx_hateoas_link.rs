use serde::{Serialize, Deserialize};
///REST application constraint (Hypermedia As The Engine Of Application State)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxHateoasLink {
    ///HTTP Method to use for the request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    ///URL to invoke the action on the resource
    pub href: String,
    ///Relation of this link to its containing entity, as defined by and with many example relation values at [IETF RFC5988](https://datatracker.ietf.org/doc/html/rfc5988)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    ///Content-types that can be used in the Accept header
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}
impl std::fmt::Display for FdxHateoasLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}