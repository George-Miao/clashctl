use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct Traffic {
    pub up: u64,
    pub down: u64,
}
