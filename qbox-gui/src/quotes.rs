use crate::counter::Exchange;
use crate::trade::*;
use serde::{Deserialize, Serialize};
use ta::{Close, High, Low, Open, Volume};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum QuotesKind {
    Ticker(Exchange, TradeKind, String),
    TickToTrade(Exchange, TradeKind, String),
    Bar(Exchange, TradeKind, String),
    Depth(Exchange, TradeKind, String), //
    PriceIndex(Exchange, String),       //指数
}

impl QuotesKind {
    #[inline]
    pub fn exchange(&self) -> Exchange {
        match self {
            QuotesKind::Bar(ex, ..) => ex.clone(),
            QuotesKind::Ticker(ex, ..) => ex.clone(),
            QuotesKind::TickToTrade(ex, ..) => ex.clone(),
            // QuotesKind::TickToMaker(ex, ..) => ex,
            QuotesKind::Depth(ex, ..) => ex.clone(),
            QuotesKind::PriceIndex(ex, ..) => ex.clone(),
        }
    }
    #[inline]
    pub fn trade_kind(&self) -> TradeKind {
        match self {
            QuotesKind::Bar(_, tk, _) => tk.clone(),
            QuotesKind::Ticker(_, tk, _) => tk.clone(),
            QuotesKind::TickToTrade(_, tk, _) => tk.clone(),
            // QuotesKind::TickToMaker(_, tk, _) => tk,
            QuotesKind::Depth(_, tk, _) => tk.clone(),
            _ => TradeKind::Unknown,
        }
    }
    #[inline]
    pub fn symbol(&self) -> String {
        match self {
            QuotesKind::Bar(_, _, symbol) => symbol,
            QuotesKind::Ticker(_, _, symbol) => symbol,
            QuotesKind::TickToTrade(_, _, symbol) => symbol,
            //QuotesKind::TickToMaker(_, _, symbol) => symbol.to_string(),
            QuotesKind::Depth(_, _, symbol) => symbol,
            QuotesKind::PriceIndex(_, symbol) => symbol,
        }
        .into()
    }

