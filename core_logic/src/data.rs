use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WttrData {
    pub temperature: i8,
}
