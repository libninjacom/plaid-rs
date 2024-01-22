use serde::{Serialize, Deserialize};
///A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwkPublicKey {
    ///The alg member identifies the cryptographic algorithm family used with the key.
    pub alg: String,
    ///The timestamp when the key was created, in Unix time.
    pub created_at: i64,
    ///The crv member identifies the cryptographic curve used with the key.
    pub crv: String,
    ///The timestamp when the key expired, in Unix time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<i64>,
    ///The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover.
    pub kid: String,
    ///The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC.
    pub kty: String,
    ///The use (public key use) parameter identifies the intended use of the public key.
    #[serde(rename = "use")]
    pub use_: String,
    ///The x member contains the x coordinate for the elliptic curve point, provided as a base64url-encoded string of the coordinate's big endian representation.
    pub x: String,
    ///The y member contains the y coordinate for the elliptic curve point, provided as a base64url-encoded string of the coordinate's big endian representation.
    pub y: String,
}
impl std::fmt::Display for JwkPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}