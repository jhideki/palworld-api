use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BreedData {
    pub father: String,
    pub mother: String,
}
