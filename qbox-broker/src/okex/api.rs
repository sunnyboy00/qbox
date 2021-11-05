use anyhow::{self, Context, Result};
use chrono::prelude::*;
use parking_lot::Once;
use qbox_core::counter::*;
use rust_decimal::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use tungstenite::Message;
use url::Url;

static QUOTES_ADDR: &str = "wss://real.okex.com:8443/ws/v3?compress=true";

#[derive(Debug, Clone)]
pub struct Okex {
    uri: Url,
    apikey: String,
    secretkey: String,
    passphrase: String,
    zone: String,
}

unsafe impl Send for Okex {}
unsafe impl Sync for Okex {}

impl Okex {
    pub fn get_instance(uri: Url) -> Okex {
        static mut INSTANCE: Option<Mutex<Okex>> = None;
        unsafe {
            INSTANCE
                .get_or_insert_with(|| {
                    println!("init"); // do once
                    Mutex::new(Okex::new(uri))
                })
                .lock()
                .unwrap()
                .clone()
        }
    }

    pub fn new(uri: Url) -> Self {
        let apikey = uri.username().into();
        let secretkey = uri.password().unwrap_or_default().into();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        let passphrase = query
            .get("passphrase")
            .unwrap_or(&String::from(""))
            .to_owned();
        let zone = query.get("zone").unwrap_or(&String::from("")).to_owned();
        Self {
            uri,
            apikey,
            secretkey,
            passphrase,
            zone,
        }
    }

