use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Traffic {
    pub up: u64,
    pub down: u64,
}
