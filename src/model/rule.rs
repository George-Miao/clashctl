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
    pub rule_type: RuleType,
    pub payload: String,
    pub proxy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash)]
pub struct Rules {
    pub rules: Vec<Rule>,
}
