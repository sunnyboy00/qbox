use crate::trade::*;
use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Data, Copy, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Exchange {
    SSE,       //上交所
    SZE,       //深交所
    SHFE,      //上期所
    DCE,       //大商所
    DZCE,      //郑商所
    CFFEX,     //中金所
    INE,       //能源中心
    OKEX,      //OKEX
    BINANCE,   //BINANCE
    HUOBI,     //HUOBI
    KRX,       //韩国交易所
    NSE,       //印度国家交易所
    EUREX,     //欧洲期货交易所(EUREX)
    CBOE,      //芝加哥期权交易所(CBOE)
    TAIFEX,    //台湾期货交易所(TAIFEX)
    TASE,      //以色列特拉维夫证交所(TASE)
    CME,       //芝加哥商业交易所(CME)
    OSAKE,     //大阪交易所(OSAKE)
    NYSELIFFE, //泛欧交易所(NYSE.LIFFE)
    HKFX,      //香港交易所（HKFX）
}

impl Default for Exchange {
    fn default() -> Self {
        Self::SHFE
    }
}

impl From<&str> for Exchange {
    fn from(s: &str) -> Self {
        let s = s.to_uppercase();
        match s.as_str() {
            "SSE" => Exchange::SSE,
            "SZE" => Exchange::SZE,
            "SHFE" => Exchange::SHFE,
            "DCE" => Exchange::DCE,
            "DZCE" => Exchange::DZCE,
            "CFFEX" => Exchange::CFFEX,
            "INE" => Exchange::INE,
            "OKEX" => Exchange::OKEX,
            "BINANCE" => Exchange::BINANCE,
            "HUOBI" => Exchange::HUOBI,
            "KRX" => Exchange::KRX,
            "NSE" => Exchange::NSE,
            "EUREX" => Exchange::EUREX,
            "CBOE" => Exchange::CBOE,
            "TAIFEX" => Exchange::TAIFEX,
            "TASE" => Exchange::TASE,
            "CME" => Exchange::CME,
            "OSAKE" => Exchange::OSAKE,
            "NYSE.LIFFE" => Exchange::NYSELIFFE,
            "HKFX" => Exchange::HKFX,
            _ => panic!("unknown exchange {}", s),
        }
    }
}

impl<'a> Into<&'a str> for Exchange {
    fn into(self) -> &'a str {
        match self {
            Exchange::SSE => "SSE",
            Exchange::SZE => "SZE",
            Exchange::SHFE => "SHFE",
            Exchange::DCE => "DCE",
            Exchange::DZCE => "DZCE",
            Exchange::CFFEX => "CFFEX",
            Exchange::INE => "INE",
            Exchange::OKEX => "OKEX",
            Exchange::BINANCE => "BINANCE",
            Exchange::HUOBI => "HUOBI",
            Exchange::KRX => "KRX",
            Exchange::NSE => "NSE",
            Exchange::EUREX => "EUREX",
            Exchange::CBOE => "CBOE",
            Exchange::TAIFEX => "TAIFEX",
            Exchange::TASE => "TASE",
            Exchange::CME => "CME",
            Exchange::OSAKE => "OSAKE",
            Exchange::NYSELIFFE => "NYSE.LIFFE",
            Exchange::HKFX => "HKFX",
        }
    }
}

impl Into<String> for Exchange {
    fn into(self) -> String {
        match self {
            Exchange::SSE => "SSE",
            Exchange::SZE => "SZE",
            Exchange::SHFE => "SHFE",
            Exchange::DCE => "DCE",
            Exchange::DZCE => "DZCE",
            Exchange::CFFEX => "CFFEX",
            Exchange::INE => "INE",
            Exchange::OKEX => "OKEX",
            Exchange::BINANCE => "BINANCE",
            Exchange::HUOBI => "HUOBI",
            Exchange::KRX => "KRX",
            Exchange::NSE => "NSE",
            Exchange::EUREX => "EUREX",
            Exchange::CBOE => "CBOE",
            Exchange::TAIFEX => "TAIFEX",
            Exchange::TASE => "TASE",
            Exchange::CME => "CME",
            Exchange::OSAKE => "OSAKE",
            Exchange::NYSELIFFE => "NYSE.LIFFE",
            Exchange::HKFX => "HKFX",
        }
        .into()
    }
}
