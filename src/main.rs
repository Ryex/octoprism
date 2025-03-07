use std::{str::FromStr, sync::Arc};

use clap::Parser;
use tracing::{debug, warn};
use tracing_subscriber::{Layer, filter, layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod rules;
mod types;

use tokio::signal::ctrl_c;
#[cfg(target_family = "unix")]
use tokio::signal::unix::{SignalKind, signal};
#[cfg(target_family = "windows")]
use tokio::signal::windows::ctrl_close;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, value_name = "FILE")]
    config: Option<String>,
    #[arg(long)]
    use_dotenv: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = CliArgs::try_parse()?;

    if args.use_dotenv {
        dotenvy::dotenv().ok();
    }

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Couldn't inittialize crypto provider");

    let config_path = match args.config {
        Some(path) => path,
        None => String::from("./config.toml"),
    };

    let config: Arc<config::Config> = Arc::new(config::Config::from_config(&config_path)?);

    let file_appender =
        tracing_appender::rolling::hourly(&config.debug_log.path, &config.debug_log.prefix);
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);
    let stdout_log = tracing_subscriber::fmt::layer().compact();

    let debug_log = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_level(true)
        .with_file(true)
        .with_line_number(true)
        .with_writer(non_blocking_file)
        .with_filter(filter::LevelFilter::from_level(
            tracing::Level::from_str(&config.debug_log.level).unwrap_or(tracing::Level::DEBUG),
        ));

    if config.debug_log.enable {
        tracing_subscriber::registry()
            .with(stdout_log.with_filter(filter::EnvFilter::from_default_env()))
            .with(debug_log)
            .init();
    } else {
        tracing_subscriber::registry()
            .with(stdout_log.with_filter(filter::EnvFilter::from_default_env()))
            .init();
    }

    debug!("Config: {:#?}", config);

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            #[cfg(target_family = "unix")]
            let mut sigterm = signal(SignalKind::terminate())?;
            #[cfg(target_family = "windows")]
            let mut sigterm = ctrl_close()?;

            tokio::select! {
                result = run() => result,
                _ = sigterm.recv() => {
                    handle_shutdown("Received SIGTERM");
                    std::process::exit(0);
                }
                _ = ctrl_c() => {
                    handle_shutdown("Interrupted");
                    std::process::exit(130);
                }
            }
        })
}

async fn run() -> eyre::Result<()> {
    use octocrab::{models, params};

    let octocrab = octocrab::instance();

    let mut page = octocrab
        .issues("Prismlauncher", "Prismlauncher")
        .list()
        .state(params::State::Open)
        .send()
        .await?;

    for item in page
        .into_iter()
        .map(std::convert::Into::<types::PullRequestOrIssue>::into)
    {
        println!(
            "Title: {} | Labels: {:?}",
            item.title().unwrap_or(&String::new()),
            item.labels()
                .unwrap_or(&vec![])
                .iter()
                .map(|l| &l.name).collect::<Vec<_>>()
        );
    }

    // println!("{:#?}", page);
    Ok(())
}

fn handle_shutdown(reason: &str) {
    warn!("{reason}! Shutting down bot...");
    println!("Everything is shutdown. Goodbye!");
}
