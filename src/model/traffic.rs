use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Traffic {
    pub up: u64,
    pub down: u64,
}
