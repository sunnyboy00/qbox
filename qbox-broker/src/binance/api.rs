use anyhow::{self, Context, Result};
use chrono::prelude::*;
use parking_lot::Once;
use qbox_core::counter::*;
use rust_decimal::prelude::*;
use std::sync::Mutex;
use std::time::Duration;
use tungstenite::Message;
use url::Url;

static QUOTES_ADDR: &str = "wss://stream.binance.com:9443";

#[derive(Debug, Clone)]
pub struct Binance {
    url: String,
    apikey: String,
    secretkey: String,
    passphrase: String,
    zone: String,
}

unsafe impl Send for Binance {}
unsafe impl Sync for Binance {}

impl Binance {
    pub fn get_instance(url: &str) -> Binance {
        static mut INSTANCE: Option<Mutex<Binance>> = None;
        unsafe {
            INSTANCE
                .get_or_insert_with(|| {
                    // println!("init"); // do once
                    Mutex::new(Binance::new(url))
                })
                .lock()
                .unwrap()
                .clone()
        }
    }
    pub fn new(url: &str) -> Self {
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

    fn subscribe<F>(&self, filter: &[Instrument]) {
        static START: Once = Once::new();
        START.call_once(|| {
            std::thread::spawn(move || {
                println!("启动订阅线程");
                let chs: Vec<String> = filter
                    .iter()
                    .map(|kind: &Instrument| kind.to_string())
                    .collect();

                loop {
                    let addr = format!("{}/stream?streams={}", QUOTES_ADDR, chs.join("/"));
                    println!("连接服务器 {}", addr);
                    if let Ok((mut stream, response)) = tungstenite::connect(addr) {
                        if response.status().is_success() {
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
                                    match decode(&b, filter) {
                                        Ok(quotes) => {
                                            quotes
                                                .into_iter()
                                                .for_each(|ev| qbox_core::quotes_event(ev));
                                        }
                                        Err(err) => println!(
                                            "binance 解码错误 {:#?} raw {}",
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

impl Counter for Binance {}
// impl IAccount for Binance {
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

impl Trades for Binance {
    fn offer(&self, order: Order) -> Result<Order> {
        unimplemented!()
    }
    fn cancel(&self, id: &str) -> Result<()> {
        unimplemented!()
    }
    //查订单
    fn orders(&self, after: &str, before: &str, limit: u8, filters: &[&str]) -> Option<Vec<Order>> {
        unimplemented!()
    }
    //订单详情
    fn order(&self, id: &str, symbol: &str) -> Option<Order> {
        unimplemented!()
    }
    fn name(&self) -> &'static str {
        "BINANCE"
    }
    fn currencies(&self) -> Vec<Currency> {
        vec![]
    }
    fn instruments(&self) -> Vec<Instrument> {
        vec![]
    }
    fn timezone(&mut self, zone: &str) {
        self.zone = zone.into()
    }
    // //成交记录
    // fn tx_list(
    //     &self,
    //     after: &str,
    //     before: &str,
    //     limit: u8,
    //     accounts: &[&str],
    // ) -> Option<Vec<Order>> {
    //     unimplemented!()
    // }
}

impl Quotes for Binance {
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
    let mut ret = vec![];
    if let Ok(tick) = serde_json::from_slice::<serde_json::Value>(&b[..]) {
        let ch = tick["stream"].as_str().context("stream error")?;
        let tick = tick["data"].clone();
        let channel = find(ch);
        if channel.is_none() {
            return Ok(ret);
        }
        let channel = channel.unwrap();
        if ch.ends_with("@ticker") {
            let time = tick["E"].as_i64().context("E error")?;
            let open = Decimal::from_str(tick["o"].as_str().context("error")?)?
                .to_f64()
                .context("o error")?;
            let high = Decimal::from_str(tick["h"].as_str().context("error")?)?
                .to_f64()
                .context("h error")?;
            let low = Decimal::from_str(tick["l"].as_str().context("error")?)?
                .to_f64()
                .context("l error")?;
            let last = Decimal::from_str(tick["c"].as_str().context("error")?)?
                .to_f64()
                .context("c error")?;
            let last_quantity = Decimal::from_str(tick["Q"].as_str().context("error")?)?
                .to_f64()
                .context("Q error")?;
            let bid = Decimal::from_str(tick["b"].as_str().context("error")?)?
                .to_f64()
                .context("error")?;
            let bid_quantity = Decimal::from_str(tick["B"].as_str().context("error")?)?
                .to_f64()
                .context("error")?;
            let ask = Decimal::from_str(tick["a"].as_str().context("error")?)?
                .to_f64()
                .context("error")?;
            let ask_quantity = Decimal::from_str(tick["A"].as_str().context("error")?)?
                .to_f64()
                .context("error")?;
            let volume = Decimal::from_str(tick["v"].as_str().context("error")?)?
                .to_f64()
                .context("error")?;
            let turnover = Decimal::from_str(tick["q"].as_str().context("error")?)?
                .to_f64()
                .context("error")?;
            ret.push(Quotes::Level1(Level1 {
                channel,
                time,
                open,
                high,
                low,
                close: last,
                bids: vec![[bid, bid_quantity, f64::NAN, f64::NAN]],
                asks: vec![[ask, ask_quantity, f64::NAN, f64::NAN]],
                average: f64::NAN,
                last,
                last_quantity,
                volume,
                turnover,
            }));
        } else if ch.ends_with("@aggTrade") {
            let time = tick["T"].as_i64().context("T error")?;
            let price = Decimal::from_str(tick["p"].as_str().context("p error")?)?
                .to_f64()
                .context("p error")?;
            let quantity = Decimal::from_str(tick["q"].as_str().context("q error")?)?
                .to_f64()
                .context("q error")?;
            let side = if tick["m"].as_bool().context("m error")? {
                Side::Sell
            } else {
                Side::Buy
            };
            let id = tick["a"].as_u64().context("a error")?.to_string(); //str_to_static_str(&tick["a"].as_u64().context("a error")?.to_string());
            ret.push(Quotes::TickToTrade(TickToTrade {
                channel,
                id,
                price,
                quantity,
                time,
                into_side: Some(side),
                order_side: None,
                take_order_id: None,
                make_order_id: None,
            }));
        } else if ch.contains("@kline_") {
            let bar = tick["k"].clone();
            let time = bar["t"].as_i64().context("t error")?;
            let open = Decimal::from_str(bar["o"].as_str().context("o error")?)?
                .to_f64()
                .context("o error")?;
            let high = Decimal::from_str(bar["h"].as_str().context("h error")?)?
                .to_f64()
                .context("h error")?;
            let low = Decimal::from_str(bar["l"].as_str().context("l error")?)?
                .to_f64()
                .context("l error")?;
            let close = Decimal::from_str(bar["c"].as_str().context("c error")?)?
                .to_f64()
                .context("c error")?;
            let volume = Decimal::from_str(bar["v"].as_str().context("v error")?)?
                .to_f64()
                .context("v error")?;
            ret.push(Quotes::Bar(Bar {
                channel: Some(channel),
                time: time.to_string(),
                open,
                close,
                low,
                high,
                volume,
                turnover: None,
            }));
        } else if ch.contains("@depth") {
            let time = Local::now().timestamp_millis();
            let mut bids = vec![];
            for bid in tick["bids"].as_array().context("bids error")? {
                let bid = bid.as_array().context("bid error")?;
                bids.push([
                    Decimal::from_str(bid[0].as_str().context("c error")?)?
                        .to_f64()
                        .context("c error")?,
                    Decimal::from_str(bid[1].as_str().context("c error")?)?
                        .to_f64()
                        .context("c error")?,
                    0.0,
                    0.0,
                ])
            }
            let mut asks = vec![];
            for ask in tick["asks"].as_array().context("asks error")? {
                let ask = ask.as_array().context("ask error")?;
                asks.push([
                    Decimal::from_str(ask[0].as_str().context("c error")?)?
                        .to_f64()
                        .context("c error")?,
                    Decimal::from_str(ask[1].as_str().context("c error")?)?
                        .to_f64()
                        .context("c error")?,
                    0.0,
                    0.0,
                ])
            }
            ret.push(Quotes::Level2(Level2 {
                channel,
                time,
                bids,
                asks,
                raw: Some(b.clone()),
            }));
        }
    }
    Ok(ret)
}
