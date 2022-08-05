use std::path::PathBuf;

use clap::{Parser, value_parser, Subcommand, ValueEnum};
use directories::ProjectDirs;
use clap::builder::EnumValueParser;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, clap::ValueEnum)]
pub enum LogFormat {
    Compact,
    Full,
    Pretty,
    Json,
}

lazy_static::lazy_static! {
    pub static ref PROJECT_DIRS: ProjectDirs = directories::ProjectDirs::from("net", "Signal Garden", env!("CARGO_BIN_NAME")).expect("Couldn't get project directories");
    pub static ref DEFAULT_CONFIG_DIR: &'static str = {
        #[cfg(debug_assertions)]
        {"./config"}
        #[cfg(not(debug_assertions))]
        {PROJECT_DIRS.config_dir().to_str().unwrap()}
    };
}

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Args {
    #[clap(short, long, default_value = "warn,signal_wm=info", env = "RUST_LOG")]
    /// Logging output filters; comma-separated
    pub log_filter: String,
    #[clap(long, value_parser = value_parser!(LogFormat), default_value = "pretty")]
    /// Logging output format
    pub log_format: LogFormat,
    #[clap(short, long, default_value = &DEFAULT_CONFIG_DIR)]
    /// Path to the configuration directory
    pub config_dir: PathBuf,
    #[clap(subcommand)]
    /// Subcommand
    pub command: Command,
}

#[cfg(feature = "winit")]
mod platform {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
    pub enum Platform {
        /// Run as a client of another desktop manager, using Winit
        Winit,
        #[default]
        /// Run as a standalone wayland server
        Wayland
    }

impl clap::ValueEnum for Platform {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Winit, Self::Wayland]
    }
    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        Some(clap::PossibleValue::new(match self {
            Self::Winit => "winit",
            Self::Wayland => "wayland",
        }))
    }
}
}

#[cfg(feature = "winit")]
pub use platform::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum WGPUBackend {
    #[default]
    Vulkan,
    GL
}

impl ValueEnum for WGPUBackend {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Vulkan, Self::GL]
    }
    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        Some(clap::PossibleValue::new(match self {
            Self::Vulkan => "vulkan",
            Self::GL => "gl",
        }))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum WGPUPower {
    #[default]
    Low,
    High
}

impl ValueEnum for WGPUPower {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Low, Self::High]
    }
    fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
        Some(clap::PossibleValue::new(match self {
            Self::Low => "low",
            Self::High => "high",
        }))
    }
}

#[derive(Subcommand, Debug, Clone)]
#[clap()]
pub enum Command {
    #[clap()]
    /// Start the wayland server
    Start {
        #[cfg(feature = "winit")]
        #[clap(short = 'w', long, default_value = "wayland", default_missing_value = "winit", value_parser = EnumValueParser::<Platform>::new())]
        /// The platform to use for rendering & input handling
        platform: Platform,
        #[clap(short = 'b', long, default_value = "vulkan", value_parser = EnumValueParser::<WGPUBackend>::new(), env = "WGPU_BACKEND")]
        wgpu_backend: WGPUBackend,
        #[clap(short = 'a', long, env = "WGPU_ADAPTER_NAME")]
        wgpu_adapter: Option<String>,
        #[clap(short = 'p', long, default_value = "low", default_missing_value = "high", value_parser = EnumValueParser::<WGPUPower>::new(), env = "WGPU_POWER_PREF")]
        wgpu_power_preference: WGPUPower,
        #[clap(short, long)]
        /// Enable variable refresh rate, which, on supported hardware, prevents screen tearing,
        /// reduces input latency, and can be more energy efficient
        vfr: bool,
        #[clap(short = 'i', long)]
        /// ID with which to postfix the socket name
        socket_id: Option<usize>
    },
}
