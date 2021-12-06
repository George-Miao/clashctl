use smart_default::SmartDefault;

#[derive(Debug, SmartDefault)]
pub struct TuiOpt {
    #[default = 5.0]
    pub interval: f32,
}
