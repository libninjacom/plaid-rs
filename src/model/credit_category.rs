use serde::{Serialize, Deserialize};
/**Information describing the intent of the transaction. Most relevant for credit use cases, but not limited to such use cases.

See the [`taxonomy csv file`](https://plaid.com/documents/credit-category-taxonomy.csv) for a full list of credit categories.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditCategory {
    ///A granular category conveying the transaction's intent. This field can also be used as a unique identifier for the category.
    pub detailed: String,
    ///A high level category that communicates the broad category of the transaction.
    pub primary: String,
}
impl std::fmt::Display for CreditCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}