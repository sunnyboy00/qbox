pub mod local;

use anyhow::Result;

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
