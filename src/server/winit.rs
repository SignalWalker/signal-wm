use winit::{event_loop::{EventLoopBuilder, EventLoop}, platform::unix::EventLoopBuilderExtUnix, window::{WindowBuilder, Window}};

use super::builder::winit::WinitConfig;


#[derive(Debug, thiserror::Error)]
pub enum WinitError {
    #[error(transparent)]
    OS(#[from] winit::error::OsError)
}

#[derive(Debug)]
pub struct WinitState {
    pub evloop: EventLoop<()>,
    pub window: Window
}

impl WinitState {
    #[tracing::instrument]
    pub fn from_cfg(title: &str, cfg: WinitConfig) -> Result<Self, WinitError> {
        let evloop = EventLoopBuilder::new().with_any_thread(true).build();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&evloop)?;
        Ok(WinitState {
            evloop,
            window
        })
    }
}
