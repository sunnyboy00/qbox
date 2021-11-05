use super::{Event, CTP};
use anyhow::anyhow;
use anyhow::Result;
use crossbeam::channel::{self, Sender};
use ctp_rs::{ffi::*, Configuration, FromCBuf, Response, ResumeType, ToArray, TradeApi, TradeSpi};
use qbox_core::broker::*;
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use url::{Host, Url};

impl CTP {
    pub fn new_trade(uri: Url) -> Result<Self> {
        let params: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        let broker_id = params
            .get("broker_id")
            .unwrap_or(&String::from(""))
            .to_owned();
        let user_id = params
            .get("user_id")
            .unwrap_or(&String::from(""))
            .to_owned();
        let passwd = params.get("passwd").unwrap_or(&String::from("")).to_owned();
        let _investor_id = params
            .get("investor_id")
            .unwrap_or(&String::from(""))
            .to_owned();
        let appid = params.get("appid").unwrap_or(&String::from("")).to_owned();
        let auth_code = params
            .get("auth_code")
            .unwrap_or(&String::from(""))
            .to_owned();
        let host = uri.host().unwrap_or(Host::Domain("".into()));
        let port = uri.port().unwrap_or_default();
        let front_addr = format!("tcp://{}:{}", host, port);
        let work_path = Path::new(&qbox_core::get_exec_path()).join("ctp.trade");
        if !work_path.exists() {
            std::fs::create_dir_all(&work_path)?;
        }
        let (tx, rx) = channel::bounded(1);

        let tapi = TradeApi::new(work_path.to_str().unwrap())?
            .with_configuration(Configuration {
                broker_id,
                user_id,
                appid,
                auth_code,
                front_addr,
                passwd,
                ..Default::default()
            })
            .with_spi(TradeClient(tx));
        tapi.subscribe_public_topic(ResumeType::THOST_TERT_RESTART)?;
        tapi.subscribe_private_topic(ResumeType::THOST_TERT_RESTART)?;
        tapi.register_front()?;
        tapi.register_fens_user_info()?;
        tapi.init();
        tapi.authenticate()?;
        loop {
            match rx.recv()? {
                Event::Connected => {
                    tapi.login()?;
                }
                Event::Login(info) => {
                    return Ok(CTP {
                        quote: None,
                        login: Some(info),
                        trade: Some(tapi),
                    })
                }
                Event::Error(code, msg) => return Err(anyhow!("{} {}", code, msg)),
            }
        }
    }
}

struct TradeClient(Sender<Event>);

