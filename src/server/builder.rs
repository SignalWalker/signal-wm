use wayland_server::Display;

use crate::{cli::{WGPUBackend, WGPUPower, Command}, config::Config};

use super::Server;

#[cfg(feature = "winit")]
use crate::server::winit as s_winit;

#[cfg(feature = "winit")]
pub(crate) mod winit;

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[cfg(feature = "winit")]
    #[error(transparent)]
    Winit(#[from] s_winit::WinitError)
}

#[derive(Debug, Default)]
pub struct ServerBuilder<'cfg> {
    pub name: &'cfg str,
    pub sock_id: Option<usize>,
    #[cfg(feature = "winit")]
    pub winit: Option<winit::WinitConfig>
}

impl<'cfg> ServerBuilder<'cfg> {
    pub fn with_wgpu(self, backend: WGPUBackend, power: WGPUPower, adapter: Option<&str>) -> Result<Self, InitError> {
        todo!()
    }

    pub fn with_socket_id(mut self, id: Option<usize>) -> Self {
        self.sock_id = id;
        self
    }

    pub fn build(self) -> Result<Server, InitError> {
        Ok(Server {
            name: self.name.to_owned(),
            sock_id: match self.sock_id {
                Some(id) => id,
                None => todo!(),
            },
            display: Display::new(),
            #[cfg(feature = "winit")]
            winit: match self.winit {
                None => None,
                Some(w_cfg) => Some(s_winit::WinitState::from_cfg(self.name, w_cfg)?)
            },
        })
    }
}

impl Server {
    pub fn begin<'cfg>(args: &'cfg Command, cfg: &'cfg Config) -> Result<ServerBuilder<'cfg>, InitError> {
        let res = ServerBuilder {
            name: &cfg.server.name,
            sock_id: if let Command::Start { socket_id, .. } = args { *socket_id } else { None },
            ..Default::default()
        };
        #[cfg(feature = "winit")]
        match args {
            Command::Start { platform: crate::cli::Platform::Winit, .. } => return res.with_winit().map_err(Into::into),
            _ => {}
        }
        Ok(res)
    }
}
