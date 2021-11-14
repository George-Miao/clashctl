use serde::{Deserialize, Serialize};

use crate::model::{Rule, RuleType};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    pub connections: Vec<Connection>,
    pub download_total: u64,
    pub upload_total: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: String,
    pub upload: u64,
    pub download: u64,
    pub metadata: Metadata,
    pub rule: RuleType,
    pub rule_payload: String,
    pub start: String,
    pub chains: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "type")]
    pub connection_type: String,

    #[serde(rename = "sourceIP")]
    pub source_ip: String,
    pub source_port: String,

    #[serde(rename = "destinationIP")]
    pub destination_ip: String,
    pub destination_port: String,
    pub host: String,
    pub network: String,
}
