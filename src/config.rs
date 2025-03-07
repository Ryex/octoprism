use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DebugLogConfig {
    pub enable: bool,
    pub path: String,
    pub prefix: String,
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bind_address: String,
    pub debug_log: DebugLogConfig,
    pub rules: Vec<crate::rules::Rule>,
}

fn crate_name() -> &'static str {
    let path = module_path!();
    path.split_once("::").map_or(path, |(first, _)| first)
}

impl Config {
    pub fn from_config(path: &str) -> eyre::Result<Self> {
        let config = config::Config::builder()
            .set_default("bind_address", "127.0.0.1:8080")?
            .set_default("debug_log.enable", true)?
            .set_default("debug_log.path", "./logs")?
            .set_default("debug_log.prefix", "octoprism.log")?
            .set_default("debug_log.level", "debug")?
            .add_source(config::File::from(std::path::Path::new(path)).required(false))
            .add_source(config::Environment::with_prefix(crate_name()).separator("__"))
            .build()?;
        config.try_deserialize::<'_, Self>().map_err(Into::into)
    }
}
