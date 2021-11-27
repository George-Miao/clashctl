pub enum Action {
    TestLatency { proxies: Vec<String> },
    ApplySelection { group: String, proxy: String },
}
