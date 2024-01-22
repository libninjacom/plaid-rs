use serde::{Serialize, Deserialize};
///URLs for downloading original and cropped images for this document submission. The URLs are designed to only allow downloading, not hot linking, so the URL will only serve the document image for 60 seconds before expiring. The expiration time is 60 seconds after the `GET` request for the associated Identity Verification attempt. A new expiring URL is generated with each request, so you can always rerequest the Identity Verification attempt if one of your URLs expires.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhysicalDocumentImages {
    ///Temporary URL that expires after 60 seconds for downloading a cropped image containing just the back of the document. Might be null if the back of the document was not collected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cropped_back: Option<String>,
    ///Temporary URL that expires after 60 seconds for downloading a cropped image containing just the front of the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cropped_front: Option<String>,
    ///Temporary URL that expires after 60 seconds for downloading a crop of just the user's face from the document image. Might be null if the document does not contain a face photo.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face: Option<String>,
    ///Temporary URL that expires after 60 seconds for downloading the original image of the back of the document. Might be null if the back of the document was not collected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_back: Option<String>,
    ///Temporary URL that expires after 60 seconds for downloading the uncropped original image of the front of the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_front: Option<String>,
}
impl std::fmt::Display for PhysicalDocumentImages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}