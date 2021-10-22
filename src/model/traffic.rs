use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Traffic {
    up: u64,
    down: u64,
}
