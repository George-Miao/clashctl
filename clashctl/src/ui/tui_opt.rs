use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, clap::Parser)]
pub struct TuiOpt {
    #[clap(default_value = "5")]
    #[default = 5.0]
    /// Interval between requests
    pub interval: f32,
}
