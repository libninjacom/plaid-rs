
use serde::{Serialize, Deserialize};
use super::*;
#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
    #[serde(rename = "dashboard")]
    Dashboard,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "system")]
    System,
}