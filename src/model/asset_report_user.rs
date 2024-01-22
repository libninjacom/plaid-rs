use serde::{Serialize, Deserialize};
///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetReportUser {
    ///An identifier you determine and submit for the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_user_id: Option<String>,
    ///The user's email address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///The user's first name. Required for the Fannie Mae Day 1 Certainty™ program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///The user's last name.  Required for the Fannie Mae Day 1 Certainty™ program.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///The user's middle name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /**The user's Social Security Number. Required for the Fannie Mae Day 1 Certainty™ program.

Format: "ddd-dd-dddd"*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
}
impl std::fmt::Display for AssetReportUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}