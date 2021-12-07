use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
// #[serde(rename_all = "UPPERCASE")]
#[cfg_attr(
    feature = "enum_ext",
    derive(
        strum::EnumString,
        strum::Display,
        strum::AsRefStr,
        strum::IntoStaticStr,
        strum::EnumVariantNames
    ),
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
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    #[serde(rename = "type")]
    pub rule_type: RuleType,
    pub payload: String,
    pub proxy: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rules {
    pub rules: Vec<Rule>,
}

impl Rules {
    pub fn most_frequent_proxy(&self) -> Option<&str> {
        self.frequency()
            .into_iter()
            .max_by_key(|(_, v)| *v)
            .map(|(k, _)| k)
    }

    pub fn frequency(&self) -> HashMap<&str, usize> {
        let mut counts = HashMap::new();
        self.rules
            .iter()
            .filter(|x| x.proxy != "DIRECT" && x.proxy != "REJECT")
            .map(|x| x.proxy.as_str())
            .for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }

    pub fn owned_frequency(&self) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        self.rules
            .iter()
            .filter(|x| x.proxy != "DIRECT" && x.proxy != "REJECT")
            .map(|x| x.proxy.to_owned())
            .for_each(|item| *counts.entry(item).or_default() += 1);
        counts
    }
}
