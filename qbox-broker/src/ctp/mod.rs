mod quote;
mod trade;

use anyhow::Result;
use ctp_rs::{ffi::*, QuoteApi, TradeApi};
use qbox_core::broker::{Counter, Driver};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::sync::Arc;
use url::Url;

#[derive(Debug)]
enum Event {
    Connected,
    Error(i32, String),
    Login(CThostFtdcRspUserLoginField),
}

#[derive(Default)]
pub struct CTP {
    quote: Option<QuoteApi>,
    trade: Option<TradeApi>,
    login: Option<CThostFtdcRspUserLoginField>,
}

impl Counter for CTP {
    fn stop(&self) -> Result<()> {
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct CTPDriver;

impl Driver for CTPDriver {
    fn create(&self, uri: Url) -> Result<Arc<dyn Counter>> {
        if uri.path() == "/quotes" {
            let ctp = CTP::new_quote(uri)?;
            return Ok(Arc::new(ctp));
        }
        if uri.path() == "/trades" {
            let ctp = CTP::new_trade(uri)?;
            return Ok(Arc::new(ctp));
        }
        Err(anyhow::anyhow!("unsupported uri {}", uri))
    }
}
