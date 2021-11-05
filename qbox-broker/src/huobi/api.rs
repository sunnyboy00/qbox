use super::{FromStr, ToStr};
use anyhow::{self, Context, Result};
use chrono::prelude::*;
use parking_lot::Once;
use qbox_core::counter::*;
use serde_json::json;
use std::sync::Mutex;
use std::time::Duration;
use tungstenite::Message;
use url::Url;

static QUOTES_ADDR: &str = "wss://api.huobi.pro/ws";
#[derive(Debug, Clone)]
pub struct Huobi {
    url: String,
    apikey: String,
    secretkey: String,
    passphrase: String,
    zone: String,
}

unsafe impl Send for Huobi {}
unsafe impl Sync for Huobi {}

impl Huobi {
    pub fn get_instance(url: &str) -> Huobi {
        static mut INSTANCE: Option<Mutex<Huobi>> = None;
        unsafe {
            INSTANCE
                .get_or_insert_with(|| Mutex::new(Huobi::new(url)))
                .lock()
                .unwrap()
                .clone()
        }
    }

    pub fn new(url: &str) -> Self {
        println!("new huobi");
        let dsn = Url::parse(url).expect("非法的火币API地址");
        let apikey = dsn.username();
        Self {
            url: url.into(),
            apikey: apikey.into(),
            secretkey: "".into(),
            passphrase: "".into(),
            // symbol: Symbol {
            //     symbol: "BTC/USDT",
            //     kind: TradeKind::SPOT,
            //     base: "BTC",            //基础货币
            //     base_precision: 0.001,  //基础货币精度
            //     quote: "USDT",          //计价货币
            //     quote_precision: 0.001, //计价货币精度
            //     state: 1,
            //     order_kinds: vec![],          //支持多交易种类
            //     tick_precision: [0.01, 0.01], //0为报价精度，1为数量精度
            // },
            zone: "".into(),
        }
    }