    pub fn to_name(&self) -> String {
        match self {
            // 币安
            QuotesKind::Bar(Exchange::BINANCE, _, symbol) => {
                format!("{}@kline_1m", symbol.replace("/", "").to_lowercase())
            }
            QuotesKind::Ticker(Exchange::BINANCE, _, symbol) => {
                format!("{}@ticker", symbol.replace("/", "").to_lowercase())
            }
            QuotesKind::TickToTrade(Exchange::BINANCE, _, symbol) => {
                format!("{}@aggTrade", symbol.replace("/", "").to_lowercase())
            }
            QuotesKind::Depth(Exchange::BINANCE, _, symbol) => {
                format!("{}@depth20", symbol.replace("/", "").to_lowercase())
            }
            // 火币
            QuotesKind::Bar(Exchange::HUOBI, _, symbol) => format!(
                "market.{}.kline.1min",
                symbol.replace("/", "").to_lowercase()
            ),
            QuotesKind::Ticker(Exchange::HUOBI, _, symbol) => {
                format!("market.{}.bbo", symbol.replace("/", "").to_lowercase())
            }
            QuotesKind::TickToTrade(Exchange::HUOBI, _, symbol) => format!(
                "market.{}.trade.detail",
                symbol.replace("/", "").to_lowercase()
            ),
            QuotesKind::Depth(Exchange::HUOBI, _, symbol) => format!(
                "market.{}.depth.step5",
                symbol.replace("/", "").to_lowercase()
            ),
            // OKEX
            QuotesKind::Bar(Exchange::OKEX, TradeKind::SPOT, symbol) => {
                format!("spot/candle60s:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Bar(Exchange::OKEX, TradeKind::SWAP, symbol) => format!(
                "swap/candle60s:{}-SWAP",
                symbol.replace("/", "-").to_uppercase()
            ),
            QuotesKind::Bar(Exchange::OKEX, TradeKind::OPTIONS, symbol) => format!(
                "option/candle60s:{}",
                symbol.replace("/", "-").to_uppercase()
            ),
            QuotesKind::Bar(Exchange::OKEX, TradeKind::FUTURES, symbol) => format!(
                "futures/candle60s:{}",
                symbol.replace("/", "-").to_uppercase()
            ),
            QuotesKind::Ticker(Exchange::OKEX, TradeKind::SPOT, symbol) => {
                format!("spot/ticker:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Ticker(Exchange::OKEX, TradeKind::SWAP, symbol) => {
                format!("swap/ticker:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Ticker(Exchange::OKEX, TradeKind::OPTIONS, symbol) => {
                format!("option/ticker:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Ticker(Exchange::OKEX, TradeKind::FUTURES, symbol) => {
                format!("futures/ticker:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::TickToTrade(Exchange::OKEX, TradeKind::SPOT, symbol) => {
                format!("spot/trade:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::TickToTrade(Exchange::OKEX, TradeKind::SWAP, symbol) => {
                format!("swap/trade:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::TickToTrade(Exchange::OKEX, TradeKind::OPTIONS, symbol) => {
                format!("option/trade:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::TickToTrade(Exchange::OKEX, TradeKind::FUTURES, symbol) => {
                format!("futures/trade:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Depth(Exchange::OKEX, TradeKind::SPOT, symbol) => {
                format!("spot/depth:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Depth(Exchange::OKEX, TradeKind::SWAP, symbol) => {
                format!("swap/depth:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Depth(Exchange::OKEX, TradeKind::OPTIONS, symbol) => {
                format!("option/depth:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::Depth(Exchange::OKEX, TradeKind::FUTURES, symbol) => {
                format!("futures/depth:{}", symbol.replace("/", "-").to_uppercase())
            }
            QuotesKind::PriceIndex(Exchange::OKEX, symbol) => {
                format!("index/ticker:{}", symbol.replace("/", "-").to_uppercase())
            }
            _ => "".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_uppercase();
        let v = s.splitn(4, ':').collect::<Vec<&str>>();
        if v.len() != 4 {
            return None;
        }
        // println!("{:#?}", v);
        let ex = Exchange::from(v[0]);
        let tk = TradeKind::from(v[1]);
        let symbol = v[3].to_string();
        match v[2] {
            "TICKER" => Some(QuotesKind::Ticker(ex, tk, symbol)),
            "BAR" => Some(QuotesKind::Bar(ex, tk, symbol)),
            "TTT" => Some(QuotesKind::TickToTrade(ex, tk, symbol)),
            "DEPTH" => Some(QuotesKind::Depth(ex, tk, symbol)),
            _ => None,
        }
    }
}

#[doc = "行情深度，价、量、委托笔数、委托额"]
pub type Depth = [f64; 4]; //[价,量,委托数,委托额]

#[doc = "行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Quotes {
    //逐笔委托
    TickToOffer(TickToOffer),
    //逐笔成交/快照
    TickToTrade(TickToTrade),
    //基本行情
    Level1(Level1),
    //深度行情
    Level2(Level2),
    //k线
    Bar(Bar),
}

impl ToString for Quotes {
    fn to_string(&self) -> String {
        if let Ok(s) = serde_json::to_string(&self) {
            s
        } else {
            "".to_string()
        }
    }
}
impl Quotes {
    pub fn kind(&self) -> Option<QuotesKind> {
        match self {
            Quotes::Level1(level) => Some(level.channel.clone()),
            Quotes::Level2(level) => Some(level.channel.clone()),
            Quotes::Bar(bar) => Some(bar.channel.clone().unwrap()),
            Quotes::TickToTrade(trade) => Some(trade.channel.clone()),
            _ => None,
        }
    }
}

#[doc = "逐笔委托"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickToOffer {
    // exchange: Exchange,
    // symbol: &'static str,
    channel: QuotesKind,
    time: i64,
    side: Side,
    price: f64,
    quantity: f64,
    bids: Option<Vec<Depth>>,
    asks: Option<Vec<Depth>>,
}

#[doc = "逐笔成交"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TickToTrade {
    pub channel: QuotesKind,
    pub id: String,
    pub time: i64,                     //时间
    pub price: f64,                    //价
    pub quantity: f64,                 //量
    pub order_side: Option<Side>,      //订单方向
    pub into_side: Option<Side>,       //主动（taker）成交方向
    pub take_order_id: Option<String>, //买单ID
    pub make_order_id: Option<String>, //卖单ID
}

#[doc = "基本行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level1 {
    pub channel: QuotesKind,
    pub time: i64,
    pub open: f64,          //开盘价
    pub high: f64,          //最高价
    pub low: f64,           //最低价
    pub close: f64,         //收盘价
    pub bids: Vec<Depth>,   //出价
    pub asks: Vec<Depth>,   //要价
    pub average: f64,       //均价
    pub last: f64,          //最新价
    pub last_quantity: f64, //最新成交量
    pub volume: f64,        //24小时成交量
    pub turnover: f64,      //24小时最新成交额
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Bar {
    #[serde(skip)]
    pub channel: Option<QuotesKind>,
    #[serde(rename = "Date")]
    pub time: String,
    #[serde(rename = "Open")]
    pub open: f64, //开盘价
    #[serde(rename = "Close")]
    pub close: f64, //收盘价
    #[serde(rename = "High")]
    pub high: f64, //最高价
    #[serde(rename = "Low")]
    pub low: f64, //最低价
    #[serde(rename = "Volume")]
    pub volume: f64, //成交量
    #[serde(rename = "Tunrnover")]
    pub turnover: Option<f64>, //成交额
}

impl Open for Bar {
    #[inline]
    fn open(&self) -> f64 {
        self.open
    }
}

impl Close for Bar {
    #[inline]
    fn close(&self) -> f64 {
        self.close
    }
}

impl Low for Bar {
    #[inline]
    fn low(&self) -> f64 {
        self.low
    }
}

impl High for Bar {
    #[inline]
    fn high(&self) -> f64 {
        self.high
    }
}

impl Volume for Bar {
    #[inline]
    fn volume(&self) -> f64 {
        self.volume
    }
}

#[doc = "深度行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Level2 {
    pub channel: QuotesKind,
    pub time: i64, //时间
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
    #[serde(skip)]
    pub raw: Option<Vec<u8>>,
}

#[doc = "订单簿"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopOfOrderBook {
    pub exchange: Exchange,
    pub symbol: String,
    pub bids: Vec<Depth>,
    pub asks: Vec<Depth>,
}
