mod api;

use super::{FromStr, ToStr};
use anyhow::Result;
use qbox_core::counter::{Counter, Driver, Instrument};
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;
use std::sync::Arc;
use url::Url;

#[derive(Debug, Default)]
pub struct BinanceDriver;

impl Driver for BinanceDriver {
    fn create(&self, opt: Url) -> Result<Arc<dyn Counter>> {
        unimplemented!()
    }
}
