use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, clap::Parser)]
pub struct TuiOpt {
    #[clap(about = "Interval between requests", default_value = "5")]
    #[default = 5.0]
    pub interval: f32,
}
