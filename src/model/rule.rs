use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Rule {
    Domain,
    DomainSuffix,
    DomainKeyword,
    GeoIP,
    IpCidr,
    IpCidr6,
    SrcIpCidr,
    SrcPort,
    DstPort,
    ProcessName,
    Match,
    Direct,
    Reject,
}
