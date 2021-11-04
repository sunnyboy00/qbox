mod ctp;
mod femas;
mod xtp;

use anyhow::Result;
use serde::{Deserialize, Serialize};

//加载柜台驱动
#[cfg(target_arch = "x86_64")]
pub fn load_counter() -> Result<()> {
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        log::debug!("加载CTP驱动");
        qbox_core::counter::register_driver(Counter::CTP.into(), ctp::CTPDriver::default())?;
    }
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        log::debug!("加载XTP驱动");
        qbox_core::counter::register_driver(Counter::XTP.into(), xtp::XTPDriver::default())?;
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        log::debug!("加载FEMAS驱动");
        qbox_core::counter::register_driver(Counter::FEMAS.into(), femas::FemasDriver::default())?;
    }
    Ok(())
}

#[doc = "柜台"]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Deserialize, Serialize)]
pub enum Counter {
    CTP,
    XTP,
    FEMAS,
    HUOBI,
    OKEX,
    BINANCE,
}

impl<S: AsRef<str>> From<S> for Counter {
    fn from(s: S) -> Self {
        let s = s.as_ref().to_lowercase();
        match s.as_str() {
            "ctp" => Counter::CTP,
            "xtp" => Counter::XTP,
            "femas" => Counter::FEMAS,
            "huobi" => Counter::HUOBI,
            "binance" => Counter::BINANCE,
            "okex" => Counter::OKEX,
            _ => panic!(""),
        }
    }
}

impl Into<&'static str> for Counter {
    fn into(self) -> &'static str {
        match self {
            Counter::CTP => "ctp",
            Counter::XTP => "xtp",
            Counter::FEMAS => "femas",
            Counter::HUOBI => "huobi",
            Counter::BINANCE => "binance",
            Counter::OKEX => "okex",
        }
    }
}

impl Into<String> for Counter {
    fn into(self) -> String {
        match self {
            Counter::CTP => "ctp",
            Counter::XTP => "xtp",
            Counter::FEMAS => "femas",
            Counter::HUOBI => "huobi",
            Counter::BINANCE => "binance",
            Counter::OKEX => "okex",
        }
        .into()
    }
}
