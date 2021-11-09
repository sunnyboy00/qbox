use super::topics::*;
use crate::broker::*;
use crate::bus::local::LocalBus;
use crate::bus::{EventBus, Token};
use anyhow::Result;
use lazy_static::lazy_static;
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

#[derive(Debug)]
pub enum Event {
    //启动
    Startup,
    //关机
    Shutdown,
    StartQuoter(Url),
    StopQuoter(Url),
    StartTrader(Url),
    StopTrader(Url),
    Trade(TradeEvent),
    Quote(QuoteEvent),
    Log(String),
}

impl Event {
    pub fn arced(self) -> Arc<Self> {
        Arc::new(self)
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
