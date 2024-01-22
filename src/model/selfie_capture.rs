use serde::{Serialize, Deserialize};
///The image or video capture of a selfie. Only one of image or video URL will be populated per selfie.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelfieCapture {
    ///Temporary URL for downloading an image selfie capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    ///Temporary URL for downloading a video selfie capture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
}
impl std::fmt::Display for SelfieCapture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}