use serde::{Serialize, Deserialize};
///Analysis of which fields matched between the originally reported Beacon User and the Beacon User that the report was syndicated to.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconReportSyndicationAnalysis {
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub address: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub date_of_birth: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub email_address: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub id_number: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub ip_address: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub name: String,
    /**An enum indicating the match type between two Beacon Users.


`match` indicates that the provided input data was a strong match against the other Beacon User.

`partial_match` indicates the data approximately matched the other Beacon User. For example, "Knope" vs. "Knope-Wyatt" for last name.

`no_match` indicates that Plaid was able to compare this field against the other Beacon User and it did not match the provided input data.

`no_data` indicates that Plaid was unable to compare this field against the original Beacon User because the field was not present in one of the Beacon Users.*/
    pub phone_number: String,
}
impl std::fmt::Display for BeaconReportSyndicationAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}