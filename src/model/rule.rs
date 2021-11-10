use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(
    feature = "cli",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive, serialize_all = "lowercase")
)]
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