impl Deref for TradeClient {
    type Target = Sender<Event>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TradeSpi for TradeClient {
    fn on_connected(&self) {
        log::trace!("on_connected");
        let _ = self.send(Event::Connected);
    }

    fn on_disconnected(&self, reason: i32) {
        log::trace!("on_disconnected {}", reason);
        let _ = qbox_core::log(format!("on_disconnected 0x{:#04x}", reason));
    }

    fn on_error(&self, result: &Response) {
        log::trace!("on_error {:?}", result);
        let _ = qbox_core::log(format!("on_error {:?}", result));
    }
    ///登录请求响应
    fn on_user_login(&self, info: &CThostFtdcRspUserLoginField, result: &Response) {
        log::trace!("on_login {:?} {:?}", info, result);
        if result.code != 0 {
            let _ = self.send(Event::Error(result.code, result.message.clone()));
        } else {
            self.send(Event::Login(info.clone())).ok();
        }
    }

    fn on_user_password_update(&self, info: &CThostFtdcUserPasswordUpdateField, result: &Response) {
        let _ = qbox_core::log(format!("on_user_password_update {:?} {:?}", info, result));
    }

    ///请求查询合约响应
    fn on_qry_instrument(&self, instr: Option<&CThostFtdcInstrumentField>, result: &Response) {
        if result.code != 0 {
            let _ = qbox_core::log(format!("on_qry_instrument {:?} {:?}", instr, result));
            return;
        }
        log::trace!("on_qry_instrument {:?} {:?}", instr, result);

        if let Some(info) = instr {
            let instrument = Instrument::new()
                .with_exchange(Exchange::from(String::from_c_buf(&info.ExchangeID)))
                .with_secrity_id(String::from_c_buf(&info.InstrumentID))
                .with_symbol(String::from_c_buf(&info.InstrumentName))
                .with_kind(TradeKind::FUTURES)
                .with_multiplier(info.VolumeMultiple as usize)
                .with_item(
                    "CreateDate",
                    Value::String(String::from_c_buf(&info.CreateDate)),
                )
                .with_item(
                    "OpenDate",
                    Value::String(String::from_c_buf(&info.OpenDate)),
                )
                .with_item(
                    "ExpireDate",
                    Value::String(String::from_c_buf(&info.ExpireDate)),
                )
                .with_item("DeliveryYear", Value::I32(info.DeliveryYear))
                .with_item("DeliveryMonth", Value::I32(info.DeliveryMonth))
                .with_item(
                    "StartDelivDate",
                    Value::String(String::from_c_buf(&info.StartDelivDate)),
                )
                .with_item(
                    "EndDelivDate",
                    Value::String(String::from_c_buf(&info.EndDelivDate)),
                )
                .with_item("LongMarginRatio", Value::F64(info.LongMarginRatio))
                .with_item("ShortMarginRatio", Value::F64(info.ShortMarginRatio))
                .with_item("StrikePrice", Value::F64(info.StrikePrice))
                .with_item(
                    "MaxMarketOrderVolume",
                    Value::I32(info.MaxMarketOrderVolume),
                )
                .with_item(
                    "MinMarketOrderVolume",
                    Value::I32(info.MinMarketOrderVolume),
                )
                .with_item("MaxLimitOrderVolume", Value::I32(info.MaxLimitOrderVolume))
                .with_item("MinLimitOrderVolume", Value::I32(info.MinLimitOrderVolume))
                .with_item("PriceTick", Value::F64(info.PriceTick))
                .with_state(if info.IsTrading == 1 {
                    InstState::Trading
                } else if info.InstLifePhase == '0' as i8 {
                    InstState::NotStart
                } else if info.InstLifePhase == '1' as i8 {
                    InstState::Started
                } else if info.InstLifePhase == '2' as i8 {
                    InstState::Pause
                } else if info.InstLifePhase == '3' as i8 {
                    InstState::Expired
                } else {
                    InstState::Unknown
                });
            let ev = TradeEvent::InstrumentsResponse(instrument);
            let _ = qbox_core::query_event(ev);
        }
    }
}

impl Trades for CTP {
    fn name(&self) -> &'static str {
        "ctp"
    }
    //查结算货币
    fn currencies(&self) -> Vec<Currency> {
        vec![]
    }
    //查证券
    fn instruments(&self, filter: &[&str]) {
        log::trace!("instruments {:?}", filter);
        if let Some(tapi) = &self.trade {
            if filter.len() == 0 {
                let mut req = CThostFtdcQryInstrumentField::default();
                tapi.query_instrument(&mut req).ok();
            } else {
                filter.iter().for_each(|instr| {
                    let mut req = CThostFtdcQryInstrumentField::default();
                    req.InstrumentID = instr.into_array::<81>();
                    tapi.query_instrument(&mut req).ok();
                });
            }
        }
    }
    //查账户
    fn accounts(&self, filter: &[&str]) {}
    //查时区
    fn timezone(&mut self, zone: &'static str) -> String {
        "".to_string()
    }
    //报单
    fn offer(&self, order: Order) -> Result<Order> {
        unimplemented!()
    }
    //撤单
    fn cancel(&self, order: Order) -> Result<()> {
        unimplemented!()
    }
    //查订单
    fn query(&self, order: Order) -> Result<Order> {
        unimplemented!()
    }
    //查持仓
    fn positions(&self, after: &str, before: &str, limit: u8, filters: &[&str]) -> Vec<Position> {
        unimplemented!()
    }
}
