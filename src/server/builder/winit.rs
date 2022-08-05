use crate::server::winit::WinitError;

use super::ServerBuilder;


#[derive(Debug, Default)]
pub struct WinitConfig {

}

impl WinitConfig {
    pub fn new() -> Self {
        Default::default()
    }
}

impl ServerBuilder<'_> {
    pub fn with_winit(mut self) -> Result<Self, WinitError> {
        self.winit.replace(WinitConfig::new());
        Ok(self)
    }
}
