use super::{EventBus, Token, Topic};
use ahash::AHashMap;
use anyhow::Result;
use parking_lot::RwLock;
use rayon::prelude::*;
use std::{any::Any, sync::Arc};

#[derive(Clone)]
pub struct LocalBus<T> {
    subscriber: Arc<RwLock<AHashMap<Topic, Vec<(Token, Box<dyn Fn(Topic, T) + Send + Sync>)>>>>,
}

impl<T> LocalBus<T> {
    pub fn new() -> Self {
        let subscriber = Arc::new(RwLock::new(AHashMap::new()));
        Self { subscriber }
    }
}

impl<T: Send + Sync + Clone> EventBus for LocalBus<T> {
    type Message = T;

    fn publish(&self, topic: Topic, msg: T) -> Result<()> {
        let guard = self.subscriber.read();
        if let Some(list) = guard.get(topic) {
            list.par_iter().for_each(|(_, f)| f(topic, msg.clone()));
        }
        Ok(())
    }

    fn subscribe(
        &self,
        topic: Topic,
        f: impl Fn(Topic, T) + Send + Sync + 'static,
    ) -> Result<Token> {
        let tid = f.type_id();
        let token = Token {
            topic,
            id: format!("{:?}", tid),
        };
        let mut guard = self.subscriber.write();
        if let Some(list) = guard.get_mut(topic) {
            list.push((token.clone(), Box::new(f)));
        } else {
            let bf = Box::new(f) as Box<dyn Fn(Topic, T) + Send + Sync>;
            let list = vec![(token.clone(), bf)];
            guard.insert(topic, list);
        }
        Ok(token)
    }

    fn unsubscribe(&self, token: &Token) {
        let mut guard = self.subscriber.write();
        if let Some(list) = guard.get_mut(token.topic) {
            if let Some(idx) = list.iter().position(|(item, _)| item.id == token.id) {
                let _ = list.remove(idx);
            }
        }
    }
}

mod tests {
    use super::super::EventBus;
    use super::LocalBus;
    use std::time::Duration;
    use url::Url;

    #[derive(Debug, Clone)]
    enum Message {
        Offer(String),
        Trade(String),
        Quote(String),
    }

    #[test]
    fn test() {
        let bus = LocalBus::<Message>::new();

        let token = bus
            .subscribe("/ctp/abc", |topic, ev| {
                println!("on {:?} {:?} {:?}", std::thread::current().id(), topic, ev)
            })
            .unwrap();
        println!("subscribe {:?}", token);
        let token = bus
            .subscribe("/ctp/abc", |topic, ev| {
                println!("on {:?} {:?} {:?}", std::thread::current().id(), topic, ev)
            })
            .unwrap();
        println!("subscribe {:?}", token);
        let bus1 = bus.clone();
        drop(bus);
        std::thread::spawn(move || {
            let mut n = 0;

            loop {
                bus1.publish("/ctp/abc", Message::Offer("".to_string()))
                    .ok();
                std::thread::sleep(Duration::from_millis(1000));
                n += 1;
                if n == 10 {
                    bus1.unsubscribe(&token);
                }
            }
        })
        .join()
        .ok();
    }
}
