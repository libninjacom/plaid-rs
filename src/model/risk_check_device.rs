use serde::{Serialize, Deserialize};
///Result summary object specifying values for `device` attributes of risk check.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RiskCheckDevice {
    /**An enum indicating whether a network proxy is present and if so what type it is.

`none_detected` indicates the user is not on a detectable proxy network.

`tor` indicates the user was using a Tor browser, which sends encrypted traffic on a decentralized network and is somewhat similar to a VPN (Virtual Private Network).

`vpn` indicates the user is on a VPN (Virtual Private Network)

`web_proxy` indicates the user is on a web proxy server, which may allow them to conceal information such as their IP address or other identifying information.

`public_proxy` indicates the user is on a public web proxy server, which is similar to a web proxy but can be shared by multiple users. This may allow multiple users to appear as if they have the same IP address for instance.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_proxy_type: Option<String>,
    ///Count of spam lists the IP address is associated with if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_spam_list_count: Option<i64>,
    ///UTC offset of the timezone associated with the IP address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_timezone_offset: Option<String>,
}
impl std::fmt::Display for RiskCheckDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}