use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::model::Rule;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connections {
    connections: Vec<Connection>,
    download_total: u64,
    upload_total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    id: String,
    upload: u64,
    download: u64,
    metadata: Metadata,
    rule: Rule,
    rule_payload: String,
    start: DateTime<Local>,
    chains: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(rename = "destinationIP")]
    destination_ip: String,
    destination_port: String,
    host: String,
    network: String,
    #[serde(rename = "sourceIP")]
    source_ip: String,
    source_port: String,
    #[serde(rename = "type")]
    connection_type: String,
}