    pub fn subscribe<F>(&self, filter: &[Instrument]) {
        static START: Once = Once::new();
        START.call_once(|| {
            std::thread::spawn(move || {
                println!("启动订阅线程");
                fn csymbol(instr: &Instrument) -> Message {
                    let channel = instr.security_id;
                    let text = json!({
                        "op":"subscribe",
                        "args":[channel]
                    });
                    Message::from(text.to_string())
                }

                loop {
                    println!("连接服务器 {}", QUOTES_ADDR);
                    if let Ok((mut stream, response)) = tungstenite::connect(QUOTES_ADDR) {
                        if response.status().is_success() {
                            let submsg: Vec<Message> =
                                filter.iter().map(|kind| csymbol(kind)).collect();
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
                                    match decode(&b) {
                                        Ok(quotes) => {
                                            quotes.into_iter().for_each(|ev| {
                                                let _ = qbox_core::quotes_event(ev);
                                            });
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

impl Counter for Okex {}

impl Quotes for Okex {
    fn subscribe(&self, filter: &[Instrument]) {
        self.subscribe(filter)
    }
    fn unsubscribe(&self, filter: &[Instrument]) {}
}

impl Trades for Okex {
    fn name(&self) -> &'static str {
        "okex"
    }
    //查结算货币
    fn currencies(&self) -> Vec<Currency> {
        vec![]
    }
    //查证券
    fn instruments(&self, filter: &[&str]) -> Vec<Instrument> {
        vec![]
    }
    //查账户
    fn accounts(&self, filter: &[&str]) {}
    //查时区
    fn timezone(&mut self, zone: &'static str) -> String {
        ""
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

fn decode(b: &Vec<u8>) -> Result<Vec<QuoteEvent>> {
    //println!("decode {}", String::from_utf8_lossy(&b[..]));
    //{"table":"spot/ticker","data":[{"instrument_id":"ETH-USDT","last":"8.8","best_bid":"3","best_ask":"8.1","open_24h":"5.1","high_24h":"8.8","low_24h":"3",
    //"base_volume_24h":"13.77340909","quote_volume_24h":"78.49886361","timestamp":"2018-12-20T03:13:41.664Z"}]}
    let mut ret = vec![];
    if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&b[..]) {
        let ch: Vec<&str> = v["table"].as_str().context("error")?.split('/').collect();
        //QuotesKind::from("okex:spot:ticker:btc/usdt")
        if ch[1] == "ticker" {
            for tick in v["data"].as_array().context("error")? {
                let symbol = tick["instrument_id"]
                    .as_str()
                    .context("error")?
                    .replace("-", "/");
                let instrument =
                    Instrument::from_str(format!("okex:{}:ticker:{}", ch[0], symbol).as_str())
                        .unwrap();
                let open = Decimal::from_str(tick["open_24h"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let high = Decimal::from_str(tick["high_24h"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let low = Decimal::from_str(tick["low_24h"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let last = Decimal::from_str(tick["last"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let last_quantity = Decimal::from_str(tick["last_qty"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let bid = Decimal::from_str(tick["best_bid"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let ask = Decimal::from_str(tick["best_ask"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let time = tick["timestamp"]
                    .as_str()
                    .context("error")?
                    .parse::<DateTime<Utc>>()?
                    .timestamp_millis();
                let ask_quantity =
                    Decimal::from_str(tick["best_ask_size"].as_str().context("error")?)?
                        .to_f64()
                        .context("error")?;
                let bid_quantity =
                    Decimal::from_str(tick["best_bid_size"].as_str().context("error")?)?
                        .to_f64()
                        .context("error")?;
                let volume = Decimal::from_str(tick["base_volume_24h"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let turnover =
                    Decimal::from_str(tick["quote_volume_24h"].as_str().context("error")?)?
                        .to_f64()
                        .context("error")?;
                ret.push(QuoteEvent::Level1(Level1 {
                    instrument,
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
            }
        } else if ch[1] == "trade" {
            //decode {"table":"spot/trade","data":[{"side":"buy","trade_id":"96559756","price":"9235","size":"0.00104423","instrument_id":"BTC-USDT","timestamp":"2020-07-02T03:53:03.431Z"}]}
            for tick in v["data"].as_array().context("error")? {
                let symbol = tick["instrument_id"]
                    .as_str()
                    .context("error")?
                    .replace("-", "/");
                let instrument =
                    Instrument::from_str(format!("okex:{}:ttt:{}", ch[0], symbol).as_str())
                        .unwrap();
                let price = Decimal::from_str(tick["price"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let quantity = Decimal::from_str(tick["size"].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let time = tick["timestamp"]
                    .as_str()
                    .context("error")?
                    .parse::<DateTime<Utc>>()?
                    .timestamp_millis();
                let side = if tick["side"].as_str().context("error")? == "buy" {
                    Side::Buy //买方主动成交
                } else {
                    Side::Sell //卖放主动成交
                };
                let id = tick["trade_id"].to_string(); //str_to_static_str(tick["trade_id"].as_str().context("error")?);
                ret.push(QuoteEvent::TickToTrade(TickToTrade {
                    instrument,
                    id,
                    price,
                    quantity,
                    time,
                    into_side: Some(side),
                    order_side: None,
                    take_order_id: None,
                    make_order_id: None,
                }));
            }
        } else if ch[1].starts_with("candle") {
            for bar in v["data"].as_array().context("error")? {
                let symbol = bar["instrument_id"]
                    .as_str()
                    .context("error")?
                    .replace("-", "/");
                let instrument =
                    Instrument::from_str(format!("okex:{}:bar:{}", ch[0], symbol).as_str())
                        .unwrap();
                let bar = bar["candle"].as_array().context("error")?;
                let time = bar[0]
                    .as_str()
                    .context("error")?
                    .parse::<DateTime<Utc>>()?
                    .timestamp_millis();
                let open = Decimal::from_str(bar[1].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let high = Decimal::from_str(bar[2].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let low = Decimal::from_str(bar[3].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let close = Decimal::from_str(bar[4].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                let volume = Decimal::from_str(bar[5].as_str().context("error")?)?
                    .to_f64()
                    .context("error")?;
                ret.push(QuoteEvent::Bar(Bar {
                    instrument,
                    time: time.to_string(),
                    open,
                    close,
                    low,
                    high,
                    volume,
                    turnover: None,
                }));
            }
        } else if ch[1].starts_with("depth") {
            for depth in v["data"].as_array().context("data error")? {
                let symbol = depth["instrument_id"]
                    .as_str()
                    .context("instrument_id error")?
                    .replace("-", "/");
                let instrument =
                    Instrument::from_str(format!("okex:{}:depth:{}", ch[0], symbol).as_str())
                        .unwrap();
                let time = depth["timestamp"]
                    .as_str()
                    .context("timestamp error")?
                    .parse::<DateTime<Utc>>()?
                    .timestamp_millis();
                let mut bids = vec![];
                for bid in depth["bids"].as_array().context("bids error")? {
                    let bid = bid.as_array().context("bid error")?;
                    bids.push([
                        Decimal::from_str(bid[0].as_str().context("bid[0] error")?)?
                            .to_f64()
                            .context("bid[0] error")?,
                        Decimal::from_str(bid[1].as_str().context("bid[1] error")?)?
                            .to_f64()
                            .context("bid[1] error")?,
                        Decimal::from_str(bid[2].as_str().context("bid[1] error")?)?
                            .to_f64()
                            .context("bid[2] error")?,
                        0.0,
                    ])
                }
                let mut asks = vec![];
                for ask in depth["asks"].as_array().context("asks error")? {
                    let ask = ask.as_array().context("ask error")?;
                    asks.push([
                        Decimal::from_str(ask[0].as_str().context("ask[0] error")?)?
                            .to_f64()
                            .context("ask[0] error")?,
                        Decimal::from_str(ask[1].as_str().context("ask[1] error")?)?
                            .to_f64()
                            .context("ask[1] error")?,
                        Decimal::from_str(ask[2].as_str().context("ask[1] error")?)?
                            .to_f64()
                            .context("ask[2] error")?,
                        0.0,
                    ])
                }
                ret.push(QuoteEvent::Level2(Level2 {
                    instrument,
                    time,
                    bids,
                    asks,
                    raw: Some(b.clone()),
                }));
            }
        }
    }
    Ok(ret)
}
