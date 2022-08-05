
use std::{path::PathBuf, marker::PhantomData, ffi::c_int, time::Duration};

use crate::{config::Config, cli::{Args, Command, WGPUBackend, WGPUPower}};

#[cfg(feature = "winit")]
pub mod winit;

mod builder;
pub use builder::*;
use nix::{poll::{PollFd, PollFlags}, errno::Errno};
use wayland_server::Display;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Socket(#[from] SocketError)
}

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub sock_id: usize,
    pub display: Display,
    #[cfg(feature = "winit")]
    pub winit: Option<winit::WinitState>,
}

macro_rules! set_env {
    ($key:expr, $val:expr) => {
        tracing::trace!("env {} = {}", $key, $val);
        std::env::set_var($key, $val);
    };
}

impl Server {
    pub fn socket_dir() -> PathBuf {
        std::env::var("XDG_RUNTIME_DIR").unwrap().into()
    }

    pub fn socket_name(&self) -> String {
        format!("{}-{}", &self.name, self.sock_id)
    }

    pub fn socket_path(&self) -> PathBuf {
        format!("{}/{}", Self::socket_dir().to_str().unwrap(), self.socket_name()).into()
    }
}

impl Server {
    fn set_env_vars(&self) {
        let sock_name = self.socket_name();
        set_env!("WAYLAND_DISPLAY", &sock_name);
    }

    fn register_signal_handlers(&self) -> Result<(), ServerError> {
        tracing::trace!("Registering signal handlers...");
        Ok(())
    }

    #[tracing::instrument]
    pub fn run(&mut self) -> Result<(), ServerError> {

        self.set_env_vars();

        let mut socket = WaySocket::new(&mut self.display, None)?; // name provided by WAYLAND_DISPLAY
                                                                   // env var
        tracing::info!("Created listen socket at {}", self.socket_path().to_str().unwrap());

        self.register_signal_handlers()?;

        tracing::trace!("Entering server event loop...");
        loop {
            match socket.poll(None) {
                Err(SocketError::NotReady) => {
                    tracing::debug!("event stream not yet ready; retrying...");
                    continue
                },
                Err(SocketError::Interrupt) => {
                    tracing::info!("received interrupting signal during event polling");
                    break
                },
                Err(e) => return Err(e.into()),
                Ok(_) => {
                    tracing::trace!("received events");
                    self.display.dispatch(Duration::new(0, 0), &mut ())?;
                    tracing::trace!("flushing client events...");
                    self.display.flush_clients(&mut ());
                }
                _ => todo!(),
            }
        }
        Ok(())
    }

}

#[derive(Debug, thiserror::Error)]
pub enum SocketError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("poll() timed out while awaiting new events")]
    Timeout,
    #[error("poll() failed recoverably")]
    NotReady,
    #[error("poll() interrupted")]
    Interrupt
}

#[derive(Debug)]
pub struct WaySocket {
    fd: PollFd,
}

impl WaySocket {
    fn new(display: &mut Display, name: Option<&str>) -> Result<Self, SocketError> {
        display.add_socket(name)?; // name provided by WAYLAND_DISPLAY variable
        Ok(Self {
            fd: PollFd::new(display.get_poll_fd(), PollFlags::POLLIN | PollFlags::POLLPRI),
        })
    }

    fn poll(&mut self, timeout_ms: Option<c_int>) -> Result<(), SocketError> {
        debug_assert!(timeout_ms != Some(0), "poll() will always return immediately when called with timeout = 0");
        match nix::poll::poll(&mut [self.fd], timeout_ms.unwrap_or(-1)) {
            Ok(0) => Err(SocketError::Timeout),
            Err(Errno::EAGAIN) => Err(SocketError::NotReady),
            Err(Errno::EINTR) => Err(SocketError::Interrupt),
            Err(Errno::EINVAL) => unreachable!("nix library internal error"),
            Err(e) => unreachable!("poll() shouldn't produce this error code: {}", e),
            Ok(_i) => Ok(())
        }
    }
}
