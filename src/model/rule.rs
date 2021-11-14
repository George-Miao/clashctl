use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
// #[serde(rename_all = "UPPERCASE")]
#[cfg_attr(
    feature = "cli",
    derive(strum::EnumString, strum::Display, strum::EnumVariantNames),
    strum(ascii_case_insensitive)
)]
pub enum RuleType {
    Domain,
    DomainSuffix,
    DomainKeyword,
    GeoIP,
    IPCIDR,
    SrcIPCIDR,
    SrcPort,
    DstPort,
    Process,
    Match,
    Direct,
    Reject,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rule {
    #[serde(rename = "type")]
    rule_type: RuleType,
    payload: String,
    proxy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash)]
pub struct Rules {
    rules: Vec<Rule>,
}