    fn subscribe<F>(&mut self, kinds: &[Instrument]) {
        static START: Once = Once::new();
        START.call_once(|| {
            std::thread::spawn(move || {
                println!("启动订阅线程");
                fn csymbol(instr: &Instrument) -> Message {
                    let channel: String = instr.to_string();
                    let now = Local::now();
                    let text = json!({
                        "sub":channel,
                        "id":now.format("%Y%m%d%H%M%S").to_string()
                    });
                    Message::from(text.to_string())
                }

                loop {
                    println!("连接服务器 {}", QUOTES_ADDR);
                    if let Ok((mut stream, response)) = tungstenite::connect(QUOTES_ADDR) {
                        if response.status().is_success() {
                            let submsg: Vec<Message> =
                                kinds.iter().map(|kind| csymbol(kind)).collect();
                            submsg
                                .into_iter()
                                .for_each(|msg| stream.write_message(msg).unwrap_or_default());

                            while let Ok(msg) = stream.read_message() {
                                if let Some(b) = match msg {
                                    Message::Binary(b) => {
                                        if let Ok(s) = String::from_utf8(b.clone()) {
                                            if s.contains("ping") {
                                                let s = s.replace("ping", "pong");
                                                stream
                                                    .write_message(Message::from(s))
                                                    .unwrap_or_default();
                                                None
                                            } else {
                                                Some(b)
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                    Message::Ping(b) => None,
                                    Message::Close(b) => None,
                                    Message::Pong(b) => None,
                                    Message::Text(str) => Some(str.into_bytes()),
                                } {
                                    match decode(&b, &kinds) {
                                        Ok(quotes) => {
                                            quotes
                                                .into_iter()
                                                .for_each(|ev| qbox_core::quotes_event(ev));
                                        }
                                        Err(err) => println!(
                                            "huobi 解码错误 {:#?} raw {}",
                                            err,
                                            String::from_utf8_lossy(&b[..])
                                        ),
                                    }
                                }
                            }
                        }
                    }
                    println!("失去服务器连接");
                    std::thread::sleep(Duration::from_secs(5));
                }
            });
        });
    }
}

impl Counter for Huobi {}

// impl IAccount for Huobi {
//     //查某个交易账户
//     fn account(&self, _account: &str) -> Option<Account> {
//         unimplemented!()
//     }
//     //用计价货币估值，可以是发布CNY，USD等，也可以是数字货币BTC，ETH等
//     fn valuation(&self, _account: Account, _currency: &str) -> Result<Account> {
//         unimplemented!()
//     }
//     //查某些账户
//     fn accounts(&self, accounts: Vec<&str>) -> Vec<Account> {
//         unimplemented!()
//     }
//     //内部转账 from=6:usdt,to=1:btc-usdt
//     fn transfer(&self, from: &str, to: &str, amount: &str, biz: &[u8]) -> Result<()> {
//         unimplemented!()
//     }
//     //deposit充值
//     fn deposit(&self, account: &str, amount: &str, bizid: &str, biz: &[u8]) -> Result<()> {
//         unimplemented!()
//     }
//     //withdraw提现
//     fn withdraw(&self, account: &str, amount: &str, bizid: &str, biz: &[u8]) -> Result<()> {
//         unimplemented!()
//     }
//     //账户流水
//     fn ledgers(
//         &self,
//         after: &str,
//         before: &str,
//         limit: u8,
//         accounts: &[&str],
//     ) -> Option<Vec<Ledger>> {
//         unimplemented!()
//     }
//     //账户流水明细
//     fn ledger(&self, account: &str, id: &str) -> Option<Ledger> {
//         unimplemented!()
//     }
// }

impl Trades for Huobi {
    fn offer(&self, order: Order) -> Result<Order> {
        unimplemented!()
    }
    fn cancel(&self, id: &str) -> Result<()> {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "HUOBI"
    }
    fn currencies(&self) -> Vec<Currency> {
        let mut ret = vec![];
        if let Ok(resp) = reqwest::blocking::get("https://api.huobi.pro/v1/common/currencys")
            .unwrap()
            .json::<serde_json::Value>()
        {
            //println!("{:#?}", resp);
            if let Some(curs) = resp["data"].as_array() {
                let v: Vec<Currency> = curs
                    .into_iter()
                    .map(|item| Currency::new(&item.to_string()))
                    .collect();
                ret = v;
            }
        }
        ret
    }
    fn instruments(&self) -> Vec<Instrument> {
        let mut ret = vec![];
        if let Ok(resp) = reqwest::blocking::get("https://api.huobi.pro/v1/common/symbols")
            .unwrap()
            .json::<serde_json::Value>()
        {
            if let Some(curs) = resp["data"].as_array() {
                let v: Vec<Instrument> = curs
                    .into_iter()
                    .map(|item| {
                        Instrument::default()
                            .with_exchange(Exchange::HUOBI)
                            .with_kind(TradeKind::SPOT)
                            .with_symbol(item["symbol"].to_string())
                            .with_secrity_id(item["symbol"].to_string())
                            .with_base(item["base-currency"].to_string())
                            .with_base_precision(
                                1.0 / item["value-precision"].as_f64().unwrap_or_default(),
                            )
                            .with_quote(item["quote-currency"].to_string())
                            .with_quote_precision(
                                1.0 / item["value-precision"].as_f64().unwrap_or_default(),
                            )
                            .with_margin_level(1)
                            .with_tick_precision((
                                1.0 / item["price-precision"].as_f64().unwrap_or_default(),
                                1.0 / item["amount-precision"].as_f64().unwrap_or_default(),
                            ))
                    })
                    .collect();
                ret = v;
            }
        }
        ret
    }
    fn timezone(&mut self, zone: &str) {
        self.zone = zone.into()
    }
}

impl Quotes for Huobi {
    fn subscribe<F>(&self, filter: &[Instrument]) {
        self.subscribe(filter)
    }
}

fn decode(b: &Vec<u8>, filter: &[Instrument]) -> Result<Vec<QuoteEvent>> {
    let find = |ch: &str| -> Option<Instrument> {
        for kind in filter {
            let k = kind.to_string();
            if ch == &k {
                return Some(kind.clone());
            }
        }
        None
    };
    if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&b[..]) {
        let ch = v["ch"].as_str().context("error")?.to_owned();
        let instrument = find(&ch).unwrap();
        if ch.starts_with("market.") && ch.ends_with(".bbo") {
            let ask = v["tick"]["ask"].as_f64().context("error")?;
            let ask_quantity = v["tick"]["askSize"].as_f64().context("error")?;
            let bid = v["tick"]["bid"].as_f64().context("error")?;
            let bid_quantity = v["tick"]["bidSize"].as_f64().context("error")?;
            let time = v["tick"]["quoteTime"].as_u64().context("error")? as i64;
            return Ok(vec![QuoteEvent::Level1(Level1 {
                instrument,
                time,
                open: f64::NAN,
                high: f64::NAN,
                low: f64::NAN,
                close: f64::NAN,
                bids: vec![[bid, bid_quantity, f64::NAN, f64::NAN]],
                asks: vec![[ask, ask_quantity, f64::NAN, f64::NAN]],
                average: f64::NAN,
                last: f64::NAN,
                last_quantity: f64::NAN,
                volume: f64::NAN,
                turnover: f64::NAN,
            })]);
        }
        if ch.starts_with("market.") && ch.ends_with(".trade.detail") {
            let data = v["tick"]["data"].as_array().context("error")?;
            let mut ret = vec![];
            for v in data {
                let id = v["tradeId"].as_u64().context("error")?.to_string(); //str_to_static_str(v["tradeId"].as_u64().context("error")?.to_string());
                let price = v["price"].as_f64().context("error")?;
                let quantity = v["amount"].as_f64().context("error")?;
                let time = v["ts"].as_u64().context("error")? as i64;
                let side = if v["direction"].as_str().context("error")? == "buy" {
                    Side::Buy
                } else {
                    Side::Sell
                };
                ret.push(QuoteEvent::TickToTrade(TickToTrade {
                    instrument: instrument.clone(),
                    id,
                    price,
                    quantity,
                    time,
                    into_side: Some(side),
                    order_side: None,
                    take_order_id: None,
                    make_order_id: None,
                }))
            }

            return Ok(ret);
        }
        if ch.starts_with("market.") && ch.contains(".kline.") {
            let time = v["ts"].as_u64().context("error")? as i64;
            let open = v["tick"]["open"].as_f64().context("error")?;
            let close = v["tick"]["close"].as_f64().context("error")?;
            let low = v["tick"]["low"].as_f64().context("error")?;
            let high = v["tick"]["high"].as_f64().context("error")?;
            let volume = v["tick"]["amount"].as_f64().context("error")?;
            let turnover = v["tick"]["vol"].as_f64().context("error")?;
            let size = v["tick"]["count"].as_u64().context("error")? as i64;
            return Ok(vec![QuoteEvent::Bar(Bar {
                instrument,
                time: time.to_string(),
                open,
                close,
                low,
                high,
                volume,
                turnover: Some(turnover),
            })]);
        }
        if ch.starts_with("market.") && ch.contains(".depth.") {
            let time = v["tick"]["ts"].as_u64().context("error")? as i64;
            let mut bids = vec![];
            for bid in v["tick"]["bids"].as_array().context("error")? {
                let bid = bid.as_array().context("error")?;
                bids.push([
                    bid[0].as_f64().context("error")?,
                    bid[1].as_f64().context("error")?,
                    0.0,
                    0.0,
                ])
            }
            let mut asks = vec![];
            for ask in v["tick"]["asks"].as_array().context("error")? {
                let ask = ask.as_array().context("error")?;
                asks.push([
                    ask[0].as_f64().context("error")?,
                    ask[1].as_f64().context("error")?,
                    0.0,
                    0.0,
                ])
            }

            return Ok(vec![QuoteEvent::Level2(Level2 {
                instrument,
                time,
                bids,
                asks,
                raw: Some(b.clone()),
            })]);
        }
    }
    Ok(vec![])
}
