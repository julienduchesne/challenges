pub trait GroupConfig {
    fn name(&self) -> &'static str;
    fn url(&self) -> &'static str;
}
