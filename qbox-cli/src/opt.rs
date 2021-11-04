use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use structopt::StructOpt;
use structopt_toml::StructOptToml;

#[derive(StructOpt, Debug, Clone, Deserialize, StructOptToml)]
#[serde(default)]
pub(crate) struct Opt {
    #[structopt(short = "f", default_value = "/etc/qbox/qbox.toml")]
    pub file: String,
    #[structopt(long, default_value = "info")]
    pub level: String,
    #[structopt(
        name = "quote_dsn",
        long,
        default_value = "ctp://180.168.146.187:10131/quotes?broker_id=9999&appid=simnow_client_test&auth_code=0000000000000000&user_id=0000&passwd=0000&udp=false&multicast=false"
    )]
    pub quote_dsn: String,
    #[structopt(
        name = "trade_dsn",
        long,
        default_value = "ctp://180.168.146.187:10130/trades?broker_id=9999&appid=simnow_client_test&auth_code=0000000000000000&user_id=0000&passwd=0000"
    )]
    pub trade_dsn: String,
}

impl Opt {
    pub(crate) fn from_file<P: AsRef<Path>>(p: P) -> Result<Opt> {
        let toml = std::fs::read_to_string(p)?;
        Opt::from_args_with_toml(toml.as_str())
    }
}
