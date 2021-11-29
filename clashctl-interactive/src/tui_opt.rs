use smart_default::SmartDefault;

#[derive(Debug, SmartDefault)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
pub struct TuiOpt {
    #[default = 5.0]
    #[cfg_attr(feature = "cli", clap(about = "interval of requests"))]
    pub interval: f32,
}
