use serde::{Serialize, Deserialize};
/**A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint.
Can be null if there are no transfers to include in a sweep.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimulatedTransferSweep {}
impl std::fmt::Display for SimulatedTransferSweep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}