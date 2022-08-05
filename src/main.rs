use clap::Parser;
use cli::LogFormat;
use config::{Config, ConfigError};
use server::{Server, ServerError};
use time::{UtcOffset, macros::format_description};
use tracing_subscriber::fmt::time::OffsetTime;

mod cli;
mod config;
mod server;

fn initialize_tracing(args: &cli::Args) {
    let tsub = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_timer(OffsetTime::new(
            UtcOffset::current_local_offset().expect("couldn't get local time offset"),
            format_description!("[hour]:[minute]:[second]"),
        ))
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(&args.log_filter);

    match args.log_format {
        LogFormat::Compact => tsub.compact().init(),
        LogFormat::Full => tsub.init(),
        LogFormat::Pretty => tsub.pretty().init(),
        LogFormat::Json => tsub.json().init(),
    }
}

#[tracing::instrument]
fn initialize_config(args: &cli::Args) -> Result<Config, ConfigError> {
    let cfg_path = args.config_dir.join("config.toml");
    Config::from_path(&cfg_path).or_else(|err| match err {
        ConfigError::Io(err) => {
            tracing::warn!("could not access {cfg_path:?} ({err}); using default values");
            Ok(Config::default())
        }
        ConfigError::Deserialize(_) => Err(err),
    })
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    ServerInit(#[from] server::InitError),
    #[error(transparent)]
    Server(#[from] ServerError)
}

fn main() -> Result<(), Error> {
    let args = cli::Args::parse();
    initialize_tracing(&args);
    let cfg = initialize_config(&args)?;
    match &args.command {
        cli::Command::Start { .. } => {
            Server::begin(&args.command, &cfg)?.build()?.run().map_err(Into::into)
        },
        _ => { todo!() }
    }
}
