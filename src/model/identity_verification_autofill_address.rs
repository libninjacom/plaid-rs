use serde::{Serialize, Deserialize};
/**Even if an address has been autofilled, some fields may be null depending on the region's addressing system. For example:

Addresses from the United Kingdom will not include a region

Addresses from Hong Kong will not include postal code*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationAutofillAddress {
    ///City from the end user's address
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///Valid, capitalized, two-letter ISO code representing the country of this object. Must be in ISO 3166-1 alpha-2 form.
    pub country: String,
    ///Field describing whether the associated address is a post office box. Will be `yes` when a P.O. box is detected, `no` when Plaid confirmed the address is not a P.O. box, and `no_data` when Plaid was not able to determine if the address is a P.O. box.
    pub po_box: String,
    ///The postal code for the associated address. Between 2 and 10 alphanumeric characters. For US-based addresses this must be 5 numeric digits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///An ISO 3166-2 subdivision code. Related terms would be "state", "province", "prefecture", "zone", "subdivision", etc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    ///The primary street portion of an address. If an address is provided, this field will always be filled.
    pub street: String,
    ///Extra street information, like an apartment or suite number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    /**Field describing whether the associated address is being used for commercial or residential purposes.

Note: This value will be `no_data` when Plaid does not have sufficient data to determine the address's use.*/
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for IdentityVerificationAutofillAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}