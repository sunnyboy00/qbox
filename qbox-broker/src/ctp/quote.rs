use super::{Event, CTP};
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, QuoteApi, QuoteSpi, Response};
use qbox_core::broker::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ops::Deref;
use std::path::Path;
use url::{Host, Url};

impl Quotes for CTP {
    fn subscribe(&self, filter: &[&str]) {
        if let Some(qapi) = &self.quote {
            let symbols: Vec<CString> = filter.iter().map(|x| CString::new(*x).unwrap()).collect();
            qapi.subscribe_market_data(&symbols).ok();
        }
    }
    fn unsubscribe(&self, filter: &[&str]) {
        if let Some(qapi) = &self.quote {
            let symbols: Vec<CString> = filter.iter().map(|x| CString::new(*x).unwrap()).collect();
            qapi.unsubscribe_market_data(&symbols).ok();
        }
    }
}

impl CTP {
    pub fn new_quote(uri: Url) -> Result<Self> {
        let params: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        let broker_id = params
            .get("broker_id")
            .unwrap_or(&String::from(""))
            .to_owned();
        let user_id = uri.username().to_owned();
        let passwd = uri.password().unwrap_or("").to_owned();
        let _investor_id = params
            .get("investor_id")
            .unwrap_or(&String::from(""))
            .to_owned();
        let appid = params.get("appid").unwrap_or(&String::from("")).to_owned();
        let auth_code = params
            .get("auth_code")
            .unwrap_or(&String::from(""))
            .to_owned();
        let host = uri.host_str().unwrap_or_default();
        let port = uri.port().unwrap_or_default();
        let front_addr = format!("tcp://{}:{}", host, port);
        let work_path = Path::new(
            &std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        )
        .join("ctp.quote");
        if !work_path.exists() {
            std::fs::create_dir_all(&work_path)?;
        }
        let udp: bool = params
            .get("udp")
            .unwrap_or(&String::from("false"))
            .parse()?;
        let multicast: bool = params
            .get("multicast")
            .unwrap_or(&String::from("false"))
            .parse()?;
        let (tx, rx) = channel::bounded(1);

        let qapi = QuoteApi::new(work_path.to_str().unwrap(), udp, multicast)?
            .with_configuration(Configuration {
                broker_id,
                user_id,
                appid,
                auth_code,
                front_addr,
                passwd,
                ..Default::default()
            })
            .with_spi(QuoteClient(tx));
        qapi.register_front()?;
        qapi.register_fens_user_info()?;
        qapi.init();
        loop {
            match rx.recv()? {
                Event::Connected => {
                    log::debug!("Event::Connected");
                    qapi.login()?;
                }
                Event::Login(info) => {
                    return Ok(CTP {
                        quote: Some(qapi),
                        login: Some(info),
                        trade: None,
                    })
                }
                Event::Error(code, msg) => return Err(anyhow!("{} {}", code, msg)),
            }
        }
    }
}

struct QuoteClient(Sender<Event>);

impl Deref for QuoteClient {
    type Target = Sender<Event>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl QuoteSpi for QuoteClient {
    fn on_connected(&self) {
        log::trace!("on_connected");
        let _ = self.send(Event::Connected);
    }

    fn on_disconnected(&self, reason: i32) {
        log::trace!("on_disconnected");
        let _ = qbox_core::log(format!("disconnected 0x{:#04x}", reason));
    }
    fn on_heartbeat_warning(&self, timelapse: i32) {
        log::trace!("on_heartbeat_warning");
        let _ = qbox_core::log(format!("heartbeat_warning {}", timelapse));
    }
    fn on_error(&self, result: &Response) {
        log::trace!("on_error {:?}", result);
        let _ = qbox_core::log(format!("error {:?}", result));
    }
    fn on_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::trace!("on_login {:?} {:?}", info, result);
        if result.code != 0 {
            self.send(Event::Error(result.code, result.message.clone()))
                .ok();
        } else {
            self.send(Event::Login(info.clone())).ok();
        }
    }
    fn on_sub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        log::trace!(
            "InstrumentID:{} {}",
            String::from_c_buf(&info.InstrumentID),
            result.code
        );
    }
    fn on_unsub_market_data(&self, info: &CThostFtdcSpecificInstrumentField, result: &Response) {
        if result.code != 0 {
            log::warn!("{:?} {:?}", info, result);
        }
    }
    fn on_depth_market_data(&self, q: &CThostFtdcDepthMarketDataField) {
        log::trace!("on_depth_market_data {:?}", q,);
        let exchange = Exchange::UNKNOWN;
        let trading_date = String::from_c_buf(&q.TradingDay);
        let action_date = String::from_c_buf(&q.ActionDay);
        let security_id = String::from_c_buf(&q.InstrumentID);
        let updatetime = String::from_c_buf(&q.UpdateTime);
        let time = NaiveTime::parse_from_str("%H:%M:%S", updatetime.as_str())
            .unwrap_or(Local::now().naive_local().time());
        let time = Local::now()
            .with_hour(time.hour())
            .unwrap()
            .with_minute(time.minute())
            .unwrap()
            .with_second(time.second())
            .unwrap()
            .timestamp();
        let ev = QuoteEvent::Level1(Level1 {
            exchange,
            trading_date,
            action_date,
            security_id,
            time,
            open: if q.OpenPrice != f64::MAX {
                q.OpenPrice
            } else {
                f64::NAN
            },
            high: if q.HighestPrice != f64::MAX {
                q.HighestPrice
            } else {
                f64::NAN
            },
            low: if q.LowestPrice != f64::MAX {
                q.LowestPrice
            } else {
                f64::NAN
            },
            close: if q.ClosePrice != f64::MAX {
                q.ClosePrice
            } else {
                q.LastPrice
            },
            bids: vec![
                (
                    if q.BidPrice1 != f64::MAX {
                        q.BidPrice1
                    } else {
                        f64::NAN
                    },
                    q.BidVolume1 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.BidPrice2 != f64::MAX {
                        q.BidPrice2
                    } else {
                        f64::NAN
                    },
                    q.BidVolume2 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.BidPrice3 != f64::MAX {
                        q.BidPrice3
                    } else {
                        f64::NAN
                    },
                    q.BidVolume3 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.BidPrice4 != f64::MAX {
                        q.BidPrice5
                    } else {
                        f64::NAN
                    },
                    q.BidVolume4 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.BidPrice5 != f64::MAX {
                        q.BidPrice5
                    } else {
                        f64::NAN
                    },
                    q.BidVolume5 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
            ],
            asks: vec![
                (
                    if q.AskPrice1 != f64::MAX {
                        q.AskPrice1
                    } else {
                        f64::NAN
                    },
                    q.AskVolume1 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.AskPrice2 != f64::MAX {
                        q.AskPrice2
                    } else {
                        f64::NAN
                    },
                    q.AskVolume2 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.AskPrice3 != f64::MAX {
                        q.AskPrice3
                    } else {
                        f64::NAN
                    },
                    q.AskVolume3 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.AskPrice4 != f64::MAX {
                        q.AskPrice4
                    } else {
                        f64::NAN
                    },
                    q.AskVolume4 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
                (
                    if q.AskPrice5 != f64::MAX {
                        q.AskPrice5
                    } else {
                        f64::NAN
                    },
                    q.AskVolume5 as f64,
                    f64::NAN,
                    f64::NAN,
                ),
            ],
            average: q.AveragePrice,
            last: q.LastPrice,
            last_quantity: f64::NAN,
            volume: q.Volume as f64,
            turnover: q.Turnover,
            score: 1.0,
        });
        let _ = qbox_core::quotes_event(ev);
    }
}
