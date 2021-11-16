use super::topics::*;
use crate::broker::*;
use crate::bus::local::LocalBus;
use crate::bus::{EventBus, Token};
use anyhow::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use url::Url;

lazy_static! {
    //消息总线
    static ref BUS:LocalBus<Arc<Event>>=LocalBus::new();
}

//广播消息
#[inline]
pub fn broadcast(msg: Event) -> Result<()> {
    BUS.publish(BROADCAST, msg.arced())
}
#[inline]
pub fn call<S: AsRef<str>>(topic: S, msg: Event) -> Result<Arc<Event>> {
    log::trace!("call {} {:?}", topic.as_ref(), msg);
    BUS.call(topic, msg.arced())
}
//发布消息
#[inline]
pub fn publish<S: AsRef<str>>(topic: S, msg: Event) -> Result<()> {
    log::trace!("publish {} {:?}", topic.as_ref(), msg);
    BUS.publish(topic, msg.arced())
}

//订阅消息
#[inline]
pub fn subscribe<S: AsRef<str>>(
    topic: S,
    f: impl Fn(&str, Arc<Event>) + Send + Sync + 'static,
) -> Result<Token> {
    log::trace!("subscribe {}", topic.as_ref());
    BUS.subscribe(topic, f)
}

#[inline]
pub fn unsubscribe(token: &Token) {
    log::trace!("unsubscribe {:?}", token);
    BUS.unsubscribe(token)
}

#[inline]
pub fn log(msg: String) -> Result<()> {
    publish(LOG.to_string(), Event::Log(msg))
}

#[inline]
pub fn trade_event(msg: TradeEvent) -> Result<()> {
    publish(TRADES_EVENT, Event::Trade(msg))
}

#[inline]
pub fn quotes_event(msg: QuoteEvent) -> Result<()> {
    publish(QUOTES_EVENT, Event::Quote(msg))
}

#[inline]
pub fn query_event(msg: TradeEvent) -> Result<()> {
    publish(QUERY_EVENT, Event::Trade(msg))
}

#[doc = "交易事件"]
#[derive(Debug, Deserialize, Serialize)]
pub enum TradeEvent {
    Offer(Order),
    OfferResponse(Order),
    Cancel(Order),
    CancelResponse(Order),
    //QueryPosition(String),
    PositionResponse(Position),
    // QueryInstrument(Vec<String>),
    InstrumentsResponse(Instrument),
    TransactionNotify(Transaction),
}

#[doc = "行情"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum QuoteEvent {
    //订阅
    Subscribe(Vec<String>),
    //取消订阅
    Unsubscribe(Vec<String>),
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

impl ToString for QuoteEvent {
    fn to_string(&self) -> String {
        let qe = serde_json::from_slice::<QuoteEvent>(b"aaa").unwrap();
        if let Ok(s) = serde_json::to_string(&self) {
            s
        } else {
            "".to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Event {
    //启动
    Startup,
    //关机
    Shutdown,
    Log(String),
    StartQuoter(String),
    StopQuoter(String),
    StartTrader(String),
    StopTrader(String),

    Trade(TradeEvent),
    Quote(QuoteEvent),
}

impl Event {
    pub fn arced(self) -> Arc<Self> {
        Arc::new(self)
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
