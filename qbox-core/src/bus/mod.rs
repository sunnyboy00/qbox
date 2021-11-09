pub mod local;
pub mod shm;
pub mod tcp;
pub mod topics;
pub mod unix_socket;

use crate::broker::*;
use anyhow::Result;
use lazy_static::lazy_static;
use local::LocalBus;
use std::sync::Arc;
use url::Url;

lazy_static! {
    //消息总线
    static ref BUS:LocalBus<Arc<Event>>=LocalBus::new();
}

pub fn startup() -> Result<()> {
    broadcast(Event::Startup)
}

pub fn shutdown() -> Result<()> {
    broadcast(Event::Shutdown)
}

//广播消息
#[inline]
pub fn broadcast(msg: Event) -> Result<()> {
    BUS.publish(topics::BROADCAST, msg.arced())
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
    publish(topics::LOG.to_string(), Event::Log(msg))
}

#[inline]
pub fn trade_event(msg: TradeEvent) -> Result<()> {
    publish(topics::TRADES_EVENT, Event::Trade(msg))
}

#[inline]
pub fn quotes_event(msg: QuoteEvent) -> Result<()> {
    publish(topics::QUOTES_EVENT, Event::Quote(msg))
}

#[inline]
pub fn query_event(msg: TradeEvent) -> Result<()> {
    publish(topics::QUERY_EVENT, Event::Trade(msg))
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

#[derive(Debug, Clone)]
pub struct Token {
    pub topic: String,
    pub id: String,
}

pub trait EventBus {
    type Message;
    fn publish<TOPIC: AsRef<str>>(&self, topic: TOPIC, msg: Self::Message) -> Result<()>;
    fn subscribe<TOPIC: AsRef<str>>(
        &self,
        topic: TOPIC,
        f: impl Fn(&str, Self::Message) + Send + Sync + 'static,
    ) -> Result<Token>;
    fn unsubscribe(&self, token: &Token);
    fn call<TOPIC: AsRef<str>>(&self, topic: TOPIC, msg: Self::Message) -> Result<Self::Message>;
    fn register_fn<TOPIC: AsRef<str>>(
        &self,
        topic: TOPIC,
        f: impl Fn(&str, Self::Message) -> Result<Self::Message> + Send + Sync + 'static,
    ) -> Result<()>;
    fn unregister_fn<TOPIC: AsRef<str>>(&self, topic: TOPIC) -> Result<()>;
}
